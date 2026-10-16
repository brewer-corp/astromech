/**
 * The outfitting catalogue (data/catalog/outfitting.json, from EDCD/FDevIDs): every module a
 * station sells, with the classes, ratings and mounts it comes in, and every ship.
 */
import catalog from "../../../data/catalog/outfitting.json";
import type { ModuleWant, Mount } from "./stations";

export interface Size {
  class: number;
  rating: string;
  mount?: Mount;
  /** The journal `Item` symbols for this size. */
  symbols?: string[];
}

export interface ModuleKind {
  name: string;
  category: "standard" | "internal" | "hardpoint" | "utility";
  sizes: Size[];
}

export const MODULES = catalog.modules as ModuleKind[];
export const SHIPS = catalog.ships as { symbol: string; name: string }[];

export const CATEGORIES: { id: ModuleKind["category"]; title: string }[] = [
  { id: "standard", title: "Core internals" },
  { id: "internal", title: "Optional internals" },
  { id: "hardpoint", title: "Hardpoints" },
  { id: "utility", title: "Utility mounts" },
];

/** Sizes of `m` that fit what is picked so far (null: any). */
function fitting(m: ModuleKind, w: ModuleWant, skip: keyof ModuleWant): Size[] {
  return m.sizes.filter(
    (s) =>
      (skip === "class" || w.class == null || s.class === w.class) &&
      (skip === "rating" || w.rating == null || s.rating === w.rating) &&
      (skip === "mount" || w.mount == null || s.mount === w.mount),
  );
}

const uniq = <T,>(xs: T[]): T[] => [...new Set(xs)].sort();

/** The classes, ratings and mounts offered for a module, given the other picks. */
export function choices(w: ModuleWant): { classes: number[]; ratings: string[]; mounts: Mount[] } {
  const m = MODULES.find((x) => x.name === w.name);
  if (!m) return { classes: [], ratings: [], mounts: [] };
  return {
    classes: uniq(fitting(m, w, "class").map((s) => s.class)),
    ratings: uniq(fitting(m, w, "rating").map((s) => s.rating)),
    mounts: uniq(fitting(m, w, "mount").flatMap((s) => (s.mount ? [s.mount] : []))),
  };
}

/** `w` with picks its module doesn't come in cleared: class first, then rating, then mount. */
export function settle(w: ModuleWant): ModuleWant {
  const any: ModuleWant = { name: w.name, class: null, rating: null, mount: null };
  const cls = w.class != null && choices(any).classes.includes(w.class) ? w.class : null;
  const withClass = { ...any, class: cls };
  const rating = w.rating != null && choices(withClass).ratings.includes(w.rating) ? w.rating : null;
  const withRating = { ...withClass, rating };
  const mount = w.mount != null && choices(withRating).mounts.includes(w.mount) ? w.mount : null;
  return { ...withRating, mount };
}

const BY_SYMBOL = new Map(MODULES.flatMap((m) => m.sizes.flatMap((s) => (s.symbols ?? []).map((sym) => [sym.toLowerCase(), { m, s }] as const))));

/** What to buy to get a fitted module again, from its journal `Item` symbol; null when the catalogue doesn't sell it (armour). */
export function wantFor(symbol: string): ModuleWant | null {
  const hit = BY_SYMBOL.get(symbol.toLowerCase());
  return hit ? { name: hit.m.name, class: hit.s.class, rating: hit.s.rating, mount: hit.s.mount ?? null } : null;
}

/**
 * The Outfitting page's route for `w`: `outfitting/<module>[?class=&rating=&mount=]`, each part
 * only when picked, e.g. "outfitting/Pulse%20Laser?mount=Fixed". Nothing picked searches any size.
 */
export function moduleRoute(w: ModuleWant): string {
  const q = new URLSearchParams();
  if (w.class != null) q.set("class", String(w.class));
  if (w.rating) q.set("rating", w.rating);
  if (w.mount) q.set("mount", w.mount);
  const qs = q.toString();
  return `outfitting/${encodeURIComponent(w.name)}${qs ? `?${qs}` : ""}`;
}

const MOUNTS: Mount[] = ["Fixed", "Gimballed", "Turreted"];

/**
 * The module a `#/outfitting/...` hash asks for, settled to what it comes in; null for no or an
 * unknown module. A size it doesn't come in, or a bad one, is dropped. Also reads the older
 * `/<class><rating>[/<mount>]` form.
 */
export function parseModuleRoute(hash: string): ModuleWant | null {
  const m = /^#\/outfitting\/([^/?]+)([^?]*)(?:\?(.*))?$/.exec(hash);
  if (!m?.[1]) return null;
  let name: string;
  try {
    name = decodeURIComponent(m[1]);
  } catch {
    return null;
  }
  if (!MODULES.some((x) => x.name === name)) return null;
  const old = /^\/(\d)([A-I])(?:\/(\w+))?$/.exec(m[2] ?? "");
  const q = new URLSearchParams(m[3] ?? "");
  const cls = q.get("class") ?? old?.[1] ?? "";
  const rating = q.get("rating") ?? old?.[2] ?? "";
  const mount = q.get("mount") ?? old?.[3];
  return settle({
    name,
    class: /^\d$/.test(cls) ? Number(cls) : null,
    rating: /^[A-I]$/.test(rating) ? rating : null,
    mount: MOUNTS.find((x) => x === mount) ?? null,
  });
}

/** The Shipyard page's route for a ship: `shipyard/<ship name>`. */
export const shipRoute = (name: string) => `shipyard/${encodeURIComponent(name)}`;

/** The ship a `#/shipyard/<name>` hash asks for, or null for no or an unknown ship. */
export function parseShipRoute(hash: string): string | null {
  const m = /^#\/shipyard\/(.+)$/.exec(hash);
  if (!m?.[1]) return null;
  try {
    const name = decodeURIComponent(m[1]);
    return SHIPS.some((s) => s.name === name) ? name : null;
  } catch {
    return null;
  }
}
