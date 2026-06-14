import type { AudioFile } from "../audioFile/AudioFile";

export type Player = {
  lastUpdateTime: number;
  pathFile: string | null;
  audioFile: AudioFile | null;
  duration: number;
  status: PlayerStatus;
  rustPosition: number;
  jsPosition: number;
  trackId: string | null;
  /**
   * `true` quand le backend pré-décode le morceau en RAM avant de jouer
   * (profil Minimal). L'animation JS de la progress bar doit s'arrêter
   * pendant cette phase pour ne pas avancer dans le vide.
   */
  isPreparing: boolean;
};

export type PlayerStatus = "ready" | "idle" | "playing" | "paused" | "ended";
