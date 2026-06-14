import type { AttachedImage } from "./AttachedImage";

export interface AudioTags {
    id3_version?: string;

    // ID3v1, ID3v2 et autres formats
    title?: string;
    artist?: string;
    album?: string;
    year?: string;
    comment?: string;
    track_number?: number;
    genre?: string;

    // Extensions pour ID3v2 et autres formats
    album_artist?: string;
    composer?: string;
    original_artist?: string;
    part_of_set?: string;
    publisher?: string;
    encoded_by?: string;
    encoding_settings?: string;
    bpm?: string;
    duration?: number; // Durée en secondes
    language?: string;
    media_type?: string;
    file_type?: string;

    // Droit d'auteur et licence
    copyright?: string;
    internet_radio_station_name?: string;
    internet_radio_station_owner?: string;

    // Performance, enregistrement et musiciens
    conductor?: string;
    lyricist?: string;
    remix_artist?: string;
    arranged_by?: string;
    interpreted_by?: string; // ou 'performer'
    
    // Informations supplémentaires
    mood?: string;
    isrc?: string; // International Standard Recording Code
    disc_number?: number;
    total_discs?: number;
    compilation?: string; // Indicateur de compilation
    subtitle?: string;
    key?: string; // Clé musicale

    // Paroles et notation
    lyrics?: string;
    unsynchronised_lyrics?: string; // Paroles non synchronisées

    // URLs et identifiants uniques
    official_audio_source_url?: string;
    official_audio_file_url?: string;
    official_artist_url?: string;
    payment_url?: string;
    publisher_url?: string;

    // Tags définis par l'utilisateur et divers
    custom_tags?: Array<{ key: string, value: string }>; // Paires clé-valeur pour les tags définis par l'utilisateur

    // Artwork de couverture et autres images
    attached_images?: Array<AttachedImage>;
}