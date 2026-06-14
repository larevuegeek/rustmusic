use crate::{entity::{artist::artist::Artist}, 
        mapper::library::artist::{
            artist_detail_view::ArtistDetailView, 
            artist_list_view::ArtistListView
        }};


pub fn to_artist_list_view(
    artist: Option<&Artist>,
    total_albums: i64,
    total_tracks: i64,
    total_duration: f64,
    thumbnail_path: Option<String>,
) -> ArtistListView {

    let (artist_id, artist_name) = match artist {
        Some(a) => (a.id.clone(), a.name.clone()),
        None => ("unknown".to_string(), "Unknown Artist".to_string()),
    };

    ArtistListView {
        id: artist_id,
        name: artist_name,
        total_albums,
        total_tracks,
        total_duration,
        thumbnail_path,
    }
}

pub fn to_artist_detail_view(
    artist: Option<&Artist>,
    total_albums: i64,
    total_tracks: i64,
    total_duration: f64,
    thumbnail_path: Option<String>,
) -> ArtistDetailView {

    let (artist_id, artist_name) = match artist {
        Some(a) => (a.id.clone(), a.name.clone()),
        None => ("unknown".to_string(), "Unknown Artist".to_string()),
    };

    ArtistDetailView {
        id: artist_id,
        name: artist_name,
        total_albums,
        total_tracks,
        total_duration,
        thumbnail_path,
    }
}