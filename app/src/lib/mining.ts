// The current mining run, from the core's status reply (`mining`): tons refined by ore and tons
// per hour. The browser preview's sample is the same JSON the core's mining_status test builds
// from the mining fixture.
import { coreStatus } from "./core";

export interface MiningRun {
  /** Journal time of the first limpet, prospect or refine since the last dock; null before one. */
  started: string | null;
  /** Tons refined by commodity symbol (`platinum`). */
  refined: Record<string, number>;
  total: number;
  /** Null until a minute into the run. */
  per_hour: number | null;
}

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
