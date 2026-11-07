/**
 * The browser preview's options (`?core=ready&route=exact`, `?fail=`, `?press=`…): the query a
 * preview build baked in, with the page's own query on top, key by key. scripts/preview-publish
 * sets VITE_PREVIEW_QUERY because an artifact link can't carry a query string.
 */
export function previewQuery(): URLSearchParams {
  const q = new URLSearchParams(import.meta.env.VITE_PREVIEW_QUERY ?? "");
  if (typeof location === "undefined") return q;
  for (const [k, v] of new URLSearchParams(location.search)) q.set(k, v);
  return q;
}
