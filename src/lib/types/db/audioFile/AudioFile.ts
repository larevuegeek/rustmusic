import type { AudioTags } from "./AudioTags";

export interface AudioFile {
    path: string;
    audio_format?: string;
    mime_type: string;
    file_size: number;
    duration: number;
    bitrate: number;
    bits_per_sample: number;
    sample_rate: number;
    padding: boolean;
    channel: number;
    mode_extension: boolean;
    is_vbr: boolean;
    version: string;
    layer: string;
    is_protected_by_crc: boolean;
    tags: AudioTags;
}