/**
 * Keybinds data for the app window (M1): types matching astromech-keybinds (profile.rs, api.rs),
 * the calls to the shell, and a browser fallback that serves the example fixture and keeps edits
 * in memory (vite dev, screenshots).
 */
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { inTauri } from "./core";
import fixture from "./fixtures/sample.json";

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
export const CONTEXTS = ["General", "Ship", "SRV", "On foot"] as const;

// ---- Browser fallback: the fixture, with edits applied in memory.
const mem = structuredClone(fixture) as unknown as { state: State; view: View };
const memViews: Record<string, View> = { [mem.view.path]: mem.view };

function memSetSlot(e: { path: string; action: string; slot: string; binding: Slot | null }) {
  const v = memViews[e.path];
  if (!v) throw new Error("no such preset");
  for (const g of v.groups)
    for (const r of g.rows)
      if (r.id === e.action) {
        const s = r.slots.find((x) => x.slot === e.slot);
        if (s) Object.assign(s, e.binding ?? { device: "", key: "", modifiers: [] }, { slot: e.slot });
        r.in_file = true;
        r.issues = r.issues.filter((i) => !(i.code === "no_keyboard" && r.slots.some((x) => x.device === "Keyboard")));
      }
  v.issue_count = v.groups.flatMap((g) => g.rows).reduce((n, r) => n + r.issues.filter((i) => i.level !== "info").length, 0);
  const b = { path: `${e.path}.bak`, file_name: `${e.path.split("\\").pop()}.bak`, original: e.path.split("\\").pop() ?? "", created: "just now" };
  mem.state.backups.unshift(b);
  return { path: e.path, backup: b.path };
}

async function call<T>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  if (inTauri) return await invoke<T>(cmd, args);
  switch (cmd) {
    case "binds_state":
    case "binds_refresh_devices":
      return structuredClone(mem.state) as T;
    case "binds_view": {
      const p = String(args.path);
      const v = memViews[p] ?? (memViews[p] = { ...structuredClone(mem.view), path: p, name: mem.state.presets.find((x) => x.path === p)?.name ?? "" });
      return structuredClone(v) as T;
    }
    case "binds_set_slot":
      return memSetSlot(args.edit as never) as T;
    case "binds_copy": {
      const name = String(args.name);
      const path = `${mem.state.bindings_dir}\\${name}.4.2.binds`;
      mem.state.presets.push({ name, source: "yours", editable: true, path, files: [], active_for: [], devices: [], problems: [] });
      memViews[path] = { ...structuredClone(memViews[String(args.src)] ?? mem.view), path, name, editable: true };
      return path as T;
    }
    case "binds_set_active": {
      const ctx = args.contexts as number[];
      for (const c of ctx) mem.state.active[c] = String(args.name);
      for (const p of mem.state.presets) p.active_for = CONTEXTS.filter((_, i) => mem.state.active[i] === p.name);
      return {} as T;
    }
    case "binds_restore":
      return "" as T;
    case "binds_set_override": {
      const { device, control, joy } = args as { device: string; control: string; joy: string | null };
      const o = (mem.state.overrides[device] ??= {});
      if (joy) o[control] = joy;
      else delete o[control];
      return structuredClone(mem.state.overrides) as T;
    }
  }
  throw new Error(`unknown command ${cmd}`);
}

export const api = {
  state: () => call<State>("binds_state"),
  refreshDevices: () => call<State>("binds_refresh_devices"),
  view: (path: string) => call<View>("binds_view", { path }),
  setSlot: (edit: { path: string; action: string; slot: string; analogue: boolean; binding: Slot | null }) =>
    call<{ path: string; backup: string }>("binds_set_slot", { edit }),
  copy: (src: string, name: string) => call<string>("binds_copy", { src, name }),
  setActive: (name: string, contexts: number[]) => call<unknown>("binds_set_active", { name, contexts }),
  restore: (backup: string) => call<string>("binds_restore", { backup }),
  setOverride: (device: string, control: string, joy: string | null) =>
    call<State["overrides"]>("binds_set_override", { device, control, joy }),
};

// ---- Presses: from the shell's listener, or (browser) the keyboard plus `?press=DEV:KEY`.
const KEYS: Record<string, string> = {
  " ": "Key_Space", Enter: "Key_Enter", Tab: "Key_Tab", Backspace: "Key_Backspace", Escape: "Key_Escape",
  ArrowUp: "Key_UpArrow", ArrowDown: "Key_DownArrow", ArrowLeft: "Key_LeftArrow", ArrowRight: "Key_RightArrow",
  Insert: "Key_Insert", Delete: "Key_Delete", Home: "Key_Home", End: "Key_End", PageUp: "Key_PageUp", PageDown: "Key_PageDown",
  ",": "Key_Comma", ".": "Key_Period", "/": "Key_Slash", ";": "Key_SemiColon", "-": "Key_Minus", "=": "Key_Equals",
};
function browserKey(e: KeyboardEvent): string | null {
  if (e.code.startsWith("Shift")) return e.code === "ShiftLeft" ? "Key_LeftShift" : "Key_RightShift";
  if (e.code.startsWith("Control")) return e.code === "ControlLeft" ? "Key_LeftControl" : "Key_RightControl";
  if (e.code.startsWith("Alt")) return e.code === "AltLeft" ? "Key_LeftAlt" : "Key_RightAlt";
  if (KEYS[e.key]) return KEYS[e.key] ?? null;
  if (/^F\d+$/.test(e.key)) return `Key_${e.key}`;
  if (e.key.length === 1 && /[a-z0-9]/i.test(e.key)) return `Key_${e.key.toUpperCase()}`;
  return null;
}

export function onPress(cb: (p: Press) => void): () => void {
  if (inTauri) {
    let un: (() => void) | undefined;
    let dead = false;
    void listen<Press>("input", (e) => cb(e.payload)).then((u) => (dead ? u() : (un = u)));
    return () => {
      dead = true;
      un?.();
    };
  }
  const kd = (e: KeyboardEvent) => {
    if (e.target instanceof HTMLInputElement) return;
    const key = browserKey(e);
    if (key) cb({ device: "Keyboard", key, kind: "key", down: true, device_name: "Keyboard", t_ms: performance.now() });
  };
  window.addEventListener("keydown", kd);
  return () => window.removeEventListener("keydown", kd);
}

/** `?press=231D3201:Joy_4` (browser preview / screenshots): one simulated press on load. */
export function simulatedPress(): Press | null {
  const m = new URLSearchParams(location.search).get("press");
  if (!m) return null;
  const [device = "", key = ""] = m.split(":");
  return { device, key, kind: device === "Keyboard" ? "key" : "button", down: true, device_name: device, t_ms: 0 };
}
