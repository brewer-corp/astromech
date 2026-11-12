// How a list inside a ListDetail fills the side panel (SearchKit's result rows do): the list
// sets the panel to describe its selected row, and the narrowest its rows keep all their columns.
import { getContext, setContext, type Snippet } from "svelte";

export type Detail = {
  /** The selected row's details; null, no panel. */
  panel: Snippet | null;
  /** The selected row's key. */
  selected: unknown;
  /** The narrowest the list keeps all its columns at. */
  min: number | null;
};

const KEY = Symbol("list-detail");

/** ListDetail's side of it. */
export function provideDetail(): Detail {
  const d = $state<Detail>({ panel: null, selected: null, min: null });
  setContext(KEY, d);
  return d;
}

/** A list's side: the ListDetail it is in, if any. */
export function useDetail(): Detail | undefined {
  return getContext<Detail | undefined>(KEY);
}
