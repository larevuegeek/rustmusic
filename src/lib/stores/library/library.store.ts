import type { Library } from "$lib/types/db/library/Library"
import { profilSelector } from "$lib/stores/profil/profil.store";
import { writable, get } from "svelte/store"
import { invoke } from "@tauri-apps/api/core";

export type LibraryState = {
    librarySelected: Library | null
    libraries: Library[],
    isLoading: boolean,
    isImporting: boolean,
}

const initialState: LibraryState = {
    librarySelected: null,
    libraries: [],
    isLoading: false,
    isImporting: false
}

const libraryWriter = writable<LibraryState>(initialState);

export const libraryStore = {
  subscribe: libraryWriter.subscribe,
  init: async () => {
    libraryWriter.update(state => ({
      ...state,
      isLoading: true
    }));

     const profilState = get(profilSelector);
     const profilId = profilState.profilSelected?.id

    if (!profilId) {
        console.warn("No profil selected, skip library init");
        return;
    }

    try {
      const libraries = await invoke<Library[]>('get_libraries', { 
          profilId: profilId,
      });

      libraryWriter.update(state => ({
        ...state,
        libraries,
        librarySelected: libraries.length > 0 ? libraries[0] : null,
        isLoading: false
      }));
    } catch (error) {
      console.error("Failed to load libraries", error);
      libraryWriter.update(state => ({
        ...state,
        isLoading: false
      }));
    }

  },
  refresh: async () => {
    const profilState = get(profilSelector);
    const profilId = profilState.profilSelected?.id;

    if (!profilId) return;

    try {
      const libraries = await invoke<Library[]>('get_libraries', { 
          profilId: profilId,
      });

      libraryWriter.update(state => ({
        ...state,
        libraries,
        librarySelected: libraries.find(l => l.id === state.librarySelected?.id) ?? null
      }));
    } catch (error) {
      console.error("Failed to refresh libraries", error);
    }
  },
  selectLibrary: (library: Library | null) => {
    libraryWriter.update(state => ({
      ...state,
      librarySelected: library
    }));
  },
  setImporting: (importing: boolean) => {
    libraryWriter.update(state => ({
      ...state,
      isImporting: importing
    }));
  },
  addLibrary: async (profilId: number, name: string, description: string | null)  => {

    const newLibrary: Library = await invoke<Library>('create_library', { 
      payload: {
        profil_id: profilId,
        name: name.trim(),
        description: description?.trim() || null
      }
    });

    libraryWriter.update(state => {
      const libraries = [...state.libraries, newLibrary];

      return {
        ...state,
        libraries,
        librarySelected: state.librarySelected ?? newLibrary
      };
    });
  },
  removeLibrary: async (library: Library) => {
    
    if(library.id) {
      await invoke<Library>('remove_library', {
        libraryId: library.id
      });
    }

    libraryWriter.update(state => {
      const libraries = state.libraries.filter(l => l.id !== library.id);

      const librarySelected =
        state.librarySelected?.id === library.id
          ? libraries[0] ?? null
          : state.librarySelected;

      return {
        ...state,
        libraries,
        librarySelected
      };
    });
  },
  clear: () => {
    libraryWriter.set(initialState);
  }
};