/**
 * Odyssey on-foot micro-resources: the ShipLocker/Backpack inventory, on-foot engineers, and
 * what picked suit/weapon upgrades and modifications need (astromech-odyssey, in the core).
 * Picks are kept in localStorage. Outside the app only the sample picks have a shopping list.
 */
import type { ModRef } from "./api/types/ModRef";
import type { OdysseyView } from "./api/types/OdysseyView";
import type { OwnedGear } from "./api/types/OwnedGear";
import type { Pick } from "./api/types/Pick";
import type { Shopping } from "./api/types/Shopping";
import { call, previewOption } from "./transport";

export type Category = "goods" | "assets" | "data" | "consumables";

export type { BarterPlan } from "./api/types/BarterPlan";
export type { EngineerLine } from "./api/types/EngineerLine";
export type { FitBy } from "./api/types/FitBy";
export type { GearRef } from "./api/types/GearRef";
export type { Give } from "./api/types/Give";
export type { ItemLine } from "./api/types/ItemLine";
export type { Left } from "./api/types/Left";
export type { ModRef } from "./api/types/ModRef";
export type { OdysseyNeed as Need } from "./api/types/OdysseyNeed";
export type { OdysseyView } from "./api/types/OdysseyView";
export type { OwnedGear } from "./api/types/OwnedGear";
export type { Pick } from "./api/types/Pick";
export type { PickResult } from "./api/types/PickResult";
export type { Shopping } from "./api/types/Shopping";
/** A short asset covered from surplus at a bartender. */
export type { Swap } from "./api/types/Swap";

export const MAKERS: Record<string, string> = { kinematic: "Kinematic Armaments", manticore: "Manticore", takada: "Takada" };

/** Whether the picker still offers `m`: not already on everything owned of its kind. */
export function offered(m: ModRef): boolean {
  if (m.fitted) return false;
  return !(m.makers?.length && m.makers.every((mk) => m.fitted_makers?.includes(mk)));
}

/** The grades of `gear` the commander owns, lowest first. */
export function ownedGrades(owned: OwnedGear[], gear: string): number[] {
  return [...new Set(owned.filter((g) => g.gear === gear).map((g) => g.grade))].sort((a, b) => a - b);
}

/** The grades a pick can upgrade `gear` from: the owned ones (below 5), else any. */
export function fromGrades(owned: OwnedGear[], gear: string): number[] {
  const mine = ownedGrades(owned, gear);
  return mine.length ? mine.filter((g) => g < 5) : [1, 2, 3, 4];
}

export async function loadOdyssey(): Promise<OdysseyView> {
  return await call("odyssey_view");
}

export const pickKey = (p: Pick) => (p.kind === "upgrade" ? `u:${p.gear}:${p.from}:${p.to}` : `m:${p.id}:${p.maker ?? ""}`);

export function samePicks(a: Pick[], b: Pick[]): boolean {
  return a.length === b.length && a.every((p, i) => pickKey(p) === pickKey(b[i]!));
}

/** The list for `picks`. The browser preview has one only for the sample picks; others fail. */
export async function loadShopping(picks: Pick[]): Promise<Shopping> {
  return await call("odyssey_shopping", { picks });
}

const KEY = "astromech.odyssey.picks";

function picksFrom(raw: string | null): Pick[] | null {
  try {
    const p = raw ? (JSON.parse(raw) as unknown) : null;
    return Array.isArray(p) ? (p as Pick[]) : null;
  } catch {
    return null; // a damaged entry
  }
}

/** The picks kept here, else (the browser preview) the sample's. */
export function savedPicks(): Pick[] {
  return picksFrom(localStorage.getItem(KEY)) ?? picksFrom(previewOption(KEY)) ?? [];
}

export function savePicks(picks: Pick[]): void {
  localStorage.setItem(KEY, JSON.stringify(picks));
}

export function hasPick(picks: Pick[], p: Pick): boolean {
  return picks.some((x) => pickKey(x) === pickKey(p));
}
