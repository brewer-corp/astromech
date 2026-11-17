/**
 * The engineer unlock guide (astromech_engineering::unlocks): every ship and suit engineer as a
 * tech tree, the engineers who introduce or refer each one, the commander's progress from the
 * journal's EngineerProgress, and what is left to unlock each.
 */
import type { EngineerSearch } from "./api/types/EngineerSearch";
import type { Node as UnlockNode } from "./api/types/Node";
import type { Tree as UnlockTree } from "./api/types/Tree";
import { call } from "./transport";

export type { EngineerStatus as Status } from "./api/types/EngineerStatus";
export type { Node as UnlockNode } from "./api/types/Node";
export type { Tree as UnlockTree } from "./api/types/Tree";

export async function loadUnlocks(): Promise<UnlockTree> {
  return await call("engineering_unlocks");
}

export type { WorksOn } from "./api/types/WorksOn";

/** A node's place in its tree's grid: its column (`Node.depth`) and the rows it spans. */
export type Placed = { node: UnlockNode; col: number; row: number; span: number };

/**
 * One kind's tree laid out as a grid, in the order the nodes come (the catalog's): each engineer
 * spans the rows of the engineers they introduce, so it sits beside them, and one with several
 * introducers (Yi Shen) spans its introducers' rows when they sit together in the column before.
 */
export function placeTree(nodes: UnlockNode[]): { placed: Placed[]; rows: number; cols: number } {
  const byName = new Map(nodes.map((n) => [n.name, n]));
  const kids = new Map<string, UnlockNode[]>();
  const shared: UnlockNode[] = [];
  const roots: UnlockNode[] = [];
  for (const n of nodes) {
    const from = n.introduced_by.filter((r) => byName.has(r));
    if (!from.length) roots.push(n);
    else if (from.length === 1 || nodes.some((m) => m.introduced_by.includes(n.name))) {
      const k = kids.get(from[0]!) ?? [];
      k.push(n);
      kids.set(from[0]!, k);
    } else shared.push(n);
  }
  const at = new Map<string, Placed>();
  let next = 0;
  const place = (n: UnlockNode, row: number): number => {
    let end = row;
    for (const k of kids.get(n.name) ?? []) end = place(k, end);
    const span = Math.max(1, end - row);
    at.set(n.name, { node: n, col: n.depth, row, span });
    return row + span;
  };
  for (const r of roots) next = place(r, next);
  for (const n of shared) {
    const from = n.introduced_by.map((r) => at.get(r)).filter((p) => p !== undefined);
    const top = Math.min(...from.map((p) => p.row));
    const end = Math.max(...from.map((p) => p.row + p.span));
    const free = [...at.values()].every((p) => p.col !== n.depth || p.row + p.span <= top || p.row >= end);
    if (from.length && free) at.set(n.name, { node: n, col: n.depth, row: top, span: end - top });
    else next = place(n, next);
  }
  // Anything left (an introducer missing from the catalog, a cycle) goes at the foot.
  for (const n of nodes) if (!at.has(n.name)) next = place(n, next);
  const placed = nodes.map((n) => at.get(n.name)!);
  return { placed, rows: next, cols: Math.max(0, ...placed.map((p) => p.col)) + 1 };
}

/** "10 of 25 ship and 4 of 13 suit engineers unlocked · 15 to work on", counted from the tree. */
export function treeSummary(t: UnlockTree): string {
  const of = (kind: string) => t.nodes.filter((n) => n.kind === kind);
  const unlocked = (ns: UnlockNode[]) => ns.filter((n) => n.status === "unlocked").length;
  const [ship, suit] = [of("ship"), of("suit")];
  const work = t.nodes.filter((n) => n.next && n.status !== "unlocked").length;
  return `${unlocked(ship)} of ${ship.length} ship and ${unlocked(suit)} of ${suit.length} suit engineers unlocked · ${work} to work on`;
}

/** An engineer's name as the core matches it across catalogs (astromech_engineering::engineers::key):
 * lowercase letters and digits, so 'Tod "The Blaster" McQuinn' finds "Tod 'The Blaster' McQuinn". */
export function engineerKey(name: string): string {
  return name.toLowerCase().replace(/[^a-z0-9]/g, "");
}

/** Straight-line light years between two positions. */
export function lightYears(a: [number, number, number], b: [number, number, number]): number {
  return Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
}

export type { EngineerSearch } from "./api/types/EngineerSearch";
export type { SearchHit } from "./api/types/SearchHit";

/**
 * The Engineers page's search (astromech_engineering::search): who offers a module type ("laser",
 * "fsd") or a suit modification ("range"), the best grade, what the commander can get today and
 * how far.
 */
export async function searchEngineers(query: string): Promise<EngineerSearch> {
  return await call("engineering_search", { query });
}

/** "Grade 5 on all three" when every type is the same, "Pulse 4 · burst 3 · beam 3" when not;
 * types an engineer doesn't offer are left out. One type: "Grade 5". */
export function gradeLine(grades: { label: string; grade: number }[], types: number): string {
  if (!grades.length) return "";
  const g0 = grades[0]!.grade;
  if (types === 1 || grades.length === 1) return grades.length === types ? `Grade ${g0}` : `${grades[0]!.label} ${g0}`;
  if (grades.length === types && grades.every((g) => g.grade === g0)) return `Grade ${g0} on all ${types === 2 ? "both" : types === 3 ? "three" : types}`;
  return grades.map((g, i) => `${i ? g.label.toLowerCase() : g.label} ${g.grade}`).join(" · ");
}
