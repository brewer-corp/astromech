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
