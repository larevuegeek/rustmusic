<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import {
    audioDevicesStore,
    formatSampleRate,
    bestFormatLabel,
    maxSampleRate,
    type AudioDeviceInfo,
  } from "$lib/stores/audio/audioDevices.store";
  import { t } from "$lib/i18n";
  import AudioDeviceDetailsModal from "./AudioDeviceDetailsModal.svelte";

  let selected = $state<AudioDeviceInfo | null>(null);

  onMount(() => {
    audioDevicesStore.ensureLoaded();
  });

  function open(device: AudioDeviceInfo) {
    selected = device;
  }
  function close() {
    selected = null;
  }

  function handleSelect(device: AudioDeviceInfo) {
    audioDevicesStore.setActive(device.displayName);
  }

  // Sélection rapide sans ouvrir le modal — clic direct sur la row.
  async function quickSelect(device: AudioDeviceInfo, e: MouseEvent) {
    e.stopPropagation();
    try {
      await invoke("set_device", { deviceName: device.name });
      audioDevicesStore.setActive(device.displayName);
    } catch (err) {
      console.error("Failed to set device", err);
    }
  }
</script>

<div class="mb-2">
  <div class="flex items-center justify-between mb-2.5 px-1">
    <div class="flex items-center gap-3">
      <Icon icon="lucide:speaker" width="18" class="text-neutral-400" />
      <div>
        <p class="text-sm font-medium text-neutral-800 dark:text-neutral-200">
          {$t("settings.audio_devices")}
        </p>
        <p class="text-[11px] text-neutral-400 dark:text-neutral-500">
          {$t("settings.audio_devices_desc")}
        </p>
      </div>
    </div>
    <button
      class="cursor-pointer p-1.5 rounded-md text-neutral-500 hover:text-neutral-800
             dark:hover:text-neutral-200 hover:bg-neutral-100 dark:hover:bg-white/5
             transition-colors"
      onclick={() => audioDevicesStore.refresh()}
      aria-label={$t("settings.audio_devices_refresh")}
      title={$t("settings.audio_devices_refresh")}
    >
      <Icon
        icon="lucide:refresh-cw"
        width="14"
        class={$audioDevicesStore.loading ? "animate-spin" : ""}
      />
    </button>
  </div>

  {#if $audioDevicesStore.error}
    <div class="mx-1 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-[11px] text-red-400">
      {$audioDevicesStore.error}
    </div>
  {:else if !$audioDevicesStore.loaded && $audioDevicesStore.loading}
    <div class="mx-1 p-4 rounded-lg bg-neutral-100/60 dark:bg-white/3 border border-neutral-200/60 dark:border-white/8
                text-[11px] text-neutral-500 flex items-center gap-2">
      <Icon icon="lucide:loader-2" width="14" class="animate-spin" />
      {$t("settings.audio_devices_loading")}
    </div>
  {:else if $audioDevicesStore.devices.length === 0}
    <div class="mx-1 p-4 rounded-lg bg-neutral-100/60 dark:bg-white/3 border border-neutral-200/60 dark:border-white/8
                text-[11px] text-neutral-500">
      {$t("settings.audio_devices_empty")}
    </div>
  {:else}
    <div class="space-y-1.5">
      {#each $audioDevicesStore.devices as device (device.displayName)}
        {@const maxRate = maxSampleRate(device.sampleRates)}
        {@const bestFmt = bestFormatLabel(device.sampleFormats)}
        {@const isActive = $audioDevicesStore.activeDisplayName === device.displayName}
        <div
          class="w-full flex items-stretch rounded-xl border transition-colors
                 {isActive
                   ? 'border-emerald-500/40 bg-emerald-500/8 dark:bg-emerald-500/10'
                   : 'border-neutral-200/70 dark:border-white/8 bg-neutral-50/60 dark:bg-white/2 hover:bg-neutral-100 dark:hover:bg-white/5'}"
        >
          <!-- Zone principale : ouvre le modal détails -->
          <button
            class="flex-1 min-w-0 flex items-center gap-3 px-4 py-3 rounded-l-xl cursor-pointer text-left"
            onclick={() => open(device)}
          >
            <!-- Icon -->
            <div
              class="shrink-0 w-9 h-9 rounded-lg flex items-center justify-center
                     {isActive
                       ? 'bg-emerald-500/15 text-emerald-500 dark:text-emerald-400'
                       : 'bg-neutral-200/60 dark:bg-white/5 text-neutral-500 dark:text-neutral-400'}"
            >
              <Icon icon={device.isHires ? "lucide:zap" : "lucide:speaker"} width="18" />
            </div>

            <!-- Nom + badges + résumé -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-sm font-medium text-neutral-800 dark:text-neutral-200 truncate">
                  {device.displayName}
                </span>
                {#if isActive}
                  <span class="shrink-0 text-[9px] px-1.5 py-0.5 rounded font-semibold
                               bg-emerald-500/15 text-emerald-600 dark:text-emerald-400
                               uppercase tracking-wider">
                    {$t("settings.audio_devices_active_badge")}
                  </span>
                {:else if device.isDefault}
                  <span class="shrink-0 text-[9px] px-1.5 py-0.5 rounded font-semibold
                               bg-neutral-200 dark:bg-white/8 text-neutral-500 dark:text-neutral-400
                               uppercase tracking-wider">
                    {$t("settings.audio_devices_default_badge")}
                  </span>
                {/if}
                {#if device.isHires}
                  <span class="shrink-0 text-[9px] px-1.5 py-0.5 rounded font-semibold
                               bg-amber-500/15 text-amber-600 dark:text-amber-300
                               uppercase tracking-wider">
                    Hi-Res
                  </span>
                {/if}
              </div>
              <div class="mt-0.5 flex items-center gap-2 text-[11px] text-neutral-500 dark:text-neutral-400">
                {#if bestFmt}
                  <span>{bestFmt}</span>
                {/if}
                {#if maxRate}
                  {#if bestFmt}<span class="text-neutral-300 dark:text-neutral-600">·</span>{/if}
                  <span class="tabular-nums">{$t("settings.audio_devices_up_to")} {formatSampleRate(maxRate)}</span>
                {/if}
                <span class="text-neutral-300 dark:text-neutral-600">·</span>
                <span>{device.maxChannels} ch</span>
              </div>
            </div>

            <!-- Indice "voir + " -->
            <div class="shrink-0 flex items-center gap-1 text-[11px] text-neutral-400 dark:text-neutral-500">
              {$t("settings.audio_devices_details")}
              <Icon icon="lucide:chevron-right" width="14" />
            </div>
          </button>

          <!-- Séparateur vertical -->
          <div class="w-px my-2 bg-neutral-200/70 dark:bg-white/8"></div>

          <!-- Bouton sélection rapide -->
          {#if isActive}
            <div
              class="shrink-0 flex items-center justify-center px-3 rounded-r-xl
                     text-emerald-600 dark:text-emerald-400"
              title={$t("settings.audio_devices_active")}
            >
              <Icon icon="lucide:check-circle-2" width="18" />
            </div>
          {:else}
            <button
              class="shrink-0 flex items-center gap-1.5 px-3 rounded-r-xl cursor-pointer
                     text-[11px] font-medium text-neutral-500 dark:text-neutral-400
                     hover:text-emerald-600 dark:hover:text-emerald-400
                     hover:bg-emerald-500/8 transition-colors"
              onclick={(e) => quickSelect(device, e)}
              title={$t("settings.audio_devices_select_action")}
            >
              <Icon icon="lucide:check" width="14" />
              <span class="hidden md:inline">{$t("settings.audio_devices_select_short")}</span>
            </button>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if selected}
  {@const activeDn = $audioDevicesStore.activeDisplayName}
  <AudioDeviceDetailsModal
    device={selected}
    isActive={activeDn === selected.displayName}
    onClose={close}
    onSelect={handleSelect}
  />
{/if}
