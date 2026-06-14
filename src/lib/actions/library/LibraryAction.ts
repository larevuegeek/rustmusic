import { goto } from "$app/navigation";
import addLibraryFiles, { addLibraryDirectory } from "$lib/services/library/library.service";
import { libraryStore } from "$lib/stores/library/library.store";
import { libraryContentStore } from "$lib/stores/library/libraryContent.store";
import { toasts } from "$lib/stores/ui/toast.store";
import type { TrackListView } from "$lib/types/ui/library/track/TrackListView";

export async function handleAddFiles(libraryId: number, redirectToLibrary = false): Promise<void> {

    try {
        if (redirectToLibrary) goto(`/library/${libraryId}`);

        const newTracks = await addLibraryFiles(libraryId);

        if (newTracks.length === 0) return;

        await libraryContentStore.load(libraryId);
        await libraryStore.refresh();

        toasts.push({
            type: "success",
            title: "Fichiers ajoutés",
            message: `${newTracks.length} piste(s) ajoutée(s)`
        });

    } catch (e) {
        toasts.push({
            type: "error",
            title: "Erreur",
            message: "Impossible d'ajouter les fichiers"
        });
    } finally {
        libraryStore.setImporting(false);
    }
}

export async function handleAddDirectory(libraryId: number, redirectToLibrary = false) {

    try {
        if (redirectToLibrary) goto(`/library/${libraryId}`);

        const newTracks = await addLibraryDirectory(libraryId);

        // Si annulation ou aucun résultat, on ne fait rien
        if (newTracks.length === 0) return;

        await libraryContentStore.load(libraryId);
        await libraryStore.refresh();

        toasts.push({
            type: "success",
            title: "Fichiers ajoutés",
            message: `${newTracks.length} piste(s) ajoutée(s)`
        });

    } catch (e) {
        toasts.push({
            type: "error",
            title: "Erreur",
            message: "Impossible d'ajouter le dossier"
        });
    } finally {
        libraryStore.setImporting(false);
    }
}

export function handleRemoveTrackItem(track: TrackListView) {
    toasts.push({
        type: "info",
        title: "Bientôt disponible",
        message: "La suppression de morceaux sera disponible prochainement."
    });
}