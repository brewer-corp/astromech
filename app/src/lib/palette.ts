/**
 * The Ctrl+K command palette's model (docs/ARCHITECTURE.md §18.2): every page, plus the entries
 * each module's manifest offers from data the window already has (materials, the fleet's
 * ships, modules and blueprints). Picking one opens its route: a material opens selected, with
 * its nearby search; a ship, module, blueprint or experimental opens the ship's page.
 */
import type { ModuleManifest } from "./modules";

export interface PaletteItem {
  label: string;
  /** What it is, shown dim: "Page · Engineering", "Material · Manufactured G3". */
  detail: string;
  /** Route to open, without "#/": "materials/protoheatradiators". */
  route: string;
  /** Words that match but aren't the label, e.g. the module a blueprint is for. */
  keywords?: string;
}

const words = (s: string) => s.toLowerCase().split(/[^a-z0-9]+/).filter(Boolean);

/**
 * How well `item` matches the normalised query (lower is better), or null for no match: the
 * whole label, its start, the start of its words ("heat rad"), its initials ("fsd"), anywhere in
 * it, then every word somewhere in the label, detail or keywords.
 */
function score(item: PaletteItem, q: string, qw: string[]): number | null {
  const label = item.label.toLowerCase();
  if (label === q) return 0;
  if (label.startsWith(q)) return 1;
  const lw = words(label);
  if (qw.every((w) => lw.some((l) => l.startsWith(w)))) return 2;
  if (qw.length === 1 && q.length > 1 && lw.map((l) => l[0]).join("").startsWith(q)) return 2;
  if (label.includes(q)) return 3;
  const all = words(`${item.label} ${item.detail} ${item.keywords ?? ""}`);
  if (qw.every((w) => all.some((a) => a.startsWith(w)))) return 4;
  return null;
}

/** The best `limit` matches for `query`, ties in `items` order; a blank query lists the first ones. */
export function rank(items: PaletteItem[], query: string, limit = 12): PaletteItem[] {
  const q = query.trim().toLowerCase().replace(/\s+/g, " ");
  if (!q) return items.slice(0, limit);
  const qw = words(q);
  if (!qw.length) return [];
  return items
    .map((item, i) => ({ item, i, s: score(item, q, qw) }))
    .filter((x): x is { item: PaletteItem; i: number; s: number } => x.s !== null)
    .sort((a, b) => a.s - b.s || a.i - b.i)
    .slice(0, limit)
    .map((x) => x.item);
}

/** One entry per page, then what each module offers; a module whose entries fail to load is skipped. */
export async function collect(modules: ModuleManifest[], groupTitle: (id: string) => string): Promise<PaletteItem[]> {
  const pages = modules.map((m) => ({ label: m.title, detail: `Page · ${groupTitle(m.group)}`, route: m.id, keywords: m.summary }));
  const offered = await Promise.all(modules.map((m) => (m.palette ? m.palette().catch(() => []) : [])));
  return [...pages, ...offered.flat()];
}
