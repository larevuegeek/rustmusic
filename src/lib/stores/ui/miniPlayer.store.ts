/**
 * Mode « mini-player » : réduit la fenêtre principale en un lecteur compact
 * toujours au premier plan (always-on-top). Bascule aller/retour en gardant
 * la taille précédente pour la restaurer.
 *
 * Le panneau déroulant (file d'attente / paroles) agrandit la fenêtre VERS LE
 * BAS avec une animation de hauteur (easeOutCubic) pour un effet fluide.
 */

import { writable, get } from "svelte/store";
import { getCurrentWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";

export const miniPlayerActive = writable(false);

/** Largeur fixe (points logiques) + hauteur du panneau déroulé qd ouvert. */
const MINI_W = 392;
/** Hauteur de repli par défaut, remplacée par la mesure réelle du contenu. */
const MINI_H = 150;
/** Hauteur ajoutée par le panneau (file d'attente / paroles) quand déroulé. */
const PANEL_H = 236;

/** Taille physique sauvegardée avant d'entrer en mode mini (pour restaurer). */
let savedSize: PhysicalSize | null = null;
/** Hauteur logique courante (suivie pour animer depuis la bonne valeur). */
let currentH = MINI_H;
/** Hauteur réelle du contenu replié, mesurée par le composant (ResizeObserver). */
let collapsedH = MINI_H;
/** Panneau déroulé ou non (pour réajuster si le contenu change de taille). */
let expanded = false;
let resizeRaf = 0;

async function setH(h: number): Promise<void> {
  currentH = h;
  try {
    await getCurrentWindow().setSize(new LogicalSize(MINI_W, h));
  } catch (e) {
    console.error("[mini-player] setSize failed:", e);
  }
}

export async function enterMiniPlayer(): Promise<void> {
  if (get(miniPlayerActive)) return;
  try {
    const win = getCurrentWindow();
    savedSize = await win.innerSize();
    await win.setAlwaysOnTop(true);
    await win.setResizable(false);
    expanded = false;
    currentH = collapsedH;
    await win.setSize(new LogicalSize(MINI_W, collapsedH));
    miniPlayerActive.set(true);
  } catch (e) {
    console.error("[mini-player] enter failed:", e);
  }
}

export async function exitMiniPlayer(): Promise<void> {
  if (!get(miniPlayerActive)) return;
  cancelAnimationFrame(resizeRaf);
  try {
    const win = getCurrentWindow();
    await win.setAlwaysOnTop(false);
    await win.setResizable(true);
    if (savedSize) await win.setSize(savedSize);
    else await win.setSize(new LogicalSize(1200, 800));
    miniPlayerActive.set(false);
  } catch (e) {
    console.error("[mini-player] exit failed:", e);
  }
}

export async function toggleMiniPlayer(): Promise<void> {
  if (get(miniPlayerActive)) await exitMiniPlayer();
  else await enterMiniPlayer();
}

/**
 * Anime la hauteur de la fenêtre mini de la valeur courante vers `to`
 * (easeOutCubic, ~220 ms). Donne l'effet de déroulement fluide du panneau.
 */
function animateMiniHeight(to: number): void {
  cancelAnimationFrame(resizeRaf);
  const from = currentH;
  if (from === to) return;
  const start = Date.now();
  const dur = 220;
  const step = () => {
    const tt = Math.min(1, (Date.now() - start) / dur);
    const eased = 1 - Math.pow(1 - tt, 3); // easeOutCubic
    const h = Math.round(from + (to - from) * eased);
    void setH(h);
    if (tt < 1) resizeRaf = requestAnimationFrame(step);
  };
  step();
}

/** Ouvre (déroulé) ou ferme le panneau, avec animation. */
export function setMiniExpanded(exp: boolean): void {
  expanded = exp;
  animateMiniHeight(exp ? collapsedH + PANEL_H : collapsedH);
}

/**
 * Signale la hauteur réelle du contenu replié (top bar + lecteur + progression
 * + onglets), mesurée par le composant. La fenêtre s'ajuste au pixel près :
 * ni bande vide en bas, ni onglets coupés. Sans effet si le panneau est ouvert.
 */
export function reportCollapsedHeight(h: number): void {
  const nh = Math.round(h);
  if (nh <= 0 || Math.abs(nh - collapsedH) < 1) return;
  collapsedH = nh;
  if (get(miniPlayerActive) && !expanded) void setH(collapsedH);
}
