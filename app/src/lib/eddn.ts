/**
 * EDDN (core `eddn_log`, `set_eddn`): the sharing switch, queue depth, last send and last error,
 * the gateway's answers and, apart, the messages that were never queued. Outside Tauri (vite
 * dev) it shows a synthetic sample, or the fake core with `?core=<scenario>`.
 */
import { coreCall, inTauri, type Reply } from "./core";
import { fakeScenario } from "./fakecore";
import sample from "./fixtures/eddn-log.json";

export interface EddnLogEntry {
  /** Unix seconds. */
  at: number;
  schema_ref: string;
  outcome: "sent" | "rejected" | "retry" | "gave_up" | "discarded" | "dropped";
  detail: string;
}

export interface EddnLog {
  /** The user's switch (`[eddn] enabled`). */
  enabled: boolean;
  /** Messages go to the /test schemas (debug builds, or `[eddn] test`). */
  test: boolean;
  pending: number;
  last_sent: EddnLogEntry | null;
  /** The newest rejected, retried or given-up message (not one discarded here: switched off, outbox full). */
  last_error: EddnLogEntry | null;
  /** The gateway's answers (and give-ups and discards), newest first. */
  log: EddnLogEntry[];
  /** Messages never queued (beta client, location mismatch, ...), newest first. */
  skipped: EddnLogEntry[];
}

export const sampleLog = sample as EddnLog;

export async function eddnLog(): Promise<EddnLog | null> {
  if (!inTauri && !fakeScenario()) return sampleLog;
  const r = await coreCall("eddn_log");
  return r.ok ? (r as unknown as EddnLog) : null;
}

export async function setEddn(enabled: boolean): Promise<Reply> {
  return coreCall("set_eddn", { enabled });
}

/** "commodity/3", "journal/1 (test)". */
export function schemaName(ref: string): string {
  const m = /\/schemas\/(.+?)(\/test)?$/.exec(ref);
  return m ? `${m[1]}${m[2] ? " (test)" : ""}` : ref;
}

/** "Oct 16, 12:01:00 PM": the log keeps entries across days and restarts. */
export function when(at: number): string {
  return new Date(at * 1000).toLocaleString([], { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit", second: "2-digit" });
}
