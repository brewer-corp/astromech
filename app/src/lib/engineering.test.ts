import { describe, expect, it } from "vitest";
import sample from "./fixtures/engineering.json";
import { shoppingChange, type PlanResult } from "./engineering";

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
