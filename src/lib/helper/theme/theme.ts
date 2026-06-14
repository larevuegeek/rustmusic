export type ThemeMode = 'auto' | 'light' | 'dark';

let mediaQueryListener: ((e: MediaQueryListEvent) => void) | null = null;

/**
 * Applique le thème :
 * - 'light' / 'dark' : force la classe correspondante
 * - 'auto' : suit la préférence système et écoute les changements en live
 */
export function applyThemeMode(mode: ThemeMode) {
    cleanupAutoListener();

    if (mode === 'auto') {
        const mq = window.matchMedia('(prefers-color-scheme: dark)');
        applyDarkClass(mq.matches);
        mediaQueryListener = (e) => applyDarkClass(e.matches);
        mq.addEventListener('change', mediaQueryListener);
        return;
    }

    applyDarkClass(mode === 'dark');
}

function applyDarkClass(isDark: boolean) {
    document.documentElement.classList.toggle('dark', isDark);
}

function cleanupAutoListener() {
    if (mediaQueryListener) {
        const mq = window.matchMedia('(prefers-color-scheme: dark)');
        mq.removeEventListener('change', mediaQueryListener);
        mediaQueryListener = null;
    }
}

// Compat avec l'ancien code (SwitchTheme.svelte) — sera supprimé plus tard
export function toogleTheme(isDarkTheme: boolean): boolean {
    const next = !isDarkTheme;
    applyDarkClass(next);
    return next;
}

export function applyTheme(isDark: boolean) {
    applyDarkClass(isDark);
}
