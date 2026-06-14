import { writable } from "svelte/store";

/**
 * Store pour la visibilité de la barre de navigation alphabétique.
 * Permet de la toggler depuis n'importe où (toolbar, etc.)
 */
export const alphabetNavVisible = writable(true);
