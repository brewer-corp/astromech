import { describe, expect, it } from "vitest";
import { collect, rank, type PaletteItem } from "./palette";
import { modules } from "./modules";

const item = (label: string, detail = "Material", route = label.toLowerCase(), keywords?: string): PaletteItem => ({ label, detail, route, keywords });

const items = [
  item("Materials", "Page · Engineering", "materials"),
  item("Proto Heat Radiators", "Material · Manufactured G5"),
  item("Heat Conduction Wiring", "Material · Manufactured G1"),
  item("5A Frame Shift Drive", "Module · Frame shift drive on Heavy Lifter"),
  item("Increased range", "Blueprint · Frame Shift Drive", "engineering/3", "fsd"),
  item("Phase Alloys", "Material · Manufactured G3"),
];
const labels = (q: string) => rank(items, q).map((i) => i.label);

describe("palette matching", () => {
  it("puts the whole label, then its start, then word starts first", () => {
    expect(labels("materials")[0]).toBe("Materials");
    expect(labels("heat")).toEqual(["Heat Conduction Wiring", "Proto Heat Radiators"]);
    expect(labels("proto heat")).toEqual(["Proto Heat Radiators"]);
    expect(labels("heat rad")).toEqual(["Proto Heat Radiators"]);
  });

  it("matches initials, substrings, and words in the detail or keywords", () => {
    expect(labels("phr")).toEqual(["Proto Heat Radiators"]);
    expect(labels("adiat")).toEqual(["Proto Heat Radiators"]);
    expect(labels("fsd increased")).toEqual(["Increased range"]);
    expect(labels("blueprint")).toEqual(["Increased range"]);
  });

  it("ignores case and spacing, lists the first entries for a blank query, and finds nothing for nonsense", () => {
    expect(labels("  PROTO   heat ")).toEqual(["Proto Heat Radiators"]);
    expect(rank(items, "", 2).map((i) => i.label)).toEqual(["Materials", "Proto Heat Radiators"]);
    expect(labels("zzz")).toEqual([]);
    expect(labels("+")).toEqual([]);
    expect(labels("? -")).toEqual([]);
  });

  it("collects every page and what the modules offer, skipping a module whose entries fail", async () => {
    const pages = modules.map((m) => ({ ...m, palette: undefined }));
    const failing = { ...pages[0]!, palette: () => Promise.reject(new Error("no journal")) };
    const offering = { ...pages[1]!, palette: () => Promise.resolve([item("Proto Heat Radiators", "Material", "materials/protoheatradiators")]) };
    const all = await collect([failing, offering, ...pages.slice(2)], (g) => g.toUpperCase());
    expect(all.length).toBe(modules.length + 1);
    expect(all[0]).toMatchObject({ label: pages[0]!.title, route: pages[0]!.id, detail: `Page · ${pages[0]!.group.toUpperCase()}` });
    expect(all.at(-1)?.route).toBe("materials/protoheatradiators");
  });

  it("offers the sample materials and fleet from the modules that have them", async () => {
    const all = await collect(modules, (g) => g);
    expect(rank(all, "proto heat")[0]).toMatchObject({ label: "Proto Heat Radiators", route: "materials/protoheatradiators" });
    expect(rank(all, "heavy lifter")[0]).toMatchObject({ detail: "Ship · Anaconda", route: "engineering/3" });
    expect(rank(all, "frame shift drive")[0]).toMatchObject({ label: "Frame Shift Drive", route: "outfitting/Frame%20Shift%20Drive" });
    expect(rank(all, "6a frame shift")[0]).toMatchObject({ detail: "Outfitting · fitted on Heavy Lifter", route: "outfitting/Frame%20Shift%20Drive/6A" });
    expect(rank(all, "2f pulse laser")[0]?.route).toBe("outfitting/Pulse%20Laser/2F/Gimballed");
    expect(rank(all, "lightweight alloy")[0]).toMatchObject({ detail: "Module · Armour on Heavy Lifter", route: "engineering/3" });
    expect(rank(all, "anaconda")[0]).toMatchObject({ detail: "Shipyard", route: "shipyard/Anaconda" });
    expect(rank(all, "increased range")[0]?.detail).toMatch(/^Blueprint · /);
  });
});
