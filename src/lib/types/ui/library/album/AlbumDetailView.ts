export interface AlbumDetailView {
  id: string;
  library_id: number;
  title: string;
  title_normalized: string;
  album_type: string; // album | single | ep | compilation
  musicbrainz_id: string | null;
  artist_id: string | null;
  artist: string | null;
  year: number | null;
  genre: string | null;
  notes: string | null;
  cover_url: string | null;
  thumbnail_path: string | null;
  total_tracks: number;
  total_duration: number;
  created_at: string; // ISO string depuis Rust DateTime<Utc>
  updated_at: string;
}
