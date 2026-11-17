import { describe, expect, it } from "vitest";
import { firstStep, migratedStep } from "./scale.svelte";

describe("Text size (DESIGN.md The Text Size Rule)", () => {
  it.each([
    [1080, 100],
    [1199, 100],
    [1200, 115],
    [1300, 130],
    [1440, 145],
    [1600, 160],
    [1800, 180],
    [2160, 180],
  ])("a first run on a %i px display picks %i", (height, step) => {
    expect(firstStep(height)).toBe(step);
  });

  // The step nearest what the old word drew at with the window filling the display: its height
  // zoom, capped at 1 to 1.8, times 0.85, 1 or 1.15 (astromech-k6pl).
  it.each([
    ["normal", 2560, 1440, 145], // 1.44: the approved captures' size, not 100
    ["smaller", 2560, 1440, 115], // 1.224
    ["larger", 2560, 1440, 160], // 1.656
    ["normal", 1920, 1080, 115], // 1.08 is nearer 115 than 100
    ["smaller", 1920, 1080, 100], // 0.918
    ["larger", 1920, 1080, 130], // 1.242
    ["normal", 3840, 2160, 180], // capped at 1.8
    ["larger", 3840, 2160, 180], // 2.07: the largest step
  ] as const)("an old %s on a %i x %i display becomes %i", (size, w, h, step) => {
    expect(migratedStep(size, w, h)).toBe(step);
  });
});
