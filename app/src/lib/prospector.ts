/**
 * Prospector callout thresholds (Settings → Callouts). The core keeps them; see
 * `crates/astromech-core/src/callouts.rs` (`prospector_thresholds`, `prospector_set`).
 */
import type { Answer } from "./api/types/Answer";
import type { OreThreshold } from "./api/types/OreThreshold";
import type { Ores } from "./api/types/Ores";
import { coreCall } from "./core";

export type { OreThreshold } from "./api/types/OreThreshold";

/** What the commander typed in a threshold box: a percentage, off (empty), or invalid. */
export function parseThreshold(input: string): number | null | "invalid" {
  const t = input.trim().replace(/%$/, "").trim();
  if (t === "") return null;
  const n = Number(t);
  return Number.isFinite(n) && n >= 0 && n <= 100 ? n : "invalid";
}

async function reply(r: Promise<Answer<Ores>>): Promise<OreThreshold[]> {
  const v = await r;
  if (!v.ok) throw new Error(v.error ?? "the core didn't answer");
  if (!Array.isArray(v.ores)) throw new Error("the core's reply has no thresholds (is it an older version?)");
  return v.ores;
}

export function loadThresholds(): Promise<OreThreshold[]> {
  return reply(coreCall("prospector_thresholds"));
}

/** Set (a number), turn off (null) or reset to the default ("reset") one ore. */
export function setThreshold(ore: string, threshold: number | null | "reset"): Promise<OreThreshold[]> {
  const change = threshold === "reset" ? { ore, reset: true } : { ore, threshold };
  return reply(coreCall("prospector_set", JSON.stringify(change)));
}
