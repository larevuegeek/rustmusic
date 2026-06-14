use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Une piste individuelle dans la file
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QueueTrack {
    pub queue_id: String,
    pub profil_id: i64,
    pub position: i32,
    pub path: String,
    pub title: String,
    pub artist: Option<String>,   // Option = NULL en SQLite / undefined en JS
    pub duration: Option<f64>,
    pub cover: Option<String>,
}