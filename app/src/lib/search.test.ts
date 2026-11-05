import { describe, expect, it } from "vitest";
import { ageCell, DEFAULT_FILTERS, factColumns, isStale, jumpsFigure, summary, widening, type Row } from "./search";

const NOW = Date.parse("2026-11-05T12:00:00Z");
const row: Row = { key: 1, system: "Sol", station: "Galileo", ly: 1426.25, jumps: 44, exact: false, pad: "L", arrival_ls: 6784404, updated_at: "2026-10-26T12:00:00Z" };

describe("result row columns", () => {
  it("adds pad and arrival for stations, arrival for bodies, and neither for systems", () => {
    expect(factColumns({ place: "station" }).map((c) => c.head)).toEqual(["Distance", "Pad", "Arrival", "Updated"]);
    expect(factColumns({ place: "body" }).map((c) => c.head)).toEqual(["Distance", "Arrival", "Updated"]);
    expect(factColumns({ place: "system" }).map((c) => c.head)).toEqual(["Distance", "Updated"]);
  });

  it("puts a lookup's own columns first, and jumps among the facts when something else ranks", () => {
    const cols = factColumns({ place: "station", main: { head: "Price", unit: "cr" }, columns: [{ key: "volume", head: "Demand", unit: "t", width: 88 }] });
    expect(cols.map((c) => c.head)).toEqual(["Demand", "Distance", "Jumps", "Pad", "Arrival", "Updated"]);
    const cells = cols.map((c) => c.cell({ ...row, facts: { volume: "3,120" } }, NOW));
    expect(cells).toEqual([
      { value: "3,120", unit: "t", text: undefined },
      { value: "1,426.3", unit: "ly" },
      { value: "~44" },
      { value: "L" },
      { value: "6,784,404", unit: "ls" },
      { value: "10", unit: "d" },
    ]);
    // Unknown facts are a dash, never an empty cell.
    expect(cols.map((c) => c.cell({ ...row, pad: null, arrival_ls: null, updated_at: null }, NOW).value)).toEqual(["—", "1,426.3", "~44", "—", "—", "—"]);
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
