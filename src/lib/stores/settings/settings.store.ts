import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { enable as enableAutostart, disable as disableAutostart, isEnabled as isAutostartEnabled } from "@tauri-apps/plugin-autostart";
import { applyThemeMode, type ThemeMode } from "$lib/helper/theme/theme";

export type AppSettings = {
  language: string;
  auto_start: string;
  minimize_to_tray: string;
  show_notifications: string;
  scan_on_startup: string;
  audio_quality: string;
  hardware_acceleration: string;
  single_click_play: string;
  system_media_controls: string;     // 'true' | 'false' — SMTC / MPRIS / Now Playing
  theme: string;                     // 'auto' | 'light' | 'dark'
  window_controls_style: string;     // 'auto' | 'macos' | 'windows'
  window_controls_position: string;  // 'right' | 'left'
};

const defaults: AppSettings = {
  language: 'fr',
  auto_start: 'false',
  minimize_to_tray: 'true',
  show_notifications: 'false',
  scan_on_startup: 'false',
  audio_quality: 'high',
  hardware_acceleration: 'true',
  single_click_play: 'false',
  system_media_controls: 'true',
  theme: 'dark',
  window_controls_style: 'auto',
  window_controls_position: 'right',
};

// Actions spéciales par clé — exécutées APRÈS la sauvegarde en BDD
// C'est ici qu'on câble les effets réels de chaque paramètre
const sideEffects: Partial<Record<keyof AppSettings, (value: string) => Promise<void>>> = {

  // Lancement au démarrage : on appelle le plugin Tauri autostart
  // Il écrit/supprime l'entrée dans le registre Windows (ou LaunchAgent macOS)
  auto_start: async (value: string) => {
    try {
      if (value === 'true') {
        await enableAutostart();
        console.log('[settings] Autostart activé');
      } else {
        await disableAutostart();
        console.log('[settings] Autostart désactivé');
      }
    } catch (e) {
      console.error('[settings] Erreur autostart:', e);
    }
  },

  // Thème : applique la classe `.dark` sur <html> et écoute la préférence
  // système si mode 'auto'. Appelé aussi au chargement initial.
  theme: async (value: string) => {
    applyThemeMode((value as ThemeMode) ?? 'auto');
  },

  // SMTC / MPRIS / Now Playing : on appelle le service qui parle au backend
  // souvlaki pour enregistrer/désenregistrer l'app auprès de l'OS.
  // Import dynamique pour éviter une dépendance circulaire au load.
  system_media_controls: async (_value: string) => {
    try {
      const { mediaControlsService } = await import('$lib/services/mediaControls/mediaControls.service');
      await mediaControlsService.sync();
    } catch (e) {
      console.error('[settings] Erreur sync SMTC:', e);
    }
  },
};

const settingsWriter = writable<AppSettings>({ ...defaults });

export const settingsStore = {
  subscribe: settingsWriter.subscribe,

  // Chargement initial depuis la BDD
  // On synchronise aussi l'état réel de l'autostart avec ce que dit le registre
  init: async () => {
    try {
      const all = await invoke<Record<string, string>>('get_all_settings');

      // Vérifier l'état réel de l'autostart (le registre peut avoir été modifié manuellement)
      try {
        const realAutostart = await isAutostartEnabled();
        all['auto_start'] = realAutostart ? 'true' : 'false';
      } catch (e) {
        console.warn('[settings] Impossible de vérifier autostart:', e);
      }

      settingsWriter.update(state => ({
        ...state,
        ...Object.fromEntries(
          Object.entries(all).filter(([key]) => key in defaults)
        ),
      }));

      // Applique le thème dès le chargement
      const finalTheme = get(settingsWriter).theme;
      applyThemeMode((finalTheme as ThemeMode) ?? 'dark');
    } catch (e) {
      console.error('[settingsStore] Failed to load settings', e);
    }
  },

  // Sauvegarder un paramètre :
  // 1. Met à jour le store Svelte (UI réactive immédiatement)
  // 2. Persiste en BDD via la commande Tauri
  // 3. Exécute l'effet de bord si défini (ex: activer/désactiver autostart)
  set: async (key: keyof AppSettings, value: string) => {
    settingsWriter.update(state => ({ ...state, [key]: value }));

    try {
      await invoke('set_setting', { key, value });
    } catch (e) {
      console.error(`[settingsStore] Failed to save ${key}`, e);
    }

    // Exécuter l'effet de bord (autostart, notifications, etc.)
    const effect = sideEffects[key];
    if (effect) {
      await effect(value);
    }
  },

  get: (key: keyof AppSettings): string => {
    return get(settingsWriter)[key];
  },

  // Toggle un booléen : inverse la valeur et appelle set()
  toggle: async (key: keyof AppSettings) => {
    const current = get(settingsWriter)[key];
    const next = current === 'true' ? 'false' : 'true';
    await settingsStore.set(key, next);
  },
};
