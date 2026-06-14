export type FormatStat = {
  name: string;
  count: number;
};

export type GenreStat = {
  name: string;
  count: number;
};

export type ArtistStat = {
  name: string;
  count: number;
};

export type TrackPlayStat = {
  title: string;
  artist: string;
  play_count: number;
  thumbnail_path: string | null;
};

export type LibraryStats = {
  total_tracks: number;
  total_albums: number;
  total_artists: number;
  total_genres: number;
  total_duration_sec: number;
  total_size_bytes: number;
  avg_bitrate: number;
  total_play_count: number;
  formats: FormatStat[];
  top_genres: GenreStat[];
  top_artists: ArtistStat[];
  top_played: TrackPlayStat[];
  quality_hires: number;
  quality_lossless: number;
  quality_lossy: number;
};
