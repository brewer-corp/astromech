/**
 * EDSM journal upload (core `edsm_status`, `set_edsm`, `edsm_flight_log`): the switch, the
 * commander name, whether an API key is saved in the OS keyring (never the key itself), the
 * queue and send log, and the commander's EDSM flight log. Outside Tauri (vite dev) it shows a
 * synthetic sample, or the fake core with `?core=<scenario>`.
 */
import { coreCall, inTauri, type Reply } from "./core";
import { fakeScenario } from "./fakecore";
import type { EddnLogEntry } from "./eddn";

export interface EdsmStatus {
  /** `[edsm] enabled`. */
  enabled: boolean;
  /** As registered on EDSM. */
  commander: string;
  /** An API key is saved in the OS keyring. */
  has_key: boolean;
  /** The keyring couldn't be read. */
  key_error: string | null;
  /** On, with a commander name and a key: live journal events go to EDSM. */
  uploading: boolean;
  pending: number;
  last_sent: EddnLogEntry | null;
  last_error: EddnLogEntry | null;
  log: EddnLogEntry[];
  skipped: EddnLogEntry[];
}

export interface Visit {
  system: string;
  /** UTC, "YYYY-MM-DD HH:MM:SS". */
  date: string;
  first_discover: boolean;
  ship_id: number | null;
}

export const sampleStatus: EdsmStatus = {
  enabled: true,
  commander: "Jameson",
  has_key: true,
  key_error: null,
  uploading: true,
  pending: 0,
  last_sent: { at: 1792166460, schema_ref: "journal", outcome: "sent", detail: "12 events" },
  last_error: null,
  log: [{ at: 1792166460, schema_ref: "journal", outcome: "sent", detail: "12 events" }],
  skipped: [],
};

export async function edsmStatus(): Promise<EdsmStatus | null> {
  if (!inTauri && !fakeScenario()) return sampleStatus;
  const r = await coreCall("edsm_status");
  return r.ok ? (r as unknown as EdsmStatus) : null;
}

/** Any of: the switch, the commander name, the API key ("" forgets it). */
export async function setEdsm(change: { enabled?: boolean; commander?: string; api_key?: string }): Promise<Reply> {
  return coreCall("set_edsm", change);
}

export async function flightLog(): Promise<{ visits: Visit[] } | { error: string }> {
  const r = await coreCall("edsm_flight_log");
  return r.ok ? { visits: (r.visits as Visit[] | undefined) ?? [] } : { error: String(r.error) };
}
