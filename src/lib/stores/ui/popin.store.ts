import type { Component } from "svelte"
import { writable } from "svelte/store";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyComponent = Component<any, any, any>;

export type PopinState = {
    isOpen: boolean,
    title: string,
    component: AnyComponent | null
    props?: Record<string, any>;
}

const initialState: PopinState = {
  isOpen: false,
  title: "",
  component: null,
  props: {}
};

const popinWriter = writable<PopinState>(initialState);

export const popinStore = {
  subscribe: popinWriter.subscribe,
  open: (
    title: string,
    component: AnyComponent,
    props: Record<string, any> = {}
  ) => {
    popinWriter.set({
      isOpen: true,
      title,
      component,
      props
    });
  },

  close: () => {
    popinWriter.set(initialState);
  }
};