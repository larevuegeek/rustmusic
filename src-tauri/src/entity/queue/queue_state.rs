use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// L'état global du lecteur
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")] // 🪄 La magie pour Svelte !
pub struct QueueState {
    pub profil_id: i64,
    pub current_index: i32, // -1 quand rien n'est lu
    pub is_shuffled: bool,
    pub repeat_mode: String, // "off", "one", "all"
}