export interface Artist {
  id: string;
  
  name: string;
  sort_name: string | null;
  
  bio: string | null;
  image_url: string | null;
  musicbrainz_id: string | null;
  
  created_at: string;
  updated_at: string;
}
