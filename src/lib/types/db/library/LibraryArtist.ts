export interface LibraryArtist {
  id: string;
  library_id: number;
  artist_id: string;
  
  total_albums: number;
  total_tracks: number;
  total_duration: number;
  
  custom_image_url: string | null;
  notes: string | null;
  
  added_at: string;
}