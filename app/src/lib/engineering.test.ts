import { describe, expect, it } from "vitest";
import sample from "./fixtures/engineering.json";
import { shoppingChange, type PlanResult } from "./engineering";

describe("shoppingChange", () => {
  it("sends the ship and what its plan still has to collect, in the plan's order", () => {
    const plan = sample.plan as unknown as PlanResult;
    const c = shoppingChange(plan);
    expect(c.ship_id).toBe(plan.ship_id);
    expect(c.materials).toEqual(plan.farm.map((f) => f.id));
    expect(c.materials.slice(0, 2)).toEqual(["dataminedwake", "pharmaceuticalisolators"]);
  });
});
