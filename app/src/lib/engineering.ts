/**
 * Ship engineering planner: fleet loadouts from the journal, plans per ship (kept in
 * localStorage), and the materials and ship stats a plan gives, computed by the core.
 */
import type { Compared } from "./api/types/Compared";
import type { Engineers } from "./api/types/Engineers";
import type { Fitted } from "./api/types/Fitted";
import type { Fleet } from "./api/types/Fleet";
import type { Headline } from "./api/types/Headline";
import type { Plan } from "./api/types/Plan";
import type { PlanResult } from "./api/types/PlanResult";
import type { PlanTarget as Target } from "./api/types/PlanTarget";
import type { Rolls } from "./api/types/Rolls";
import type { Ship } from "./api/types/Ship";
import type { Trip } from "./api/types/Trip";
import { coreCall } from "./core";
import { call, previewOption } from "./transport";

export type { Applied } from "./api/types/Applied";
export type { Fitted } from "./api/types/Fitted";
export type { Ship } from "./api/types/Ship";
export type { Choice } from "./api/types/Choice";
export type { GroupOptions } from "./api/types/GroupOptions";
export type { Stored as StoredModule } from "./api/types/Stored";
export type { Fleet } from "./api/types/Fleet";
export type { PlanTarget as Target } from "./api/types/PlanTarget";
export type { Plan } from "./api/types/Plan";
export type { Rolls } from "./api/types/Rolls";
export type { TargetCost } from "./api/types/TargetCost";
export type { MatLine } from "./api/types/MatLine";
export type { Trade } from "./api/types/Trade";
export type { FarmStep } from "./api/types/FarmStep";
export type { PlanResult } from "./api/types/PlanResult";
export type { EngineerView } from "./api/types/EngineerView";
export type { Engineers } from "./api/types/Engineers";
export type { Hop } from "./api/types/Hop";
export type { Job } from "./api/types/Job";
export type { Trip } from "./api/types/Trip";
export type { Capacitor } from "./api/types/Capacitor";
export type { ShipSide as ShipStats } from "./api/types/ShipSide";
export type { ShipComparison as StatsComparison } from "./api/types/ShipComparison";
/** The plan's stats now and done, or the hull the stats tables don't know. */
export type { Compared } from "./api/types/Compared";
export type { Headline } from "./api/types/Headline";

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

export const DEFAULT_ROLLS: Rolls = [1, 2, 3, 4, 5];

export async function loadFleet(): Promise<Fleet> {
  return await call("engineering_fleet");
}

/** What the plan costs. The browser preview has no planner: only the sample ship's sample plan
 * has a result, and any other fails. */
export async function evaluatePlan(plan: Plan): Promise<PlanResult> {
  return await call("engineering_plan", { plan });
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

/** Current and planned stats. In the browser preview only the sample's precomputed plans have
 * them, and any other fails. */
export async function shipStats(plan: Plan): Promise<Compared> {
  return await call("engineering_stats", { plan });
}

/** Card stats by ship id, for every ship with a known loadout. */
export async function loadHeadlines(): Promise<Record<number, Headline>> {
  return await call("engineering_headlines");
}

const key = (shipId: number) => `astromech.engineering.plan.${shipId}`;

function planFrom(shipId: number, raw: string | null): Plan | null {
  try {
    const p = raw ? (JSON.parse(raw) as Plan) : null;
    return p && Array.isArray(p.targets) ? { ship_id: shipId, targets: p.targets, rolls: p.rolls?.length === 5 ? p.rolls : DEFAULT_ROLLS } : null;
  } catch {
    return null; // a damaged entry
  }
}

/** The ship's plan kept here, else (the browser preview) the sample's, else an empty one. */
export function savedPlan(shipId: number): Plan {
  const k = key(shipId);
  return planFrom(shipId, localStorage.getItem(k)) ?? planFrom(shipId, previewOption(k)) ?? { ship_id: shipId, targets: [], rolls: DEFAULT_ROLLS };
}

export function savePlan(plan: Plan): void {
  localStorage.setItem(key(plan.ship_id), JSON.stringify(plan));
}

export async function loadEngineers(): Promise<Engineers> {
  return await call("engineering_engineers");
}

/** The engineer trip for the plan. In the browser preview only the sample plan has one, and any
 * other fails. */
export async function planTrip(plan: Plan): Promise<Trip> {
  return await call("engineering_trip", { plan });
}

export function shipTitle(s: Ship): string {
  return s.name ? `${s.name} · ${s.type_name}` : s.type_name;
}
