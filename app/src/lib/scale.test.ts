import { describe, expect, it } from "vitest";
import { windowZoom } from "./scale.svelte";

describe("the window's scale (DESIGN.md The Height Scale Rule)", () => {
  it.each([
    [1100, 700, "normal", 1], // the smallest window: as drawn before the rule
    [1920, 1080, "normal", 1.08],
    [2560, 1440, "normal", 1.44],
    [3840, 2160, "normal", 1.8], // capped
    [1100, 1400, "normal", 1], // tall and narrow: the width keeps the layout 1100 px wide
    [1650, 1600, "normal", 1.5], // the width term wins over the height
    [2560, 1440, "smaller", 1.224],
    [1100, 700, "larger", 1.15],
  ] as const)("%i x %i at %s text is %f", (w, h, size, zoom) => {
    expect(windowZoom(w, h, size)).toBe(zoom);
  });
});
