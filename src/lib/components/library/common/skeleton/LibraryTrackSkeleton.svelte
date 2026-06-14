<script lang="ts">
  let { rows = 10 }: { rows?: number } = $props();
</script>
{#each Array(rows) as _, i}
<div
  class="flex items-center justify-between py-3 px-3 rounded-md
          animate-pulse"
>
  <!-- LEFT -->
  <div class="flex items-center gap-4 min-w-0">

    <!-- Track number -->
    <div class="w-6 h-3 bg-neutral-200 dark:bg-neutral-700 rounded shrink-0"></div>

    <!-- Thumbnail -->
    <div class="w-12 h-12 rounded-md bg-neutral-200 dark:bg-neutral-700 shrink-0"></div>

    <!-- TEXT -->
    <div class="flex flex-col gap-2 min-w-0 flex-1">

      <!-- Title -->
      <div class="h-4 w-48 bg-neutral-200 dark:bg-neutral-700 rounded"></div>

      <!-- Artist / Album -->
      <div class="h-3 w-64 bg-neutral-200 dark:bg-neutral-700 rounded"></div>

      <!-- Meta -->
      <div class="h-3 w-32 bg-neutral-200 dark:bg-neutral-700 rounded"></div>

    </div>
  </div>

  <!-- RIGHT -->
  <div class="flex items-center gap-6 shrink-0">

    <div class="h-3 w-10 bg-neutral-200 dark:bg-neutral-700 rounded"></div>
    <div class="h-3 w-12 bg-neutral-200 dark:bg-neutral-700 rounded"></div>
    <div class="w-4 h-4 bg-neutral-200 dark:bg-neutral-700 rounded"></div>
    <div class="w-6 h-6 bg-neutral-200 dark:bg-neutral-700 rounded"></div>

  </div>
</div>
{/each}