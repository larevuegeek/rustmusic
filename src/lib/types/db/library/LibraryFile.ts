export type LibraryFileStatus = 'pending' | 'processed' | 'error' | 'missing';

export interface LibraryFile {
  id: string;
  library_id: number;
  library_dir_id: string | null;
  cache_id: number | null;

  path: string;
  filename: string;
  extension: string;
  size: number;

  file_hash: string | null;
  modified_at: string | null;

  status: string; // "pending" | "indexed" | "error" | "missing"
  is_available: boolean;
  error_message: string | null;

  created_at: string; // ISO
  updated_at: string; // ISO
  last_verified_at: string | null;
}