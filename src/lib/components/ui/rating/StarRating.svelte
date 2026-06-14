<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/core";

  let {
    trackId,
    value = null,
    size = 12,
    readonly = false,
    onchange,
  }: {
    trackId?: string;
    value?: number | null;
    size?: number;
    readonly?: boolean;
    onchange?: (rating: number | null) => void;
  } = $props();

  let internalValue = $state<number | null>(null);
  let hoverValue = $state<number | null>(null);
  let current = $derived(hoverValue ?? internalValue ?? 0);

  $effect(() => { internalValue = value ?? null; });

  async function handleClick(star: number) {
    if (readonly || !trackId) return;
    const newRating = internalValue === star ? null : star;
    const previous = internalValue;
    internalValue = newRating;
    try {
      await invoke('set_track_rating', { trackId, rating: newRating });
      onchange?.(newRating);
    } catch (e) {
      internalValue = previous;
      console.error('Failed to set rating:', e);
    }
  }
</script>

<div
  class="flex items-center gap-0.5"
  onmouseleave={() => hoverValue = null}
  role="presentation"
>
  {#each [1, 2, 3, 4, 5] as star}
    <button
      type="button"
      class="cursor-pointer transition-all duration-100
             {readonly ? 'pointer-events-none' : 'hover:scale-110'}"
      disabled={readonly}
      onmouseenter={() => !readonly && (hoverValue = star)}
      onclick={(e) => { e.stopPropagation(); handleClick(star); }}
      aria-label={`${star} étoile${star > 1 ? 's' : ''}`}
    >
      <Icon
        icon={star <= current ? "mynaui:star-solid" : "lucide:star"}
        width={size}
        class={star <= current
          ? 'text-green-500 drop-shadow-[0_0_4px_rgba(34,197,94,0.35)]'
          : 'text-neutral-300 dark:text-neutral-600'}
      />
    </button>
  {/each}
</div>
