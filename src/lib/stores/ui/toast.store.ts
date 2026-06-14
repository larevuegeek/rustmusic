import { writable } from "svelte/store";

export type ToastType = "info" | "error" | "warning" | "success";

export type Toast = {
    id: number,
    type: ToastType,
    title: String,
    message?: String,
    duration?: number
}

function createToastStore() {

    const { subscribe, update } = writable<Toast[]>([]);
    let id = 0;

    function push(toast: Omit<Toast, "id">): number {
        const toastId = id++;

        update((all) => [
            ...all, { id : toastId, duration: toast.duration ?? 2500, ...toast}
        ]);

        return toastId;
    }

    function remove(id: number) {
        update((all) => 
            all.filter((t) => t.id !== id)
        );
    }

    return {
        subscribe,
        push,
        remove
    }
}

export const toasts = createToastStore();

