/**
 * Reactive snapshot of the current playback pipeline (source → resampler → output).
 *
 * Populated by listening to the Tauri event `"playback-pipeline"` which the
 * Rust audio threads emit at the start of each track. Lets the player UI
 * display things like "DSD64 → 88.2 kHz" with a visible arrow when a
 * conversion is in flight.
 */

import { writable } from "svelte/store";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type PlaybackPipelineInfo = {
  /** "DSD64", "FLAC", "MP3", ... */
  source_format: string;
  source_sample_rate: number;
  source_bits: number;
  source_channels: number;

  /** Only set for DSD playback (intermediate PCM rate after DSD2PCM filter). */
  intermediate_pcm_rate: number | null;
  /** DSD filter taps (e.g. 2048 / 1024 / 512). Only set for DSD. */
  dsd_filter_taps: number | null;
  /** DSD decimation factor (e.g. 32 for DSD64 → 88.2 kHz). Only set for DSD. */
  dsd_decimation: number | null;

  /** Device sample rate (what CPAL outputs). */
  output_sample_rate: number;
  output_channels: number;
  /** Human name of the active output device. */
  device_name: string;

  /** True when a resampler is in the chain (source rate ≠ device rate). */
  resampler_active: boolean;
  /** Active quality profile : "high" | "medium" | "low". */
  quality_profile: string;
};

/**
 * Derived flag describing the pipeline mode for the UI badge.
 *  - "bit-perfect" : nothing modifies the samples (source rate == device rate, no DSD)
 *  - "resampled"   : a resampler is in the chain (rate conversion)
 *  - "dsd"         : DSD source is decoded to PCM (always involves DSP)
 */
export type PipelineMode = "bit-perfect" | "resampled" | "dsd";

export function pipelineMode(info: PlaybackPipelineInfo | null): PipelineMode | null {
  if (!info) return null;
  if (info.intermediate_pcm_rate != null) return "dsd";
  if (info.resampler_active) return "resampled";
  return "bit-perfect";
}

export const playbackPipelineStore = writable<PlaybackPipelineInfo | null>(null);

let unlisten: UnlistenFn | null = null;

/**
 * Subscribe to the backend event. Idempotent — calling twice doesn't double-subscribe.
 * Should be called once at app boot from `+layout.svelte`.
 */
export async function initPlaybackPipelineListener(): Promise<void> {
  if (unlisten !== null) return;
  unlisten = await listen<PlaybackPipelineInfo>("playback-pipeline", (event) => {
    playbackPipelineStore.set(event.payload);
  });
}

/** Stop listening — only useful for hot-reload / cleanup in dev. */
export async function disposePlaybackPipelineListener(): Promise<void> {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}
