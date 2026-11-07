/**
 * Keybinds data for the app window (M1): types matching astromech-keybinds (profile.rs, api.rs),
 * the calls to the shell, and the shell's presses. The browser preview serves the example
 * fixture and keeps edits in memory (app/src/preview/binds.ts).
 */
import { call, subscribe } from "./transport";

export interface Modifier { device: string; key: string }
export interface Slot { slot: string; device: string; key: string; modifiers: Modifier[] }
export interface Issue { code: string; level: "error" | "warning" | "info"; text: string }
export interface Row { id: string; name: string; group: string; context: string; analogue: boolean; slots: Slot[]; issues: Issue[]; in_file: boolean }
export interface Group { name: string; context: string; rows: Row[] }
export interface View { name: string; path: string; editable: boolean; groups: Group[]; devices: string[]; problems: string[]; issue_count: number }
export interface ProfileFile { path: string; file_name: string; version: [number, number] | null; modified: string }
export interface Preset { name: string; source: string; editable: boolean; path: string; files: ProfileFile[]; active_for: string[]; devices: string[]; problems: string[] }
export interface Backup { path: string; file_name: string; original: string; created: string }
export interface Controller { elite_id: string; product: string; buttons: number; axes: string[]; hats: number }
export interface State {
  bindings_dir: string;
  bindings_found: boolean;
  presets: Preset[];
  active: string[];
  backups: Backup[];
  inventory: { controllers: Controller[] };
  overrides: Record<string, Record<string, string>>;
  notes: string[];
}
export interface Press { device: string; key: string; kind: string; down: boolean | null; device_name: string; t_ms: number }
/**
 * The window's own clicks reach Raw Input (INPUTSINK) as Mouse_1-3 presses, so they aren't game
 * controls. Side buttons (Mouse_4, Mouse_5) don't click and are real presses.
 */
export const isWindowClick = (p: Press) => p.device === "Mouse" && /^Mouse_[123]$/.test(p.key);
export const CONTEXTS = ["General", "Ship", "SRV", "On foot"] as const;

export const api = {
  state: () => call<State>("binds_state"),
  refreshDevices: () => call<State>("binds_refresh_devices"),
  /** Discover folders and devices again (after onboarding changed the Bindings folder). */
  reset: () => call<State>("binds_reset"),
  view: (path: string) => call<View>("binds_view", { path }),
  setSlot: (edit: { path: string; action: string; slot: string; analogue: boolean; binding: Slot | null }) =>
    call<{ path: string; backup: string }>("binds_set_slot", { edit }),
  copy: (src: string, name: string) => call<string>("binds_copy", { src, name }),
  setActive: (name: string, contexts: number[]) => call<unknown>("binds_set_active", { name, contexts }),
  restore: (backup: string) => call<string>("binds_restore", { backup }),
  setOverride: (device: string, control: string, joy: string | null) =>
    call<State["overrides"]>("binds_set_override", { device, control, joy }),
};

/** Presses from the shell's input listener (the browser preview: the keyboard, and `?press=`). */
export function onPress(cb: (p: Press) => void): () => void {
  return subscribe<Press>("input", cb);
}
