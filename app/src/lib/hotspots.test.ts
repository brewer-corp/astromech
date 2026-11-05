import { describe, expect, it } from "vitest";
import { DEFAULT_FILTERS } from "./search";
import { alsoLine, hotspotResults, hotspots, ORES, ringLabel, wantNotes, type Hotspots } from "./hotspots";
import sample from "./fixtures/hotspots.json";

const any = sample.any as unknown as Hotspots;

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
      system: "Sosolingati",
      station: "5 A Ring",
      main: "4",
      exact: false,
      updated_at: null,
      facts: { also: "Monazite 2, Rhodplumsite 2, Platinum 1", type: "Metal Rich", reserve: "Depleted" },
    });
    expect(r.rows.map((x) => x.main)).toEqual(["4", "3"]);
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
    const metallic = await hotspots({ ore: "platinum", ring_type: "Metallic", reserve: "pristine" }, "Deciat", DEFAULT_FILTERS);
    expect(metallic.name).toBe("Platinum");
    expect(metallic.here.system).toBe("Deciat");
    expect(metallic.rings.length).toBeGreaterThan(0);
    expect(metallic.rings.every((r) => r.ring_type === "Metallic" && r.reserve === "Pristine")).toBe(true);
  });
});
