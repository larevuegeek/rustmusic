export interface LibraryAlbum {
  id: string;
  artist_id: string;
  library_id: number;

  title: string;
  title_normalized: string;

  year: number | null;
  genre: string | null;
  cover_url: string | null;
  musicbrainz_id: string | null;

  album_type: string; // "album" | "single" | "compilation" | "ep"

  total_tracks: number;
  total_duration: number;

  notes: string | null;

  created_at: string; // ISO date
  updated_at: string; // ISO date
}