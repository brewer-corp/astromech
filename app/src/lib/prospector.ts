/**
 * Prospector callout thresholds (Settings → Callouts). The core keeps them; see
 * `crates/astromech-core/src/callouts.rs` (`prospector_thresholds`, `prospector_set`).
 */
import { coreCall } from "./core";

export interface OreThreshold {
  ore: string;
  name: string;
  /** Default percentage, or null when the ore is only called out as a motherlode. */
  default: number | null;
  /** Percentage in force, or null when it isn't called out. */
  threshold: number | null;
}

/** What the commander typed in a threshold box: a percentage, off (empty), or invalid. */
export function parseThreshold(input: string): number | null | "invalid" {
  const t = input.trim().replace(/%$/, "").trim();
  if (t === "") return null;
  const n = Number(t);
  return Number.isFinite(n) && n >= 0 && n <= 100 ? n : "invalid";
}

async function reply(r: Promise<Record<string, unknown>>): Promise<OreThreshold[]> {
  const v = await r;
  if (!v.ok) throw new Error(String(v.error ?? "the core didn't answer"));
  return v.ores as OreThreshold[];
}

export function loadThresholds(): Promise<OreThreshold[]> {
  return reply(coreCall("prospector_thresholds"));
}

/** Set (a number), turn off (null) or reset to the default ("reset") one ore. */
export function setThreshold(ore: string, threshold: number | null | "reset"): Promise<OreThreshold[]> {
  const change = threshold === "reset" ? { ore, reset: true } : { ore, threshold };
  return reply(coreCall("prospector_set", JSON.stringify(change)));
}
