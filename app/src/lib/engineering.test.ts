import { describe, expect, it } from "vitest";
import sample from "../preview/fixtures/engineering.json";
import ships from "../preview/fixtures/ships.json";
import { fittedLine, shoppingChange, slotGroup, type PlanResult, type Ship } from "./engineering";

describe("slotGroup", () => {
  // Every slot name in the preview's recorded fleets, and the military slot they lack.
  it.each([
    ["LargeHardpoint1", "Hardpoints"],
    ["MediumHardpoint2", "Hardpoints"],
    ["TinyHardpoint1", "Utility"],
    ["TinyHardpoint4", "Utility"],
    ["Armour", "Core"],
    ["PowerPlant", "Core"],
    ["MainEngines", "Core"],
    ["FrameShiftDrive", "Core"],
    ["LifeSupport", "Core"],
    ["PowerDistributor", "Core"],
    ["Radar", "Core"],
    ["FuelTank", "Core"],
    ["Slot01_Size6", "Optional"],
    ["Slot09_Size1", "Optional"],
    ["Military01", "Optional"],
    ["PlanetaryApproachSuite", null],
    ["ShipCockpit", null],
    ["CargoHatch", null],
    ["PaintJob", null],
    ["Decal1", null],
  ])("%s is %s", (slot, group) => {
    expect(slotGroup(slot)).toBe(group);
  });

  it("lists every recorded slot but the planetary approach suite", () => {
    const slots = new Set([...(ships.fleet.ships as Ship[]), ...(sample.fleet.ships as Ship[])].flatMap((s) => s.modules.map((m) => m.slot)));
    const listed = [...slots].filter((s) => slotGroup(s) === null);
    expect(listed).toEqual(["PlanetaryApproachSuite"]);
  });
});

describe("fittedLine", () => {
  it("names the applied blueprint, or stock, or nothing when the slot cannot be engineered", () => {
    const fsd = (sample.fleet.ships as Ship[]).find((s) => s.name === "Long Way Round")!.modules.find((m) => m.slot === "FrameShiftDrive")!;
    expect(fittedLine(fsd)).toBe("G3 25% Increased range");
    expect(fittedLine({ ...fsd, engineering: undefined })).toBe("Not engineered");
    expect(fittedLine({ ...fsd, group: null, engineering: undefined })).toBe("");
  });
});

describe("shoppingChange", () => {
  it("sends the ship and what its plan still has to collect, in the plan's order", () => {
    const plan = sample.plan as unknown as PlanResult;
    const c = shoppingChange(plan);
    expect(c.ship_id).toBe(plan.ship_id);
    expect(c.materials.map((m) => m.id)).toEqual(plan.farm.map((f) => f.id));
    expect(c.materials.map((m) => m.id).slice(0, 2)).toEqual(["dataminedwake", "pharmaceuticalisolators"]);
  });
  it("asks for each material until what was held plus what is left to collect", () => {
    const plan = sample.plan as unknown as PlanResult;
    const f = plan.farm[0]!;
    expect(shoppingChange(plan).materials[0]).toEqual({ id: f.id, until: f.have + f.collect });
  });
  it("sends an empty list for a plan with nothing left to collect", () => {
    const plan = { ...(sample.plan as unknown as PlanResult), farm: [] };
    expect(shoppingChange(plan).materials).toEqual([]);
  });
});
