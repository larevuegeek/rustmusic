//! Description of the running playback pipeline, emitted to the frontend
//! at the start of every track. Lets the UI display the source vs. output
//! signal (e.g. "DSD64 → 88.2 kHz") in the player status bar.

use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// Sent as a Tauri event named `"playback-pipeline"` whenever a new track
/// starts decoding.
#[derive(Debug, Clone, Serialize)]
pub struct PlaybackPipelineInfo {
    // ── Source (file on disk) ──
    /// Friendly format label : `"DSD64"`, `"DSD128"`, `"FLAC"`, `"MP3"`, ...
    pub source_format: String,
    /// Native sample rate of the encoded file (`2_822_400` for DSD64,
    /// `44_100` / `96_000` / ... for PCM).
    pub source_sample_rate: u32,
    /// Native bit depth (`1` for DSD, `16` / `24` / `32` for PCM).
    /// Lossy formats (MP3, AAC, ...) report their decoded depth (`16` typ.).
    pub source_bits: u32,
    pub source_channels: u8,

    // ── Intermediate (after DSD2PCM, before resampler) ──
    /// Only set on DSD playback : rate of the PCM stream emitted by
    /// `DsdToPcmConverter`. `None` for PCM sources (no intermediate).
    pub intermediate_pcm_rate: Option<u32>,
    /// Filter taps used by the DSD2PCM converter (`Some(2048)`, `1024`,
    /// `512` according to the active quality profile). `None` for PCM sources.
    pub dsd_filter_taps: Option<u32>,
    /// DSD decimation factor (`dsd_rate / intermediate_pcm_rate`, e.g. 32
    /// for DSD64 → 88.2 kHz). `None` for PCM sources.
    pub dsd_decimation: Option<u32>,

    // ── Output (delivered to CPAL) ──
    /// Device sample rate (after resampler, if any).
    pub output_sample_rate: u32,
    pub output_channels: u8,
    /// Human-readable name of the active output device (`"Realtek HD Audio"`,
    /// `"USB DAC"`, ...). Truncated client-side for display in the status bar.
    pub device_name: String,

    // ── Pipeline state ──
    /// `true` when a resampler is active in the chain (source rate ≠ device rate).
    pub resampler_active: bool,
    /// Active quality profile : `"high"` / `"medium"` / `"low"`.
    pub quality_profile: String,
}

impl PlaybackPipelineInfo {
    /// Emit on the `"playback-pipeline"` channel. Errors are logged but
    /// never propagated — a missing UI status indicator is non-critical.
    pub fn emit(self, app: &AppHandle) {
        if let Err(e) = app.emit("playback-pipeline", &self) {
            log::warn!("playback-pipeline emit failed: {}", e);
        }
    }
}

/// Map a DSD raw rate (e.g. 2_822_400 Hz) to a human label "DSD64", "DSD128"...
pub fn dsd_label(rate: u32) -> String {
    if rate % 44_100 == 0 {
        format!("DSD{}", rate / 44_100)
    } else if rate % 48_000 == 0 {
        format!("DSD{} (48k base)", rate / 48_000)
    } else {
        format!("DSD {} Hz", rate)
    }
}

/// Best-effort label for a Symphonia codec id.
pub fn symphonia_format_label(codec: symphonia::core::codecs::CodecType) -> &'static str {
    use symphonia::core::codecs;
    match codec {
        c if c == codecs::CODEC_TYPE_MP3 => "MP3",
        c if c == codecs::CODEC_TYPE_FLAC => "FLAC",
        c if c == codecs::CODEC_TYPE_VORBIS => "OGG Vorbis",
        c if c == codecs::CODEC_TYPE_OPUS => "Opus",
        c if c == codecs::CODEC_TYPE_AAC => "AAC",
        c if c == codecs::CODEC_TYPE_PCM_S16LE
            || c == codecs::CODEC_TYPE_PCM_S24LE
            || c == codecs::CODEC_TYPE_PCM_F32LE =>
        {
            "WAV (PCM)"
        }
        _ => "PCM",
    }
}
