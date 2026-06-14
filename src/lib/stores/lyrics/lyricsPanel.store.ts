import { writable } from "svelte/store";

const STORAGE_KEY = "lyrics-panel-open";

const initial = (() => {
    try { return localStorage.getItem(STORAGE_KEY) === "true"; }
    catch { return false; }
})();

export const lyricsPanelOpened = writable<boolean>(initial);

lyricsPanelOpened.subscribe((v) => {
    try { localStorage.setItem(STORAGE_KEY, v ? "true" : "false"); }
    catch {}
});

export function openLyricsPanel() {
    lyricsPanelOpened.set(true);
}
export function closeLyricsPanel() {
    lyricsPanelOpened.set(false);
}
export function toggleLyricsPanel() {
    lyricsPanelOpened.update((v) => !v);
}
