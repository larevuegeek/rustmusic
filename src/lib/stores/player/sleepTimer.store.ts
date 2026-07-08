/**
 * Minuteur de mise en veille (« sleep timer »).
 *
 * Deux modes :
 *   - `duration`     : la lecture s'arrête après N minutes (décompte visible).
 *   - `end-of-track` : la lecture s'arrête à la fin du morceau en cours
 *                      (au lieu d'enchaîner la piste suivante).
 *
 * Le décompte tourne côté JS (setInterval 1s). Quand il atteint 0, on appelle
 * le callback `onFire` enregistré par le player service (typiquement une pause).
 * Le mode `end-of-track` est consommé par le handler `playback-ended`.
 */

import { writable, get } from "svelte/store";

export type SleepTimerMode = "off" | "duration" | "end-of-track";

export type SleepTimerState = {
  mode: SleepTimerMode;
  /** Timestamp (ms) de fin en mode `duration`, sinon `null`. */
  endsAt: number | null;
  /** Secondes restantes en mode `duration` (0 sinon). */
  remaining: number;
};

const initial: SleepTimerState = { mode: "off", endsAt: null, remaining: 0 };

function createSleepTimer() {
  const { subscribe, set, update } = writable<SleepTimerState>({ ...initial });
  let interval: ReturnType<typeof setInterval> | null = null;
  let onFire: (() => void) | null = null;

  function stopInterval() {
    if (interval) {
      clearInterval(interval);
      interval = null;
    }
  }

  function fire() {
    stopInterval();
    set({ ...initial });
    onFire?.();
  }

  function tick() {
    const s = get({ subscribe });
    if (s.mode !== "duration" || s.endsAt == null) return;
    const remaining = Math.max(0, Math.round((s.endsAt - Date.now()) / 1000));
    update((st) => ({ ...st, remaining }));
    if (remaining <= 0) fire();
  }

  return {
    subscribe,

    /** Enregistre l'action déclenchée à échéance (ex. pause). Appelé au boot. */
    setOnFire(cb: () => void) {
      onFire = cb;
    },

    /** Démarre un minuteur de `minutes` minutes. */
    startDuration(minutes: number) {
      stopInterval();
      const endsAt = Date.now() + minutes * 60_000;
      set({ mode: "duration", endsAt, remaining: minutes * 60 });
      interval = setInterval(tick, 1000);
    },

    /** Arrêt à la fin du morceau en cours. */
    startEndOfTrack() {
      stopInterval();
      set({ mode: "end-of-track", endsAt: null, remaining: 0 });
    },

    /** Annule le minuteur. */
    cancel() {
      stopInterval();
      set({ ...initial });
    },

    /**
     * Appelé par le player service sur `playback-ended`. Si le mode est
     * `end-of-track`, consomme le minuteur et retourne `true` (→ le service
     * doit STOPPER au lieu d'enchaîner la piste suivante).
     */
    consumeEndOfTrack(): boolean {
      const s = get({ subscribe });
      if (s.mode === "end-of-track") {
        this.cancel();
        return true;
      }
      return false;
    },
  };
}

export const sleepTimer = createSleepTimer();

/** Format mm:ss pour l'affichage du décompte. */
export function formatSleepRemaining(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}
