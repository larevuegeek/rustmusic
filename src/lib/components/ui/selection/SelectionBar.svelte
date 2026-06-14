<script lang="ts">
  import Icon from "@iconify/svelte";
  import { selectionStore } from "$lib/stores/ui/selection.store";
  import { toQueueTracks } from "$lib/helper/tools/queueTools";
  import { queueState } from "$lib/stores/queue/queueState.store";
  import { playerService } from "$lib/services/player/player.service";
  import { playlistStore } from "$lib/stores/playlist/playlist.store";
  import { invoke } from "@tauri-apps/api/core";
  import { toasts } from "$lib/stores/ui/toast.store";
  import type { Playlist } from "$lib/types/db/playlist/Playlist";
  import { fade, fly } from "svelte/transition";

  let selection = $derived($selectionStore);
  let showPlaylistMenu = $state(false);
  let playlists = $derived($playlistStore.playlists);

  async function handlePlayAll() {
    const tracks = selectionStore.getSelectedTracks();
    if (tracks.length === 0) return;
    const queueTracks = toQueueTracks(tracks);
    await queueState.loadTracks(queueTracks);
    playerService.playFile(queueTracks[0]);
    selectionStore.stop();
  }

  async function handleAddToQueue() {
    const tracks = selectionStore.getSelectedTracks();
    if (tracks.length === 0) return;
    const queueTracks = toQueueTracks(tracks);
    for (const t of queueTracks) {
      queueState.enqueue(t);
    }
    toasts.push({ type: "success", title: "Ajouté à la file", message: `${tracks.length} morceau(x) ajoutés` });
    selectionStore.stop();
  }

  async function handleAddToPlaylist(pl: Playlist) {
    const tracks = selectionStore.getSelectedTracks();
    if (tracks.length === 0) return;

    let added = 0;
    for (const track of tracks) {
      if (!track.path) continue;
      const params: Record<string, any> = { playlistId: pl.id, path: track.path };
      if ((track as any).id && typeof (track as any).id === 'string') {
        params.libraryTrackId = (track as any).id;
      }
      try {
        await invoke('add_track_to_playlist', params);
        added++;
      } catch { /* skip duplicates */ }
    }

    await playlistStore.refresh();
    toasts.push({ type: "success", title: "Ajouté", message: `${added} morceau(x) ajoutés à ${pl.name}` });
    showPlaylistMenu = false;
    selectionStore.stop();
  }
</script>

{#if selection.active && selection.count > 0}
  <div
    class="fixed bottom-24 left-1/2 -translate-x-1/2 z-50"
    transition:fly={{ y: 20, duration: 200 }}
  >
    <div class="flex items-center gap-2 px-4 py-2.5 rounded-2xl
                bg-neutral-950/95 backdrop-blur-xl
                border border-white/10
                shadow-[0_8px_32px_rgba(0,0,0,0.4)]">

      <!-- Count -->
      <span class="text-xs font-semibold text-emerald-400 tabular-nums px-2">
        {selection.count} sélectionné{selection.count > 1 ? 's' : ''}
      </span>

      <div class="w-px h-5 bg-white/10"></div>

      <!-- Play -->
      <button
        type="button"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer
               text-white bg-emerald-500/15 hover:bg-emerald-500/25
               transition-colors"
        onclick={handlePlayAll}
      >
        <Icon icon="lucide:play" width={13} />
        Lire
      </button>

      <!-- Add to queue -->
      <button
        type="button"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer
               text-neutral-300 hover:text-white hover:bg-white/8
               transition-colors"
        onclick={handleAddToQueue}
      >
        <Icon icon="lucide:list-end" width={13} />
        File
      </button>

      <!-- Add to playlist -->
      <div class="relative">
        <button
          type="button"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer
                 text-neutral-300 hover:text-white hover:bg-white/8
                 transition-colors"
          onclick={() => showPlaylistMenu = !showPlaylistMenu}
        >
          <Icon icon="lucide:list-music" width={13} />
          Playlist
        </button>

        {#if showPlaylistMenu}
          <button type="button" class="fixed inset-0 z-10 cursor-default"
                  onclick={() => showPlaylistMenu = false} aria-label="Fermer"></button>
          <div
            class="absolute bottom-full left-0 mb-2 z-20 w-48 py-1
                   bg-neutral-950/95 backdrop-blur-xl
                   border border-white/10
                   rounded-xl shadow-2xl shadow-black/30
                   max-h-48 overflow-y-auto scrollbar-app"
            transition:fly={{ y: 8, duration: 150 }}
          >
            {#if playlists.length === 0}
              <p class="text-xs text-neutral-500 text-center py-3">Aucune playlist</p>
            {:else}
              {#each playlists as pl (pl.id)}
                <button
                  type="button"
                  class="w-full flex items-center gap-2.5 px-3 py-2 text-xs text-left cursor-pointer
                         text-neutral-300 hover:bg-white/8 transition-colors"
                  onclick={() => handleAddToPlaylist(pl)}
                >
                  <Icon icon={pl.icon ?? "lucide:list-music"} width="13"
                        style="color: {pl.color ?? '#22c55e'};" class="opacity-70" />
                  <span class="truncate">{pl.name}</span>
                  <span class="ml-auto text-[10px] text-neutral-600">{pl.track_count}</span>
                </button>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

      <div class="w-px h-5 bg-white/10"></div>

      <!-- Deselect all -->
      <button
        type="button"
        class="flex items-center gap-1.5 px-2 py-1.5 rounded-lg text-xs cursor-pointer
               text-neutral-500 hover:text-neutral-300 hover:bg-white/5
               transition-colors"
        onclick={() => selectionStore.deselectAll()}
      >
        Tout désélectionner
      </button>

      <!-- Close -->
      <button
        type="button"
        class="flex items-center justify-center w-7 h-7 rounded-lg cursor-pointer
               text-neutral-500 hover:text-white hover:bg-white/8
               transition-colors"
        onclick={() => selectionStore.stop()}
        aria-label="Quitter la sélection"
      >
        <Icon icon="lucide:x" width={14} />
      </button>
    </div>
  </div>
{/if}
