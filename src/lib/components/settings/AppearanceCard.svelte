<script lang="ts">
  import Icon from "@iconify/svelte";

  let {
    label,
    selected = false,
    onclick = () => {},
    children,
  }: {
    label: string;
    selected?: boolean;
    onclick?: () => void;
    children: any;
  } = $props();
</script>

<button
  type="button"
  {onclick}
  class="group relative flex flex-col gap-1.5 p-2 rounded-lg
         border transition-all duration-150 cursor-pointer
         {selected
           ? 'border-green-500 bg-green-500/5'
           : 'border-neutral-200 dark:border-neutral-700/40 hover:border-neutral-300 dark:hover:border-neutral-600/60 hover:bg-neutral-50 dark:hover:bg-white/2'}"
>
  <!-- Aperçu visuel (compact) -->
  <div class="w-full aspect-5/2 rounded-md overflow-hidden flex items-center justify-center
              bg-neutral-100 dark:bg-neutral-900/60 border border-neutral-200/50 dark:border-white/5">
    {@render children()}
  </div>

  <!-- Label + check -->
  <div class="flex items-center justify-center gap-1">
    <span class="text-[11px] font-medium {selected ? 'text-green-600 dark:text-green-400' : 'text-neutral-700 dark:text-neutral-300'}">
      {label}
    </span>
    {#if selected}
      <Icon icon="lucide:check" width={11} class="text-green-500" />
    {/if}
  </div>
</button>
