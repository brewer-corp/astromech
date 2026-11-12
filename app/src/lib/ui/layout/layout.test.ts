import { describe, expect, it } from "vitest";
import { GUTTER, PANEL_TRACK, stacks } from "./layout";

describe("stacks", () => {
  it("moves the panel below content that would be narrower than its minimum beside the panel's whole track", () => {
    expect(stacks(952 + GUTTER + PANEL_TRACK - 1, 952)).toBe(true);
    expect(stacks(952 + GUTTER + PANEL_TRACK, 952)).toBe(false);
  });
});
