//! User override for the WebKit / GTK rendering mode.
//!
//! By default (`Auto`), we detect a VM at startup and apply software
//! rendering env vars only there. But some users may want to force one
//! mode or the other :
//!   - `ForceGpu`      → trust the system, don't touch env vars (best perf
//!                       on native ; risky on KDE Wayland / weird stacks)
//!   - `ForceSoftware` → always apply the software-rendering env vars
//!                       (works everywhere ; less smooth animations)
//!
//! IMPORTANT : env vars are read by WebKitGTK at process init, so changing
//! this setting requires an app restart to take effect.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenderMode {
    /// Auto-detection : software rendering on VMs, GPU on native.
    Auto,
    /// Always use the GPU (no env vars set).
    ForceGpu,
    /// Always force software rendering (set all the WebKit/GDK env vars).
    ForceSoftware,
}

impl RenderMode {
    /// Parse the value persisted in the `settings` table (defaults to `Auto`).
    pub fn parse_or_auto(value: Option<&str>) -> Self {
        match value.map(str::to_ascii_lowercase).as_deref() {
            Some("force-gpu") | Some("gpu") => Self::ForceGpu,
            Some("force-software") | Some("software") => Self::ForceSoftware,
            _ => Self::Auto,
        }
    }

    /// Stable string form for persistence.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::ForceGpu => "force-gpu",
            Self::ForceSoftware => "force-software",
        }
    }
}
