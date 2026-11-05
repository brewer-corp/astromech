import { beforeAll, describe, expect, it } from "vitest";
import { find, type Match } from "./find";
import { modules } from "./modules";
import { collect, type PaletteItem } from "./palette";
import { FINDERS } from "./places";

// The header's Find or plot box: the window's entries (pages, finders and their values, materials,
// the sample fleet) and Spansh's systems, ranked together.
let entries: PaletteItem[];
beforeAll(async () => {
  entries = await collect(modules, (g) => g);
});

const route = (m: Match | undefined) => (m?.kind === "entry" ? m.item.route : m?.kind === "system" ? `system:${m.name}` : undefined);
const top = (q: string, systems: string[] = []) => route(find(entries, systems, q)[0]);
const all = (q: string, systems: string[] = []) => find(entries, systems, q).map(route);

describe("Find or plot matching", () => {
  it("ranks a system typed in full first, and a partial one after pages that start with it", () => {
    expect(top("maia", ["Maia", "Maialino"])).toBe("system:Maia");
    expect(top("Colonia", ["Colonia Sector AA-A h1", "Colonia"]), "Spansh's order is fuzzy").toBe("system:Colonia");
    const col = all("col", ["Colonia", "Col 285 Sector"]);
    expect(col[0]).toBe("colonisation");
    expect(col).toContain("system:Colonia");
    expect(col.indexOf("system:Colonia")).toBeLessThan(col.indexOf("system:Col 285 Sector"));
    expect(find(entries, ["Sol"], "  ")).toEqual([]);
  });

  it("opens the Services finder filled in from a service's name or a phrase", () => {
    expect(top("interstellar factors")).toBe("places/services/interstellar_factors");
    expect(top("services repair")).toBe("places/services/repair");
    expect(top("black market")).toBe("places/services/black_market");
  });

  it("opens the Outfitting finder filled in from a module's initials after the finder's name", () => {
    expect(top("outfitting fsd")).toBe("places/outfitting/Frame%20Shift%20Drive");
    expect(top("outfitting pulse laser")).toBe("places/outfitting/Pulse%20Laser");
    expect(top("frame shift drive")).toBe("places/outfitting/Frame%20Shift%20Drive");
  });

  it("opens the Shipyard finder filled in from a ship after the finder's name", () => {
    expect(top("shipyard python")).toBe("places/shipyard/Python");
    expect(top("shipyard krait mk")).toBe("places/shipyard/Krait%20MkII");
    expect(top("anaconda")).toBe("places/shipyard/Anaconda");
  });

  it("opens the Material traders finder, not Services, for a trader's name", () => {
    expect(top("raw trader")).toBe("places/material-traders/raw");
    expect(top("encoded trader")).toBe("places/material-traders/encoded");
    expect(top("material traders manufactured")).toBe("places/material-traders/manufactured");
    expect(top("traders any")).toBe("places/material-traders/any");
  });

  it("opens the Trade, Engineering and Mining finders filled in", () => {
    expect(top("sell painite")).toBe("places/sell/painite");
    expect(top("buy ltd")).toBe("places/buy/lowtemperaturediamond");
    expect(top("painite")).toBe("places/sell/painite");
    expect(all("painite")).toEqual(expect.arrayContaining(["places/buy/painite", "places/mining-hotspots/painite"]));
    expect(top("raw sites brain trees")).toBe("places/raw-sites/brain_trees");
    expect(top("crystalline shards")).toBe("places/raw-sites/crystalline_shards");
    expect(top("faction states boom")).toBe("places/faction-states/proto");
    expect(top("painite hotspots")).toBe("places/mining-hotspots/painite");
    expect(top("mining platinum")).toBe("places/mining-hotspots/platinum");
  });

  it("matches every finder's values after its name, from the finders' own lists", async () => {
    for (const f of FINDERS) {
      const inputs = (await f.inputs?.()) ?? [];
      expect(inputs.length > 0 || f.id === "sell-cargo", f.id).toBe(true);
      for (const i of inputs.slice(0, 3)) expect(top(`${f.title} ${i.label}`), `${f.title} ${i.label}`).toBe(i.route);
    }
  });

  it("names a finder with the start of its title's words, in either order", () => {
    expect(top("sell")).toBe("places/sell");
    expect(top("sel painite")).toBe("places/sell/painite");
    expect(top("hotspots painite")).toBe("places/mining-hotspots/painite");
    expect(top("fsd outfitting")).toBe("places/outfitting/Frame%20Shift%20Drive");
  });

  it("still finds pages, materials, modules and the fleet", () => {
    expect(top("powerplay")).toBe("powerplay");
    expect(top("proto heat")).toBe("materials/protoheatradiators");
    expect(top("heavy lifter")).toBe("engineering/3");
    expect(top("2f pulse laser")).toBe("places/outfitting/Pulse%20Laser?class=2&rating=F&mount=Gimballed");
    expect(all("zzzz")).toEqual([]);
  });
});
