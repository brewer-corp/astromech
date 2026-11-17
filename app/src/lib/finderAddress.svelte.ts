// Keeps a Places finder's query and the address in step, for every finder page: the address
// names the query, so back, reload and the tile reopen it, and a hash change to another query
// replaces it. Call it once from the page's script, which owns the query's $state.
import { showing, type FinderId } from "./places";

export interface FinderAddress<T> {
  /** The query the address asks for now, or the page's default when it names none. */
  read: () => T;
  /** The query's route, without "#/": "places/outfitting/Pulse%20Laser?mount=Fixed". */
  route: (query: T) => string;
  /** The page's query. */
  get: () => T;
  /** Replace the page's query; its search runs again. */
  set: (query: T) => void;
}

export function followAddress<T>(id: FinderId, a: FinderAddress<T>): void {
  // `id` is the page's own, never its `finder` prop (see `showing`).
  const here = () => showing(id);

  $effect(() => {
    // Only a different query replaces the page's: an equal one would still be a new object, and
    // the kit would ask the source again for what it is already showing. That happens when the
    // page sets its query and then the hash (a pick), and when it mounts with the hash change
    // that named it still to come.
    const onHash = () => {
      if (!here()) return;
      const next = a.read();
      if (a.route(next) !== a.route(a.get())) a.set(next);
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  $effect(() => {
    const to = `#/${a.route(a.get())}`;
    if (here() && location.hash !== to) history.replaceState(null, "", to);
  });
}
