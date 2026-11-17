/**
 * The engineer unlock guide (astromech_engineering::unlocks): every ship and suit engineer as a
 * tech tree, the engineers who introduce or refer each one, the commander's progress from the
 * journal's EngineerProgress, and what is left to unlock each.
 */
import type { EngineerSearch } from "./api/types/EngineerSearch";
import type { EngineerStatus as Status } from "./api/types/EngineerStatus";
import type { Node as UnlockNode } from "./api/types/Node";
import type { Tree as UnlockTree } from "./api/types/Tree";
import { call } from "./transport";

export type { EngineerStatus as Status } from "./api/types/EngineerStatus";
export type { Node as UnlockNode } from "./api/types/Node";
export type { Tree as UnlockTree } from "./api/types/Tree";

export async function loadUnlocks(): Promise<UnlockTree> {
  return await call("engineering_unlocks");
}

/** The tree's columns for one kind: nodes by depth, next ones first, then by name. */
export function columns(t: UnlockTree, kind: "ship" | "suit"): UnlockNode[][] {
  const nodes = t.nodes.filter((n) => n.kind === kind);
  const out: UnlockNode[][] = [];
  for (const n of nodes) (out[n.depth] ??= []).push(n);
  const order: Record<Status, number> = { invited: 0, known: 1, locked: 2, unlocked: 3, barred: 4 };
  return out.map((col) => col.sort((a, b) => Number(b.next) - Number(a.next) || order[a.status] - order[b.status] || a.name.localeCompare(b.name)));
}

export const STATUS_LABEL: Record<Status, string> = {
  unlocked: "Unlocked",
  invited: "Invited",
  known: "Known",
  locked: "Locked",
  barred: "Barred",
};

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
