import { recent } from "$lib/stores/recent/recent.store";
import { toasts } from "$lib/stores/ui/toast.store";
import type { RecentFile } from "$lib/types/db/recent/RecentFile";
import { invoke } from "@tauri-apps/api/core";

export async function handleRemoveRecentItem(recentFile: RecentFile) {

    await invoke<void>('remove_recent_file', { path : recentFile.path });

    recent.refreshRecent();

    //AddToast
    toasts.push({
      type: "info",
      title: "Retiré de la file",
      message: "Le morceau a été retiré des morceaux récents"
    });
}