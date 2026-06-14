//! Tauri commands for the audio quality profile (settings UI).
//!
//! Persists the user's choice in the `settings` table under
//! `audio_quality_profile` (`"auto" | "high" | "medium" | "low"`), and keeps
//! the in-memory resolved profile up to date so player threads pick changes
//! up on the next track.

use serde::Serialize;
use tauri::State;

use crate::core::audio_quality::{
    self, AudioQualityProfile, AudioQualitySetting,
};
use crate::core::system_detect;
use crate::repository::settings::settings_repository::SettingsRepository;
use crate::state::AppState;

const KEY: &str = "audio_quality_profile";

/// Status payload returned to the frontend. Bundles everything the UI needs
/// in one round-trip.
#[derive(Debug, Serialize)]
pub struct AudioQualityStatus {
    /// User-chosen setting (auto / high / medium / low).
    pub setting: AudioQualitySetting,
    /// Concrete profile currently driving the pipeline.
    pub resolved: AudioQualityProfile,
    /// `Some("kvm" | "vmware" | ...)` when the host is virtualised; `None` on bare metal.
    pub virt_kind: Option<String>,
    /// Logical core count detected on the host.
    pub cpu_cores: usize,
}

fn detect_virt_kind() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        return system_detect::detect_linux_virt();
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Read the persisted setting, resolve it, and apply it to the global
/// in-memory profile. Called once at app boot from `run()`.
pub async fn init_from_settings(state: &AppState) {
    let raw = SettingsRepository::get(&state.pool, KEY)
        .await
        .ok()
        .flatten();
    let setting = AudioQualitySetting::parse_or_auto(raw.as_deref());
    let resolved = setting.resolve();
    audio_quality::set_current_profile(resolved);
    log::info!(
        "🎚  Audio quality : setting={:?}, resolved={:?}",
        setting, resolved
    );
}

#[tauri::command]
pub async fn get_audio_quality_status(
    state: State<'_, AppState>,
) -> Result<AudioQualityStatus, String> {
    let raw = SettingsRepository::get(&state.pool, KEY)
        .await
        .map_err(|e| format!("read {KEY}: {e}"))?;
    let setting = AudioQualitySetting::parse_or_auto(raw.as_deref());
    let resolved = setting.resolve();
    Ok(AudioQualityStatus {
        setting,
        resolved,
        virt_kind: detect_virt_kind(),
        cpu_cores: system_detect::logical_cpu_count(),
    })
}

#[tauri::command]
pub async fn set_audio_quality_setting(
    state: State<'_, AppState>,
    value: String,
) -> Result<AudioQualityStatus, String> {
    let setting = AudioQualitySetting::parse_or_auto(Some(&value));
    SettingsRepository::set(&state.pool, KEY, setting.as_str())
        .await
        .map_err(|e| format!("save {KEY}: {e}"))?;

    let resolved = setting.resolve();
    audio_quality::set_current_profile(resolved);
    log::info!(
        "🎚  Audio quality updated : setting={:?}, resolved={:?}",
        setting, resolved
    );

    Ok(AudioQualityStatus {
        setting,
        resolved,
        virt_kind: detect_virt_kind(),
        cpu_cores: system_detect::logical_cpu_count(),
    })
}
