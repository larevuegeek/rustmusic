export type LibraryView = {
  id: number;
  title: string | null;
  artist: string | null;
  album: string | null;
  duration: number | null;
  total_tracks: number | null;
  total_albums: number | null;
  total_artists: number | null;
  thumbnail_path: string | null;
  bits_per_sample: number | null;
  audio_format: string | null;
  mime_type: string | null;
};