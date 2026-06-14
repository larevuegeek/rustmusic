export type ProfilRole = 'user' | 'admin' | 'guest';

export interface Profil {
  id: number;
  name: string;
  avatar: string | null;
  color: string;
  bio: string | null;
  role: ProfilRole;
  is_active: boolean;
  created_at: string;
  updated_at: string | null;
}