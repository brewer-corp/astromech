import { describe, expect, it } from "vitest";
import sample from "./fixtures/engineering.json";
import { fittedLine, groupFitted, shoppingChange, type Fitted, type PlanResult, type Ship } from "./engineering";

describe("groupFitted", () => {
  const ship = (sample.fleet.ships as Ship[]).find((s) => s.name === "Long Way Round")!;

  it("splits a loadout into the game's outfitting groups", () => {
    expect(groupFitted(ship.modules).map((g) => [g.title, g.mods.map((m) => m.slot)])).toEqual([
      ["Hardpoints", ["MediumHardpoint1"]],
      ["Utility", ["TinyHardpoint1"]],
      ["Core internals", ["Armour", "PowerPlant", "MainEngines", "FrameShiftDrive"]],
      ["Optional internals", ["Slot01_Size6"]],
    ]);
  });

  it("can keep only modules that have an engineering group", () => {
    const cargo: Fitted = { slot: "Slot02_Size2", slot_name: "Optional 2", item: "int_cargorack_size2_class1", name: "2E Cargo Rack", group: null };
    expect(groupFitted([...ship.modules, cargo], true).flatMap((g) => g.mods.map((m) => m.slot))).not.toContain("Slot02_Size2");
    expect(groupFitted([...ship.modules, cargo]).flatMap((g) => g.mods.map((m) => m.slot))).toContain("Slot02_Size2");
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
