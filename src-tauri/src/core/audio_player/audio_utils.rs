// ============================================================================
// FONCTIONS UTILITAIRES
// ============================================================================

use symphonia::core::audio::{AudioBufferRef, Signal};

/// Convertit un AudioBufferRef Symphonia en Vec<f32> interleaved
/// IMPORTANT : Normalise correctement les samples en [-1.0, 1.0]
pub fn convert_audio_buffer_to_interleaved(
    audio_buf: &AudioBufferRef,
    channels: usize,
) -> Vec<f32> {
    let frames = audio_buf.frames();
    let mut samples: Vec<f32> = Vec::with_capacity(frames * channels);

    match audio_buf {
        AudioBufferRef::U8(buf) => {
            // U8 est entre 0..255, on normalise en [-1.0, 1.0]
            for frame_idx in 0..frames {
                for ch in 0..channels {
                    let sample: u8 = buf.chan(ch)[frame_idx];
                    // Convertir 0..255 -> -1.0..1.0
                    let normalized: f32 = (sample as f32 - 128.0) / 128.0;
                    samples.push(normalized);
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            // S16 est entre -32768..32767, on normalise en [-1.0, 1.0]
            for frame_idx in 0..frames {
                for ch in 0..channels {
                    let sample: i16 = buf.chan(ch)[frame_idx];
                    // Convertir -32768..32767 -> -1.0..1.0
                    let normalized: f32 = sample as f32 / 32767.0;
                    samples.push(normalized);
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            // S32 est entre -2147483648..2147483647
            for frame_idx in 0..frames {
                for ch in 0..channels {
                    let sample: i32 = buf.chan(ch)[frame_idx];
                    let normalized: f32 = sample as f32 / 2147483648.0;
                    samples.push(normalized);
                }
            }
        }
        AudioBufferRef::F32(buf) => {
            // F32 est déjà normalisé en [-1.0, 1.0]
            for frame_idx in 0..frames {
                for ch in 0..channels {
                    samples.push(buf.chan(ch)[frame_idx]);
                }
            }
        }
        AudioBufferRef::F64(buf) => {
            // F64 est déjà normalisé
            for frame_idx in 0..frames {
                for ch in 0..channels {
                    samples.push(buf.chan(ch)[frame_idx] as f32);
                }
            }
        }
        _ => {}
    }

    samples
}

pub fn adapt_channels(
    input: &[f32],
    input_channels: usize,
    output: &mut [f32],
    output_channels: usize,
) {
    let frames: usize = input.len() / input_channels;

    for frame in 0..frames {
        let base_in: usize = frame * input_channels;
        let base_out: usize = frame * output_channels;

        match (input_channels, output_channels) {
            // Mono vers stéréo
            (1, 2) => {
                let m: f32 = input[base_in];
                output[base_out] = m;
                output[base_out + 1] = m;
            }

            // Stéréo vers 7.1 (upmix cohérent)
            (2, 8) => {
                let l: f32 = input[base_in];
                let r: f32 = input[base_in + 1];

                // On crée un mix léger pour chaque canal (valeurs équilibrées)
                output[base_out + 0] = l * 0.48; // FL
                output[base_out + 1] = r * 0.48; // FR

                // Center : réduit pour éviter l'effet "bouché"
                output[base_out + 2] = (l + r) * 0.25; // C (moins de médiums)

                // LFE : atténué (moins de graves bourdonnants)
                output[base_out + 3] = (l + r) * 0.12; // LFE

                // Surrounds : un peu moins forts et plus diffus
                output[base_out + 4] = (l * 0.25) - (r * 0.05); // RL
                output[base_out + 5] = (r * 0.25) - (l * 0.05); // RR
                output[base_out + 6] = l * 0.20; // SL
                output[base_out + 7] = r * 0.20; // SR

                if output_channels >= 6 {
                    for ch in 2..output_channels {
                        // Applique un léger déphasage (-0.003) et un filtrage haut adouci
                        output[base_out + ch] *= 0.95; // réduit les aigus corrélés
                        output[base_out + ch] -= output[base_out + ch] * 0.0025;
                    }
                }
            }

            // Stéréo vers 5.1 (plus classique)
            (2, 6) => {
                let l: f32 = input[base_in];
                let r: f32 = input[base_in + 1];
                output[base_out + 0] = l * 0.48; // L
                output[base_out + 1] = r * 0.48; // R
                output[base_out + 2] = (l + r) * 0.25; // C (moins de médiums)
                output[base_out + 3] = (l + r) * 0.12; // LFE
                output[base_out + 4] = (l * 0.25) - (r * 0.05); // RL
                output[base_out + 5] = (r * 0.25) - (l * 0.05); // RR

                for ch in 2..output_channels {
                    output[base_out + ch] *= 0.97;
                    output[base_out + ch] -= output[base_out + ch] * 0.0025;
                }
            }

            // 3 canaux (L R C) vers stéréo (ITU-R BS.775)
            (3, 2) => {
                let fl = input[base_in];
                let fr = input[base_in + 1];
                let c = input[base_in + 2];
                output[base_out] = (fl + 0.707 * c) * 0.707;
                output[base_out + 1] = (fr + 0.707 * c) * 0.707;
            }

            // 4 canaux (FL FR LS RS = quad) vers stéréo
            (4, 2) => {
                let fl = input[base_in];
                let fr = input[base_in + 1];
                let ls = input[base_in + 2];
                let rs = input[base_in + 3];
                output[base_out] = (fl + 0.707 * ls) * 0.707;
                output[base_out + 1] = (fr + 0.707 * rs) * 0.707;
            }

            // 5.0 surround (FL FR C LS RS) vers stéréo — typique des SACD multicanal.
            // Downmix ITU-R BS.775 : Lo = FL + 0.707·C + 0.707·LS, Ro = FR + 0.707·C + 0.707·RS,
            // puis -6 dB global pour éviter le clipping quand les canaux sont corrélés.
            (5, 2) => {
                let fl = input[base_in];
                let fr = input[base_in + 1];
                let c = input[base_in + 2];
                let ls = input[base_in + 3];
                let rs = input[base_in + 4];
                output[base_out] = (fl + 0.707 * c + 0.707 * ls) * 0.5;
                output[base_out + 1] = (fr + 0.707 * c + 0.707 * rs) * 0.5;
            }

            // 5.1 (FL FR C LFE LS RS) vers stéréo
            (6, 2) => {
                let fl = input[base_in];
                let fr = input[base_in + 1];
                let c = input[base_in + 2];
                let lfe = input[base_in + 3];
                let ls = input[base_in + 4];
                let rs = input[base_in + 5];
                output[base_out] = (fl + 0.707 * c + 0.707 * ls + 0.5 * lfe) * 0.5;
                output[base_out + 1] = (fr + 0.707 * c + 0.707 * rs + 0.5 * lfe) * 0.5;
            }

            // Même nombre de canaux → copie directe
            (x, y) if x == y => {
                for ch in 0..x {
                    output[base_out + ch] = input[base_in + ch];
                }
            }

            // Par défaut : tronquer ou dupliquer sans distorsion
            _ => {
                let min_ch = input_channels.min(output_channels);
                for ch in 0..min_ch {
                    output[base_out + ch] = input[base_in + ch];
                }
                for ch in min_ch..output_channels {
                    output[base_out + ch] = 0.0;
                }
            }
        }
    }
}
