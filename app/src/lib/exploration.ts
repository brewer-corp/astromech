/**
 * Exploration: the current system's bodies as they are scanned, with their estimated values
 * (crates/astromech-exploration, through the core's `exploration_system` command, which reads
 * the newest journals).
 */
import type { SystemView as System } from "./api/types/SystemView";
import type { UnsoldData } from "./api/types/UnsoldData";
import { call, type Sampled } from "./transport";

export type { BodyView as Body } from "./api/types/BodyView";
export type SystemView = Sampled<System>;
/** The commander's unsold data (astromech_exploration::unsold, through the history). */
export type Unsold = Sampled<UnsoldData>;

export async function loadSystem(): Promise<SystemView> {
  return await call("exploration_system");
}

/** "4.4 M cr", "720 k cr", "3,153 cr". */
export function credits(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)} M cr`;
  if (n >= 10_000) return `${Math.round(n / 1000).toLocaleString()} k cr`;
  return `${n.toLocaleString()} cr`;
}

/** `null`: no commander in the history yet. */
export async function loadUnsold(): Promise<Unsold | null> {
  return await call("exploration_unsold");
}
