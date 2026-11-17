import { describe, expect, it } from "vitest";
import { buyList, buyResults, loadProjects, needsOf, progressOf, type Project } from "./colonisation";
import { DEFAULT_FILTERS } from "./search";

async function site(): Promise<Project> {
  const [p] = await loadProjects();
  if (!p) throw new Error("no sample project");
  return p;
}

describe("colonisation", () => {
  it("asks only for what is left to buy, by market symbol and in-game name", async () => {
    const p = await site();
    const needs = needsOf({ ...p, shopping: { ...p.shopping, lines: [...p.shopping.lines, { symbol: "aluminium", name: "$aluminium_name;", name_localised: null, remaining: 50, in_cargo: 50, to_buy: 0 }] } });
    expect(needs.map((n) => n.symbol)).toEqual(["steel", "cmmcomposite", "ceramiccomposites", "insulatingmembrane"]);
    expect(needs[3]).toEqual({ symbol: "insulatingmembrane", name: "Insulating Membrane", tonnes: 310 });
  });

  it("asks for nothing once a project has failed, whatever it still lacked", async () => {
    const p = await site();
    expect(needsOf({ ...p, failed: true })).toEqual([]);
  });

  it("names a commodity by its market symbol when the journal gave no localised name", async () => {
    const p = await site();
    const bare = {
      ...p,
      resources: p.resources.map((r) => ({ ...r, name_localised: null })),
      shopping: { ...p.shopping, lines: p.shopping.lines.map((l) => ({ ...l, name_localised: null })) },
    };
    expect(progressOf(bare).map((r) => r.name)).toEqual(["steel", "cmmcomposite", "ceramiccomposites", "insulatingmembrane", "aluminium"]);
  });

  it("previews the recorded search with the filters applied", async () => {
    const needs = needsOf(await site());
    const all = buyResults(await buyList(needs, "Wolf 359", null), 4);
    expect(all.origin).toBe("Wolf 359");
    expect(all.main?.head).toBe("Sells");
    expect(all.rows[0]).toMatchObject({ station: "Efremov Plant", main: "3 of 4", pad: "L" });
    const strict = buyResults(await buyList(needs, null, { ...DEFAULT_FILTERS, pad: "L", exclude_carriers: true, max_distance_ly: 20 }), 4);
    expect(strict.rows.some((r) => r.facts?.stock?.includes("fleet carrier") || r.pad === "M")).toBe(false);
    expect(strict.rows.length).toBeLessThan(all.rows.length);
  });
});
