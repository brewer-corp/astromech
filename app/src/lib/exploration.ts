/**
 * Exploration: the current system's bodies as they are scanned, with their estimated values
 * (crates/astromech-exploration, through the core's `exploration_system` command, which reads
 * the newest journals).
 */
import type { SystemView as System } from "./api/types/SystemView";
import type { UnsoldData } from "./api/types/UnsoldData";
import type { BodyView as Body } from "./api/types/BodyView";
import type { BodyGlyph, StarGlyph } from "./ui/Glyph.svelte";
import { starKind } from "./route";
import { call, type Sampled } from "./transport";

export type { Body };
export type SystemView = Sampled<System>;
/** The commander's unsold data (astromech_exploration::unsold, through the history). */
export type Unsold = Sampled<UnsoldData>;

export async function loadSystem(): Promise<SystemView> {
  return await call("exploration_system");
}

/** "4.4 M cr", "720 k cr", "3,153 cr". */
export function credits(n: number): string {
  return creditParts(n).join(" ");
}

/** credits() as its figure and its unit, for a value shown with the unit set apart: ["4.4", "M cr"]. */
export function creditParts(n: number): [string, string] {
  if (n >= 1_000_000) return [(n / 1_000_000).toFixed(1), "M cr"];
  if (n >= 10_000) return [Math.round(n / 1000).toLocaleString(), "k cr"];
  return [n.toLocaleString(), "cr"];
}

/** `null`: no commander in the history yet. */
export async function loadUnsold(): Promise<Unsold | null> {
  return await call("exploration_unsold");
}

/** One row of the system tree: a body, how deep it sits and the lines that join it to its parent. */
export interface TreeRow {
  body: Body;
  /** 0 at the root (the stars, and a body with no drawn parent). */
  depth: number;
  /** The last of its parent's children: its elbow ends the parent's line. */
  last: boolean;
  /** For each level above its parent's, whether that ancestor has a later sibling, so its line
   * passes this row. */
  through: boolean[];
  /** Has children: a line leaves its icon downward. */
  kids: boolean;
}

/** The kinds of parent the tree draws (a barycentre, `Null`, and a ring are not). */
const DRAWN = new Set(["Star", "Planet"]);

/** Closest to the arrival star first; a body with no distance last, then by name. */
function byDistance(a: Body, b: Body): number {
  return (a.distance_ls ?? Infinity) - (b.distance_ls ?? Infinity) || a.short_name.localeCompare(b.short_name);
}

/**
 * The system as a tree (DESIGN.md List with detail): each body under its nearest parent that is a
 * star or a planet and has been scanned, so a body whose parent is not scanned yet hangs from its
 * nearest scanned ancestor, else the root. Barycentres are not drawn. Stars are always at the root.
 * Children, and the roots, go by distance from the arrival star. Depth-first, as the rows read.
 */
export function bodyTree(bodies: readonly Body[]): TreeRow[] {
  const byId = new Map<number, Body>();
  for (const b of bodies) if (b.body_id != null) byId.set(b.body_id, b);
  const parent = (b: Body): Body | null => {
    if (b.star) return null;
    for (const p of b.parents) {
      const up = DRAWN.has(p.kind) ? byId.get(p.body_id) : undefined;
      if (up && up !== b) return up;
    }
    return null;
  };
  const kids = new Map<Body | null, Body[]>();
  for (const b of bodies) {
    const p = parent(b);
    kids.set(p, [...(kids.get(p) ?? []), b]);
  }
  for (const list of kids.values()) list.sort(byDistance);

  const rows: TreeRow[] = [];
  const seen = new Set<Body>();
  const visit = (b: Body, depth: number, last: boolean, through: boolean[]) => {
    seen.add(b);
    const under = (kids.get(b) ?? []).filter((c) => !seen.has(c));
    rows.push({ body: b, depth, last, through, kids: under.length > 0 });
    const next = depth === 0 ? [] : [...through, !last];
    under.forEach((c, i) => visit(c, depth + 1, i === under.length - 1, next));
  };
  const roots = kids.get(null) ?? [];
  roots.forEach((b) => visit(b, 0, true, []));
  // A loop in bad parent data leaves its bodies unreached: they go at the root rather than vanish.
  for (const b of [...bodies].sort(byDistance)) if (!seen.has(b)) visit(b, 0, true, []);
  return rows;
}

const PLANETS: [RegExp, BodyGlyph][] = [
  [/^metal rich/i, "body-metal-rich"],
  [/^high metal content/i, "body-high-metal"],
  [/^rocky ice/i, "body-rocky-ice"],
  [/^rocky/i, "body-rocky"],
  [/^icy/i, "body-icy"],
  [/^earthlike/i, "body-earth-like"],
  [/^water world/i, "body-water-world"],
  [/^ammonia world/i, "body-ammonia"],
  [/^water giant/i, "body-water-giant"],
  [/gas giant/i, "body-gas-giant"],
];

/**
 * A body's type icon (DESIGN.md Icons: the star and body sets) from its journal class. A star goes
 * through the route's star table, scoopable when its class is one of KGBFOAM (giants included);
 * a planet class the set does not draw takes the nearest shape, a rocky disc.
 */
export function bodyGlyph(b: Pick<Body, "star" | "class">): StarGlyph | BodyGlyph {
  if (b.star) return starKind({ star_class: b.class, scoopable: /^[OBAFGKM](_|$)/.test(b.class) ? "yes" : "no", neutron: null }).glyph;
  return PLANETS.find(([re]) => re.test(b.class))?.[1] ?? "body-rocky";
}
