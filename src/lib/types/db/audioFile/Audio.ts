export interface AudioMetadata {
  duration_secs: number | null;
  duration_formatted: string;
  sample_rate: number;
  channels: number;
  file_path: string;
}