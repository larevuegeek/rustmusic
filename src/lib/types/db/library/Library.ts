export interface Library {
  id: number | null;
  profil_id: number | null;
  name: string;
  description: string | null;
  cover: string | null;
  position: number;
  total_tracks: number,
  total_albums: number,
  total_artists: number,
  created_at: string;
  updated_at: string | null;
}

export interface LibraryCreate {
  profil_id: number | null;
  name: string;
  description: string | null;
}