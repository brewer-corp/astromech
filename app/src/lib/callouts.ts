/**
 * Callout switches and cooldowns (Settings → Callouts). The core writes them to
 * `[callouts.<id>]` in astromech.toml; see `crates/astromech-core/src/callouts.rs` (`callouts`)
 * and `setup.rs` (`set_callout`). A change applies from the next session.
 */
import { coreCall } from "./core";

export interface CalloutSetting {
  id: string;
  title: string;
  /** In force: the configured value, or the callout's own. */
  enabled: boolean;
  cooldown_s: number;
  default_enabled: boolean;
  default_cooldown_s: number;
}

/** What the commander typed in a cooldown box: seconds, or invalid. */
export function parseCooldown(input: string): number | "invalid" {
  const t = input.trim().replace(/s$/i, "").trim();
  if (t === "") return "invalid";
  const n = Number(t);
  return Number.isFinite(n) && n >= 0 && n <= 86400 ? n : "invalid";
}

async function reply(r: Promise<Record<string, unknown>>): Promise<Record<string, unknown>> {
  const v = await r;
  if (!v.ok) throw new Error(String(v.error ?? "the core didn't answer"));
  return v;
}

export async function loadCallouts(): Promise<CalloutSetting[]> {
  const v = await reply(coreCall("callouts"));
  if (!Array.isArray(v.callouts)) throw new Error("the core's reply has no callouts (is it an older version?)");
  return v.callouts as CalloutSetting[];
}

/** Turn a callout on or off, or set its cooldown (null: the callout's own). */
export async function changeCallout(id: string, change: { enabled: boolean } | { cooldown_s: number | null }): Promise<void> {
  await reply(coreCall("set_callout", { id, ...change }));
}
