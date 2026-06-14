import { writable } from "svelte/store";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

/**
 * Store qui écoute les events "artist-image-ready" émis par le backend
 * quand une image artiste est téléchargée depuis Deezer.
 *
 * Les composants artiste s'abonnent pour mettre à jour l'image en live
 * avec un fadeIn (via CoverImg/FadeImg).
 */

type ArtistImageMap = Record<string, string>; // artist_id → image_url

const imageWriter = writable<ArtistImageMap>({});

let unlisten: UnlistenFn | null = null;

export const artistImageReadyStore = {
    subscribe: imageWriter.subscribe,

    init: async () => {
        unlisten = await listen<{ artist_id: string; image_url: string }>(
            'artist-image-ready', (e) => {
                imageWriter.update(map => ({
                    ...map,
                    [e.payload.artist_id]: e.payload.image_url,
                }));
            }
        );
    },

    get: (artistId: string, map: ArtistImageMap): string | undefined => {
        return map[artistId];
    },

    destroy: () => {
        unlisten?.();
        imageWriter.set({});
    },
};
