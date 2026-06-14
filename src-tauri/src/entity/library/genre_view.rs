use serde::Serialize;

#[derive(Serialize)]
pub struct GenreView {
    pub name: String,
    pub total_albums: i64,
    pub total_tracks: i64,
    pub covers: Vec<String>,
}
