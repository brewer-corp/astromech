/**
 * A page's own choices in the window's hash, after its route: "#/materials?kind=encoded". The
 * shell routes on the part before "/" or "?" (App.svelte), so a link, Back and reload keep the
 * choice a third level used to keep in a tab (DESIGN.md The Two Tab Rows Rule).
 */

/** One of the page's choices from the hash, or null when it has none. */
export function hashParam(key: string): string | null {
  const i = location.hash.indexOf("?");
  return i < 0 ? null : new URLSearchParams(location.hash.slice(i + 1)).get(key);
}

/** The hash for `page` with these choices; a null one (the page's default) is left out. */
export function pageHash(page: string, params: Record<string, string | null>): string {
  const q = new URLSearchParams();
  for (const [k, v] of Object.entries(params)) if (v !== null) q.set(k, v);
  const s = q.toString();
  return `#/${page}${s ? `?${s}` : ""}`;
}
