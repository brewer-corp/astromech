/**
 * Callout switches and cooldowns (Settings → Callouts). The core writes them to
 * `[callouts.<id>]` in astromech.toml; see `crates/astromech-core/src/callouts.rs` (`callouts`)
 * and `setup.rs` (`set_callout`). A change applies from the next session.
 */
import type { CalloutSetting } from "./api/types/CalloutSetting";
import { coreCall } from "./core";

export type { CalloutSetting } from "./api/types/CalloutSetting";

/** What the commander typed in a cooldown box: seconds, or invalid. */
export function parseCooldown(input: string): number | "invalid" {
  const t = input.trim().replace(/s$/i, "").trim();
  if (t === "") return "invalid";
  const n = Number(t);
  return Number.isFinite(n) && n >= 0 && n <= 86400 ? n : "invalid";
}

export async function loadCallouts(): Promise<CalloutSetting[]> {
  const r = await coreCall("callouts");
  if (!r.ok) throw new Error(r.error ?? "the core didn't answer");
  if (!Array.isArray(r.callouts)) throw new Error("the core's reply has no callouts (is it an older version?)");
  return r.callouts;
}

/** Turn a callout on or off, or set its cooldown (null: the callout's own). */
export async function changeCallout(id: string, change: { enabled: boolean } | { cooldown_s: number | null }): Promise<void> {
  const r = await coreCall("set_callout", { id, ...change });
  if (!r.ok) throw new Error(r.error ?? "the core didn't answer");
}
