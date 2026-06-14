import { writable, derived, get } from "svelte/store";
import type { TrackLike } from "$lib/helper/tools/queueTools";

type SelectionState = {
  active: boolean;
  tracks: Map<string, TrackLike>; // id → track data
};

const state = writable<SelectionState>({
  active: false,
  tracks: new Map(),
});

export const selectionStore = {
  subscribe: derived(state, ($s) => ({
    active: $s.active,
    tracks: $s.tracks,
    count: $s.tracks.size,
    ids: new Set($s.tracks.keys()),
  })).subscribe,

  /** Enter selection mode */
  start() {
    state.update((s) => ({ ...s, active: true, tracks: new Map() }));
  },

  /** Exit selection mode and clear */
  stop() {
    state.set({ active: false, tracks: new Map() });
  },

  /** Toggle a track in/out of selection */
  toggle(id: string, track: TrackLike) {
    state.update((s) => {
      const next = new Map(s.tracks);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.set(id, track);
      }
      return { ...s, tracks: next };
    });
  },

  /** Check if a track is selected */
  has(id: string): boolean {
    return get(state).tracks.has(id);
  },

  /** Select all from a list */
  selectAll(items: { id: string; track: TrackLike }[]) {
    state.update((s) => {
      const next = new Map(s.tracks);
      for (const item of items) {
        next.set(item.id, item.track);
      }
      return { ...s, tracks: next };
    });
  },

  /** Deselect all */
  deselectAll() {
    state.update((s) => ({ ...s, tracks: new Map() }));
  },

  /** Get all selected tracks as array */
  getSelectedTracks(): TrackLike[] {
    return Array.from(get(state).tracks.values());
  },
};
