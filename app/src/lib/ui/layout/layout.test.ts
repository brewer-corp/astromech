import { describe, expect, it } from "vitest";
import { at100, GUTTER, PANEL_TRACK, stacks } from "./layout";

describe("stacks", () => {
  it("moves the panel below content that would be narrower than its minimum beside the panel's whole track", () => {
    expect(stacks(952 + GUTTER + PANEL_TRACK - 1, 952)).toBe(true);
    expect(stacks(952 + GUTTER + PANEL_TRACK, 952)).toBe(false);
  });
});

describe("at100", () => {
  // The layouts' numbers are px at 100 percent Text size; a measured width is compared at that size.
  it("stacks a 1600 px column at 145 percent that holds the panel beside it at 100", () => {
    expect(stacks(at100(1600, 100), 952)).toBe(false);
    expect(stacks(at100(1600, 145), 952)).toBe(true);
  });
});
