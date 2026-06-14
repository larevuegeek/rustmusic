use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrackListView {
    // ===== Identité Track =====
    pub id: String,

    // ===== Infos fichier =====
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: i64,
    pub status: String,
    pub is_available: bool,
    pub error_message: Option<String>,

    // ===== Métadonnées audio =====
    pub title: String,
    pub title_normalized: String,
    pub artist_id: Option<String>,
    pub library_artist_id: Option<String>,
    pub album_id: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub track_number: Option<i32>,
    pub disc_number: i32,

    // ===== Infos techniques =====
    pub duration: Option<f64>,
    pub bitrate: Option<i32>,
    pub bits_per_sample: Option<i32>,
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub audio_format: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,

    // ===== Cache enrichi =====
    pub extra_tags: Option<String>,
    pub thumbnail_path: Option<String>,
    pub last_scanned_at: Option<DateTime<Utc>>,

    // ===== Stats utilisateur =====
    pub play_count: i64,
    pub last_played_at: Option<DateTime<Utc>>,
    pub rating: Option<i32>,
    pub favorite: bool,

    // ===== Timestamps =====
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}