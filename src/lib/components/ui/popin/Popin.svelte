<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import { popinStore } from "$lib/stores/ui/popin.store";
</script>

{#if $popinStore.isOpen}
  <div
    role="presentation"
    class="fixed inset-0 z-50 flex items-center justify-center
           bg-black/50 backdrop-blur-sm"
    transition:fade={{ duration: 150 }}
    onclick={() => popinStore.close()}
  >
    <div
      role="presentation"
      class="w-full max-w-md mx-4 p-6 rounded-lg border
             bg-white border-neutral-200
             dark:bg-neutral-900 dark:border-neutral-800"
      transition:scale={{ duration: 180, start: 0.96 }}
      onclick={(e) => e.stopPropagation()}
    >
      <h3 class="text-lg font-semibold mb-4 text-neutral-900 dark:text-white">
        {$popinStore.title}
      </h3>

      {#if $popinStore.component}
        <svelte:component
          this={$popinStore.component}
          {...$popinStore.props}
        />
      {/if}
    </div>
</div>
{/if}