/**
 * EDDN (core `eddn_log`, `set_eddn`): the sharing switch, queue depth, last send and last error,
 * the gateway's answers and, apart, the messages that were never queued. The browser preview
 * shows a synthetic sample, or the fake core's with `?core=<scenario>`.
 */
import type { EddnHead } from "./api/types/EddnHead";
import type { LogEntry } from "./api/types/LogEntry";
import type { Outbox } from "./api/types/Outbox";
import { coreCall } from "./core";

/** One send, give-up, discard or skip (`at` in Unix seconds). */
export type EddnLogEntry = LogEntry;
/** The switch and the outbox: queue depth, last send and last error, the gateway's answers
 * (newest first) and the messages never queued. */
export type EddnLog = Outbox<EddnHead>;

/** The sharing log; rejects (with the core's reason) when the core did not answer. */
export async function eddnLog(): Promise<EddnLog> {
  const r = await coreCall("eddn_log");
  if (!r.ok) throw new Error(r.error ?? "the core did not answer");
  return r;
}

export async function setEddn(enabled: boolean) {
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
