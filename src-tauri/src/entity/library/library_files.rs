use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LibraryFile {
    pub id: String,
    pub library_id: i64,
    pub library_dir_id: Option<String>,
    pub cache_id: Option<i64>,
    
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: i64,
    
    pub file_hash: Option<String>,
    pub modified_at: Option<String>,
    
    pub status: String, // pending, indexed, error, missing
    pub is_available: bool,
    pub error_message: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LibraryFileCreate {
    pub library_id: i64,
    pub library_dir_id: Option<String>,
    pub cache_id: Option<i64>,
    
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: i64,
    
    pub file_hash: Option<String>,
    pub modified_at: Option<String>,
    
    pub status: String, // pending, indexed, error, missing
    pub is_available: bool,
    pub error_message: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_verified_at: Option<DateTime<Utc>>,
}