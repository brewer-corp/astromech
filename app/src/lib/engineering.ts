/**
 * Ship engineering planner: fleet loadouts from the journal, plans per ship (kept in
 * localStorage), and the materials and ship stats a plan gives, computed by the shell.
 */
import { invoke } from "@tauri-apps/api/core";
import { coreCall, inTauri } from "./core";
import { previewData } from "./fakecore";
import type { SourceNote } from "./materials";
import sample from "./fixtures/engineering.json";
import shipsSample from "./fixtures/ships.json";

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

/** Outfitting groups, in the game's own order. Optional internals is the remainder. */
export const MODULE_KINDS: { title: string; test: (slot: string) => boolean }[] = [
  { title: "Hardpoints", test: (s) => /Hardpoint/.test(s) && !/Tiny/.test(s) },
  { title: "Utility", test: (s) => /TinyHardpoint/.test(s) },
  { title: "Core internals", test: (s) => /^(Armour|PowerPlant|MainEngines|FrameShiftDrive|LifeSupport|PowerDistributor|Radar|FuelTank)$/.test(s) },
  { title: "Military", test: (s) => /Military/.test(s) },
  { title: "Optional internals", test: () => true },
];

export function groupFitted(mods: Fitted[], engineerable = false): { title: string; mods: Fitted[] }[] {
  const src = engineerable ? mods.filter((m) => m.group) : mods;
  const taken = new Set<string>();
  return MODULE_KINDS.map((k) => {
    const mine = src.filter((m) => !taken.has(m.slot) && k.test(m.slot));
    for (const m of mine) taken.add(m.slot);
    return { title: k.title, mods: mine };
  }).filter((k) => k.mods.length);
}

export function fittedLine(m: Fitted): string {
  const e = m.engineering;
  if (!m.group) return "";
  if (!e) return "Not engineered";
  const pct = e.quality >= 1 ? "" : ` ${Math.round(e.quality * 100)}%`;
  return `G${e.grade}${pct} ${e.blueprint_name}${e.experimental_name ? ` · ${e.experimental_name}` : ""}`;
}

export interface Ship {
  id: number;
  ship_type: string;
  type_name: string;
  name: string | null;
  ident: string | null;
  system: string | null;
  current: boolean;
  /** Being moved to a shipyard; `system` is where it is going, if known. */
  in_transit: boolean;
  /** Market id of the station a stored ship is at, when the journal said. */
  market_id: number | null;
  loadout_ts: string | null;
  /** Unladen maximum from the last Loadout, in ly. */
  max_jump_range: number | null;
  value: number | null;
  rebuy: number | null;
  modules: Fitted[];
  /** Open this loadout in Coriolis / EDSY. Absent when the saved Loadout is stale. */
  links?: { coriolis: string; edsy: string };
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

export interface StoredModule {
  item: string;
  name: string;
  /** Null while in transit. */
  system: string | null;
  value: number | null;
  blueprint_name: string | null;
  grade: number | null;
}

export interface Fleet {
  source: string;
  ships: Ship[];
  stored_modules: StoredModule[];
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

export interface EngineerView {
  id: number;
  name: string;
  system: string;
  coords: [number, number, number];
  colonia: boolean;
  progress: string;
  rank: number | null;
  rank_progress: number | null;
}

export interface Engineers {
  engineers: EngineerView[];
  here: { system: string; pos: [number, number, number] } | null;
}

export interface Hop {
  ly: number;
  jumps: number;
  exact: boolean;
}

export interface Job {
  slot: string;
  slot_name: string;
  module: string;
  kind: "grades" | "experimental";
  blueprint: string;
  what: string;
  grade: number;
  note?: string;
}

export interface Trip {
  start: string | null;
  range: number;
  stops: { engineer: string; system: string; rank: number | null; hop: Hop; jobs: Job[] }[];
  blocked: { job: Job; reason: string; unlock: [string, string][] }[];
  total_jumps: number;
  exact: boolean;
}

export interface Capacitor {
  /** MJ */
  capacity: number;
  /** MW at four pips */
  recharge: number;
}

/** One build's stats (astromech-ship). Fractions for resistances; null where the ship has none. */
export interface ShipStats {
  hull_mass: number;
  unladen_mass: number;
  laden_mass: number;
  fuel_capacity: number;
  cargo_capacity: number;
  passenger_capacity: number;
  jump: { max: number; unladen: number; laden: number; total_unladen: number; total_laden: number } | null;
  power: { generation: number; retracted: number; deployed: number };
  shield: { strength: number; explosive: number; kinetic: number; thermal: number } | null;
  armour: {
    integrity: number;
    explosive: number;
    kinetic: number;
    thermal: number;
    caustic: number;
    module_armour: number;
    module_protection: number;
  };
  /** `sustained_dps` waits on decision astromech-bnnh and is not shown. */
  weapons: { dps: number; sustained_dps: number; eps: number; sustained_eps: number };
  distributor: { weapons: Capacitor; systems: Capacitor; engines: Capacitor } | null;
  speed: number | null;
  boost: number | null;
  /** Modules, blueprints, grades or experimentals the calculations don't know: stats leave them out. */
  unresolved: string[];
  /** Seconds of fire at four pips to weapons until the capacitor is empty; null if it keeps up. */
  time_to_drain: number | null;
}

export interface StatsComparison {
  current: ShipStats;
  planned: ShipStats;
}

/** The ship calculations don't know this hull: no stats, the ship shows as incomplete. */
export interface UnknownHull {
  unknown_hull: string;
}

export type Compared = StatsComparison | UnknownHull;

/** What a ship row shows, from the current modules. */
export interface Headline {
  /** Maximum jump range, ly */
  jump: number | null;
  /** Shield strength, MJ */
  shield: number | null;
  dps: number | null;
  /** What the calculations left out; "Hull: <ship>" when they don't know the hull. */
  unresolved: string[];
}

export const DEFAULT_ROLLS = [1, 2, 3, 4, 5];

const samplePlan = sample.plan as unknown as PlanResult;

/** The sample journal's fleet, with the synthetic combat Krait whose stats the sample has (stored). */
function sampleFleet(): Fleet {
  const f = sample.fleet as unknown as Fleet;
  const s = shipsSample.fleet as unknown as Fleet;
  const extra = s.ships.map((x) => ({ ...x, current: false }));
  return { ...f, ships: [...f.ships, ...extra], groups: { ...f.groups, ...s.groups } };
}

export async function loadFleet(): Promise<Fleet> {
  if (!inTauri) return previewData("engineering_fleet", sampleFleet, () => ({ source: "No journal yet", ships: [], stored_modules: [], groups: {} }));
  return await invoke<Fleet>("engineering_fleet");
}

/** Outside the app there is no planner: only the sample ship's sample plan has a result. */
export async function evaluatePlan(plan: Plan): Promise<PlanResult | null> {
  if (!inTauri) return previewData("engineering_plan", () => (plan.ship_id === samplePlan.ship_id ? samplePlan : null), () => null);
  return await invoke<PlanResult>("engineering_plan", { plan });
}

/** One material for the core's shopping list: collect it until holding `until`. */
export interface Want {
  id: string;
  until: number;
}

/**
 * What the core's arrival briefing listens for: the materials this plan still has to collect, each
 * with the count to hold when done. The core compares that with the materials held, so collecting
 * in game needs no new list. An empty plan sends an empty list, which drops the ship.
 */
export function shoppingChange(r: PlanResult): { ship_id: number; materials: Want[] } {
  return { ship_id: r.ship_id, materials: r.farm.map((f) => ({ id: f.id, until: f.have + f.collect })) };
}

function sendShopping(change: { ship_id: number; materials: Want[] }): void {
  if (!inTauri) return;
  coreCall("shopping_set", JSON.stringify(change)).catch(() => {});
}

/**
 * Tell the core what this plan still needs, so arriving in a system whose state gives one of them
 * is spoken (core callouts.rs `shopping_set`). Best effort: an older or stopped core is ignored.
 */
export function shareShoppingList(r: PlanResult): void {
  sendShopping(shoppingChange(r));
}

/** The ship has no plan the core should listen for (it left the fleet). */
export function dropShoppingList(shipId: number): void {
  sendShopping({ ship_id: shipId, materials: [] });
}

const targetKey = (t: Target) => [t.slot, t.blueprint, t.grade, t.experimental ?? ""].join("|");

/** Same ship and the same targets, in any order. */
export function samePlan(a: Plan, b: Plan): boolean {
  const ka = a.targets.map(targetKey).sort();
  const kb = b.targets.map(targetKey).sort();
  return a.ship_id === b.ship_id && ka.length === kb.length && ka.every((k, i) => k === kb[i]);
}

/** Current and planned stats. Outside the app only the sample's precomputed plans have them. */
export async function shipStats(plan: Plan): Promise<Compared | null> {
  if (!inTauri) {
    const sampled = () => {
      const c = shipsSample.cases.find((c) => samePlan(c.plan as unknown as Plan, plan));
      return c ? (c.stats as unknown as StatsComparison) : null;
    };
    return previewData("engineering_stats", sampled, () => null);
  }
  return await invoke<Compared>("engineering_stats", { plan });
}

/** Card stats by ship id, for every ship with a known loadout. */
export async function loadHeadlines(): Promise<Record<string, Headline>> {
  if (!inTauri) return previewData("engineering_headlines", () => shipsSample.headlines as unknown as Record<string, Headline>, () => ({}));
  return await invoke<Record<string, Headline>>("engineering_headlines");
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

export async function loadEngineers(): Promise<Engineers> {
  if (!inTauri) return previewData("engineering_engineers", () => sample.engineers as unknown as Engineers, () => ({ engineers: [], here: null }));
  return await invoke<Engineers>("engineering_engineers");
}

/** Outside the app only the sample plan has a trip. */
export async function planTrip(plan: Plan): Promise<Trip | null> {
  if (!inTauri) return previewData("engineering_trip", () => (plan.ship_id === samplePlan.ship_id ? (sample.trip as unknown as Trip) : null), () => null);
  return await invoke<Trip>("engineering_trip", { plan });
}

export function shipTitle(s: Ship): string {
  return s.name ? `${s.name} · ${s.type_name}` : s.type_name;
}
