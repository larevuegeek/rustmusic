<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";
import Icon from "@iconify/svelte";
import ViewModeToggle from "$lib/components/ui/input/ViewModeToggle.svelte";
import { t } from "$lib/i18n";

let { libraryId }: { libraryId: number } = $props();

const segments = $derived(page.url.pathname.split("/").filter(Boolean));
// Pages listing (tracks, albums, artists, folders) vs pages détail (albums/123, artists/456)
const isListingPage = $derived(segments.length <= 3);
// Pages supportant la navigation alphabétique
const hasAlphabetNav = $derived(
  isListingPage && ['albums', 'artists'].includes(segments[2])
);


type LibraryTab = "tracks" | "albums" | "artists" | "genres" | "folders";

const tabs: {
  key: LibraryTab;
  labelKey: string;
  icon: string;
}[] = [
  { key: "tracks", labelKey: "library.tracks", icon: "mynaui:music" },
  { key: "albums", labelKey: "library.albums", icon: "lucide:disc-album" },
  { key: "artists", labelKey: "library.artists", icon: "lucide:mic-2" },
  { key: "genres", labelKey: "library.genres", icon: "lucide:tag" },
  { key: "folders", labelKey: "nav.folders", icon: "lucide:folder-open" }
];

function isActive(key: LibraryTab) {
  const tab = segments[2]; // 0=library, 1=id, 2=tab
  if (!tab && key === "tracks") return true;
  return tab === key;
}

function navigateTab(key: LibraryTab) {
  // Persister le dernier onglet visité
  try { localStorage.setItem(`lib-tab-${libraryId}`, key); } catch {}
  goto(`/library/${libraryId}/${key}`);
}

</script>
<div class="px-3 md:px-6 border-b border-neutral-200 dark:border-neutral-800">
    <div class="flex items-center justify-between py-2">
      <div class="flex items-center gap-1 md:gap-2 overflow-x-auto scrollbar-none min-w-0 py-0.5">
        {#each tabs as tab}
          <button
            onclick={() => navigateTab(tab.key)}
            class="relative flex items-center gap-1.5 md:gap-2 px-3 md:px-4 py-1.5 md:py-2 rounded-full text-xs md:text-sm font-medium transition-all duration-200 cursor-pointer whitespace-nowrap
                {isActive(tab.key)
                ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 shadow-[0_0_0_1px_rgba(16,185,129,0.25)]'
                : 'text-neutral-500 hover:text-neutral-800 dark:hover:text-neutral-200 hover:bg-neutral-100 dark:hover:bg-neutral-800/60'}"
          >
            <Icon icon={tab.icon} width={14} />
            <span class="hidden sm:inline">{$t(tab.labelKey)}</span>

            {#if isActive(tab.key)}
              <span class="absolute inset-0 rounded-full bg-emerald-500/5 pointer-events-none"></span>
            {/if}
          </button>
        {/each}
      </div>

      <!-- Toggle grille/liste (uniquement sur les pages listing) -->
      {#if isListingPage}
        <div class="shrink-0 ml-2">
          <ViewModeToggle showAlphabet={hasAlphabetNav} />
        </div>
      {/if}
    </div>
</div>