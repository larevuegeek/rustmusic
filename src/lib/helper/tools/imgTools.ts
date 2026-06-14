import type { AudioFile } from "$lib/types/db/audioFile/AudioFile";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir, resolve } from "@tauri-apps/api/path";

export default async function getCoverUrl(relativePath: string): Promise<string> {
  // 1) nettoie le chemin venant de la DB
  const clean = (relativePath ?? "")
    .trim()                       // vire espaces / \n / \r
    .replace(/^[\/\\]+/, "")       // vire / ou \ au début
    .replace(/\\/g, "/");          // normalise windows -> /

  const base = await appDataDir();

  const native = await resolve(base, clean);

  return native;
}

export async function thumbnail_getter(selectedPath: string, audioFile: AudioFile): Promise<string| undefined | null> {
  
  let thumbnailPath: string| undefined | null = null;
  if (audioFile.tags?.attached_images && audioFile.tags.attached_images.length > 0) {
      const cover = audioFile.tags.attached_images[0];

      thumbnailPath = await invoke("save_thumbnail", {
          imageData: cover.image_data,
      });
  }

  return thumbnailPath;
}