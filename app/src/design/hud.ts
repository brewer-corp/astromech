// Re-tint the palette with the player's in-game HUD colour matrix
// (%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Graphics\GraphicsConfigurationOverride.xml,
// <GUIColour><Default><MatrixRed|Green|Blue>). Each output channel is a weighted mix of the
// default colour's r, g, b: out.r = MatrixRed·(r,g,b), etc. Identity = default orange.

export type Matrix = [[number, number, number], [number, number, number], [number, number, number]];
export const IDENTITY: Matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

export function parseMatrix(xml: string): Matrix | null {
  const row = (tag: string) => {
    const m = xml.match(new RegExp(`<${tag}>([^<]*)</${tag}>`));
    const v = m?.[1]?.split(",").map((s) => Number(s.trim()));
    return v && v.length === 3 && v.every(Number.isFinite) ? (v as [number, number, number]) : null;
  };
  const r = row("MatrixRed"), g = row("MatrixGreen"), b = row("MatrixBlue");
  return r && g && b ? [r, g, b] : null;
}

export function tint(hex: string, m: Matrix): string {
  const n = parseInt(hex.slice(1), 16);
  const c: [number, number, number] = [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  const out = m.map((row) => Math.max(0, Math.min(255, Math.round(row[0] * c[0] + row[1] * c[1] + row[2] * c[2]))));
  return "#" + out.map((v) => v.toString(16).padStart(2, "0")).join("");
}

/** HUD-tinted tokens. Semantic red/amber/blue stay fixed so alerts keep their meaning. */
const TINTED = ["--am-orange", "--am-orange-bright", "--am-orange-dim", "--am-text", "--am-text-strong", "--am-muted"];

export function applyMatrix(m: Matrix, root: HTMLElement = document.documentElement): void {
  const cs = getComputedStyle(root);
  for (const v of TINTED) {
    const base = root.dataset[`base${v}`] ?? cs.getPropertyValue(v).trim();
    root.dataset[`base${v}`] = base;
    root.style.setProperty(v, tint(base, m));
  }
}
