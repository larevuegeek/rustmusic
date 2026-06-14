<script lang="ts">
  import Icon from "@iconify/svelte";
  import type { Playlist } from "$lib/types/db/playlist/Playlist";
  import { goto } from "$app/navigation";
  import { popinStore } from "$lib/stores/ui/popin.store";
  import EditPlaylistPopin from "$lib/components/playlist/popin/EditPlaylistPopin.svelte";

  let { playlist }: { playlist: Playlist } = $props();

  function openEdit(e: MouseEvent) {
    e.stopPropagation();
    popinStore.open("Modifier la playlist", EditPlaylistPopin, { playlist });
  }
</script>

<div class="group relative">
  <button
    class="flex w-full items-center gap-3 px-2 py-1.5 rounded-lg text-left cursor-pointer
           transition-all duration-150
           hover:bg-neutral-100/80 dark:hover:bg-white/4"
    onclick={() => goto(`/playlist/${playlist.id}`)}
  >
    <div class="w-9 h-9 rounded-lg flex items-center justify-center shrink-0"
         style="background: {playlist.color}18; border: 1px solid {playlist.color}25;">
      <Icon icon={playlist.icon} width="16" height="16"
            style="color: {playlist.color};" />
    </div>
    <div class="min-w-0 flex-1">
      <div class="font-medium text-[13px] truncate text-neutral-800 dark:text-neutral-200">
        {playlist.name}
      </div>
      <div class="text-[11px] truncate text-neutral-400 dark:text-neutral-500">
        {playlist.track_count} titre{playlist.track_count !== 1 ? 's' : ''}
      </div>
    </div>
  </button>

  <!-- Bouton edit au hover -->
  <button
    type="button"
    class="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100
           w-7 h-7 rounded-md flex items-center justify-center
           text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-200
           hover:bg-neutral-300/50 dark:hover:bg-neutral-700/50
           transition-all duration-150 cursor-pointer"
    aria-label="Modifier la playlist"
    onclick={openEdit}
  >
    <Icon icon="lucide:pen-line" class="w-3.5 h-3.5" />
  </button>
</div>
