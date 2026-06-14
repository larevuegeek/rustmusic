use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Playlist {
    pub id: i64,
    pub profil_id: i64,
    pub library_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub icon: String,
    pub cover: Option<String>,
    pub track_count: i64,
    pub duration: i64, // en secondes
    pub position: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
}
