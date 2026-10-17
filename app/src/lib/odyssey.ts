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

export interface OdysseyView {
  source: string;
  locker_ts: string | null;
  backpack_ts: string | null;
  items: ItemLine[];
  engineers: EngineerLine[];
  gear: GearRef[];
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

export interface Give {
  id: string;
  name: string;
  count: number;
  barter: number;
}

/** A short asset covered from surplus at a bartender. */
export interface Swap {
  get_id: string;
  get_name: string;
  get: number;
  barter_wanted: number;
  gives: Give[];
  barter_given: number;
}

export interface Left {
  id: string;
  name: string;
  category: Category;
  short: number;
}

export interface BarterPlan {
  swaps: Swap[];
  left: Left[];
  barter_given: number;
}

export interface Shopping {
  picks: PickResult[];
  total: Need[];
  credits: number;
  credits_partial: boolean;
  barter_short: number;
  trade: BarterPlan;
}

export const MAKERS: Record<string, string> = { kinematic: "Kinematic Armaments", manticore: "Manticore", takada: "Takada" };

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
