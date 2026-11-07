// The current mining run, from the core's status reply (`mining`): tons refined by ore and tons
// per hour. The browser preview's sample is the same JSON the core's mining_status test builds
// from the mining fixture.
import type { Mining as MiningRun } from "./api/types/Mining";
import { coreStatus } from "./core";

export type { Mining as MiningRun } from "./api/types/Mining";

/** The run; null while no session has one. Rejects when the core did not answer. */
export async function loadMining(): Promise<MiningRun | null> {
  const status = await coreStatus();
  if (!status) throw new Error("the core did not answer");
  return status.mining ?? null;
}

/** `low_temperature_diamond` as "Low Temperature Diamond". */
export function oreName(symbol: string): string {
  return symbol.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
}
