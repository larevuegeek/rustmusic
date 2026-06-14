import { writable } from "svelte/store";

export const queuePanelOpened = writable(false);

export function openQueuePanel() {
  queuePanelOpened.set(true);
}
export function closeQueuePanel() {
  queuePanelOpened.set(false);
}
export function toggleQueuePanel() {
  queuePanelOpened.update(v => !v);
}