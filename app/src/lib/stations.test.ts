import { describe, expect, it } from "vitest";
import { DEFAULT_FILTERS } from "./search";
import { stations, toResults } from "./stations";

describe("station lookups (browser preview)", () => {
  it("adapts the answer into the kit's rows, nearest first", async () => {
    const r = toResults(await stations({ kind: "service", service: "black_market" }, null, DEFAULT_FILTERS));
    expect(r.origin).toBe("Sol");
    expect(r.rows[0]).toMatchObject({ station: "Ehrlich City", system: "Sol", pad: "L", jumps: 0, allegiance: "Federation", faction: "Mother Gaia" });
    expect(r.rows[0]?.station_type).toBe("Planetary Port");
    expect(r.rows.every((row, i) => i === 0 || (r.rows[i - 1]?.ly ?? 0) <= row.ly)).toBe(true);
  });

  it("applies the filters the way the shell does", async () => {
    const want = { kind: "service", service: "black_market" } as const;
    const all = await stations(want, "Alpha Centauri", DEFAULT_FILTERS);
    const some = await stations(want, "Alpha Centauri", { ...DEFAULT_FILTERS, exclude_carriers: true, exclude_odyssey: true, max_arrival_ls: 1000 });
    expect(some.here.system).toBe("Alpha Centauri");
    expect(some.hits.length).toBeLessThan(all.hits.length);
    expect(some.hits.some((h) => h.station_type === "Drake-Class Carrier" || h.station_type === "Settlement")).toBe(false);
    expect(some.hits.every((h) => h.arrival_ls == null || h.arrival_ls <= 1000)).toBe(true);
  });
});
