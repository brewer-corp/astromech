/**
 * Exploration: the current system's bodies as they are scanned, with their estimated values
 * (crates/astromech-exploration, through the shell's `exploration_system` command, which reads
 * the newest journals).
 */
import { invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import sample from "./fixtures/exploration.json";

/** One scanned body (astromech_exploration::system::BodyView). */
export interface Body {
  name: string;
  /** Without the system's name: "A 2". */
  short_name: string;
  body_id: number | null;
  star: boolean;
  /** The journal's class and how it reads ("Earth-like world"). */
  class: string;
  class_name: string;
  terraformable: boolean;
  landable: boolean;
  rings: boolean;
  /** Surface gravity in g. */
  gravity_g: number | null;
  distance_ls: number | null;
  was_discovered: boolean | null;
  was_mapped: boolean | null;
  was_footfalled: boolean | null;
  /** Mapped by the commander, and within the efficiency target. */
  mapped: boolean;
  mapped_efficiently: boolean;
  bio_signals: number;
  geo_signals: number;
  /** Estimated value now, and mapped efficiently. */
  value: number;
  value_mapped: number;
  first_discovery: boolean;
  worth_mapping: boolean;
  first_footfall_likely: boolean;
  /** The commander's earlier landings on it. */
  landings: number;
}

export interface SystemView {
  system: string | null;
  system_address: number | null;
  population: number;
  /** From the honk. */
  body_count: number | null;
  all_found: boolean;
  bodies: Body[];
  value: number;
  value_if_mapped: number;
  /** Where it came from, when not the live journal. */
  source?: string;
}

export async function loadSystem(): Promise<SystemView> {
  if (!inTauri) return { ...(sample.system as SystemView), source: "Sample system" };
  return await invoke<SystemView>("exploration_system");
}

/** "4.4 M cr", "720 k cr", "3,153 cr". */
export function credits(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)} M cr`;
  if (n >= 10_000) return `${Math.round(n / 1000).toLocaleString()} k cr`;
  return `${n.toLocaleString()} cr`;
}
