<script lang="ts">
  import { toasts, type Toast } from "../../../stores/ui/toast.store"
  import { onDestroy } from "svelte";

  let timers = new Map<number, ReturnType<typeof setTimeout>>();

  function startTimer(toast: Toast) {
    // 💡 Petite sécurité au cas où toast.duration ne serait pas défini (ex: 3000ms par défaut)
    const duration = toast.duration || 4000; 
    
    const timer = setTimeout(() => {
      toasts.remove(toast.id);
    }, duration);

    timers.set(toast.id, timer);
  }

  function pauseTimer(id: number) {
    const timer = timers.get(id);
    if (timer) clearTimeout(timer);
  }

  function toastLifecycle(node: HTMLElement, toast: Toast) {
    startTimer(toast);

    return {
      destroy() {
        pauseTimer(toast.id);
      }
    };
  }

  onDestroy(() => {
    timers.forEach(clearTimeout);
  });
</script>
<div class="fixed top-4 right-4 z-50 flex w-[calc(100%-2rem)] max-w-sm flex-col gap-3">
  {#each $toasts as toast (toast.id)}
    <div
      use:toastLifecycle={toast}
      onmouseenter={() => pauseTimer(toast.id)}
      onmouseleave={() => startTimer(toast)}
      data-variant={toast.type}
      class="group relative overflow-hidden rounded-xl border shadow-lg backdrop-blur-xl
             bg-white/55 text-neutral-900 border-white/20
             dark:bg-neutral-950/45 dark:text-neutral-100 dark:border-white/10

             data-[variant=success]:bg-emerald-50/35
             data-[variant=error]:bg-rose-50/35
             data-[variant=warning]:bg-amber-50/35
             data-[variant=info]:bg-sky-50/35

             dark:data-[variant=success]:bg-emerald-500/10
             dark:data-[variant=error]:bg-rose-500/10
             dark:data-[variant=warning]:bg-amber-500/10
             dark:data-[variant=info]:bg-sky-500/10

             data-[variant=success]:border-emerald-400/40
             data-[variant=error]:border-rose-400/40
             data-[variant=warning]:border-amber-400/40
             data-[variant=info]:border-sky-400/40

             dark:data-[variant=success]:border-emerald-500/25
             dark:data-[variant=error]:border-rose-500/25
             dark:data-[variant=warning]:border-amber-500/25
             dark:data-[variant=info]:border-sky-500/25

             ring ring-black/5 dark:ring-white/5

             transition-all duration-200 ease-out
             hover:shadow-xl hover:-translate-y-px
             animate-[toast-in_.25s_ease-out]"
      role="status"
      >

      <div class="flex gap-3 p-4 pl-5">
        <div class="min-w-0 flex-1">
          <p class="text-sm font-semibold">{toast.title}</p>
          {#if toast.message}
            <p class="mt-1 text-sm text-neutral-600 dark:text-neutral-300">
              {toast.message}
            </p>
          {/if}
        </div>

        <button
          onclick={() => toasts.remove(toast.id)}
          class="ml-1 inline-flex h-9 w-9 items-center justify-center rounded-xl
                 text-neutral-500 hover:text-neutral-900 hover:bg-neutral-100
                 dark:text-neutral-400 dark:hover:text-white dark:hover:bg-white/10"
        >
          ✕
        </button>
      </div>

      <div 
        class="progress-bar absolute bottom-0 left-0 h-0.5 bg-black/20 dark:bg-white/30 
               group-hover:animate-none! group-hover:w-full"
        style="animation-duration: {toast.duration || 3000}ms;"
      ></div>

    </div>
  {/each}
</div>

<style>
  @keyframes toast-in {
    from { transform: translateY(-8px); opacity: 0 }
    to { transform: translateY(0); opacity: 1 }
  }

  /* La magie opère ici maintenant */
  .progress-bar {
    animation: toast-progress linear forwards;
  }

  @keyframes toast-progress {
    from { width: 100%; }
    to { width: 0%; }
  }
</style>