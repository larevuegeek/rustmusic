<script lang="ts">
  import Icon from "@iconify/svelte";
  import { t } from "$lib/i18n";
  import type { PlaybackPipelineInfo } from "$lib/stores/player/playbackPipeline.store";

  let { info }: { info: PlaybackPipelineInfo } = $props();

  // Helpers d'affichage des fréquences.
  function formatRate(hz: number): string {
    if (hz >= 1_000_000) return `${(hz / 1_000_000).toFixed(2)} MHz`;
    if (hz >= 1_000) return `${(hz / 1_000).toFixed(hz % 1000 === 0 ? 0 : 1)} kHz`;
    return `${hz} Hz`;
  }

  function channelsLabel(n: number): string {
    if (n === 1) return $t("pipeline.channels_mono");
    if (n === 2) return $t("pipeline.channels_stereo");
    if (n === 3) return "3.0";
    if (n === 4) return "4.0";
    if (n === 5) return "5.0";
    if (n === 6) return "5.1";
    if (n === 7) return "6.1";
    if (n === 8) return "7.1";
    return `${n} ch`;
  }

  let qualityLabel = $derived(
    info.quality_profile === "high"
      ? $t("settings.audio_quality_high")
      : info.quality_profile === "medium"
      ? $t("settings.audio_quality_medium")
      : info.quality_profile === "low"
      ? $t("settings.audio_quality_low")
      : info.quality_profile === "minimal"
      ? $t("settings.audio_quality_minimal")
      : info.quality_profile,
  );

  let isDsd = $derived(info.intermediate_pcm_rate != null);
</script>

<!-- Popover : positionné par le parent en absolute -->
<div
  class="rounded-xl shadow-2xl border text-left text-[11px]
         bg-white dark:bg-neutral-900
         border-neutral-200 dark:border-white/8
         text-neutral-700 dark:text-neutral-300
         p-4 w-72"
>
  <!-- ── Source ── -->
  <div class="mb-3">
    <p class="text-[10px] font-semibold uppercase tracking-wider text-neutral-400 dark:text-neutral-500 mb-1.5 flex items-center gap-1">
      <Icon icon="lucide:file-audio" width={11} />
      {$t("pipeline.source")}
    </p>
    <p class="font-mono text-neutral-800 dark:text-neutral-200">
      {info.source_format} · {formatRate(info.source_sample_rate)}
      {#if !isDsd && info.source_bits > 0}
        · {info.source_bits}-bit
      {:else if isDsd}
        · 1-bit
      {/if}
    </p>
    <p class="text-neutral-500 dark:text-neutral-400 mt-0.5">
      {channelsLabel(info.source_channels)}
    </p>
  </div>

  <!-- ── Décodage (uniquement DSD) ── -->
  {#if isDsd && info.intermediate_pcm_rate}
    <div class="mb-3 pt-3 border-t border-neutral-100 dark:border-white/5">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-neutral-400 dark:text-neutral-500 mb-1.5 flex items-center gap-1">
        <Icon icon="lucide:settings-2" width={11} />
        {$t("pipeline.decoding")}
      </p>
      <p class="font-mono text-neutral-800 dark:text-neutral-200">
        DSD2PCM → {formatRate(info.intermediate_pcm_rate)}
      </p>
      <p class="text-neutral-500 dark:text-neutral-400 mt-0.5">
        {#if info.dsd_filter_taps}
          {$t("pipeline.dsd_filter").replace("{taps}", String(info.dsd_filter_taps))}
        {/if}
        {#if info.dsd_decimation}
          · ×{info.dsd_decimation}
        {/if}
      </p>
    </div>
  {/if}

  <!-- ── Resampling ── -->
  {#if info.resampler_active}
    <div class="mb-3 pt-3 border-t border-neutral-100 dark:border-white/5">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-neutral-400 dark:text-neutral-500 mb-1.5 flex items-center gap-1">
        <Icon icon="lucide:waveform" width={11} />
        {$t("pipeline.resampling")}
      </p>
      <p class="font-mono text-neutral-800 dark:text-neutral-200">
        {formatRate(info.intermediate_pcm_rate ?? info.source_sample_rate)} → {formatRate(info.output_sample_rate)}
      </p>
    </div>
  {/if}

  <!-- ── Sortie ── -->
  <div class="mb-3 pt-3 border-t border-neutral-100 dark:border-white/5">
    <p class="text-[10px] font-semibold uppercase tracking-wider text-neutral-400 dark:text-neutral-500 mb-1.5 flex items-center gap-1">
      <Icon icon="lucide:speaker" width={11} />
      {$t("pipeline.output")}
    </p>
    <p class="font-mono text-neutral-800 dark:text-neutral-200 truncate" title={info.device_name}>
      {info.device_name}
    </p>
    <p class="text-neutral-500 dark:text-neutral-400 mt-0.5">
      {formatRate(info.output_sample_rate)} · {channelsLabel(info.output_channels)}
    </p>
  </div>

  <!-- ── Profil actif ── -->
  <div class="pt-3 border-t border-neutral-100 dark:border-white/5 flex items-center justify-between">
    <span class="text-[10px] font-semibold uppercase tracking-wider text-neutral-400 dark:text-neutral-500">
      {$t("pipeline.profile")}
    </span>
    <span class="text-emerald-600 dark:text-emerald-400 font-medium">
      {qualityLabel}
    </span>
  </div>
</div>
