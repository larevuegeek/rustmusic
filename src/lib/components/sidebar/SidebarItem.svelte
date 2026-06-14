<script lang="ts">
    import Icon from "@iconify/svelte";

  const props = $props<{
    class?: string;
    onclick?: (e: MouseEvent) => void;
    children?: any;
    icon?: string | null;
    active?: boolean;
  }>();
</script>

<button
  class="group relative flex items-center gap-2.5 w-full rounded-lg font-medium justify-start cursor-pointer text-[13px]
         px-3 py-2 my-0.5 transition-all duration-150
         {props.active
           ? 'bg-green-500/10 text-green-600 dark:text-green-400'
           : 'text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-neutral-100 hover:bg-white/70 dark:hover:bg-white/4'}
         {props.class ?? ''}"
  onclick={props.onclick}
  aria-current={props.active ? "page" : undefined}
>
  <!-- Barre indicateur actif -->
  {#if props.active}
    <div class="absolute left-0 top-1/2 -translate-y-1/2 w-0.75 h-4 rounded-r-full bg-green-500"></div>
  {/if}

  {#if props.icon}
    <Icon icon={props.icon} width="18" class="shrink-0" />
  {/if}
  <span class="truncate">{@render props.children?.()}</span>
</button>
