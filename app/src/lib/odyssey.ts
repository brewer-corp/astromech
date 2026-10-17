/**
 * Odyssey on-foot micro-resources: the ShipLocker/Backpack inventory, on-foot engineers, and
 * what picked suit/weapon upgrades and modifications need (astromech-odyssey, via the shell).
 * Picks are kept in localStorage. Outside the app only the sample picks have a shopping list.
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/odyssey.json";

export type Category = "goods" | "assets" | "data" | "consumables";

export interface ItemLine {
  id: string;
  name: string;
  category: Category;
  locker: number;
  backpack: number;
  mission: number;
  barter: number | null;
  value: number | null;
}

export interface ModRef {
  id: string;
  name: string;
  kind: "suit" | "weapon" | string;
  unconfirmed?: boolean;
  makers?: string[];
  /** Everything owned of its kind already has it (hidden from the picker). */
  fitted?: boolean;
  /** Weapon makers whose owned weapons all have it. */
  fitted_makers?: string[];
}

export interface EngineerLine {
  id: number;
  name: string;
  system: string;
  colonia: boolean;
  progress: string;
  mods: ModRef[];
  referred_by: string[];
  refers: string[];
  invite: string;
  referral_task: string | null;
  next: string;
}

export interface GearRef {
  id: string;
  name: string;
  kind: "suit" | "weapon";
  maker: string | null;
}

/** A suit or weapon the commander owns, from the journal. */
export interface OwnedGear {
  gear: string;
  name: string;
  kind: "suit" | "weapon";
  maker: string | null;
  grade: number;
  /** Modification ids fitted. */
  mods: string[];
}

export interface OdysseyView {
  source: string;
  locker_ts: string | null;
  backpack_ts: string | null;
  items: ItemLine[];
  engineers: EngineerLine[];
  gear: GearRef[];
  owned: OwnedGear[];
  mods: ModRef[];
  data_source: string;
  data_date: string;
  data_uncertain: string[];
}

export type Pick =
  | { kind: "upgrade"; gear: string; from: number; to: number }
  | { kind: "mod"; id: string; maker?: string | null };

export interface Need {
  id: string;
  name: string;
  category: Category;
  need: number;
  have: number;
  short: number;
  barter: number | null;
}

export interface FitBy {
  name: string;
  system: string;
  unlocked: boolean;
  unconfirmed: boolean;
}

export interface PickResult {
  pick: Pick;
  title: string;
  lines: Need[];
  credits: number;
  credits_partial: boolean;
  fitted_by: FitBy[];
  ready: boolean;
  /** Why this pick can't be costed; it adds nothing to the list. */
  error: string | null;
}

export interface Shopping {
  picks: PickResult[];
  total: Need[];
  credits: number;
  credits_partial: boolean;
  barter_short: number;
}

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
  if (!inTauri) return sample.view as unknown as OdysseyView;
  return await invoke<OdysseyView>("odyssey_view");
}

export const pickKey = (p: Pick) => (p.kind === "upgrade" ? `u:${p.gear}:${p.from}:${p.to}` : `m:${p.id}:${p.maker ?? ""}`);

export function samePicks(a: Pick[], b: Pick[]): boolean {
  return a.length === b.length && a.every((p, i) => pickKey(p) === pickKey(b[i]!));
}

/** The list for `picks`; outside the app only for the sample picks (else null). */
export async function loadShopping(picks: Pick[]): Promise<Shopping | null> {
  if (!inTauri) return samePicks(picks, sample.picks as Pick[]) ? (sample.shopping as unknown as Shopping) : null;
  return await invoke<Shopping>("odyssey_shopping", { picks });
}

const KEY = "astromech.odyssey.picks";

export function savedPicks(): Pick[] {
  try {
    const raw = localStorage.getItem(KEY);
    if (raw) {
      const p = JSON.parse(raw) as unknown;
      if (Array.isArray(p)) return p as Pick[];
    }
  } catch {
    // ignore a damaged entry
  }
  return inTauri ? [] : structuredClone(sample.picks as Pick[]);
}

export function savePicks(picks: Pick[]): void {
  localStorage.setItem(KEY, JSON.stringify(picks));
}

export function hasPick(picks: Pick[], p: Pick): boolean {
  return picks.some((x) => pickKey(x) === pickKey(p));
}
