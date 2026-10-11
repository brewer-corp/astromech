/** How bindings read in the app: "L · Joy 4", "Left shift + X", "R · POV1 up". */
import type { Slot } from "./binds";
import { mapFor } from "./devices";

const WORDS: Record<string, string> = {
  LeftShift: "Left shift", RightShift: "Right shift", LeftControl: "Left Ctrl", RightControl: "Right Ctrl",
  LeftAlt: "Left Alt", RightAlt: "Right Alt", UpArrow: "Up arrow", DownArrow: "Down arrow", LeftArrow: "Left arrow",
  RightArrow: "Right arrow", PageUp: "Page up", PageDown: "Page down", Backspace: "Backspace", SemiColon: ";",
  Comma: ",", Period: ".", Slash: "/", Minus: "-", Equals: "=", Space: "Space", Enter: "Enter", Tab: "Tab",
  Grave: "`", Apostrophe: "'", LeftBracket: "[", RightBracket: "]", BackSlash: "\\", Insert: "Insert", Delete: "Delete",
};

export function keyName(key: string): string {
  if (key.startsWith("Key_")) {
    const k = key.slice(4);
    if (WORDS[k]) return WORDS[k] ?? k;
    if (k.startsWith("Numpad_")) return `Numpad ${k.slice(7)}`;
    return k.replace(/([a-z])([A-Z])/g, "$1 $2");
  }
  if (key.startsWith("Mouse_")) return `Mouse ${key.slice(6)}`;
  if (key.startsWith("Pad_")) return `Pad ${key.slice(4).replace(/([a-z])([A-Z])/g, "$1 $2")}`;
  const pov = /^Joy_POV(\d)(Up|Down|Left|Right)$/.exec(key);
  if (pov) return `POV${pov[1]} ${pov[2]?.toLowerCase()}`;
  const ax = /^Joy_(\w+)Axis$/.exec(key);
  if (ax) return `${ax[1]} axis`;
  const j = /^Joy_(\d+)$/.exec(key);
  if (j) return `Joy ${j[1]}`;
  return key.replace(/_/g, " ");
}

export function deviceShort(device: string): string {
  if (device === "Keyboard" || device === "Mouse" || device === "GamePad") return "";
  const m = mapFor(device);
  return m ? m.side : device;
}

export function control(device: string, key: string): string {
  const d = deviceShort(device);
  return d ? `${d} · ${keyName(key)}` : keyName(key);
}

export function slotText(s: Slot | undefined): string {
  if (!s || !s.device || !s.key) return "";
  const mods = s.modifiers.map((m) => control(m.device, m.key));
  return [...mods, control(s.device, s.key)].join(" + ");
}

export function bound(s: Slot | undefined): boolean {
  return !!s && !!s.device && !!s.key;
}

/** A short form for sheet boxes: keyboard keys only, e.g. "L", "Shift+X". */
export function shortKey(s: Slot | undefined): string {
  if (!bound(s) || !s) return "";
  return slotText(s).replace("Left shift", "Shift").replace("Left Ctrl", "Ctrl").replace(/ \+ /g, "+");
}
