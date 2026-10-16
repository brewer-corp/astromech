/**
 * The EDDN send log (core `eddn_log`): messages waiting in the outbox and the latest outcomes.
 * Outside Tauri (vite dev) it shows a synthetic sample.
 */
import { coreCall, inTauri } from "./core";
import sample from "./fixtures/eddn-log.json";

export interface EddnLogEntry {
  /** Unix seconds. */
  at: number;
  schema_ref: string;
  outcome: "sent" | "rejected" | "retry" | "gave_up" | "dropped";
  detail: string;
}

export interface EddnLog {
  pending: number;
  log: EddnLogEntry[];
}

export async function eddnLog(): Promise<EddnLog | null> {
  if (!inTauri) return sample as EddnLog;
  const r = await coreCall("eddn_log");
  return r.ok ? (r as unknown as EddnLog) : null;
}

/** "commodity/3", "journal/1 (test)". */
export function schemaName(ref: string): string {
  const m = /\/schemas\/(.+?)(\/test)?$/.exec(ref);
  return m ? `${m[1]}${m[2] ? " (test)" : ""}` : ref;
}
