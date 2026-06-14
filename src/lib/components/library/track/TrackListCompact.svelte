<script lang="ts">
import { handleSelectTrack, handlePlayTrack } from "$lib/actions/player/PlayerAction";
import TrackContextMenu from "$lib/components/ui/contextmenu/TrackContextMenu.svelte";
import { liked } from "$lib/stores/playlist/like.store";
import { selectionStore } from "$lib/stores/ui/selection.store";
import { settingsStore } from "$lib/stores/settings/settings.store";
import Icon from "@iconify/svelte";
import CoverImg from "$lib/components/ui/image/CoverImg.svelte";
import StarRating from "$lib/components/ui/rating/StarRating.svelte";

let { libraryId, track }: { libraryId: any; track: any } = $props();

let contextMenu = $state<{ x: number; y: number } | null>(null);
let isLiked = $derived($liked.paths.has(track.path));
let selection = $derived($selectionStore);
let isSelected = $derived(selection.active && selection.ids.has(track.id));
let singleClickPlay = $derived(settingsStore.get('single_click_play') === 'true');

function handleClick() {
    if (selection.active) {
        selectionStore.toggle(track.id, track);
    } else if (singleClickPlay) {
        handlePlayTrack(track.path);
    } else {
        handleSelectTrack(track.path);
    }
}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
    class="group flex items-center gap-3 py-1.5 px-2 rounded-md
           transition-colors duration-100
           {isSelected ? 'bg-emerald-500/10 dark:bg-emerald-500/10' : 'hover:bg-neutral-100 dark:hover:bg-neutral-900'}
           {selection.active ? 'cursor-pointer' : ''}"
    ondblclick={() => { if (!selection.active) handlePlayTrack(track.path); }}
    onclick={() => { if (selection.active) handleClick(); }}
    oncontextmenu={(e) => { e.preventDefault(); contextMenu = { x: e.clientX, y: e.clientY }; }}
>
    <!-- # / Checkbox -->
    {#if selection.active}
      <button
        type="button"
        class="w-5 h-5 rounded shrink-0 flex items-center justify-center cursor-pointer
               transition-all duration-150
               {isSelected
                 ? 'bg-emerald-500 text-white'
                 : 'bg-white/5 border border-white/15 text-transparent hover:border-emerald-500/40'}"
        onclick={(e) => { e.stopPropagation(); selectionStore.toggle(track.id, track); }}
      >
        {#if isSelected}
          <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round">
            <path d="m4.5 12.75 6 6 9-13.5"/>
          </svg>
        {/if}
      </button>
    {:else}
      <div class="w-5 text-[10px] text-neutral-400 text-right shrink-0 tabular-nums">
          {track.track_number ?? '—'}
      </div>
    {/if}

    <!-- Cover mini -->
    <button onclick={handleClick} class="shrink-0 cursor-pointer">
        <div class="w-8 h-8 rounded overflow-hidden bg-neutral-200 dark:bg-neutral-800">
            {#if track.thumbnail_path}
                <CoverImg path={track.thumbnail_path} alt="" size="1x"
                     class="w-full h-full object-cover" />
            {:else}
                <div class="w-full h-full flex items-center justify-center">
                    <Icon icon="lucide:music" width={12} class="text-neutral-400" />
                </div>
            {/if}
        </div>
    </button>

    <!-- Titre -->
    <button onclick={() => handleSelectTrack(track.path)} class="flex-1 min-w-0 text-left cursor-pointer">
        <span class="text-sm text-neutral-800 dark:text-neutral-200 truncate block"
              title={track.title}>
            {track.title}
        </span>
    </button>

    <!-- Artiste -->
    <span class="hidden sm:block w-40 text-xs text-neutral-500 dark:text-neutral-400 truncate shrink-0">
        {track.artist ?? ''}
    </span>

    <!-- Album -->
    <span class="hidden md:block w-44 text-xs text-neutral-500 dark:text-neutral-400 truncate shrink-0">
        {track.album ?? ''}
    </span>

    <!-- Rating -->
    <div class="hidden lg:flex shrink-0 w-20">
        <StarRating trackId={track.id} value={track.rating} size={11} />
    </div>

    <!-- Like -->
    <button
        onclick={() => liked.toggle(track.path)}
        class="p-1 rounded cursor-pointer shrink-0 transition-colors
               {isLiked ? 'text-pink-500' : 'text-transparent group-hover:text-neutral-300 dark:group-hover:text-neutral-600 hover:!text-pink-400'}"
        aria-label="Liker"
    >
        <Icon icon={isLiked ? "mynaui:heart-solid" : "lucide:heart"} width={13} />
    </button>

    <!-- Durée -->
    <span class="text-[11px] tabular-nums text-neutral-400 dark:text-neutral-500 w-10 text-right shrink-0">
        {#if track.duration}
            {Math.floor(track.duration / 60)}:{String(Math.floor(track.duration % 60)).padStart(2, '0')}
        {:else}
            —
        {/if}
    </span>

    <!-- Menu -->
    <button
        onclick={(e) => { contextMenu = { x: e.clientX, y: e.clientY }; }}
        class="p-1 rounded cursor-pointer shrink-0 opacity-0 group-hover:opacity-100
               text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300 transition-all"
        aria-label="Actions"
    >
        <Icon icon="lucide:more-horizontal" width={14} />
    </button>
</div>

{#if contextMenu}
    <TrackContextMenu
        {track}
        x={contextMenu.x}
        y={contextMenu.y}
        {libraryId}
        showNavigation={true}
        onclose={() => contextMenu = null}
    />
{/if}
