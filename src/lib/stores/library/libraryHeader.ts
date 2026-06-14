import { writable } from 'svelte/store';

export const libraryHeader = writable({
  subtitle: 'Morceaux',
  icon: 'mynaui:music',
  total: 0
});