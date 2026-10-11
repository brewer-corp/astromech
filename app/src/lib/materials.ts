/**
 * Engineering materials: trader-style inventory from the journal.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/materials.json";

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
}

export async function loadMaterials(): Promise<Inventory> {
  if (!inTauri) return sample as Inventory;
  try {
    return await invoke<Inventory>("materials_inventory");
  } catch {
    return { ...(sample as Inventory), source: "Sample journal (no live journal folder)" };
  }
}
