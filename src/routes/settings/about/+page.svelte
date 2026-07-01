<script lang="ts">
  import Icon from "@iconify/svelte";
  import { t } from "$lib/i18n";
  import AboutContent from "$lib/components/settings/AboutContent.svelte";
</script>

<div class="py-6 px-10 scrollbar-app overflow-y-auto" style="height: calc(100vh - 290px);">

  <!-- Header -->
  <div class="flex items-center gap-3 mb-8">
    <button
      class="w-8 h-8 flex items-center justify-center rounded-lg cursor-pointer
             text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-200
             hover:bg-neutral-100 dark:hover:bg-neutral-800
             transition-all"
      onclick={() => history.back()}
    >
      <Icon icon="lucide:arrow-left" width="16" />
    </button>
    <div>
      <h1 class="text-2xl font-bold tracking-tight text-neutral-900 dark:text-neutral-100">
        {$t('about.title')}
      </h1>
      <div class="mt-1 h-0.5 w-10 rounded-full bg-green-500"></div>
    </div>
  </div>

  <AboutContent />
</div>
