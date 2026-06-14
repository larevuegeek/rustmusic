// Détection de l'OS via le User-Agent du WebView.
// Suffisant pour différencier Windows / macOS / Linux dans le contexte Tauri.
// Évite d'ajouter @tauri-apps/plugin-os pour ce besoin simple.

export type DetectedOS = 'macos' | 'windows' | 'linux' | 'unknown';

export function detectOS(): DetectedOS {
    if (typeof navigator === 'undefined') return 'unknown';
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes('mac')) return 'macos';
    if (ua.includes('windows') || ua.includes('win64') || ua.includes('win32')) return 'windows';
    if (ua.includes('linux') || ua.includes('x11')) return 'linux';
    return 'unknown';
}
