import type { QueueTrack } from "./QueueTrack";

export type QueueState = {
  tracks: QueueTrack[]; 
  currentIndex: number; // Vaut -1 si la file est vide ou rien n'est joué
  isShuffled: boolean;
  repeatMode: "off" | "one" | "all";
};