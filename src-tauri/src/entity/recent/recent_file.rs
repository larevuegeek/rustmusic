
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecentFile {
    pub id: i64,
    pub library_id: Option<i64>,
    pub path: String,
    pub last_played_at: DateTime<Utc>,
    pub last_position: i64,
    pub play_count: i64,
}