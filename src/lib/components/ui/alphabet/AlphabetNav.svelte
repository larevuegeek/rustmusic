<script lang="ts">
  import { fly } from "svelte/transition";
  import { alphabetNavVisible } from "$lib/stores/ui/alphabetNav.store";

  let {
    availableLetters = new Set<string>(),
    onletter,
  }: {
    availableLetters?: Set<string>;
    onletter: (letter: string) => void;
  } = $props();

  const LETTERS = ['#', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                   'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

  let activeLetter = $state<string | null>(null);

  function handleClick(letter: string) {
    if (!availableLetters.has(letter)) return;
    activeLetter = letter;
    onletter(letter);
    setTimeout(() => { activeLetter = null; }, 600);
  }
</script>

{#if $alphabetNavVisible}
  <div
    class="absolute right-1 top-1/2 -translate-y-1/2 z-20
           flex flex-col items-center gap-px
           py-2 px-1
           bg-white/5 dark:bg-black/30 backdrop-blur-sm
           border border-white/6
           rounded-md"
    transition:fly={{ x: 10, duration: 200 }}
  >
    {#each LETTERS as letter}
      {@const available = availableLetters.has(letter)}
      {@const active = activeLetter === letter}
      <button
        type="button"
        class="w-5 h-4 flex items-center justify-center
               text-[10px] font-semibold tabular-nums
               rounded transition-all duration-150
               {active
                 ? 'bg-emerald-500 text-black scale-110'
                 : available
                   ? 'text-neutral-400 hover:text-white hover:bg-white/10 cursor-pointer'
                   : 'text-neutral-700 dark:text-neutral-700 cursor-default'}"
        disabled={!available}
        onclick={() => handleClick(letter)}
      >
        {letter}
      </button>
    {/each}
  </div>
{/if}
