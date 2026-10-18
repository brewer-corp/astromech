/**
 * The engineer unlock guide (astromech_engineering::unlocks): every ship and suit engineer as a
 * tech tree, the engineers who introduce or refer each one, the commander's progress from the
 * journal's EngineerProgress, and what is left to unlock each.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewData } from "./fakecore";
import sample from "./fixtures/unlocks.json";

export type Status = "unlocked" | "invited" | "known" | "locked" | "barred";

export interface UnlockNode {
  name: string;
  kind: "ship" | "suit";
  system: string;
  permit: boolean;
  /** Column: 0 for common knowledge, else one more than the deepest introducer. */
  depth: number;
  introduced_by: string[];
  status: Status;
  rank: number | null;
  /** What is left; null once unlocked. */
  left: string | null;
  /** Can be worked on now. */
  next: boolean;
}

export interface UnlockTree {
  nodes: UnlockNode[];
}

export async function loadUnlocks(): Promise<UnlockTree> {
  if (!inTauri) {
    // Nothing unlocked yet: every engineer locked, none known.
    const locked = (): UnlockTree => ({ nodes: (sample as UnlockTree).nodes.map((n) => ({ ...n, status: "locked", rank: null, next: false })) });
    return previewData("engineering_unlocks", () => sample as UnlockTree, locked);
  }
  return await invoke<UnlockTree>("engineering_unlocks");
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
