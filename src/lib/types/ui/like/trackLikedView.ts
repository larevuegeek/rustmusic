export type TrackLikedView = {
  id: number;                 // Correspond au `pub id: i64` de Rust
  liked_at: string;           // Serde transforme DateTime<Utc> en string (format ISO 8601)
  path: string;
  library_cache_id: number | null;  // Option<i64> devient number | null
  
  // --- Les champs fusionnés de la bibliothèque (JOIN) ---
  title: string | null;
  artist: string | null;
  album: string | null;
  duration: number | null;
  thumbnail_path: string | null;
  bits_per_sample: number | null;
  audio_format: string | null;
  mime_type: string | null;
};