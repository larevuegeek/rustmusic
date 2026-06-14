import { queueState } from "$lib/stores/queue/queueState.store";
import { toasts } from "$lib/stores/ui/toast.store";
import type { RecentFile } from "$lib/types/db/recent/RecentFile";
import type { TrackListView } from "$lib/types/ui/library/track/TrackListView";
import { get } from "svelte/store";
import { loadTracksByAlbum } from "$lib/services/library/library.service";
import { libraryStore } from "$lib/stores/library/library.store";
import { toQueueTrack, toQueueTracks, type TrackLike } from "$lib/helper/tools/queueTools";
import { playerService } from "$lib/services/player/player.service";

export async function handleAlbumEnqueue(libraryAlbumId: string) {
  const state = get(libraryStore);
  const libraryId = state.librarySelected?.id;
  if (!libraryId) return;

  try {
    const result = await loadTracksByAlbum(libraryId, libraryAlbumId);
    if (!result || result.length === 0) return;

    const queueTracks = toQueueTracks(result);
    await queueState.loadTracks(queueTracks);
    playerService.playFile(queueTracks[0]);

    toasts.push({
      type: "success",
      title: "Lecture de l'album",
      message: `${queueTracks.length} morceau(x) ajouté(s) à la file`
    });
  } catch (e) {
    console.error("Impossible de charger l'album", e);
  }
}

export function handleEnqueue(trackSelected: TrackListView | RecentFile) {
  const track = toQueueTrack(toTrackLike(trackSelected));
  queueState.enqueue(track);

  toasts.push({
    type: "success",
    title: "Ajouté à la file",
    message: "Le morceau a été ajouté à la suite de la lecture"
  });
}

export function handleAddTrackToQueue(trackSelected: TrackListView | RecentFile) {
  const track = toQueueTrack(toTrackLike(trackSelected));
  queueState.addTrack(track);

  toasts.push({
    type: "success",
    title: "Ajouté en priorité",
    message: "Le morceau sera lu juste après"
  });
}

// Convertit un TrackListView ou RecentFile en TrackLike pour toQueueTrack
function toTrackLike(obj: TrackListView | RecentFile): TrackLike {
  if ("library_artist_id" in obj) {
    // TrackListView
    return {
      path: obj.path,
      title: obj.title,
      artist: obj.artist,
      duration: obj.duration,
      thumbnail_path: obj.thumbnail_path,
    };
  } else {
    // RecentFile
    return {
      path: obj.path,
      title: obj.library?.title ?? null,
      artist: obj.library?.artist ?? null,
      duration: obj.library?.duration ?? null,
      thumbnail_path: obj.library?.thumbnail_path ?? null,
    };
  }
}
