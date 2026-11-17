import { describe, expect, it } from "vitest";
import { ageCell, DEFAULT_FILTERS, groupHead, groups, isStale, jumpsFigure, panelFacts, placeHead, summary, widening, type Results, type Row } from "./search";

const NOW = Date.parse("2026-11-05T12:00:00Z");
const row: Row = { key: 1, system: "Sol", station: "Galileo", ly: 1426.25, jumps: 44, exact: false, pad: "L", arrival_ls: 6784404, updated_at: "2026-10-26T12:00:00Z" };

describe("result row groups", () => {
  const sell: Results = {
    origin: "Sol",
    range: 30,
    source: "Ardent Insight",
    fetched: "2026-11-05T12:00:00Z",
    place: "station",
    main: { head: "Price", unit: "cr" },
    columns: [{ key: "volume", head: "Demand", unit: "t", at: "qual", bars: true }],
    rows: [row],
  };

  it("puts demand under the price and the trip in a group of its own, with nothing else on the row", () => {
    const g = groups(sell, [row], true);
    expect(g).toMatchObject({ qual: { key: "volume" }, under: null, own: [], aside: [], same: [], trip: true });
    expect(placeHead("station", null)).toBe("Station · pad · arrival");
    expect(groupHead(["Price", "Demand"])).toBe("Price · demand");
  });

  it("keeps a fact the commander doesn't pick by in the side panel, or on the row without one", () => {
    const res: Results = { ...sell, main: undefined, place: "system", columns: [{ key: "faction", head: "Faction", text: true, at: "row" }, { key: "pop", head: "Population" }] };
    const rows = [row, { ...row, key: 2, facts: { faction: "Ross 154 Boys", pop: "35k" } }];
    // A system's first prose fact is its second line; the trip ranks the list, so no trip group.
    expect(groups(res, rows, true)).toMatchObject({ qual: null, under: { key: "faction" }, own: [], aside: [{ key: "pop" }], trip: false });
    expect(groups(res, rows, false)).toMatchObject({ under: { key: "faction" }, own: [{ key: "pop" }], aside: [] });
    expect(placeHead("system", { key: "faction", head: "Faction" })).toBe("System · faction");
  });

  it("leaves out a prose fact every row shares, and says it once in the summary", () => {
    const res: Results = { ...sell, main: undefined, columns: [{ key: "detail", head: "Module", text: true, at: "row", same: (v) => v }, { key: "faction", head: "Faction", text: true }] };
    const rows = [0, 1].map((k) => ({ ...row, key: k, facts: { detail: "5A Frame Shift Drive", faction: k ? "Mother Gaia" : "Sol Workers" } }));
    expect(groups(res, rows, true)).toMatchObject({ own: [], aside: [{ key: "faction" }], same: ["5A Frame Shift Drive"] });
    // One row is not "every row": nothing to compare it with.
    expect(groups(res, rows.slice(0, 1), true)).toMatchObject({ own: [{ key: "detail" }], same: [] });
  });

  it("gives the side panel every fact under its name, the report's age among them", () => {
    expect(panelFacts(sell, { ...row, facts: { volume: "3,120" } }, NOW)).toEqual([
      { head: "Demand", cell: { value: "3,120", unit: "t", text: undefined } },
      { head: "Pad", cell: { value: "L" } },
      { head: "Arrival", cell: { value: "6,784,404", unit: "ls" } },
      { head: "Trip", cell: { value: "~44 jumps · 1,426.3", unit: "ly" } },
      { head: "Updated", cell: { value: "10", unit: "d ago" } },
    ]);
    // Unknown facts are a dash; data that never ages (ring hotspots) has no Updated.
    const ring = panelFacts({ place: "ring", undated: true }, { ...row, arrival_ls: null }, NOW);
    expect(ring.map((f) => [f.head, f.cell.value])).toEqual([
      ["Arrival", "—"],
      ["Trip", "~44 jumps · 1,426.3"],
    ]);
  });

  it("says how old a report is in minutes, hours or days", () => {
    expect(ageCell("2026-11-05T11:48:00Z", NOW)).toEqual({ value: "12", unit: "min" });
    expect(ageCell("2026-11-04T07:00:00Z", NOW)).toEqual({ value: "29", unit: "h" });
    expect(ageCell("2021-02-12T12:00:00Z", NOW)).toEqual({ value: "2,092", unit: "d" });
    expect(jumpsFigure({ jumps: 3, exact: true })).toBe("3");
  });

  it("calls a report stale only past the limit, and an undated one never", () => {
    expect(isStale({ updated_at: "2026-10-05T11:00:00Z" }, 30, NOW)).toBe(true);
    expect(isStale({ updated_at: "2026-10-06T13:00:00Z" }, 30, NOW)).toBe(false);
    expect(isStale({ updated_at: null }, 1, NOW)).toBe(false);
  });
});

describe("constraint summary", () => {
  it("names the radius, pad and report age the way DESIGN.md's example does", () => {
    const f = { ...DEFAULT_FILTERS, pad: "L" as const };
    expect(summary("Best prices near", "Sol", f, ["distance", "pad", "age"], { distance: 100, age: 30 }, 0)).toBe(
      "Within 100 ly of Sol · large pad · reports up to 30 days old",
    );
  });

  it("names only the filters a lookup offers, and counts the reports hidden for their age", () => {
    const f = { ...DEFAULT_FILTERS, pad: "M" as const, max_age_days: 7, exclude_carriers: true, min_volume: 500, max_arrival_ls: 5000 };
    expect(summary("Nearest to", "Deciat", f, ["age"], {}, 2)).toBe("Nearest to Deciat · reports up to 7 days old · 2 older reports hidden");
    expect(summary("Nearest to", "Deciat", DEFAULT_FILTERS, ["age"], {}, 0)).toBe("Nearest to Deciat · reports of any age");
    expect(summary("Best prices anywhere · distances from", "Sol", f, ["volume", "pad", "arrival", "carriers", "odyssey"], { volume: "Demand" }, 1)).toBe(
      "Best prices anywhere · distances from Sol · demand at least 500 t · medium or large pad · arrival up to 5,000 ls · no carriers · 1 older report hidden",
    );
  });

  it("names an S pad the commander chose, but not the S of a small ship", () => {
    expect(summary("Nearest to", "Sol", { ...DEFAULT_FILTERS, pad: "S", pad_set: true }, ["pad"], {}, 0)).toBe("Nearest to Sol · any pad");
    expect(summary("Nearest to", "Sol", { ...DEFAULT_FILTERS, pad: "S" }, ["pad"], {}, 0)).toBe("Nearest to Sol");
  });

  it("puts a lookup's own constraints after where it searched", () => {
    expect(summary("Nearest to", "Sol", DEFAULT_FILTERS, ["distance"], { distance: 100 }, 0, ["metallic rings", "pristine reserves"])).toBe(
      "Within 100 ly of Sol · metallic rings · pristine reserves",
    );
  });

  it("names the distance an answer reaches when it covers less than the radius", () => {
    const f = { ...DEFAULT_FILTERS, max_distance_ly: 200 };
    expect(summary("Nearest to", "Sol", f, ["distance"], { distance: 100 }, 0, ["icy rings"], 32)).toBe("Within 32 ly of Sol · icy rings");
  });
});

describe("widening an empty search", () => {
  it("widens the radius first, to the next step the filter offers", () => {
    const w = widening(DEFAULT_FILTERS, ["distance", "age"], { distance: 100, age: 30 });
    expect(w).toMatchObject({ phrase: "within 100 ly", label: "Widen to 200 ly" });
    expect(w?.filters.max_distance_ly).toBe(200);
  });

  it("then allows older reports, then any age where the lookup has no default", () => {
    expect(widening({ ...DEFAULT_FILTERS, max_distance_ly: 500 }, ["distance", "age"], { age: 30 })).toMatchObject({ label: "Allow 90 days", filters: { max_age_days: 90 } });
    expect(widening({ ...DEFAULT_FILTERS, max_age_days: 365 }, ["age"], {})).toMatchObject({
      phrase: "with reports up to 365 days old",
      label: "Allow any age",
      filters: { max_age_days: null },
    });
  });

  it("never relaxes the pad, and has nothing to offer when nothing else is set", () => {
    expect(widening({ ...DEFAULT_FILTERS, pad: "L", pad_set: true }, ["pad", "age"], {})).toBeNull();
    expect(widening({ ...DEFAULT_FILTERS, exclude_carriers: true }, ["carriers"], {})).toMatchObject({ label: "Include carriers", filters: { exclude_carriers: false } });
  });
});
