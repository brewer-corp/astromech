// The current mining run, from the core's status reply (`mining`): tons refined by ore and tons
// per hour. The sample is the same JSON the core's mining_status test builds from the mining
// fixture, used outside the app.
import { coreStatus, inTauri } from "./core";
import sample from "./fixtures/mining.json";

export interface MiningRun {
  /** Journal time of the first limpet, prospect or refine since the last dock; null before one. */
  started: string | null;
  /** Tons refined by commodity symbol (`platinum`). */
  refined: Record<string, number>;
  total: number;
  /** Null until a minute into the run. */
  per_hour: number | null;
}

/** The run, or null while the core isn't running (outside the app: the sample). */
export async function loadMining(): Promise<MiningRun | null> {
  if (!inTauri) return sample as MiningRun;
  return (await coreStatus())?.mining ?? null;
}

/** `low_temperature_diamond` as "Low Temperature Diamond". */
export function oreName(symbol: string): string {
  return symbol.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
}
