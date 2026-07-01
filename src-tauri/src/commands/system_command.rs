//! System-level Tauri commands (rendering mode, etc.).
//!
//! Currently exposes the WebKit/GDK render-mode override. Changing this
//! setting requires an app restart since env vars are read by WebKitGTK at
//! process init.

use serde::Serialize;
use tauri::State;

use crate::core::render_mode::RenderMode;
use crate::repository::settings::settings_repository::SettingsRepository;
use crate::state::AppState;

const KEY: &str = "render_mode";

#[derive(Debug, Serialize)]
pub struct RenderModeStatus {
    /// Current persisted setting (`"auto" | "force-gpu" | "force-software"`).
    pub mode: RenderMode,
    /// Detected virt kind (e.g. `"kvm"`) on Linux, or `null` on native / non-Linux.
    pub virt_kind: Option<String>,
}

fn detect_virt_kind() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        return crate::core::system_detect::detect_linux_virt();
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

#[tauri::command]
pub async fn get_render_mode(
    state: State<'_, AppState>,
) -> Result<RenderModeStatus, String> {
    let raw = SettingsRepository::get(&state.pool, KEY)
        .await
        .map_err(|e| format!("read {KEY}: {e}"))?;
    Ok(RenderModeStatus {
        mode: RenderMode::parse_or_auto(raw.as_deref()),
        virt_kind: detect_virt_kind(),
    })
}

#[tauri::command]
pub async fn set_render_mode(
    state: State<'_, AppState>,
    value: String,
) -> Result<RenderModeStatus, String> {
    let mode = RenderMode::parse_or_auto(Some(&value));
    SettingsRepository::set(&state.pool, KEY, mode.as_str())
        .await
        .map_err(|e| format!("save {KEY}: {e}"))?;
    log::info!("🖥  Render mode updated : {:?} (effective at next restart)", mode);
    Ok(RenderModeStatus {
        mode,
        virt_kind: detect_virt_kind(),
    })
}
