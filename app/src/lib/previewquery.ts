/**
 * The browser preview's options (`?core=ready&route=exact`, `?fail=`, `?press=`…): the page's
 * own query, else the one a preview build baked in. scripts/preview-publish sets
 * VITE_PREVIEW_QUERY because an artifact link can't carry a query string.
 */
export function previewQuery(): URLSearchParams {
  if (typeof location === "undefined") return new URLSearchParams();
  return new URLSearchParams(location.search || (import.meta.env.VITE_PREVIEW_QUERY ?? ""));
}
