import { describe, expect, it } from "vitest";
import { byDay, type Visit } from "./logbook";

const v = (ts: string, kind: "system" | "station", via: string): Visit => ({
  ts,
  kind,
  via,
  star_system: "Sol",
  system_address: 10477373803,
  station_name: kind === "station" ? "Abraham Lincoln" : null,
  market_id: kind === "station" ? 128016640 : null,
  jump_dist: via === "FSDJump" ? 4.38 : null,
});

describe("byDay", () => {
  const cases: { name: string; timeline: Visit[]; want: [string, number, number, number][] }[] = [
    { name: "nothing yet", timeline: [], want: [] },
    {
      name: "newest first, one group per UTC date; only FSD jumps count as jumps",
      timeline: [
        v("2026-09-03T10:25:00Z", "station", "Docked"),
        v("2026-09-03T00:00:00Z", "system", "FSDJump"),
        v("2026-09-02T23:59:59Z", "system", "CarrierJump"),
        v("2026-09-02T10:20:00Z", "system", "FSDJump"),
        v("2026-09-02T10:00:07Z", "system", "Location"),
        v("2026-09-01T10:00:00Z", "station", "Docked"),
      ],
      // date, jumps, dockings, rows
      want: [
        ["2026-09-03", 1, 1, 2],
        ["2026-09-02", 1, 0, 3],
        ["2026-09-01", 0, 1, 1],
      ],
    },
  ];
  it.each(cases)("$name", ({ timeline, want }) => {
    expect(byDay(timeline).map((d) => [d.date, d.jumps, d.dockings, d.visits.length])).toEqual(want);
  });
});
