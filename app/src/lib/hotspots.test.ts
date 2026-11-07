import { describe, expect, it } from "vitest";
import { DEFAULT_FILTERS } from "./search";
import { alsoLine, cutNotes, hotspotResults, hotspots, ORES, ringLabel, wantNotes, type Hotspots } from "./hotspots";
import sample from "../preview/fixtures/hotspots.json";

const any = sample as unknown as Hotspots;

describe("Mining hotspots rows", () => {
  it("ranks by the ring's hotspots and lists the ring, its other hotspots, type and reserve, with no report age", () => {
    const r = hotspotResults({ ...any, rings: any.rings.slice(0, 2) });
    expect(r).toMatchObject({ origin: "Sol", source: "Spansh", place: "ring", main: { head: "Hotspots" }, undated: true });
    expect(r.columns?.map((c) => [c.head, c.text, c.width])).toEqual([
      ["Also", true, undefined],
      ["Type", true, 88],
      ["Reserve", true, 72],
    ]);
    expect(r.fetched).toBe(new Date(any.fetched_at * 1000).toISOString());
    expect(r.rows[0]).toMatchObject({
      system: "LHS 3602",
      station: "3 A Ring",
      main: "10",
      exact: false,
      updated_at: null,
      facts: { also: "Platinum 5, Monazite 3, Serendibite 3, Rhodplumsite 2", type: "Metal Rich", reserve: "Common" },
    });
    expect(r.rows.map((x) => x.main)).toEqual(["10", "9"]);
    expect(r.reach).toBeUndefined();
  });

  it("says what the rings were ranked from, and what was ranked when Spansh's answer was too big", () => {
    const ten = { ...any, rings: any.rings.slice(0, 10) };
    expect(cutNotes(ten)).toEqual(["top 10 of 1,101 bodies with Painite"]);
    expect(hotspotResults(ten).notes).toEqual(["top 10 of 1,101 bodies with Painite"]);
    // Every ring there is (fewer than a full answer): nothing was cut.
    expect(cutNotes({ ...any, bodies: 6, rings: any.rings.slice(0, 6) })).toEqual([]);
    expect(cutNotes({ ...ten, bodies: 10000 })).toEqual(["top 10 of 10,000+ bodies with Painite"]);
    const least = { ...any, least: 5, rings: any.rings.slice(0, 6) };
    expect(cutNotes(least)).toEqual(["5 or more hotspots only: 1,101 bodies, too many to rank"]);
    const near = hotspotResults({ ...any, reach_ly: 32.5, rings: [] });
    expect(near.reach).toBe(32);
    expect(near.notes).toEqual(["1,101 bodies within 100 ly, too many to rank"]);
  });

  it("shortens a ring named after its system, and says the finder's own choices", () => {
    expect(ringLabel("Barnard's Star 5 A Ring", "Barnard's Star")).toBe("5 A Ring");
    expect(ringLabel("Mercury A Ring", "Sol")).toBe("Mercury A Ring");
    expect(alsoLine([])).toBe("");
    expect(wantNotes({ ore: "painite", ring_type: null, reserve: null })).toEqual([]);
    expect(wantNotes({ ore: "painite", ring_type: "Metal Rich", reserve: "major" })).toEqual(["metal rich rings", "major or pristine reserves"]);
    expect(ORES.map((o) => o.symbol)).toContain("lowtemperaturediamond");
  });

  it("answers the browser preview with the recorded rings, the ring type and reserve applied as the shell does", async () => {
    const all = await hotspots({ ore: "painite", ring_type: null, reserve: null }, null, DEFAULT_FILTERS);
    expect(all.rings).toHaveLength(10);
    expect(all.here.system).toBe("Sol");
    const major = await hotspots({ ore: "platinum", ring_type: "Metal Rich", reserve: "major" }, "Deciat", DEFAULT_FILTERS);
    expect(major.name).toBe("Platinum");
    expect(major.here.system).toBe("Deciat");
    expect(major.rings.length).toBeGreaterThan(0);
    expect(major.rings.every((r) => r.ring_type === "Metal Rich" && (r.reserve === "Major" || r.reserve === "Pristine"))).toBe(true);
  });
});
