/**
 * The window's scale (DESIGN.md Typography, The Height Scale Rule): one root `zoom`, so every px
 * token keeps its value and the whole window grows with its height, as the game's UI does. The
 * width term keeps the layout at least 1100 px wide (DESIGN.md Frame), the floor of 1 means text
 * never shrinks below its 1000 px size, and the commander's Text size (Settings → General, the
 * core's `[window] text_size`) multiplies the result.
 *
 * CSS zoom scales `vh` and `vw` too, so a size taken from the window divides by `--zoom`
 * (`--win-h`, `--win-w` in tokens.css), and a page's narrow-window rules are container queries on
 * the frame (`@container win`), which measure the layout, not the screen.
 */
import type { TextSize } from "./api/types/TextSize";
import { coreCall } from "./core";

export type { TextSize };

export const TEXT_SIZES: Record<TextSize, number> = { smaller: 0.85, normal: 1, larger: 1.15 };

/** The zoom for a window `width` x `height` CSS px across, at a text size. */
export function windowZoom(width: number, height: number, size: TextSize = "normal"): number {
  const base = Math.min(1.8, Math.max(1, Math.min(height / 1000, width / 1100)));
  return Math.round(base * TEXT_SIZES[size] * 1000) / 1000;
}

const state = $state({ size: "normal" as TextSize, zoom: 1 });

/** The zoom the window is drawn at now. */
export const scale = {
  get zoom() {
    return state.zoom;
  },
  get size() {
    return state.size;
  },
};

function apply() {
  state.zoom = windowZoom(window.innerWidth, window.innerHeight, state.size);
  const root = document.documentElement;
  root.style.zoom = String(state.zoom);
  root.style.setProperty("--zoom", String(state.zoom));
}

/** Draw the window at its scale from now on: on every resize, with the saved text size. */
export function startScale(): () => void {
  apply();
  window.addEventListener("resize", apply);
  void coreCall("window_settings").then((r) => {
    if (r.ok) {
      state.size = r.text_size;
      apply();
    }
  });
  return () => window.removeEventListener("resize", apply);
}

/** Save a text size and draw the window at it at once; the error is the core's. */
export async function setTextSize(size: TextSize): Promise<string | null> {
  const r = await coreCall("set_window", { text_size: size });
  if (!r.ok) return String(r.error);
  state.size = size;
  apply();
  return null;
}
