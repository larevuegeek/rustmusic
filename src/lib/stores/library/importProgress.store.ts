import { writable } from "svelte/store";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { dataCache } from "$lib/stores/cache/dataCache.store";
import { libraryContentStore } from "$lib/stores/library/libraryContent.store";

export type ImportProgressState = {
  active: boolean;
  current: number;
  total: number;
  percent: number;
  fileName: string;
  elapsedMs: number;
};

export type ImportCompletePayload = {
  total: number;
  duration_ms: number;
};

export type ImportProgressPayload = {
  current: number;
  total: number;
  percent: number;
  file_name: string;
};

const initialState: ImportProgressState = {
  active: false,
  current: 0,
  total: 0,
  percent: 0,
  fileName: '',
  elapsedMs: 0,
};

const importWriter = writable<ImportProgressState>(initialState);

let unlistenProgress: UnlistenFn | null = null;
let unlistenComplete: UnlistenFn | null = null;
let startTime = 0;

export const importProgressStore = {
  subscribe: importWriter.subscribe,

  init: async () => {
    // Écouter les events de progression
    unlistenProgress = await listen<ImportProgressPayload>('import-progress', (event) => {
      // Auto-start le timer au premier event si pas déjà lancé
      if (startTime === 0) startTime = Date.now();

      importWriter.set({
        active: true,
        current: event.payload.current,
        total: event.payload.total,
        percent: event.payload.percent,
        fileName: event.payload.file_name,
        elapsedMs: Date.now() - startTime,
      });
    });

    // Écouter la fin d'import
    unlistenComplete = await listen<ImportCompletePayload>('import-complete', (event) => {
      importWriter.update(state => ({
        ...state,
        active: false,
        percent: 100,
        elapsedMs: event.payload.duration_ms,
      }));

      // Invalider le cache et rafraîchir les données immédiatement
      dataCache.invalidateAll();
      libraryContentStore.refresh();

      // Reset après 3 secondes
      setTimeout(() => {
        importWriter.set(initialState);
        startTime = 0;
      }, 3000);
    });
  },

  start: () => {
    startTime = Date.now();
    importWriter.set({ ...initialState, active: true });
  },

  destroy: () => {
    unlistenProgress?.();
    unlistenComplete?.();
  }
};
