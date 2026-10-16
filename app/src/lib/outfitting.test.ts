import { describe, expect, it } from "vitest";
import { choices, settle, MODULES } from "./outfitting";

describe("outfitting catalogue", () => {
  it("offers the sizes a module comes in", () => {
    expect(MODULES.some((m) => m.name === "Frame Shift Drive")).toBe(true);
    const mc = choices({ name: "Multi-Cannon", class: 3, rating: null, mount: "Gimballed" });
    expect(mc.ratings).toEqual(["C"]);
    expect(choices({ name: "Multi-Cannon", class: null, rating: null, mount: null }).mounts).toEqual(["Fixed", "Gimballed", "Turreted"]);
    expect(choices({ name: "Frame Shift Drive", class: null, rating: null, mount: null }).mounts).toEqual([]);
  });

  it("drops picks the new module doesn't come in", () => {
    expect(settle({ name: "Fuel Scoop", class: 3, rating: "C", mount: "Gimballed" })).toEqual({ name: "Fuel Scoop", class: 3, rating: "C", mount: null });
    expect(settle({ name: "Shield Booster", class: 5, rating: "A", mount: null }).class).toBe(null);
  });
});
