<script lang="ts">
  let { position = 0, onseek }: { position?: number; onseek?: (percent: number) => void } = $props();

  let localPosition = $state(0);
  let seeking = $state(false);
  let seekTarget = $state(-1);

  // Synchroniser avec la prop quand on ne seek pas
  $effect(() => {
    if (seeking) {
      // Pendant un seek, vérifier si la position prop a rattrapé la cible
      if (seekTarget >= 0 && Math.abs(position - seekTarget) < 2) {
        seeking = false;
        seekTarget = -1;
        localPosition = position;
      }
    } else {
      localPosition = position;
    }
  });

  function handleInput(e: Event) {
    seeking = true;
    localPosition = Number((e.target as HTMLInputElement).value);
  }

  function handleChange(e: Event) {
    const value = Number((e.target as HTMLInputElement).value);
    localPosition = value;
    seekTarget = value; // On attend que la prop rattrape cette valeur
    onseek?.(value);
  }
</script>

<div class="w-full group/progress relative">
  <div class="relative h-1 group-hover/progress:h-1.5 rounded-full bg-neutral-300 dark:bg-neutral-700/40
              transition-all duration-150 overflow-hidden">
    <!-- Fill -->
    <div
      class="absolute inset-y-0 left-0 rounded-full bg-emerald-500
             transition-[width] duration-75"
      style="width: {localPosition}%"
    ></div>

    <!-- Glow on fill end -->
    <div
      class="absolute top-1/2 -translate-y-1/2 w-3 h-3 rounded-full
             bg-emerald-400 blur-sm opacity-0 group-hover/progress:opacity-60
             transition-opacity duration-150"
      style="left: calc({localPosition}% - 6px)"
    ></div>
  </div>

  <!-- Thumb -->
  <div
    class="absolute top-1/2 -translate-y-1/2 w-3 h-3 rounded-full
           bg-white shadow-md shadow-black/20
           border-2 border-emerald-500
           opacity-0 group-hover/progress:opacity-100 scale-75 group-hover/progress:scale-100
           transition-all duration-150 pointer-events-none"
    style="left: calc({localPosition}% - 6px)"
  ></div>

  <!-- Range invisible -->
  <input
    type="range"
    min="0"
    max="100"
    step="0.1"
    value={localPosition}
    oninput={handleInput}
    onchange={handleChange}
    class="absolute inset-0 w-full h-4 -top-1.5 cursor-pointer opacity-0"
    aria-label="Progression"
  />
</div>
