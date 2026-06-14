export type LibraryDirStatus = 'pending' | 'scanning' | 'completed' | 'error';

export interface LibraryDir {
  id: string;
  library_id: number;

  path: string;
  name: string;

  is_recursive: boolean;
  is_active: boolean;
  watch_enabled: boolean;

  include_patterns: string | null; // JSON string
  exclude_patterns: string | null; // JSON string

  total_files: number;
  total_size: number;

  last_scan_at: string | null;

  scan_status: string; // "pending" | "scanning" | "completed" | "error"
  scan_error: string | null;

  created_at: string; // ISO Date
  updated_at: string; // ISO Date
}