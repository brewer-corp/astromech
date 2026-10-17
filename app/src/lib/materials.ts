/**
 * Engineering materials: trader-style inventory from the journal.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewData } from "./fakecore";
import sample from "./fixtures/materials.json";

export interface SourceNote {
  kind: "hge" | "not_hge" | string;
  summary: string;
  hint: string;
  /** Live nearest-systems lookup id (lib/galaxy.ts), if one applies. */
  query?: string;
}

export interface Cell {
  id: string;
  name: string;
  kind: "raw" | "manufactured" | "encoded";
  group: string;
  grade: number;
  rarity: string;
  count: number;
  capacity: number;
  trader: boolean;
  sources?: SourceNote[];
}

export interface Here {
  system: string;
  allegiance: string;
  population: number;
  hge_signal: boolean;
  pool: { id: string; name: string }[];
  line: string;
}

export interface FarmLine {
  id: string;
  name: string;
  count: number;
  capacity: number;
  summary: string;
}

export interface GroupView {
  name: string;
  cells: Cell[];
}

export interface KindView {
  id: string;
  title: string;
  trader: string;
  grades: number[];
  groups: GroupView[];
  held: number;
  capacity: number;
}

export interface Inventory {
  source: string;
  snapshot_ts: string | null;
  kinds: KindView[];
  other: Cell[];
  here?: Here | null;
  farm?: FarmLine[];
}

/** The same layout with nothing held, no snapshot and nowhere known (the preview's `?core=empty`). */
function nothingHeld(): Inventory {
  const inv = structuredClone(sample) as Inventory;
  const zero = (c: Cell) => ({ ...c, count: 0 });
  return {
    ...inv,
    source: "No journal yet",
    snapshot_ts: null,
    kinds: inv.kinds.map((k) => ({ ...k, held: 0, groups: k.groups.map((g) => ({ ...g, cells: g.cells.map(zero) })) })),
    other: [],
    here: null,
    farm: [],
  };
}

export async function loadMaterials(): Promise<Inventory> {
  if (!inTauri) return previewData("materials_inventory", () => sample as Inventory, nothingHeld);
  return await invoke<Inventory>("materials_inventory");
}
