import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import type { TrackListView } from "$lib/types/ui/library/track/TrackListView";
import type { AlbumListView } from "$lib/types/ui/library/album/AlbumListView";
import type { ArtistListView } from "$lib/types/ui/library/artist/ArtistListView";
import type { ArtistDetailView } from "$lib/types/ui/library/artist/ArtistDetailView";
import type { AlbumDetailView } from "$lib/types/ui/library/album/AlbumDetailView";
import type { TrackDetailView } from "$lib/types/ui/library/track/TrackDetailView";
import type { Library } from "$lib/types/db/library/Library";
import { toasts } from "$lib/stores/ui/toast.store";
import { goto } from "$app/navigation";
import { libraryStore } from "$lib/stores/library/library.store";

export default async function addLibraryFiles(libraryId: number): Promise<TrackListView[]> {

    let tracks: TrackListView[] = [];

    try {
        const selectedFiles = await open({
            multiple: true,
            title: "Ouvrir des fichiers",
            filters: [
                { 
                    name: 'Fichiers audio', 
                    extensions: ['mp3', 'flac', 'ogg', 'm4a', 'dsf', 'dff', 'wav', 'aac'] 
                }
            ]
        });

        if (!selectedFiles) return tracks;

        libraryStore.setImporting(true);

        // Normaliser en array
        const files: string[] = Array.isArray(selectedFiles) 
            ? selectedFiles 
            : [selectedFiles];

        if(files.length == 0) return tracks;

        // Importer les fichiers
        const result = await invoke<TrackListView[]>('add_files', {
            libraryId: libraryId,
            files: files
        });

        return result ?? [];

    } catch(err) {
        console.error(err);
        return tracks;
    } finally {
        libraryStore.setImporting(false);
    }
}


export async function addLibraryDirectory(libraryId: number): Promise<TrackListView[]> {

    let tracks: TrackListView[] = [];

    try {
        const selectedPath = await open({
            directory: true,  // ← La clé importante !
            multiple: false,
            title: "Sélectionner un dossier"
        });

        if (!selectedPath) {
            return tracks;
        }

        libraryStore.setImporting(true);

        // Importer les fichiers
        const result = await invoke<TrackListView[]>('add_directory', { 
            libraryId: libraryId,
            directory: selectedPath 
        });

        return result ?? [];

    } catch(err) {
        console.error(err);
        return tracks;
    } finally {
        libraryStore.setImporting(false);
    }
}

export async function loadTrack(
    libraryTrackId: string
): Promise<TrackDetailView | null> {
    
    try {

        const result = await invoke<TrackDetailView>('get_track', { 
            libraryTrackId: libraryTrackId
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return null;
    }
}

export async function loadTracks(
    libraryId: number
): Promise<TrackListView[]> {
    
    try {

        const result = await invoke<TrackListView[]>('get_tracks', { 
            libraryId: libraryId
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return [];
    }
}

export async function loadTracksByAlbum(
    libraryId: number,
    libraryAlbumId: string,
): Promise<TrackListView[]> {
    
    try {

        const result = await invoke<TrackListView[]>('get_tracks_by_album', { 
            libraryId: libraryId,
            libraryAlbumId: libraryAlbumId
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return [];
    }
}

export async function loadAlbum(
    libraryAlbumId: string
): Promise<AlbumDetailView | null> {
    
    try {

        const result = await invoke<AlbumDetailView>('get_album', { 
            libraryAlbumId: libraryAlbumId
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return null;
    }
}

export async function loadAlbums(
    libraryId: number,
    missingCover: boolean = false
): Promise<AlbumListView[]> {
    
    try {

        const result = await invoke<AlbumListView[]>('get_albums', { 
            libraryId: libraryId,
            missingCover: missingCover
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return [];
    }
}

export async function loadArtist(
    libraryArtistId: string
): Promise<ArtistDetailView | null> {
    
    try {
        const result = await invoke<ArtistDetailView>('get_artist', { 
            libraryArtistId: libraryArtistId
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return null;
    }
}

export async function loadArtists(
    libraryId: number
): Promise<ArtistListView[]> {
    
    try {

        const result = await invoke<ArtistListView[]>('get_artists', { 
            libraryId: libraryId
        });

        return result;
        
    } catch(err) {
        console.error(err);
        return [];
    }
}


export async function loadLibrary(id: number, profilId: number, tag: number, currentTag: number): Promise<Library | null> {
    
    try {

       const result: Library | null = await invoke<Library | null>('get_library', { 
           libraryId: id,
       });

      if (tag !== currentTag) return null;

      if (!result || result.profil_id !== profilId) {
        toasts.push({
          type: "error",
          title: "Accès refusé",
          message: "Tu n'as pas accès à cette bibliothèque ou elle n'existe plus.",
        });
        goto("/");
        return null;
      }

      return result;

    } catch (e) {
      if (tag !== currentTag) return null;
      
      toasts.push({
          type: "error",
          title: "Accès impossible",
          message: "Impossible de récupérer la bibliothèque",
      });
      
      goto("/");
      return null;
    }
}