<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let value = $state(80);
  let previousValue = $state(80);
  let isMuted = $derived(value === 0);
  let showPercent = $state(false);

  // 0 = mute, 1 = low, 2 = high
  let level = $derived(value === 0 ? 0 : value < 50 ? 1 : 2);

  // Devices
  let devices: string[] = $state([]);
  let selectedDevice: string | null = $state(null);
  let showDevices = $state(false);
  let deviceBtnEl: HTMLElement | null = $state(null);

  onMount(async () => {
    try {
      value = await invoke<number>('get_volume');
      previousValue = value;
    } catch (e) {
      console.error('Failed to get volume', e);
    }

    try {
      devices = await invoke<string[]>('get_devices');
      if (devices.length > 0) selectedDevice = devices[0];
    } catch (e) {
      console.error('Failed to get devices', e);
    }
  });

  let volumeTimer: ReturnType<typeof setTimeout> | null = null;

  function handleChange() {
    // Debounce : envoyer au max toutes les 50ms au lieu de chaque pixel
    if (volumeTimer) clearTimeout(volumeTimer);
    volumeTimer = setTimeout(async () => {
      try {
        await invoke('set_volume', { volume: value });
      } catch (e) {
        console.error('Failed to set volume', e);
      }
    }, 50);
  }

  function toggleMute() {
    if (isMuted) {
      value = previousValue > 0 ? previousValue : 80;
    } else {
      previousValue = value;
      value = 0;
    }
    handleChange();
  }

  function toggleDevices() {
    showDevices = !showDevices;
  }

  async function selectDevice(device: string) {
    selectedDevice = device;
    showDevices = false;
    // Envoyer le nom du device au backend pour les prochaines lectures
    try {
      await invoke('set_device', { deviceName: device });
    } catch (e) {
      console.error('Failed to set device', e);
    }
  }

  // Fermer le dropdown au clic extérieur
  $effect(() => {
    if (!showDevices) return;

    function handleClickOutside(e: MouseEvent) {
      if (deviceBtnEl && !deviceBtnEl.contains(e.target as Node)) {
        showDevices = false;
      }
    }

    const timer = setTimeout(() => {
      document.addEventListener('click', handleClickOutside);
    }, 50);

    return () => {
      clearTimeout(timer);
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

<div
  class="flex items-center gap-2"
  onmouseenter={() => showPercent = true}
  onmouseleave={() => showPercent = false}
  role="group"
>
  <!-- Bouton device audio -->
  <div class="relative" bind:this={deviceBtnEl}>
    <button
      class="cursor-pointer shrink-0 flex items-center justify-center w-8 h-8 rounded-full
             text-neutral-500 dark:text-neutral-400 hover:text-emerald-500 dark:hover:text-emerald-400 transition-colors"
      onclick={toggleDevices}
      aria-label="Périphérique audio"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5">
        <rect x="4" y="2" width="16" height="20" rx="2" />
        <circle cx="12" cy="14" r="4" />
        <line x1="12" y1="6" x2="12.01" y2="6" />
      </svg>
    </button>

    <!-- Dropdown devices -->
    {#if showDevices && devices.length > 0}
      <div class="absolute bottom-8 right-0 z-50 w-56
                  rounded-lg border border-white/10 bg-neutral-950/95 backdrop-blur-xl
                  shadow-2xl shadow-black/40 p-1.5
                  text-sm text-neutral-200">
        <div class="px-2.5 py-1.5 text-[10px] font-semibold uppercase tracking-widest text-neutral-500">
          Sortie audio
        </div>
        {#each devices as device}
          <button
            class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-md text-left cursor-pointer
                   transition-colors duration-100
                   {selectedDevice === device
                     ? 'bg-emerald-500/15 text-emerald-400'
                     : 'hover:bg-white/8 text-neutral-300'}"
            onclick={() => selectDevice(device)}
          >
            <!-- Speaker icon -->
            <svg class="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M11 5L6 9H2v6h4l5 4V5z" />
              {#if selectedDevice === device}
                <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
                <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
              {/if}
            </svg>
            <span class="truncate text-xs">{device}</span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Icône volume (SVG inline, pas de flash) -->
  <button
    class="cursor-pointer shrink-0 flex items-center justify-center w-8 h-8 rounded-full transition-colors
           {isMuted ? 'text-red-400/70' : 'text-neutral-500 dark:text-neutral-400 hover:text-emerald-500 dark:hover:text-emerald-400'}"
    onclick={toggleMute}
    aria-label={isMuted ? 'Activer le son' : 'Couper le son'}
  >
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5">
      <path d="M11 5L6 9H2v6h4l5 4V5z" />
      {#if level === 0}
        <line x1="23" y1="9" x2="17" y2="15" />
        <line x1="17" y1="9" x2="23" y2="15" />
      {:else if level === 1}
        <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
      {:else}
        <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
        <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
      {/if}
    </svg>
  </button>

  <!-- Slider -->
  <div class="relative w-24 group/vol" role="presentation">
    <div class="relative h-1.5 rounded-full bg-neutral-300 dark:bg-neutral-700/50 overflow-hidden">
      <div
        class="absolute inset-y-0 left-0 rounded-full transition-[width] duration-75
               {isMuted ? 'bg-red-400/50' : 'bg-emerald-500'}"
        style="width: {value}%"
      ></div>
    </div>

    <div
      class="absolute top-1/2 -translate-y-1/2 w-3.5 h-3.5 rounded-full
             bg-white shadow-md shadow-black/20
             border-2 transition-all duration-75
             opacity-0 group-hover/vol:opacity-100
             {isMuted ? 'border-red-400' : 'border-emerald-500'}"
      style="left: calc({value}% - 7px)"
    ></div>

    <input
      type="range"
      min="0"
      max="100"
      step="1"
      bind:value={value}
      oninput={handleChange}
      class="absolute inset-0 w-full h-4 -top-1.5 cursor-pointer opacity-0"
      aria-label="Volume"
    />
  </div>

  <!-- Pourcentage -->
  <span
    class="text-[10px] tabular-nums font-medium w-7 text-right shrink-0 transition-opacity duration-150
           {showPercent ? 'opacity-100' : 'opacity-0'}
           {isMuted ? 'text-red-400/70' : 'text-neutral-500'}"
  >
    {value}%
  </span>
</div>
