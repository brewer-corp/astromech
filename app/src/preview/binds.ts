/**
 * The keybinds commands and presses as the browser preview answers them: the example fixture
 * (fixtures/sample.json, astromech-keybinds' `fixture` example) with edits kept in memory, and
 * presses from the keyboard plus one simulated `?press=DEV:KEY` on load.
 */
import { CONTEXTS, type Press, type Slot, type State, type View } from "../lib/binds";
import { fakeScenario, previewGate } from "./fakecore";
import fixture from "./fixtures/sample.json";
import { previewQuery } from "./query";

let mem: { state: State; view: View } | null = null;
let memViews: Record<string, View> = {};

function m(): { state: State; view: View } {
  if (!mem) {
    mem = structuredClone(fixture) as unknown as { state: State; view: View };
    memViews = { [mem.view.path]: mem.view };
  }
  return mem;
}

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
  m().state.backups.unshift(b);
  return { path: e.path, backup: b.path };
}

/** `?core=empty`: a PC where Elite never ran and no sticks are plugged in. */
function emptyState(): State {
  return {
    ...structuredClone(m().state),
    bindings_found: false,
    presets: [],
    active: [],
    backups: [],
    inventory: { controllers: [] },
    overrides: {},
    notes: ["Game install not found: built-in presets aren't listed."],
  };
}

const state = () => (fakeScenario() === "empty" ? emptyState() : structuredClone(m().state));

/** Each keybinds command, waiting or failing as `?slow=` and `?fail=` ask. */
function gated(cmd: string, f: (args: Record<string, unknown>) => unknown) {
  return async (args: Record<string, unknown>) => {
    await previewGate(cmd);
    return f(args);
  };
}

export const bindsCommands: Record<string, (args: Record<string, unknown>) => unknown> = {
  binds_state: gated("binds_state", state),
  binds_refresh_devices: gated("binds_refresh_devices", state),
  binds_reset: gated("binds_reset", state),
  binds_view: gated("binds_view", (args) => {
    const { state: st, view } = m();
    const p = String(args.path);
    const v = memViews[p] ?? (memViews[p] = { ...structuredClone(view), path: p, name: st.presets.find((x) => x.path === p)?.name ?? "" });
    return structuredClone(v);
  }),
  binds_set_slot: gated("binds_set_slot", (args) => memSetSlot(args.edit as never)),
  binds_copy: gated("binds_copy", (args) => {
    const { state: st, view } = m();
    const name = String(args.name);
    const path = `${st.bindings_dir}\\${name}.4.2.binds`;
    st.presets.push({ name, source: "yours", editable: true, path, files: [], active_for: [], devices: [], problems: [] });
    memViews[path] = { ...structuredClone(memViews[String(args.src)] ?? view), path, name, editable: true };
    return path;
  }),
  binds_set_active: gated("binds_set_active", (args) => {
    const st = m().state;
    for (const c of args.contexts as number[]) st.active[c] = String(args.name);
    for (const p of st.presets) p.active_for = CONTEXTS.filter((_, i) => st.active[i] === p.name);
    return {};
  }),
  binds_restore: gated("binds_restore", () => ""),
  binds_set_override: gated("binds_set_override", (args) => {
    const st = m().state;
    const { device, control, joy } = args as { device: string; control: string; joy: string | null };
    const o = (st.overrides[device] ??= {});
    if (joy) o[control] = joy;
    else delete o[control];
    return structuredClone(st.overrides);
  }),
};

// ---- Presses: the keyboard, and `?press=231D3201:Joy_4` once on load (screenshots).
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

export function onPreviewPress(cb: (p: Press) => void): () => void {
  const kd = (e: KeyboardEvent) => {
    if (e.target instanceof HTMLInputElement) return;
    const key = browserKey(e);
    if (key) cb({ device: "Keyboard", key, kind: "key", down: true, device_name: "Keyboard", t_ms: performance.now() });
  };
  window.addEventListener("keydown", kd);
  const press = previewQuery().get("press");
  let sim: ReturnType<typeof setTimeout> | undefined;
  if (press) {
    const [device = "", key = ""] = press.split(":");
    sim = setTimeout(() => cb({ device, key, kind: device === "Keyboard" ? "key" : "button", down: true, device_name: device, t_ms: 0 }), 300);
  }
  return () => {
    window.removeEventListener("keydown", kd);
    clearTimeout(sim);
  };
}
