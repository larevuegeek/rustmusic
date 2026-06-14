import { writable } from "svelte/store";

export type ViewMode = "grid" | "list";

const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('viewMode') : null;

const viewModeWriter = writable<ViewMode>((stored as ViewMode) ?? "grid");

viewModeWriter.subscribe(value => {
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('viewMode', value);
  }
});

export const viewMode = {
  subscribe: viewModeWriter.subscribe,
  toggle: () => viewModeWriter.update(v => v === "grid" ? "list" : "grid"),
  set: (mode: ViewMode) => viewModeWriter.set(mode),
};
