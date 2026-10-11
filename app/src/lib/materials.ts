/**
 * Engineering materials: trader-style inventory from the journal.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/materials.json";

export interface SourceNote {
  kind: "hge" | "not_hge" | string;
  summary: string;
  hint: string;
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

export async function loadMaterials(): Promise<Inventory> {
  if (!inTauri) return sample as Inventory;
  try {
    return await invoke<Inventory>("materials_inventory");
  } catch {
    return { ...(sample as Inventory), source: "Sample journal (no live journal folder)" };
  }
}
