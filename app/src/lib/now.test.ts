import { describe, expect, it } from "vitest";
import type { ShipStatus } from "./api/types/ShipStatus";
import { saleKey } from "./now";

// The status bar's sample ship (fixtures/ship-status.json `flying`), in Sol.
const ship: ShipStatus = {
  ship_id: 3, ship_name: "Heavy Lifter", max_jump_range: 18, fuel: 24.6, fuel_capacity: 32, cargo: 120, cargo_capacity: 256, cargo_free: 136, pad: "L",
  sellable: [{ symbol: "painite", name: "Painite", tonnes: 100 }, { symbol: "lowtemperaturediamond", name: "Low Temperature Diamonds", tonnes: 20 }],
};
const key = saleKey(ship, "Sol");

describe("the Sell your cargo answer's change key", () => {
  it.each([
    { what: "a jump with the same cargo", ship, system: "Alpha Centauri", same: false },
    {
      what: "the same tonnes of another commodity",
      ship: { ...ship, sellable: [{ symbol: "platinum", name: "Platinum", tonnes: 100 }, { symbol: "lowtemperaturediamond", name: "Low Temperature Diamonds", tonnes: 20 }] },
      system: "Sol",
      same: false,
    },
    { what: "another ship of the same capacity", ship: { ...ship, ship_id: 4, ship_name: "Hauler Two" }, system: "Sol", same: false },
    { what: "mission cargo or limpets aboard (not sellable)", ship: { ...ship, cargo: 136, cargo_free: 120 }, system: "Sol", same: true },
    { what: "fuel used", ship: { ...ship, fuel: 20 }, system: "Sol", same: true },
    { what: "identical inputs", ship: structuredClone(ship), system: "Sol", same: true },
  ])("$what: same key $same", ({ ship: s, system, same }) => {
    expect(saleKey(s, system) === key).toBe(same);
  });
});
