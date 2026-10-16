/**
 * Loading and changing a lookup's remembered filters without losing the search when storage
 * fails. Pure (storage is passed in) so it runs outside Svelte and Tauri.
 */
import type { Filters } from "./search";

export interface FilterStore {
  load(lookup: string): Promise<Filters>;
  save(lookup: string, filters: Filters): Promise<void>;
}

export interface FilterState {
  filters: Filters;
  /** Why the filters shown are not what the commander asked for, if they aren't. */
  error: string | null;
}

/** The saved filters, or `fallback` (logged) if they can't be read, so the search still runs. */
export async function openFilters(store: FilterStore, lookup: string, fallback: Filters): Promise<FilterState> {
  try {
    return { filters: await store.load(lookup), error: null };
  } catch (e) {
    console.warn(`couldn't read the saved filters for ${lookup}; using defaults`, e);
    return { filters: fallback, error: null };
  }
}

/**
 * Save `next`, then reload it (the pad goes back to the ship's when the commander stops
 * choosing one). A failed save keeps `prev` and says why; a failed reload keeps `next`.
 */
export async function changeFilters(store: FilterStore, lookup: string, prev: Filters, next: Filters): Promise<FilterState> {
  try {
    await store.save(lookup, next);
  } catch (e) {
    return { filters: prev, error: `Couldn't save the filter: ${e instanceof Error ? e.message : String(e)}` };
  }
  try {
    return { filters: await store.load(lookup), error: null };
  } catch {
    return { filters: next, error: null };
  }
}
