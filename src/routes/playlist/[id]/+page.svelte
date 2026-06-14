<script lang="ts">
  import { page } from "$app/state";
  import { invoke } from "@tauri-apps/api/core";
  import CoverImg from "$lib/components/ui/image/CoverImg.svelte";
  import { onMount } from "svelte";
  import type { Playlist } from "$lib/types/db/playlist/Playlist";
  import type { PlaylistTrackView } from "$lib/types/ui/playlist/playlistTrackView";
  import { formatTime } from "$lib/helper/tools/dateTools";
  import { handleSelectTrack, handlePlayTrack } from "$lib/actions/player/PlayerAction";
  import TrackContextMenu from "$lib/components/ui/contextmenu/TrackContextMenu.svelte";
  import Icon from "@iconify/svelte";
  import { popinStore } from "$lib/stores/ui/popin.store";
  import { playlistStore } from "$lib/stores/playlist/playlist.store";
  import EditPlaylistPopin from "$lib/components/playlist/popin/EditPlaylistPopin.svelte";
  import PageHeader from "$lib/components/ui/header/PageHeader.svelte";
  import { goto } from "$app/navigation";
  import { t } from "$lib/i18n";
  import { toQueueTracks } from "$lib/helper/tools/queueTools";
  import { queueState } from "$lib/stores/queue/queueState.store";
  import { playerService } from "$lib/services/player/player.service";

  let playlist: Playlist | null = $state(null);
  let tracks: PlaylistTrackView[] = $state([]);
  let loading = $state(true);
  let contextMenu = $state<{ x: number; y: number; track: PlaylistTrackView } | null>(null);

  const playlistId = $derived(Number(page.params.id));

  async function loadPlaylist() {
    loading = true;
    try {
      playlist = await invoke<Playlist>('get_playlist', { playlistId });
      tracks = await invoke<PlaylistTrackView[]>('get_playlist_tracks', { playlistId });
    } catch (e) {
      console.error('Failed to load playlist', e);
    } finally {
      loading = false;
    }
  }

  async function handleRemoveFromPlaylist(playlistItemId: number) {
    try {
      await invoke('remove_track_from_playlist', { playlistItemId });
      tracks = tracks.filter(t => t.playlist_item_id !== playlistItemId);
      // Mettre à jour le compteur dans le store sidebar
      await playlistStore.refresh();
      // Mettre à jour le compteur local
      if (playlist) playlist.track_count = tracks.length;
    } catch (e) {
      console.error('Failed to remove track', e);
    }
  }

  let showDeleteConfirm = $state(false);

  async function handleDeletePlaylist() {
    if (!playlist) return;
    try {
      await playlistStore.removePlaylist(playlist.id);
      goto('/');
    } catch (e) {
      console.error('Failed to delete playlist', e);
    }
  }

  async function playAll() {
    if (tracks.length === 0) return;
    const queueTracks = toQueueTracks(tracks);
    await queueState.loadTracks(queueTracks);
    playerService.playFile(queueTracks[0]);
  }

  onMount(() => {
    loadPlaylist();
  });
</script>

<div class="py-5 px-4 md:px-10">
  {#if loading}
    <div class="flex items-center gap-3 py-20 justify-center text-neutral-500">
      <Icon icon="lucide:loader-2" class="w-5 h-5 animate-spin" />
      Chargement…
    </div>
  {:else if playlist}
    <PageHeader
      title={playlist.name}
      subtitle="Playlist"
      icon={playlist.icon}
      iconColor={playlist.color}
      count={playlist.track_count}
      countLabel="titre"
    >
      {#snippet actions()}
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="text-xs text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-200
                   cursor-pointer transition-colors flex items-center gap-1"
            onclick={() => playlist && popinStore.open("Modifier la playlist", EditPlaylistPopin, { playlist })}
          >
            <Icon icon="lucide:pen-line" class="w-3 h-3" />
            {$t('playlist_page.modify')}
          </button>

          <div class="w-px h-4 bg-neutral-200 dark:bg-white/10"></div>

          <button
            type="button"
            class="p-1.5 rounded-lg cursor-pointer
                   text-neutral-300 dark:text-neutral-600
                   hover:text-red-500 hover:bg-red-500/10
                   transition-all duration-150"
            title="Supprimer la playlist"
            onclick={() => showDeleteConfirm = true}
          >
            <Icon icon="lucide:trash-2" width="14" />
          </button>
        </div>
      {/snippet}

      {#snippet extra()}
        {#if tracks.length > 0}
          <button
            type="button"
            onclick={playAll}
            class="w-fit flex items-center gap-2 px-6 py-2 rounded-full
                   bg-green-600 text-white font-medium
                   hover:bg-green-700 transition-all duration-150 cursor-pointer"
          >
            <Icon icon="lucide:play" width={16} />
            {$t('library.play_all')}
          </button>
        {/if}
      {/snippet}
    </PageHeader>

    <!-- Track list -->
    {#if tracks.length === 0}
      <div class="flex flex-col items-center justify-center py-24 px-6 text-center">
        <div class="relative mb-6">
          <div class="absolute inset-0 rounded-full blur-2xl scale-[2.5] animate-pulse opacity-30 pointer-events-none"
               style="background: {playlist.color};"></div>
          <div class="relative w-20 h-20 rounded-full flex items-center justify-center
                      border"
               style="background: {playlist.color}12; border-color: {playlist.color}25;">
            <Icon icon={playlist.icon} width="36" height="36"
                  style="color: {playlist.color}; filter: drop-shadow(0 0 8px {playlist.color}60);" />
          </div>
        </div>

        <h2 class="text-lg font-semibold text-neutral-800 dark:text-neutral-100 mb-2">
          {$t('playlist_page.empty_playlist')}
        </h2>

        <p class="text-sm text-neutral-500 dark:text-neutral-400 max-w-xs leading-relaxed">
          {$t('playlist_page.empty_playlist_desc')}
        </p>
      </div>
    {:else}
      {#each tracks as track, index (track.playlist_item_id)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="flex items-center justify-between py-3 px-3 rounded-md
                    hover:bg-neutral-100 dark:hover:bg-neutral-900
                    transition-colors duration-150"
             ondblclick={() => track.path && handlePlayTrack(track.path)}
             oncontextmenu={(e) => { e.preventDefault(); contextMenu = { x: e.clientX, y: e.clientY, track }; }}>
          <!-- LEFT -->
          <div class="flex items-center gap-4 min-w-0">
            <div class="w-6 text-xs text-neutral-400 text-right shrink-0">
              {String(index + 1).padStart(2, "0")}
            </div>

            <button onclick={() => track.path && handleSelectTrack(track.path)} class="cursor-pointer">
              <div class="w-12 h-12 rounded-md overflow-hidden
                          bg-neutral-200 dark:bg-neutral-700
                          flex items-center justify-center shrink-0">
                {#if track.thumbnail_path}
                  <CoverImg path={track.thumbnail_path} alt="Cover"
                       class="w-full h-full object-cover" />
                {:else}
                  <Icon icon="lucide:music" width={16} class="text-neutral-400" />
                {/if}
              </div>
            </button>

            <div class="flex flex-col items-stretch min-w-0">
              <button onclick={() => track.path && handleSelectTrack(track.path)} class="text-left cursor-pointer min-w-0 w-full">
                <span class="block font-medium text-neutral-800 dark:text-neutral-200 truncate"
                      title={track.title ?? "Titre inconnu"}>
                  {track.title ?? "Titre inconnu"}
                </span>
              </button>

              <div class="text-xs text-neutral-500 dark:text-neutral-400 truncate">
                {track.artist_name ?? "Artiste inconnu"}
              </div>

              {#if track.album_title}
                <div class="text-xs text-neutral-500 dark:text-neutral-400 truncate my-1">
                  <span class="font-semibold">{track.album_title}</span>
                </div>
              {/if}
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
  {/if}
</div>

{#if contextMenu}
  <TrackContextMenu
    track={contextMenu.track}
    x={contextMenu.x}
    y={contextMenu.y}
    onclose={() => contextMenu = null}
    showAddToPlaylist={false}
    showDelete={true}
    deleteLabel="Retirer de la playlist"
    ondelete={() => { if (contextMenu) handleRemoveFromPlaylist(contextMenu.track.playlist_item_id); }}
  />
{/if}

<!-- Dialog suppression playlist -->
{#if showDeleteConfirm && playlist}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <button type="button" class="absolute inset-0 bg-black/60 backdrop-blur-sm cursor-default"
            onclick={() => showDeleteConfirm = false} aria-label="Fermer"></button>

    <div class="relative w-full max-w-sm mx-4 p-6
                bg-neutral-50 dark:bg-neutral-900
                border border-neutral-200/60 dark:border-white/8
                rounded-2xl shadow-2xl shadow-black/20">

      <div class="flex flex-col items-center text-center">
        <div class="w-12 h-12 rounded-full bg-red-500/10 flex items-center justify-center mb-4">
          <Icon icon="lucide:trash-2" width="20" class="text-red-500" />
        </div>

        <h3 class="text-base font-semibold text-neutral-800 dark:text-neutral-100 mb-1">
          Supprimer la playlist
        </h3>
        <p class="text-sm text-neutral-500 dark:text-neutral-400 mb-6">
          La playlist <strong class="text-neutral-700 dark:text-neutral-200">{playlist.name}</strong>
          et ses {playlist.track_count} morceau{playlist.track_count !== 1 ? 'x' : ''} seront retirés.
        </p>

        <div class="flex items-center gap-3 w-full">
          <button
            class="flex-1 px-4 py-2 rounded-xl text-sm font-medium cursor-pointer
                   text-neutral-600 dark:text-neutral-400
                   hover:bg-neutral-100 dark:hover:bg-white/5
                   transition-colors"
            onclick={() => showDeleteConfirm = false}
          >
            Annuler
          </button>
          <button
            class="flex-1 px-4 py-2 rounded-xl text-sm font-semibold cursor-pointer
                   bg-red-500 text-white hover:bg-red-600
                   active:scale-[0.97] transition-all"
            onclick={handleDeletePlaylist}
          >
            Supprimer
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
