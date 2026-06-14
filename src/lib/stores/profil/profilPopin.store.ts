import { writable } from "svelte/store";

const profilPopinWriter = writable<boolean>(false);

export const profilPopinStore = {
  subscribe: profilPopinWriter.subscribe,
  open: () => profilPopinWriter.set(true),
  close: () => profilPopinWriter.set(false),
};
