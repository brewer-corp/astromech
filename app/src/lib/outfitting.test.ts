import { describe, expect, it } from "vitest";
import { choices, moduleRoute, parseModuleRoute, parseShipRoute, settle, shipRoute, wantFor, MODULES } from "./outfitting";

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

  it("finds what to buy for a fitted module from its journal symbol", () => {
    expect(wantFor("int_hyperdrive_size5_class5")).toEqual({ name: "Frame Shift Drive", class: 5, rating: "A", mount: null });
    expect(wantFor("Hpt_PulseLaser_Gimbal_Medium")).toEqual({ name: "Pulse Laser", class: 2, rating: "F", mount: "Gimballed" });
    expect(wantFor("anaconda_armour_grade1")).toBeNull();
  });

  it("round-trips a module through the Outfitting route", () => {
    const fsd = { name: "Frame Shift Drive", class: 5, rating: "A", mount: null };
    expect(moduleRoute(fsd)).toBe("outfitting/Frame%20Shift%20Drive/5A");
    expect(parseModuleRoute(`#/${moduleRoute(fsd)}`)).toEqual(fsd);
    const pl = { name: "Pulse Laser", class: 2, rating: "F", mount: "Gimballed" as const };
    expect(parseModuleRoute(`#/${moduleRoute(pl)}`)).toEqual(pl);
    expect(parseModuleRoute("#/outfitting/Fuel%20Scoop")).toEqual({ name: "Fuel Scoop", class: null, rating: null, mount: null });
  });

  it("ignores Outfitting routes for no, unknown or mangled modules, and drops sizes it doesn't come in", () => {
    expect(parseModuleRoute("#/outfitting")).toBeNull();
    expect(parseModuleRoute("#/outfitting/Warp%20Core")).toBeNull();
    expect(parseModuleRoute("#/outfitting/%E0%A4%A")).toBeNull();
    expect(parseModuleRoute("#/outfitting/Shield%20Booster/5A")).toEqual({ name: "Shield Booster", class: null, rating: "A", mount: null });
  });

  it("round-trips a ship through the Shipyard route", () => {
    expect(parseShipRoute(`#/${shipRoute("Krait MkII")}`)).toBe("Krait MkII");
    expect(parseShipRoute("#/shipyard/Millennium%20Falcon")).toBeNull();
    expect(parseShipRoute("#/shipyard/%E0%A4%A")).toBeNull();
  });
});
