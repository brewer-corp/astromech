/**
 * Ship engineering planner: fleet loadouts from the journal, plans per ship (kept in
 * localStorage), and the materials a plan needs, computed by the shell.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import type { SourceNote } from "./materials";
import sample from "./fixtures/engineering.json";

export interface Applied {
  blueprint: string;
  blueprint_name: string;
  grade: number;
  quality: number;
  experimental: string | null;
  experimental_name: string | null;
  engineer: string | null;
}

export interface Fitted {
  slot: string;
  slot_name: string;
  item: string;
  name: string;
  group: string | null;
  engineering?: Applied;
}

export interface Ship {
  id: number;
  ship_type: string;
  type_name: string;
  name: string | null;
  ident: string | null;
  system: string | null;
  current: boolean;
  loadout_ts: string | null;
  modules: Fitted[];
}

export interface Choice {
  id: string;
  name: string;
  max_grade?: number;
}

export interface GroupOptions {
  name: string;
  blueprints: Choice[];
  experimentals: Choice[];
}

export interface Fleet {
  source: string;
  ships: Ship[];
  groups: Record<string, GroupOptions>;
}

export interface Target {
  slot: string;
  blueprint: string;
  grade: number;
  experimental: string | null;
}

export interface Plan {
  ship_id: number;
  targets: Target[];
  rolls: number[];
}

export interface TargetCost {
  slot: string;
  slot_name: string;
  module: string;
  blueprint: string;
  blueprint_name: string;
  grade: number;
  experimental: string | null;
  experimental_name: string | null;
  from: Applied | null;
  steps: [number, number][];
  experimental_needed: boolean;
  materials: [string, number][];
  engineers: string[];
  note: string | null;
}

export interface MatLine {
  id: string;
  name: string;
  kind: "raw" | "manufactured" | "encoded" | null;
  group: string;
  grade: number;
  need: number;
  have: number;
  short: number;
  capacity: number;
  over_cap: boolean;
  sources?: SourceNote[];
}

export interface Trade {
  give_id: string;
  give_name: string;
  give: number;
  get_id: string;
  get_name: string;
  get: number;
  trader: string;
}

export interface FarmStep {
  id: string;
  name: string;
  kind: "raw" | "manufactured" | "encoded" | null;
  grade: number;
  collect: number;
  have: number;
  capacity: number;
  over_cap: boolean;
  sources?: SourceNote[];
  then?: Trade[];
}

export interface PlanResult {
  ship_id: number;
  rolls: number[];
  targets: TargetCost[];
  materials: MatLine[];
  trades: Trade[];
  farm: FarmStep[];
  snapshot_ts: string | null;
}

export const DEFAULT_ROLLS = [1, 2, 3, 4, 5];

const samplePlan = sample.plan as unknown as PlanResult;

export async function loadFleet(): Promise<Fleet> {
  if (!inTauri) return sample.fleet as unknown as Fleet;
  try {
    return await invoke<Fleet>("engineering_fleet");
  } catch {
    return { ...(sample.fleet as unknown as Fleet), source: "Sample journal (no live journal folder)" };
  }
}

/** Outside the app there is no planner: only the sample ship's sample plan has a result. */
export async function evaluatePlan(plan: Plan): Promise<PlanResult | null> {
  if (!inTauri) return plan.ship_id === samplePlan.ship_id ? samplePlan : null;
  return await invoke<PlanResult>("engineering_plan", { plan });
}

const key = (shipId: number) => `astromech.engineering.plan.${shipId}`;

export function savedPlan(shipId: number): Plan {
  try {
    const raw = localStorage.getItem(key(shipId));
    if (raw) {
      const p = JSON.parse(raw) as Plan;
      if (Array.isArray(p.targets)) return { ship_id: shipId, targets: p.targets, rolls: p.rolls?.length === 5 ? p.rolls : DEFAULT_ROLLS };
    }
  } catch {
    // ignore a damaged entry
  }
  if (!inTauri && shipId === sample.plan_input.ship_id) return structuredClone(sample.plan_input) as Plan;
  return { ship_id: shipId, targets: [], rolls: DEFAULT_ROLLS };
}

export function savePlan(plan: Plan): void {
  localStorage.setItem(key(plan.ship_id), JSON.stringify(plan));
}

export function shipTitle(s: Ship): string {
  return s.name ? `${s.name} · ${s.type_name}` : s.type_name;
}
