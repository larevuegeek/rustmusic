import type { UnlistenFn } from "@tauri-apps/api/event";
import { listen } from "@tauri-apps/api/event";
import { player } from "$lib/stores/player/player.store";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";
import type { QueueTrack } from "$lib/types/db/queue/QueueTrack";
import type { AudioFile } from "$lib/types/db/audioFile/AudioFile";
import { recent } from "$lib/stores/recent/recent.store";
import { queueState } from "$lib/stores/queue/queueState.store";
import { settingsStore } from "$lib/stores/settings/settings.store";
import { sendNotification, isPermissionGranted, requestPermission } from "@tauri-apps/plugin-notification";

let raf: number = 0;
let basePos = 0;
let baseTime = 0;
let playRequestId = 0;
let timer: ReturnType<typeof setTimeout> | null = null;
let seekPending = false;
let trackPlaybackEnded: UnlistenFn | null = null;
let preparingUnlisten: UnlistenFn | null = null;
let queueUnsubscribe: (() => void) | null = null;
let playStartedAt = 0; // timestamp (Date.now()) au moment où CPAL démarre vraiment

let initialized = false;

class PlayerService {

    // ==========================================
    // 🚀 INIT GLOBAL (à appeler UNE seule fois)
    // ==========================================
    async init() {
        if (initialized) return;
        initialized = true;

        await this.initPlaybackListener();
        await this.initPreparingListener();
        this.initQueueSync();
    }

    destroy() {
        if (trackPlaybackEnded) trackPlaybackEnded();
        if (preparingUnlisten) preparingUnlisten();
        if (queueUnsubscribe) queueUnsubscribe();
        this.stopAnimation();
        if (timer) clearTimeout(timer);
        initialized = false;
    }

    // ==========================================
    // 🔄 LISTENER PRE-DECODE (profil Minimal)
    // ==========================================
    // Le backend émet `playback-preparing: true` au début du pré-décodage et
    // `false` quand CPAL démarre vraiment. Pendant le pré-décodage, le frontend
    // doit FIGER la progress bar (l'animation JS avance dans le vide sinon).
    private async initPreparingListener() {
        preparingUnlisten = await listen<boolean>("playback-preparing", (e) => {
            const preparing = e.payload === true;
            player.update({
                isPreparing: preparing,
                // À la fin du pre-decode, reset les positions à 0 pour que la
                // lecture redémarre proprement (le backend démarre à 0 aussi).
                ...(preparing ? {} : { jsPosition: 0, rustPosition: 0 }),
            });
            if (preparing) {
                // Resync de la base de l'animation au cas où elle tournait déjà.
                basePos = 0;
                baseTime = Date.now();
            } else {
                // CPAL démarre vraiment maintenant : mémoriser l'instant pour
                // détecter les "playback-ended" trop rapides (= bug driver).
                playStartedAt = Date.now();
            }
        });
    }

    // ==========================================
    // 🎧 LISTENER FIN DE LECTURE
    // ==========================================
    private async initPlaybackListener() {
        trackPlaybackEnded = await listen("playback-ended", () => {
            this.stopAnimation();
            if (timer) clearTimeout(timer);

            // Garde-fou anti boucle : si le morceau "se termine" en moins de
            // 3 secondes, c'est probablement un problème CPAL (driver audio
            // qui crash, buffer mal configuré...) plutôt qu'une vraie fin de
            // lecture. On stoppe au lieu de relancer en boucle infinie.
            const elapsed = (Date.now() - playStartedAt) / 1000;
            if (playStartedAt > 0 && elapsed < 3) {
                console.warn(
                    `⚠️ playback-ended trop rapide (${elapsed.toFixed(1)}s) — ` +
                    "stream CPAL probablement KO. Stop pour éviter une boucle."
                );
                player.update({ status: "ended", isPreparing: false });
                playStartedAt = 0;
                return;
            }
            playStartedAt = 0;

            const state = get(queueState);
            if (state.repeatMode === "one") {
                // Repeat One : relancer le même morceau directement
                const track = state.tracks[state.currentIndex];
                if (track) this.playFile(track);
            } else {
                queueState.next();
            }
        });
    }

    // ==========================================
    // 🔗 REMPLACE TON $effect()
    // ==========================================
    private initQueueSync() {
        queueUnsubscribe = queueState.subscribe(state => {

            const track = state.tracks[state.currentIndex] ?? null;
            const currentPlayer = get(player);
            const currentPlayerTrackId = currentPlayer.trackId;

            if (!track) {
                this.stopPlay();
                return;
            }

            // Premier chargement → preload seulement
            if (!currentPlayerTrackId) {
                this.preloadTrack(track);
                return;
            }

            // Le track a changé (next/previous/shuffle) → lancer la lecture
            if (track.queueId !== currentPlayerTrackId) {
                this.playFile(track);
            }
        });
    }

    // ==========================================
    // 📦 PRELOAD (conservé)
    // ==========================================
    async preloadTrack(track: QueueTrack) {

        const currentRequestId = ++playRequestId;

        try {
            await this.stopPlay();

            const audioFile = await invoke("open_file", { path: track.path }) as AudioFile;

            if (currentRequestId !== playRequestId) return;

            player.update({
                status: "idle",
                pathFile: track.path,
                audioFile,
                trackId: track.queueId
            });

        } catch (err) {
            console.error("Erreur preload:", err);
        }
    }

    // ==========================================
    // ▶ PLAY
    // ==========================================
    async playFile(track: QueueTrack) {

        const currentRequestId = ++playRequestId;

        try {
            await this.stopPlay();

            const audioFile = await invoke("open_file", { path: track.path }) as AudioFile;

            if (currentRequestId !== playRequestId) return;

            player.update({
                status: "playing",
                pathFile: track.path,
                audioFile,
                trackId: track.queueId
            });

            await invoke("play_file", { path: track.path });

            // Si pas de mode Minimal/pre-decode, on considère que la lecture
            // commence immédiatement (sera écrasé par le preparing listener
            // pour les profils Low/Minimal).
            playStartedAt = Date.now();

            this.runPosition();
            this.startAnimation();

            // Gestion récents
            const libraryCacheId = await invoke<number | null>(
                "get_library_cache_id_by_path",
                { path: track.path }
            );

            await invoke<void>("insert_recent_file", {
                path: track.path,
                libraryId: libraryCacheId
            });

            recent.refreshRecent();

            // Notification système si activée dans les paramètres
            this.sendTrackNotification(track, audioFile);

        } catch (err) {
            console.error("Erreur lecture:", err);
        }
    }

    // ==========================================
    // 🔔 NOTIFICATION
    // ==========================================
    // Envoie une notification système quand un nouveau morceau commence
    // Vérifie d'abord que les notifications sont activées dans les paramètres
    // et que l'OS a accordé la permission
    private async sendTrackNotification(track: QueueTrack, audioFile: AudioFile) {
        try {
            const notifEnabled = settingsStore.get('show_notifications');
            if (notifEnabled !== 'true') return;

            const title = audioFile.tags?.title ?? track.title ?? 'Titre inconnu';
            const artist = audioFile.tags?.artist ?? track.artist ?? 'Artiste inconnu';
            const album = audioFile.tags?.album ?? '';
            const body = album ? `${artist} — ${album}` : artist;

            // Essayer d'abord le plugin Tauri (fonctionne en prod)
            try {
                let permitted = await isPermissionGranted();
                if (!permitted) {
                    const permission = await requestPermission();
                    permitted = permission === 'granted';
                }
                if (permitted) {
                    sendNotification({ title, body });
                    return;
                }
            } catch {
                // Plugin Tauri indisponible (dev mode) — fallback Web API
            }

            // Fallback : Web Notification API (fonctionne en dev + prod)
            if ('Notification' in window) {
                if (Notification.permission === 'default') {
                    await Notification.requestPermission();
                }
                if (Notification.permission === 'granted') {
                    new Notification(title, { body, silent: true });
                }
            }
        } catch (e) {
            // Silencieux — les notifications ne sont pas critiques
        }
    }

    // ==========================================
    // ⏹ STOP
    // ==========================================
    async stopPlay() {
        try {
            await invoke("stop_play");

            player.update({
                status: "idle",
                rustPosition: 0,
                jsPosition: 0
            });

            if (timer) clearTimeout(timer);
            timer = null;

            this.stopAnimation();

        } catch (err) {
            console.error(err);
        }
    }

    // ==========================================
    // ⏩ SEEK
    // ==========================================
    async seekTo(positionSeconds: number) {
        await invoke("seek_to", { position: positionSeconds });
        player.update({ jsPosition: positionSeconds, rustPosition: positionSeconds });
        basePos = positionSeconds;
        baseTime = Date.now();
        seekPending = true; // Ignorer les prochaines rustPosition stale
    }

    // ==========================================
    // ⏯ TOGGLE
    // ==========================================
    async handleTogglePlay() {

        const queue = get(queueState);
        const currentTrack = queue.tracks[queue.currentIndex] ?? null;
        if (!currentTrack) return;

        const status = get(player).status;

        if (status === "playing") {
            await this.pauseFile();
        } else if (status === "paused") {
            await this.resumePlay();
        } else {
            await this.playFile(currentTrack);
        }
    }

    async resumePlay() {
        try {
            await invoke("pause_play");
            player.update({ status: "playing" });
            this.runPosition();
            this.startAnimation();
        } catch (err) {
            console.error("Resume failed", err);
        }
    }

    // ==========================================
    // ⏸ PAUSE
    // ==========================================
    async pauseFile() {
        try {
            await invoke("pause_play");

            player.update({status: "paused" });

            if (timer) clearTimeout(timer);
            timer = null;

            this.stopAnimation();

        } catch (err) {
            console.error(err);
        }
    }

    // ==========================================
    // ⏮ PREV
    // ==========================================
    async prevTrack() {
        await this.stopPlay();
        queueState.previous();
    }

    // ==========================================
    // ⏭ NEXT
    // ==========================================
    async nextTrack() {
        await this.stopPlay();
        queueState.next();
    }

    // ==========================================
    // 📡 POLLING RUST
    // ==========================================
    private async runPosition() {

        if (get(player).status !== "playing") return;

        try {
            const [current, total] = await invoke<[number, number]>("get_progress");

            if (seekPending) {
                // Après un seek, ignorer les positions Rust stale
                // Accepter seulement quand Rust a rattrapé la position seekée
                const expectedPos = get(player).jsPosition;
                if (Math.abs(current - expectedPos) < 1.5) {
                    seekPending = false;
                } else {
                    // Position stale, on skip cette mise à jour
                    timer = setTimeout(() => this.runPosition(), 300);
                    return;
                }
            }

            player.update({
                rustPosition: current,
                duration: total
            });

        } catch (err) {
            console.error("Erreur récupération état:", err);
        }

        timer = setTimeout(() => this.runPosition(), 300);
    }

    // ==========================================
    // 🎞 RAF JS
    // ==========================================
    private startAnimation() {

        this.stopAnimation();

        basePos = get(player).rustPosition;
        baseTime = Date.now();

        const loop = () => {

            const p = get(player);
            if (p.status !== "playing") return;

            // Pendant le pré-décodage (Minimal mode), CPAL n'a pas encore
            // démarré : on FIGE la progress bar à 0 pour ne pas afficher
            // une avance fantôme. La boucle continue (raf) mais ne met pas
            // jsPosition à jour, et on re-base le timer pour que la reprise
            // soit propre.
            if (p.isPreparing) {
                basePos = 0;
                baseTime = Date.now();
                raf = requestAnimationFrame(loop);
                return;
            }

            // Se recaler sur la position Rust si drift trop important
            // (seulement quand pas en seek — sinon le guard dans runPosition gère)
            if (p.rustPosition > 0 && !seekPending) {
                const drift = Math.abs((basePos + (Date.now() - baseTime) / 1000) - p.rustPosition);
                if (drift > 2) {
                    basePos = p.rustPosition;
                    baseTime = Date.now();
                }
            }

            const elapsed = (Date.now() - baseTime) / 1000;
            const pos = basePos + elapsed;

            player.update({
                jsPosition: Math.min(pos, p.duration)
            });

            raf = requestAnimationFrame(loop);
        };

        raf = requestAnimationFrame(loop);
    }

    private stopAnimation() {
        if (raf) cancelAnimationFrame(raf);
        raf = 0;
    }
}

export const playerService = new PlayerService();