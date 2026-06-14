<script lang="ts">
  import Icon from "@iconify/svelte";
  import { t } from "$lib/i18n";
  import { check } from "@tauri-apps/plugin-updater";

  const APP_VERSION = "0.1.3";
  const BUILD_DATE = "Avril 2026";

  let showLicense = $state(false);

  let updateStatus: 'idle' | 'checking' | 'available' | 'latest' | 'error' = $state('idle');
  let updateVersion = $state('');

  async function checkForUpdates() {
    updateStatus = 'checking';
    try {
      const update = await check();
      if (update) {
        updateStatus = 'available';
        updateVersion = update.version;
      } else {
        updateStatus = 'latest';
      }
    } catch (e) {
      console.warn('Update check failed:', e);
      updateStatus = 'error';
    }
  }

  const techStack = {
    "Framework Desktop": [
      { name: "Tauri", version: "2.x", url: "https://tauri.app", icon: "simple-icons:tauri" },
    ],
    "Frontend": [
      { name: "SvelteKit", version: "2.x", url: "https://kit.svelte.dev", icon: "simple-icons:svelte" },
      { name: "Svelte", version: "5.x", url: "https://svelte.dev", icon: "simple-icons:svelte" },
      { name: "TypeScript", version: "5.9", url: "https://typescriptlang.org", icon: "simple-icons:typescript" },
      { name: "Tailwind CSS", version: "4.x", url: "https://tailwindcss.com", icon: "simple-icons:tailwindcss" },
      { name: "Vite", version: "8.x", url: "https://vite.dev", icon: "simple-icons:vite" },
      { name: "KarbonJS UI", version: "0.3", url: "https://npmjs.com/package/@karbonjs/ui-svelte", icon: "lucide:box" },
    ],
    "Backend (Rust)": [
      { name: "Rust", version: "2021 ed.", url: "https://rust-lang.org", icon: "simple-icons:rust" },
      { name: "SQLx", version: "0.8", url: "https://github.com/launchbadge/sqlx", icon: "lucide:database" },
      { name: "SQLite", version: "3.x", url: "https://sqlite.org", icon: "simple-icons:sqlite" },
      { name: "Tokio", version: "1.x", url: "https://tokio.rs", icon: "lucide:zap" },
    ],
    "Audio": [
      { name: "Symphonia", version: "0.5", url: "https://github.com/pdeljanov/Symphonia", icon: "lucide:music" },
      { name: "CPAL", version: "0.17", url: "https://github.com/RustAudio/cpal", icon: "lucide:speaker" },
      { name: "Rubato", version: "1.0", url: "https://github.com/HEnquist/rubato", icon: "lucide:waves" },
    ],
  };

  let shortcuts = $derived([
    { keys: "Espace", action: $t('about.shortcut_play_pause') },
    { keys: "←", action: $t('about.shortcut_prev') },
    { keys: "→", action: $t('about.shortcut_next') },
    { keys: "↑ / ↓", action: $t('about.shortcut_volume') },
    { keys: "M", action: $t('about.shortcut_mute') },
  ]);
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

  <div class="space-y-8">

    <!-- ═══ APP INFO ═══ -->
    <div class="relative p-8 rounded-2xl overflow-hidden
                bg-neutral-950 border border-neutral-800/40">
      <!-- Halo vert -->
      <div class="absolute -top-32 -right-32 w-72 h-72 rounded-full blur-[80px] opacity-[0.10] pointer-events-none bg-green-500"></div>

      <div class="relative flex items-center justify-between">
        <div>
          <h2 class="text-3xl font-black tracking-tight text-white">
            Rust<span class="text-green-500">Music</span>
          </h2>
          <p class="text-sm text-neutral-500 mt-1">
            {$t('about.tagline')}
          </p>

          <div class="flex items-center gap-3 mt-4">
            <span class="px-2 py-0.5 rounded-md text-xs font-semibold bg-green-500/15 text-green-400 border border-green-500/20">
              v{APP_VERSION}
            </span>
            <span class="text-xs text-neutral-500">{BUILD_DATE}</span>
            <span class="text-xs text-neutral-600">·</span>
            <span class="text-xs text-neutral-500">{$t('about.license_short')}</span>
          </div>

          <!-- Bouton mise à jour -->
          <div class="mt-4">
            {#if updateStatus === 'idle'}
              <button
                type="button"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer
                       bg-white/5 border border-white/10 text-neutral-300
                       hover:bg-white/10 hover:border-white/20 transition-all"
                onclick={checkForUpdates}
              >
                <Icon icon="lucide:refresh-cw" width={12} />
                Vérifier les mises à jour
              </button>
            {:else if updateStatus === 'checking'}
              <span class="inline-flex items-center gap-1.5 text-xs text-neutral-400">
                <Icon icon="lucide:loader-2" width={12} class="animate-spin" />
                Vérification...
              </span>
            {:else if updateStatus === 'available'}
              <span class="inline-flex items-center gap-1.5 text-xs text-green-400">
                <Icon icon="lucide:download" width={12} />
                Version {updateVersion} disponible
              </span>
            {:else if updateStatus === 'latest'}
              <span class="inline-flex items-center gap-1.5 text-xs text-neutral-400">
                <Icon icon="lucide:check-circle" width={12} class="text-green-500" />
                Vous êtes à jour
              </span>
            {:else if updateStatus === 'error'}
              <span class="inline-flex items-center gap-1.5 text-xs text-neutral-500">
                <Icon icon="lucide:wifi-off" width={12} />
                Vérification impossible
              </span>
            {/if}
          </div>

          <p class="text-xs text-neutral-500 mt-3">
            {$t('about.developed_by')}
            <a href="https://larevuegeek.fr" target="_blank" rel="noopener noreferrer"
               class="font-semibold text-neutral-300 hover:text-green-400 transition-colors">
              LaRevueGeeK
            </a>
          </p>
        </div>

        <!-- Version badge décoratif -->
        <div class="text-[80px] font-black text-green-500/[0.04] leading-none select-none pointer-events-none">
          {APP_VERSION}
        </div>
      </div>
    </div>

    <!-- Séparateur -->
    <div class="h-px bg-gradient-to-r from-transparent via-neutral-200/80 dark:via-neutral-700/30 to-transparent"></div>

    <!-- ═══ STACK TECHNIQUE ═══ -->
    {#each Object.entries(techStack) as [category, libs]}
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-400 dark:text-neutral-500 mb-3">
          {category}
        </h2>

        <div class="grid grid-cols-2 gap-1.5">
          {#each libs as lib}
            <a
              href={lib.url}
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center gap-2.5 px-3 py-2.5 rounded-xl
                     hover:bg-neutral-50 dark:hover:bg-white/[0.03]
                     transition-colors group cursor-pointer"
            >
              <Icon icon={lib.icon} width="16"
                    class="text-neutral-400 dark:text-neutral-500 group-hover:text-neutral-600 dark:group-hover:text-neutral-300 transition-colors" />
              <div class="flex-1 min-w-0">
                <p class="text-[13px] font-medium text-neutral-700 dark:text-neutral-300 truncate">
                  {lib.name}
                </p>
              </div>
              <span class="text-[10px] tabular-nums text-neutral-400 dark:text-neutral-500 font-mono">
                {lib.version}
              </span>
            </a>
          {/each}
        </div>
      </section>
    {/each}

    <!-- Séparateur -->
    <div class="h-px bg-gradient-to-r from-transparent via-neutral-200/80 dark:via-neutral-700/30 to-transparent"></div>

    <!-- ═══ RACCOURCIS ═══ -->
    <section>
      <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-400 dark:text-neutral-500 mb-3">
        {$t('about.shortcuts')}
      </h2>

      <div class="space-y-1">
        {#each shortcuts as shortcut}
          <div class="flex items-center justify-between px-3 py-2 rounded-xl
                      hover:bg-neutral-50 dark:hover:bg-white/[0.02] transition-colors">
            <span class="text-[13px] text-neutral-600 dark:text-neutral-400">
              {shortcut.action}
            </span>
            <kbd class="px-2 py-0.5 rounded-md text-[11px] font-mono font-medium
                        bg-neutral-100 dark:bg-neutral-800
                        text-neutral-600 dark:text-neutral-400
                        border border-neutral-200 dark:border-neutral-700">
              {shortcut.keys}
            </kbd>
          </div>
        {/each}
      </div>
    </section>

    <!-- Séparateur -->
    <div class="h-px bg-gradient-to-r from-transparent via-neutral-200/80 dark:via-neutral-700/30 to-transparent"></div>

    <!-- ═══ LICENCE ═══ -->
    <section>
      <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-400 dark:text-neutral-500 mb-3">
        Licence
      </h2>

      <div class="rounded-xl bg-neutral-50 dark:bg-neutral-900/60
                  border border-neutral-200/40 dark:border-neutral-800/60 overflow-hidden">
        <div class="px-4 py-3">
          <p class="text-sm text-neutral-600 dark:text-neutral-400 leading-relaxed">
            RustMusic est un logiciel gratuit pour un usage personnel. La redistribution, la modification et l'usage commercial sont interdits sans autorisation.
          </p>
          <button
            type="button"
            class="mt-2 text-xs text-green-500 hover:text-green-400 cursor-pointer transition-colors"
            onclick={() => showLicense = !showLicense}
          >
            {showLicense ? 'Masquer la licence' : 'Lire la licence complète'}
          </button>
        </div>

        {#if showLicense}
          <div class="px-4 pb-4 pt-2 border-t border-neutral-200/30 dark:border-white/5">
            <pre class="text-[11px] text-neutral-500 dark:text-neutral-500 leading-relaxed whitespace-pre-wrap font-sans">Copyright (c) 2026 LaRevueGeeK. Tous droits reserves.

1. AUTORISATION D'UTILISATION
Le logiciel RustMusic est mis a disposition gratuitement pour un usage personnel et non commercial. Vous etes autorise a telecharger, installer et utiliser le logiciel sur vos propres appareils.

2. RESTRICTIONS
Il est interdit de :
- Redistribuer, vendre ou sous-licencier le logiciel
- Modifier, decompiler ou desassembler le logiciel
- Utiliser le logiciel a des fins commerciales sans autorisation
- Supprimer ou modifier les mentions de droits d'auteur

3. PROPRIETE INTELLECTUELLE
Le logiciel, son code source et son interface restent la propriete exclusive de LaRevueGeeK.

4. ABSENCE DE GARANTIE
Le logiciel est fourni "tel quel", sans garantie d'aucune sorte.

5. LOGICIELS TIERS
RustMusic utilise des bibliotheques open-source sous licences MIT, Apache-2.0, BSD, MPL-2.0 et similaires.

6. CONTACT
contact@rustmusic.dev</pre>
          </div>
        {/if}
      </div>
    </section>

    <!-- Séparateur -->
    <div class="h-px bg-gradient-to-r from-transparent via-neutral-200/80 dark:via-neutral-700/30 to-transparent"></div>

    <!-- ═══ CRÉDITS ═══ -->
    <section class="pb-8">
      <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-400 dark:text-neutral-500 mb-3">
        {$t('about.credits')}
      </h2>

      <div class="px-3 py-3 rounded-xl bg-neutral-50 dark:bg-neutral-900/60
                  border border-neutral-200/40 dark:border-neutral-800/60">
        <p class="text-sm text-neutral-600 dark:text-neutral-400 leading-relaxed">
          {$t('about.credits_text')}
        </p>
        <p class="text-xs text-neutral-400 dark:text-neutral-500 mt-2">
          {$t('about.icons_credits')}
        </p>
      </div>
    </section>

  </div>
</div>
