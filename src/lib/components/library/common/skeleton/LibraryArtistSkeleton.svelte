<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 2xl:grid-cols-6 min-[1800px]:grid-cols-7 min-[2200px]:grid-cols-8 xl:grid-cols-6 2xl:grid-cols-7 gap-6 p-10">
  {#each Array(12) as _, i (i)}
    <div class="flex flex-col items-center text-center">
        <div class="w-40 h-40 rounded-full
                    bg-neutral-200 dark:bg-neutral-800
                    relative overflow-hidden">

        <div class="absolute inset-0 -translate-x-full
                    bg-linear-to-r
                    from-transparent
                    via-white/40
                    to-transparent
                    dark:via-white/10
                    animate-[shimmer_1.4s_infinite]">
        </div>
        </div>
      <div class="mt-4 h-4 w-28 rounded-md
                  bg-neutral-200 dark:bg-neutral-800
                  animate-pulse">
      </div>
      <div class="mt-2 h-3 w-20 rounded-md
                  bg-neutral-200 dark:bg-neutral-800
                  animate-pulse">
      </div>
    </div>
  {/each}
</div>
