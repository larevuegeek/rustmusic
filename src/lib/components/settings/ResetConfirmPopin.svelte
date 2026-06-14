<script lang="ts">
  import { Dialog } from "@karbonjs/ui-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { profilSelector } from "$lib/stores/profil/profil.store";
  import { libraryStore } from "$lib/stores/library/library.store";
  import { playlistStore } from "$lib/stores/playlist/playlist.store";
  import { liked } from "$lib/stores/playlist/like.store";
  import { goto } from "$app/navigation";

  let { open = $bindable(true) }: { open: boolean } = $props();
  let loading = $state(false);

  async function handleConfirm() {
    loading = true;
    try {
      await invoke('reset_application');
      open = false;

      // Relancer l'app — la DB sera recréée par les migrations au démarrage
      try {
        const { relaunch } = await import('@tauri-apps/plugin-process');
        await relaunch();
      } catch {
        // En mode dev, relaunch peut échouer — on ferme simplement
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().close();
      }
    } catch (e) {
      console.error('Reset failed:', e);
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    open = false;
  }
</script>

<Dialog
  bind:open
  title="Réinitialiser l'application"
  variant="danger"
  backdrop="blur"
  classes={{
    overlay: '!bg-black/70',
    content: '!bg-neutral-950 !border-neutral-800/60'
  }}
  confirmLabel="Réinitialiser tout"
  cancelLabel="Annuler"
  confirmInput="SUPPRIMER"
  confirmInputLabel="Pour confirmer, tapez SUPPRIMER en majuscules :"
  confirmInputPlaceholder="SUPPRIMER"
  {loading}
  onconfirm={handleConfirm}
  oncancel={handleCancel}
>
  <div class="space-y-3 text-sm">
    <p class="text-neutral-300 leading-relaxed">
      Cette action est <span class="font-bold text-red-400">irréversible</span>. Toutes les données suivantes seront définitivement supprimées :
    </p>

    <ul class="space-y-1.5 text-neutral-400">
      <li class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-red-500/60 shrink-0"></span>
        Toutes vos <span class="text-neutral-200 font-medium">bibliothèques</span> et fichiers importés
      </li>
      <li class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-red-500/60 shrink-0"></span>
        Toutes vos <span class="text-neutral-200 font-medium">playlists</span> et leur contenu
      </li>
      <li class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-red-500/60 shrink-0"></span>
        Vos <span class="text-neutral-200 font-medium">titres likés</span> et historique de lecture
      </li>
      <li class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-red-500/60 shrink-0"></span>
        Tous les <span class="text-neutral-200 font-medium">paramètres</span> personnalisés
      </li>
      <li class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-red-500/60 shrink-0"></span>
        Les <span class="text-neutral-200 font-medium">miniatures</span> et cache audio
      </li>
    </ul>

    <p class="text-xs text-neutral-500 pt-1">
      Seul le profil administrateur par défaut sera conservé.
    </p>
  </div>
</Dialog>
