/**
 * Helpers for displaying audio format / quality info in the UI.
 * DSD-specific labels live here so any list / badge / detail page
 * stays consistent.
 */

/** True when the audio format is one of the DSD container formats. */
export function isDsdFormat(format: string | null | undefined): boolean {
  if (!format) return false;
  const f = format.toUpperCase();
  return f === "DSF" || f === "DFF";
}

/**
 * Compute the user-facing DSD label from a sample rate in Hz.
 * 2 822 400 Hz → DSD64, 5 644 800 Hz → DSD128, etc.
 * Falls back to "DSD" when the rate is non-standard.
 */
export function dsdLabel(sampleRate: number | null | undefined): string {
  if (!sampleRate) return "DSD";
  const ratio = Math.round(sampleRate / 44100);
  if (ratio === 64) return "DSD64";
  if (ratio === 128) return "DSD128";
  if (ratio === 256) return "DSD256";
  if (ratio === 512) return "DSD512";
  if (ratio === 1024) return "DSD1024";
  return "DSD";
}

/** "2.82 MHz" / "5.64 MHz" — appropriate scale for DSD rates. */
export function formatDsdRate(sampleRate: number | null | undefined): string {
  if (!sampleRate) return "?";
  return `${(sampleRate / 1_000_000).toFixed(2)} MHz`;
}

/** "Mono" / "Stereo" / "5.1" / "6 ch" — friendly channel label. */
export function formatChannels(channels: number | null | undefined): string {
  if (channels === 1) return "Mono";
  if (channels === 2) return "Stereo";
  if (channels === 6) return "5.1";
  if (channels === 8) return "7.1";
  if (!channels) return "?";
  return `${channels} ch`;
}

/**
 * Friendly bitrate display.
 * Below 1000 kb/s → "320 kb/s" (typical lossy / 16-bit lossless).
 * Above or equal to 1000 kb/s → "5.6 Mb/s" (HD lossless, DSD).
 */
export function formatBitrate(kbps: number | null | undefined): string {
  if (!kbps || kbps <= 0) return "—";
  if (kbps >= 1000) {
    return `${(kbps / 1000).toFixed(1)} Mb/s`;
  }
  return `${Math.round(kbps)} kb/s`;
}
