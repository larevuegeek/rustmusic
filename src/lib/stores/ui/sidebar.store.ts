import { writable } from "svelte/store";

const sidebarWriter = writable({
  open: true,      // desktop: toujours visible, mobile: toggle
  collapsed: false, // desktop: mode compact (icônes seules)
});

export const sidebarStore = {
  subscribe: sidebarWriter.subscribe,
  toggle: () => sidebarWriter.update(s => ({ ...s, open: !s.open })),
  open: () => sidebarWriter.update(s => ({ ...s, open: true })),
  close: () => sidebarWriter.update(s => ({ ...s, open: false })),
  toggleCollapse: () => sidebarWriter.update(s => ({ ...s, collapsed: !s.collapsed })),
};
