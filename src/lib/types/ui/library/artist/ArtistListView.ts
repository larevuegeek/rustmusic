export type ArtistListView = {
  id: string;
  name: string;
  total_albums: number;
  total_tracks: number;
  total_duration: number;
  thumbnail_path?: string | null;
};