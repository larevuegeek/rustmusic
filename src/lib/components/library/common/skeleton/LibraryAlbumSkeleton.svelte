<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 2xl:grid-cols-6 min-[1800px]:grid-cols-7 min-[2200px]:grid-cols-8 xl:grid-cols-6 2xl:grid-cols-7 gap-6 p-10">

  {#each Array(12) as _, i (i)}
    <div class="flex flex-col">
      <div class="aspect-square rounded-xl overflow-hidden
                  bg-neutral-200 dark:bg-neutral-800
                  relative">
        <div class="absolute inset-0 -translate-x-full
                    bg-linear-to-r
                    from-transparent
                    via-white/40
                    to-transparent
                    dark:via-white/10
                    animate-[shimmer_1.4s_infinite]">
        </div>
      </div>
      <div class="mt-3 flex flex-col gap-2">
        <div class="h-4 w-3/4 rounded-md
                    bg-neutral-200 dark:bg-neutral-800
                    animate-pulse">
        </div>
        <div class="h-3 w-1/2 rounded-md
                    bg-neutral-200 dark:bg-neutral-800
                    animate-pulse">
        </div>
        <div class="h-3 w-2/3 rounded-md
                    bg-neutral-200 dark:bg-neutral-800
                    animate-pulse">
        </div>
      </div>
    </div>
  {/each}
</div>