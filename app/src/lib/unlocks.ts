/**
 * The engineer unlock guide (astromech_engineering::unlocks): every ship and suit engineer as a
 * tech tree, the engineers who introduce or refer each one, the commander's progress from the
 * journal's EngineerProgress, and what is left to unlock each.
 */
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
