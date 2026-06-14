<script lang="ts">
import type { TrackLikedView } from "$lib/types/ui/like/trackLikedView";
import { formatTime } from "$lib/helper/tools/dateTools";
import { liked } from "$lib/stores/playlist/like.store";
import Icon from "@iconify/svelte";
import PageHeader from "$lib/components/ui/header/PageHeader.svelte";
import TrackContextMenu from "$lib/components/ui/contextmenu/TrackContextMenu.svelte";
import { handleSelectTrack, handlePlayTrack } from "$lib/actions/player/PlayerAction";
import { invoke } from "@tauri-apps/api/core";
import CoverImg from "$lib/components/ui/image/CoverImg.svelte";
import { profilSelector } from "$lib/stores/profil/profil.store";
import { onMount } from "svelte";
import { t } from "$lib/i18n";

let tracks: TrackLikedView[] = $state([]);
let loading = $state(false);
let contextMenu = $state<{ x: number; y: number; track: TrackLikedView } | null>(null);

let profil = $derived($profilSelector);

async function loadTrackLiked() {
    if (loading || !profil.profilSelected) return;
    tracks = await invoke<TrackLikedView[]>('get_tracks_liked', { profilId: profil.profilSelected.id });
    loading = false;
}

function handleRemoveLiked(path: string) {
  liked.unlike(path);
  tracks = tracks.filter(t => t.path !== path);
}

onMount(async () => {
  loadTrackLiked();
});

</script>
<div class="py-5 px-4 md:px-10">

  <PageHeader
    title="Titres likés"
    subtitle="Collection"
    icon="mynaui:heart-solid"
    iconColor="#f43f5e"
    count={tracks.length}
    countLabel="titre"
  />

{#if tracks.length === 0 && !loading}
  <div class="flex flex-col items-center justify-center py-24 px-6 text-center">
    <div class="relative mb-6">
      <div class="absolute inset-0 rounded-full bg-rose-500 blur-2xl scale-[2.5] animate-pulse opacity-20 pointer-events-none"></div>
      <div class="relative w-20 h-20 rounded-full bg-rose-500/10 border border-rose-500/20 flex items-center justify-center">
        <Icon icon="mynaui:heart-solid" width="36" class="text-rose-500 drop-shadow-[0_0_8px_rgba(244,63,94,0.5)]" />
      </div>
    </div>

    <h2 class="text-lg font-semibold text-neutral-800 dark:text-neutral-100 mb-2">
      {$t('playlist_page.empty_liked')}
    </h2>

    <p class="text-sm text-neutral-500 dark:text-neutral-400 max-w-xs leading-relaxed">
      {$t('playlist_page.empty_liked_desc')}
    </p>
  </div>
{:else}

  {#each tracks as track, index (track.path)}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="flex items-center justify-between py-3 px-3 rounded-md
                hover:bg-neutral-100 dark:hover:bg-neutral-900
                transition-colors duration-150"
         ondblclick={() => handlePlayTrack(track.path)}
         oncontextmenu={(e) => { e.preventDefault(); contextMenu = { x: e.clientX, y: e.clientY, track }; }}>
      <!-- LEFT -->
      <div class="flex items-center gap-4 min-w-0">
        <div class="w-6 text-xs text-neutral-400 text-right shrink-0">
          {String(index + 1).padStart(2, "0")}
        </div>

        <button onclick={() => handleSelectTrack(track.path)} class="cursor-pointer">
          <div class="w-22 h-22 rounded-md overflow-hidden
                      bg-neutral-200 dark:bg-neutral-700
                      flex items-center justify-center shrink-0">
            {#if track.thumbnail_path}
              <CoverImg path={track.thumbnail_path} alt="Cover"
                   class="w-full h-full object-cover" />
            {:else}
              <Icon icon="lucide:music" width={18} class="text-neutral-400" />
            {/if}
          </div>
        </button>

        <div class="flex flex-col items-stretch min-w-0">
          <button onclick={() => handleSelectTrack(track.path)} class="text-left cursor-pointer min-w-0 w-full">
            <span class="block font-medium text-neutral-800 dark:text-neutral-200 truncate"
                  title={track.title ?? "Titre inconnu"}>
              {track.title ?? "Titre inconnu"}
            </span>
          </button>

          <div class="text-xs text-neutral-500 dark:text-neutral-400 truncate">
            {track.artist ?? "Artiste inconnu"}
          </div>

          <div class="text-xs text-neutral-500 dark:text-neutral-400 truncate my-1">
            <span class="font-semibold">{track.album ?? "Album inconnu"}</span>
          </div>

          <span class="text-[11px] text-neutral-400 dark:text-neutral-500 truncate tracking-wide">
            {#if track.bits_per_sample && track.bits_per_sample >= 24}
              <span class="px-1.5 py-0.5 rounded-sm font-semibold bg-green-500/10 text-green-600 dark:text-green-400 mr-1">
                Hi-Res {track.bits_per_sample}bit
              </span>
            {/if}
            {track.audio_format ?? ""}
            {#if track.bits_per_sample}
              · {track.bits_per_sample}bit
            {/if}
          </span>
        </div>
      </div>

      <!-- RIGHT -->
      <div class="flex items-center gap-6 text-sm text-neutral-500 dark:text-neutral-400 shrink-0">
        <span class="tabular-nums">
          {formatTime(track.duration ?? 0)}
        </span>

        <button
          onclick={(e) => { contextMenu = { x: e.clientX, y: e.clientY, track }; }}
          class="p-2 rounded-md cursor-pointer text-neutral-500 dark:text-neutral-400
                 hover:bg-black/5 dark:hover:bg-white/10"
          aria-label="Actions"
        >
          <Icon icon="uit:ellipsis-v" width={24} height={24} />
        </button>
      </div>
    </div>
  {/each}
{/if}
</div>

{#if contextMenu}
  <TrackContextMenu
    track={contextMenu.track}
    x={contextMenu.x}
    y={contextMenu.y}
    onclose={() => contextMenu = null}
    showDelete={true}
    ondelete={() => { if (contextMenu) handleRemoveLiked(contextMenu.track.path); }}
  />
{/if}