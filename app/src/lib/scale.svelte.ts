/**
 * Text size (DESIGN.md Typography, The Text Size Rule): the commander's choice of one root font
 * size, as on any web page. It sets the `html` font size and nothing else does; every size that
 * should grow with the text is in rem, so a bigger window shows more and a resize changes nothing.
 *
 * The core's config (`[window] text_size`) holds the step. With none saved, the window picks one
 * from the display's height and saves it once; a word saved before revision 5 (`smaller`,
 * `normal`, `larger`) becomes the step nearest what it drew at on this display, and is saved.
 */
import type { LegacyTextSize } from "./api/types/LegacyTextSize";
import type { TextSize } from "./api/types/TextSize";
import { coreCall } from "./core";

/** The steps, in percent of a 16 px root (the core's `TEXT_SIZE_STEPS`). */
export const STEPS = [100, 115, 130, 145, 160, 180] as const;

/** The step a first run picks for a display `height` CSS px tall. */
export function firstStep(height: number): number {
  if (height < 1200) return 100;
  if (height < 1300) return 115;
  if (height < 1400) return 130;
  if (height < 1600) return 145;
  if (height < 1800) return 160;
  return 180;
}

/** The step nearest `percent`; a tie goes to the smaller. */
export function nearestStep(percent: number): number {
  return STEPS.reduce((best, s) => (Math.abs(s - percent) < Math.abs(best - percent) ? s : best), STEPS[0]);
}

const LEGACY: Record<LegacyTextSize, number> = { smaller: 0.85, normal: 1, larger: 1.15 };

/** The step nearest what an old text size drew at on a `width` x `height` display: the old zoom
 * from the window's size (astromech-k6pl), with the window filling the display, times the word's
 * factor. */
export function migratedStep(size: LegacyTextSize, width: number, height: number): number {
  const zoom = Math.min(1.8, Math.max(1, Math.min(height / 1000, width / 1100)));
  return nearestStep(zoom * LEGACY[size] * 100);
}

const state = $state({ step: 100 });

/** The step the window is drawn at now. */
export const scale = {
  get step() {
    return state.step;
  },
};

function apply(step: number) {
  state.step = step;
  document.documentElement.style.fontSize = `${step}%`;
}

/** The saved step, or the one to save now: picked from the display with none saved, or the
 * nearest to an old word. `screen` is the display in CSS px, as the old zoom measured. */
function settle(saved: TextSize | null): { step: number; save: boolean } {
  if (saved === null) return { step: firstStep(screen.height), save: true };
  if (typeof saved === "string") return { step: migratedStep(saved, screen.width, screen.height), save: true };
  return { step: nearestStep(saved), save: false };
}

/** Draw the window at the saved text size, saving a picked or migrated step first. */
export async function startScale(): Promise<void> {
  const r = await coreCall("window_settings");
  if (!r.ok) return;
  const { step, save } = settle(r.text_size);
  apply(step);
  if (save) await coreCall("set_window", { text_size: step });
}

/** Save a step and draw the window at it at once; the error is the core's. */
export async function setTextSize(step: number): Promise<string | null> {
  const r = await coreCall("set_window", { text_size: step });
  if (!r.ok) return String(r.error);
  apply(step);
  return null;
}
