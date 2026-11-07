import { describe, expect, it } from "vitest";
import { DEFAULT_FILTERS } from "./search";
import { stations, toResults } from "./stations";

describe("station lookups (browser preview)", () => {
  it("adapts the answer into the kit's rows, nearest first", async () => {
    const r = toResults(await stations({ kind: "service", service: "black_market" }, null, DEFAULT_FILTERS));
    expect(r.origin).toBe("Sol");
    // Ranked by jumps (no main value), with who runs the station as the lookup's one column.
    expect(r.main).toBeUndefined();
    expect(r.columns?.map((c) => c.head)).toEqual(["Faction"]);
    expect(r.rows[0]).toMatchObject({ station: "Ehrlich City", system: "Sol", pad: "L", jumps: 0, facts: { faction: "Federation · Mother Gaia · Boom" } });
    expect(r.rows[0]?.station_type).toBe("Planetary Port");
    expect(r.rows.every((row, i) => i === 0 || (r.rows[i - 1]?.ly ?? 0) <= row.ly)).toBe(true);
  });

  it("adds the matched module and the price as columns for Outfitting, and the price for Shipyard", async () => {
    const want = { kind: "module", name: "Frame Shift Drive", class: 5, rating: "A", mount: null } as const;
    const modules = toResults(await stations(want, null, DEFAULT_FILTERS));
    expect(modules.columns?.map((c) => [c.head, c.unit ?? null, !!c.text])).toEqual([["Module", null, true], ["Price", "cr", false], ["Faction", null, true]]);
    expect(modules.rows[0]?.facts).toMatchObject({ detail: "5A Frame Shift Drive", price: "5,103,950" });
    const ships = toResults(await stations({ kind: "ship", ship: "krait_mkii" }, null, DEFAULT_FILTERS));
    expect(ships.columns?.map((c) => c.head)).toEqual(["Price", "Faction"]);
    expect(ships.rows[0]?.facts?.price).toBe("45,814,205");
  });

  it("applies the filters the way the core does", async () => {
    const want = { kind: "service", service: "black_market" } as const;
    const all = await stations(want, "Alpha Centauri", DEFAULT_FILTERS);
    const some = await stations(want, "Alpha Centauri", { ...DEFAULT_FILTERS, exclude_carriers: true, exclude_odyssey: true, max_arrival_ls: 1000 });
    expect(some.here.system).toBe("Alpha Centauri");
    expect(some.hits.length).toBeLessThan(all.hits.length);
    expect(some.hits.some((h) => h.station_type === "Drake-Class Carrier" || h.station_type === "Settlement")).toBe(false);
    expect(some.hits.every((h) => h.arrival_ls == null || h.arrival_ls <= 1000)).toBe(true);
  });
});
