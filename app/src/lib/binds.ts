/**
 * Keybinds data for the app window (M1): astromech-keybinds' types (profile.rs, api.rs, generated
 * into api/types/), its commands, and the shell's presses. The browser preview serves the example
 * fixture and keeps edits in memory (app/src/preview/binds.ts).
 */
import type { SlotEdit } from "./api/types/SlotEdit";
import { call, subscribe } from "./transport";

export type { Modifier } from "./api/types/Modifier";
export type { Slot } from "./api/types/Slot";
export type { Issue } from "./api/types/Issue";
export type { Row } from "./api/types/Row";
export type { Group } from "./api/types/Group";
export type { BindsView as View } from "./api/types/BindsView";
export type { ProfileFile } from "./api/types/ProfileFile";
export type { ProfileSummary as Preset } from "./api/types/ProfileSummary";
export type { Backup } from "./api/types/Backup";
export type { Controller } from "./api/types/Controller";
export type { State } from "./api/types/State";
export interface Press { device: string; key: string; kind: string; down: boolean | null; device_name: string; t_ms: number }
/**
 * The window's own clicks reach Raw Input (INPUTSINK) as Mouse_1-3 presses, so they aren't game
 * controls. Side buttons (Mouse_4, Mouse_5) don't click and are real presses.
 */
export const isWindowClick = (p: Press) => p.device === "Mouse" && /^Mouse_[123]$/.test(p.key);
export const CONTEXTS = ["General", "Ship", "SRV", "On foot"] as const;

export const api = {
  state: () => call("binds_state"),
  refreshDevices: () => call("binds_refresh_devices"),
  /** Discover folders and devices again (after onboarding changed the Bindings folder). */
  reset: () => call("binds_reset"),
  view: (path: string) => call("binds_view", { path }),
  setSlot: (edit: SlotEdit) => call("binds_set_slot", { edit }),
  copy: (src: string, name: string) => call("binds_copy", { src, name }),
  setActive: (name: string, contexts: number[]) => call("binds_set_active", { name, contexts }),
  restore: (backup: string) => call("binds_restore", { backup }),
  setOverride: (device: string, control: string, joy: string | null) => call("binds_set_override", { device, control, joy }),
};

/** Presses from the shell's input listener (the browser preview: the keyboard, and `?press=`). */
export function onPress(cb: (p: Press) => void): () => void {
  return subscribe<Press>("input", cb);
}
