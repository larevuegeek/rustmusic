export interface LibraryCache {
  id: number;
  path: string;
  title: string | null;
  artist: string | null;
  album: string | null;
  album_artist: string | null;
  year: string | null;
  genre: string | null;
  track_number: number | null;
  disc_number: number | null;
  duration: number | null;
  bitrate: number | null;
  bits_per_sample: number | null;
  sample_rate: number | null;
  channels: number | null;
  audio_format: string | null;
  mime_type: string | null;
  file_size: number | null;
  extra_tags: string | null; // JSON string
  thumbnail_path: string | null;
  last_scanned_at: string | null;
}

export interface LibraryCacheCreate {
  path: string;
  title: string | null;
  artist: string | null;
  album: string | null;
  album_artist: string | null;
  year: string | null;
  genre: string | null;
  track_number: number | null;
  disc_number: number | null;
  duration: number | null;
  bitrate: number | null;
  bits_per_sample: number | null;
  sample_rate: number | null;
  channels: number | null;
  audio_format: string | null;
  mime_type: string | null;
  file_size: number | null;     // i64 côté Rust -> number côté JS
  extra_tags: string | null;    // JSON string
  thumbnail_path: string | null;
}