import { describe, expect, it } from "vitest";
import trade from "../preview/fixtures/trade.json";
import ship from "../preview/fixtures/ship-status.json";
import type { ShipStatus } from "./api/types/ShipStatus";
import { buyAgainst, depth, padTooSmall, sellAgainst, TIGHT_TIMES } from "./demand";

// The sol world's Asp, whose hold the trade samples are measured against: 300 t of Low
// Temperature Diamonds, 200 t of Painite, 100 t of Platinum, and gold that is stolen or for a
// mission; then the same ship with its hold emptied. The ship world's Anaconda carries 100 t of
// Painite. Elite not running: nothing known.
const sol = trade.ship as unknown as ShipStatus;
const emptied = trade.ship_emptied as unknown as ShipStatus;
const anaconda = ship.flying as unknown as ShipStatus;
const outside = { ...sol, sellable: null, cargo_free: null, cargo: null, cargo_capacity: null, pad: null };

describe("demand against the hold", () => {
  it("is short below the figure, tight below five times it, plenty from there", () => {
    expect(TIGHT_TIMES).toBe(5);
    expect(depth(199, 200)).toBe("short");
    expect(depth(200, 200)).toBe("tight");
    expect(depth(999, 200)).toBe("tight");
    expect(depth(1000, 200)).toBe("plenty");
  });

  it("measures Sell against the searched commodity aboard, not the whole hold", () => {
    const painite = sellAgainst(sol, "painite", "Painite");
    expect(painite).toEqual({ tonnes: 200, text: "200 t of Painite aboard" });
    // 600 t are the commander's to sell, but a 250 t buyer takes all 200 t of Painite.
    expect(depth(250, painite.tonnes)).toBe("tight");
    expect(depth(250, sellAgainst(sol, "lowtemperaturediamond", "Low Temperature Diamonds").tonnes)).toBe("short");
  });

  it("leaves out stolen and mission cargo, and tells none aboard from not known", () => {
    expect(sellAgainst(sol, "gold", "Gold")).toEqual({ tonnes: 0, text: "No Gold aboard" });
    expect(depth(5, 0)).toBeNull();
    expect(sellAgainst(outside, "painite", "Painite")).toEqual({ tonnes: null, text: "Cargo not known: Elite isn't running" });
    expect(depth(5, null)).toBeNull();
  });

  it("measures Buy against the free space, an empty hold included", () => {
    expect(buyAgainst(emptied)).toEqual({ tonnes: 200, text: "200 t free" });
    expect(depth(31, 200)).toBe("short");
    expect(buyAgainst(anaconda)).toEqual({ tonnes: 136, text: "136 t free" });
    // The sol world's hold reports more aboard than it holds: full, so no seller is short.
    expect(buyAgainst(sol)).toEqual({ tonnes: 0, text: "Hold full" });
    expect(buyAgainst(outside).tonnes).toBeNull();
  });

  it("marks a pad smaller than the ship needs, and an unknown one never", () => {
    expect([padTooSmall("M", "L"), padTooSmall("L", "L"), padTooSmall("L", "M"), padTooSmall(null, "L"), padTooSmall("S", null)]).toEqual([true, false, false, false, false]);
    expect(sol.pad).toBe("M");
    expect(anaconda.pad).toBe("L");
  });
});
