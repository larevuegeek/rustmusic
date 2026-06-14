use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::core::audio_metadata::extractor::extractor as audio_metadata_extractor;
use crate::entity::audio::audio_file::{AudioFile, AudioFormat};
use crate::entity::audio::audio_tags::{AttachedImage, AudioTags, ImageType};
use crate::helper::string::string::normalize_year;
use base64::engine::general_purpose;
use base64::Engine;
use symphonia::core::codecs::{self, CodecType};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{
    MetadataOptions, MetadataRevision, StandardTagKey, StandardVisualKey, Visual,
};
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::default::get_probe;

pub struct AudioAnalyser;

impl AudioAnalyser {
    pub fn analyse_audio_file(
        file_path: &PathBuf,
    ) -> Result<AudioFile, Box<dyn std::error::Error>> {
        // Court-circuit pour les formats gérés par notre extractor maison
        // (Symphonia ne sait pas lire DSD, DSF ni DFF).
        //
        // On enveloppe l'extracteur dans `catch_unwind` pour garantir qu'aucun
        // panic (slice out-of-bounds sur un fichier tronqué/corrompu, etc.) ne
        // remonte au scanner : à l'échelle d'un batch rayon, un panic dans
        // un worker thread propage et tue tout le batch. Une erreur normale
        // (`ExtractError`) est déjà gérée par-fichier en amont.
        if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_ascii_lowercase();
            if ext_lower == "dsf" || ext_lower == "dff" {
                let path = file_path.clone();
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    audio_metadata_extractor::extract(&path)
                }));
                return match result {
                    Ok(Ok(audio)) => Ok(audio),
                    Ok(Err(e)) => Err(Box::new(e) as Box<dyn std::error::Error>),
                    Err(_) => Err(format!(
                        "panic while parsing DSD file {:?} (corrupted or truncated)",
                        file_path
                    )
                    .into()),
                };
            }
        }

        //Indice permet de deviner plus facilement le format notamment grâce à l'extension du fichier
        let mut hint: Hint = Hint::new();
        if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        //On ouvre le fichier dans une box
        let source: Box<File> = Box::new(File::open(&file_path)?);

        //Enveloppe le fichier dans un flux média lisible par Symphonia (buffering, seeking, etc.).
        let mss: MediaSourceStream = MediaSourceStream::new(source, Default::default());

        //Options pour l’analyse du conteneur (format) et des métadonnées. Valeurs par défaut
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();

        //Sonde qui permet à symphonia de récupérer le format du fichier audio
        let mut probed: ProbeResult =
            get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
        let mut format: Box<dyn FormatReader> = probed.format;
        // Récupérer les infos de la première piste audio trouvé dans le fichier (qui possède un SampleRate)
        let (track_id, codec_params) = {
            let track = format
                .tracks()
                .iter()
                .find(|t| t.codec_params.sample_rate.is_some())
                .ok_or("Aucune piste audio trouvée")?;

            (track.id, track.codec_params.clone())
        };

        let bits_per_sample: u32 = codec_params.bits_per_sample.unwrap_or(0) as u32;
        let source_sample_rate: u32 = codec_params.sample_rate.ok_or("Pas de sample rate")?;
        let channels: usize = codec_params.channels.ok_or("Pas d'info channels")?.count();
        let codec_params: symphonia::core::codecs::CodecParameters = codec_params.clone();

        let mut duration_sec: Option<f64> = None;

        // Calculer la durée si les infos sont disponibles
        // 1️⃣ Méthode 1 : via n_frames (si dispo)
        if let Some(n_frames) = codec_params.n_frames {
            if let Some(tb) = codec_params.time_base {
                let time: symphonia::core::units::Time = tb.calc_time(n_frames);
                duration_sec = Some(time.seconds as f64 + time.frac);
            }
        }

        // 2️⃣ Méthode 2 : via total_samples (souvent pour FLAC)
        if duration_sec.is_none() {
            if let Some(sr) = codec_params.sample_rate {
                if let Some(total_samples) = codec_params.n_frames {
                    duration_sec = Some(total_samples as f64 / sr as f64);
                }
            }
        }

        let duration: f32 = duration_sec.unwrap_or(0.0) as f32;

        // Taille du fichier
        let file_size: u64 = std::fs::metadata(&file_path)?.len();

        let path_str: &str = file_path.to_str().ok_or("Chemin non-UTF8")?;

        let bitrate: u32 = match Self::read_audio_header_bitrate(path_str)? {
            Some(br) => br,
            None => {
                if duration > 0.0 {
                    (((file_size as f64 * 8.0) / duration as f64) / 1000.0).round() as u32
                } else {
                    0
                }
            }
        };

        let audio_format: AudioFormat = Self::map_codec_to_format(codec_params.codec);

        //////////////////////// Gestion des TAGS ////////////////////////////
        let mut tags: AudioTags = AudioTags::new();

        // 2. Métadonnées du probe (souvent là pour MP3)
        if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
            tags = Self::tags_reader(tags, metadata_rev);
        }

        // 🔹 3. Métadonnées spécifiques au format (FLAC, OGG, etc.)
        if let Some(meta) = format.metadata().current() {
            tags = Self::tags_reader(tags, meta);
        }

        // 3. Récupérer la vignette (cover art)
        //println!("\n=== Vignette ===\n");

        let mut cover_found: bool = false;

        // Chercher dans les métadonnées du probe
        if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
            for visual in metadata_rev.visuals() {
                cover_found = true;

                let attached_image: AttachedImage = Self::visual_tag_reader(&visual);

                //On injecte dans la struct Tags
                tags.attached_images.push(attached_image);
            }
        }
        
        // Chercher dans les métadonnées dans le fichier
        if let Some(metadata_rev) = format.metadata().current() {
            for visual in metadata_rev.visuals() {
                cover_found = true;

                let attached_image: AttachedImage = Self::visual_tag_reader(visual);
                tags.attached_images.push(attached_image);
            }
        }

        if !cover_found {
            // On check le cover.jpg ou folder.jpg présent dans le dossier du fichier
            if let Some(folder) = file_path.parent() {
                const COVER_NAMES: [&'static str; 5] =
                    ["cover", "folder", "Cover", "Folder", "front"];
                const COVER_EXTS: [&'static str; 5] = ["jpg", "jpeg", "png", "webp", "aviff"];

                if let Some(cover_filepath) = COVER_NAMES
                    .iter()
                    .flat_map(|name| {
                        COVER_EXTS
                            .iter()
                            .map(move |ext| folder.join(format!("{name}.{ext}")))
                    })
                    .find(|path| path.exists())
                {
                    if let Ok(attached_image) = Self::visual_file_reader(&cover_filepath) {
                        tags.attached_images.push(attached_image);
                    }
                }
            }

            //println!("Aucune vignette trouvée");
        }

        let audiofile: AudioFile = AudioFile {
            path: file_path.to_string_lossy().to_string(),
            audio_format,
            file_size,
            duration,
            bits_per_sample,
            bitrate,
            sample_rate: source_sample_rate,
            channels,
            track_id: Some(track_id),
            tags,
        };

        return Ok(audiofile);
    }

    fn map_codec_to_format(codec: CodecType) -> AudioFormat {
        if codec == codecs::CODEC_TYPE_MP3 {
            AudioFormat::MP3
        } else if codec == codecs::CODEC_TYPE_FLAC {
            AudioFormat::FLAC
        } else if codec == codecs::CODEC_TYPE_VORBIS || codec == codecs::CODEC_TYPE_OPUS {
            AudioFormat::OGG
        } else if codec == codecs::CODEC_TYPE_PCM_S16LE
            || codec == codecs::CODEC_TYPE_PCM_S24LE
            || codec == codecs::CODEC_TYPE_PCM_F32LE
        {
            AudioFormat::WAV
        } else {
            AudioFormat::Unknown
        }
    }

    fn tags_reader(mut tags: AudioTags, metadata_rev: &MetadataRevision) -> AudioTags {
        for tag in metadata_rev.tags() {
            if let Some(std_key) = &tag.std_key {
                //println!("key : {:?} Value : {:}", std_key, tag.value);

                match std_key {
                    StandardTagKey::TrackTitle => tags.title = Some(tag.value.to_string()),
                    StandardTagKey::Artist => {
                        // Vorbis/FLAC peut avoir plusieurs tags ARTIST séparés
                        // On les accumule avec ";" pour que split_artists() les sépare
                        let val = tag.value.to_string();
                        tags.artist = Some(match tags.artist.take() {
                            Some(existing) => format!("{}; {}", existing, val),
                            None => val,
                        });
                    }
                    StandardTagKey::Album => tags.album = Some(tag.value.to_string()),
                    StandardTagKey::Genre => tags.genre = Some(tag.value.to_string()),
                    StandardTagKey::Date => tags.year = normalize_year(Some(tag.value.to_string())),
                    StandardTagKey::TrackNumber => {
                        if let Ok(num) = tag.value.to_string().parse::<u16>() {
                            tags.track_number = Some(num);
                        }
                    }
                    StandardTagKey::Composer => tags.composer = Some(tag.value.to_string()),
                    StandardTagKey::Encoder => tags.encoded_by = Some(tag.value.to_string()),
                    StandardTagKey::Copyright => tags.copyright = Some(tag.value.to_string()),
                    StandardTagKey::AlbumArtist => {
                        let val = tag.value.to_string();
                        tags.album_artist = Some(match tags.album_artist.take() {
                            Some(existing) => format!("{}; {}", existing, val),
                            None => val,
                        });
                    }
                    StandardTagKey::Bpm => tags.bpm = Some(tag.value.to_string()),
                    StandardTagKey::DiscNumber => {
                        if let Ok(num) = tag.value.to_string().parse::<u16>() {
                            tags.disc_number = Some(num);
                        }
                    }
                    StandardTagKey::Comment => tags.comment = Some(tag.value.to_string()),
                    StandardTagKey::Rating => {
                        // POPM (ID3v2) : 0-255 → 0-5 étoiles
                        // Vorbis RATING : souvent 0-100 ou 0-5 directement
                        let val_str = tag.value.to_string();
                        if let Ok(num) = val_str.parse::<i32>() {
                            let stars = if num == 0 {
                                0
                            } else if num <= 5 {
                                num // déjà en 0-5
                            } else if num <= 100 {
                                (num + 19) / 20 // 0-100 → 0-5 (avec arrondi)
                            } else {
                                // Mapping POPM standard (0-255)
                                match num {
                                    1..=31 => 1,
                                    32..=95 => 2,
                                    96..=159 => 3,
                                    160..=223 => 4,
                                    _ => 5,
                                }
                            };
                            tags.rating = Some(stars);
                        }
                    }
                    _ => {
                        // Clé connue mais non encore mappée
                        tags.custom_tags
                            .push((format!("{:?}", std_key), tag.value.to_string()));
                    }
                }
            }
        }

        tags
    }

    fn visual_tag_reader(visual: &Visual) -> AttachedImage {
        let base64_cover: String = general_purpose::STANDARD.encode(&visual.data);

        let image_src: String = format!("data:{};base64,{}", visual.media_type, base64_cover);

        let image_type: ImageType = match visual.usage.clone() {
            Some(StandardVisualKey::FrontCover) => ImageType::CoverFront,
            Some(StandardVisualKey::BackCover) => ImageType::CoverBack,
            Some(StandardVisualKey::Leaflet) => ImageType::LeafletPage,
            Some(StandardVisualKey::Media) => ImageType::MediaLabel,
            Some(StandardVisualKey::ArtistPerformer) => ImageType::Artist,
            Some(StandardVisualKey::Conductor) => ImageType::Conductor,
            Some(StandardVisualKey::Composer) => ImageType::Composer,
            Some(StandardVisualKey::Lyricist) => ImageType::LyricistTextWriter,
            Some(StandardVisualKey::RecordingLocation) => ImageType::RecordingLocation,
            Some(StandardVisualKey::RecordingSession) => ImageType::DuringRecording,
            Some(StandardVisualKey::Performance) => ImageType::DuringPerformance,
            Some(StandardVisualKey::ScreenCapture) => ImageType::MovieVideoScreenCapture,
            Some(StandardVisualKey::Illustration) => ImageType::Illustration,
            Some(StandardVisualKey::PublisherStudioLogo) => ImageType::PublisherStudioLogo,
            _ => ImageType::Other,
        };

        let attached_image: AttachedImage = AttachedImage {
            image_type: Some(image_type),
            mime_type: visual.media_type.clone(),
            description: visual
                .tags
                .iter()
                .find(|t| t.key == "description")
                .map(|t| t.value.to_string()),
            image_data: visual.data.to_vec(),
            image_src,
        };

        attached_image
    }

    fn visual_file_reader(file: &PathBuf) -> Result<AttachedImage, std::io::Error> {
        let data: Vec<u8> = std::fs::read(file)?;
        let base64_cover: String = general_purpose::STANDARD.encode(&data);

        // Déterminer le type MIME à partir de l’extension
        let mime_type: String = match file.extension().and_then(|ext| ext.to_str()) {
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("webp") => "image/webp",
            Some("avif") | Some("aviff") => "image/avif",
            _ => "image/unknown",
        }
        .to_string();

        let image_src: String = format!("data:{};base64,{}", mime_type, base64_cover);

        let attached_image: AttachedImage = AttachedImage {
            image_type: Some(ImageType::CoverFront),
            mime_type: "image/".to_string(),
            description: None,
            image_data: data,
            image_src,
        };

        Ok(attached_image)
    }

    pub fn read_audio_header_bitrate(path: &str) -> io::Result<Option<u32>> {
        let mut file: File = File::open(path)?;

        // Lecture des 4 premiers octets pour identifier le format
        let mut signature = [0u8; 4];
        file.read_exact(&mut signature)?;
        file.seek(SeekFrom::Start(0))?;

        let is_ogg: bool = &signature == b"OggS";
        let is_mp3: bool = &signature[0..2] == b"ID" || signature[0] == 0xFF;

        // Si ce n’est ni OGG ni MP3, on sort directement
        if !is_ogg && !is_mp3 {
            return Ok(None);
        }

        // ====== OGG (Vorbis uniquement) ======
        if is_ogg {
            let mut header: Vec<u8> = vec![0u8; 128];
            file.read_exact(&mut header)?;

            // Vérifie la présence de "vorbis" dans le paquet d'identification
            if header.windows(6).any(|w| w == b"vorbis") {
                // Les bitrates sont stockés dans le header Vorbis : [min][nominal][max]
                if let Some(pos) = header.windows(6).position(|w| w == b"vorbis") {
                    if pos + 18 <= header.len() {
                        let nominal = u32::from_le_bytes([
                            header[pos + 10],
                            header[pos + 11],
                            header[pos + 12],
                            header[pos + 13],
                        ]);
                        if nominal > 0 {
                            return Ok(Some(nominal / 1000)); // convertit en kb/s
                        }
                    }
                }
            }

            return Ok(None);
        }

        // ====== MP3 ======
        let mut id3_header: [u8; 10] = [0u8; 10];
        file.read_exact(&mut id3_header)?;
        let mut start_offset = 0u64;

        // Saute un éventuel tag ID3v2
        if &id3_header[0..3] == b"ID3" {
            let size: u32 = ((id3_header[6] as u32 & 0x7F) << 21)
                | ((id3_header[7] as u32 & 0x7F) << 14)
                | ((id3_header[8] as u32 & 0x7F) << 7)
                | (id3_header[9] as u32 & 0x7F);
            start_offset = 10 + size as u64;
            file.seek(SeekFrom::Start(start_offset))?;
        } else {
            file.seek(SeekFrom::Start(0))?;
        }

        let mut buffer: [u8; 4] = [0u8; 4];
        while file.read_exact(&mut buffer).is_ok() {
            let header: u32 = u32::from_be_bytes(buffer);

            // Vérifie la frame sync (11 bits à 1)
            if (header >> 21) & 0x7FF != 0x7FF {
                start_offset += 1;
                file.seek(SeekFrom::Start(start_offset))?;
                continue;
            }

            // Extraction des champs MPEG
            let version_id: u32 = (header >> 19) & 0b11;
            let layer_index: u32 = (header >> 17) & 0b11;
            let bitrate_index: u32 = (header >> 12) & 0b1111;
            let sample_rate_index: u32 = (header >> 10) & 0b11;

            if version_id == 0b01
                || layer_index == 0b00
                || bitrate_index == 0b1111
                || sample_rate_index == 0b11
            {
                start_offset += 1;
                file.seek(SeekFrom::Start(start_offset))?;
                continue;
            }

            // Tables officielles MPEG (Layer III)
            const TABLE_MPEG1: [u32; 16] = [
                0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0,
            ];
            const TABLE_MPEG2: [u32; 16] = [
                0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0,
            ];

            let bitrate = match (version_id, layer_index) {
                (0b11, 0b01) => TABLE_MPEG1[bitrate_index as usize], // MPEG1 Layer III
                (0b10, 0b01) => TABLE_MPEG2[bitrate_index as usize], // MPEG2 Layer III
                (0b00, 0b01) => TABLE_MPEG2[bitrate_index as usize], // MPEG2.5 Layer III
                _ => 0,
            };

            if bitrate > 0 {
                return Ok(Some(bitrate));
            }

            start_offset += 1;
            file.seek(SeekFrom::Start(start_offset))?;
        }

        Ok(None)
    }
}
