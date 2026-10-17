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
  /** Asteroid belts (the journal lists them with the rings). */
  belts: boolean;
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

/** The commander's unsold data (astromech_exploration::unsold, through the history). */
export interface Unsold {
  commander: { cmdr: string; name: string };
  cartographic: number;
  systems: number;
  bodies: number;
  organic: number;
  samples: number;
  /** Samples of a species with no known value (counted as 0). */
  unknown_samples: number;
  total: number;
  /** The oldest unsold scan or sample. */
  since: string | null;
  /** Worth a reminder to sell. */
  risky: boolean;
  source?: string;
}

/** `null`: no commander in the history yet. */
export async function loadUnsold(): Promise<Unsold | null> {
  if (!inTauri) return { ...(sample.unsold as Unsold), source: "Sample history" };
  return await invoke<Unsold | null>("exploration_unsold");
}
