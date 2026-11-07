/**
 * EDSM journal upload (core `edsm_status`, `set_edsm`, `edsm_flight_log`): the switch, the
 * commander name, whether an API key is saved in the OS keyring (never the key itself), the
 * queue and send log, and the commander's EDSM flight log. The browser preview shows a
 * synthetic sample, or the fake core's with `?core=<scenario>`.
 */
import type { EdsmHead } from "./api/types/EdsmHead";
import type { EdsmVisit } from "./api/types/EdsmVisit";
import type { Outbox } from "./api/types/Outbox";
import type { SetEdsmArgs } from "./api/types/SetEdsmArgs";
import { coreCall } from "./core";

export type { EdsmVisit as Visit } from "./api/types/EdsmVisit";
/** The switch, the commander name, whether a key is in the keyring, and the outbox. */
export type EdsmStatus = Outbox<EdsmHead>;

/** The EDSM status; rejects (with the core's reason) when the core did not answer. */
export async function edsmStatus(): Promise<EdsmStatus> {
  const r = await coreCall("edsm_status");
  if (!r.ok) throw new Error(r.error ?? "the core did not answer");
  return r;
}

/** Any of: the switch, the commander name, the API key ("" forgets it). */
export async function setEdsm(change: SetEdsmArgs) {
  return coreCall("set_edsm", change);
}

export async function flightLog(): Promise<{ visits: EdsmVisit[] } | { error: string }> {
  const r = await coreCall("edsm_flight_log");
  return r.ok ? { visits: r.visits } : { error: r.error };
}
