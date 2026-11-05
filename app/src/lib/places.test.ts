import { describe, expect, it } from "vitest";
import {
  FINDERS,
  MOVED_FINDERS,
  factionRoute,
  forgetAnswers,
  homeSections,
  hotspotRoute,
  lastAnswer,
  marketRoute,
  parseFactionRoute,
  parseHotspotRoute,
  parseMarketRoute,
  parsePlaces,
  parseServiceRoute,
  parseSiteRoute,
  parseTraderRoute,
  placesRoute,
  rememberAnswer,
  serviceRoute,
  siteRoute,
  traderRoute,
} from "./places";

describe("Places finders", () => {
  it("lists the built finders under Trade, Ship, Engineering and Mining, leaving out sections with none yet", () => {
    expect(FINDERS.map((f) => [f.id, f.title, f.section, f.input])).toEqual([
      ["sell", "Sell", "Trade", "Commodity"],
      ["buy", "Buy", "Trade", "Commodity"],
      ["sell-cargo", "Sell my cargo", "Trade", "Hold"],
      ["services", "Services", "Ship", "Service"],
      ["outfitting", "Outfitting", "Ship", "Module"],
      ["shipyard", "Shipyard", "Ship", "Ship"],
      ["material-traders", "Material traders", "Engineering", "Trader"],
      ["raw-sites", "Raw sites", "Engineering", "Site"],
      ["faction-states", "Faction states", "Engineering", "State"],
      ["mining-hotspots", "Mining hotspots", "Mining", "Mineral"],
    ]);
    expect(homeSections().map(([s, fs]) => [s, fs.map((f) => f.id)])).toEqual([
      ["Trade", ["sell", "buy", "sell-cargo"]],
      ["Ship", ["services", "outfitting", "shipyard"]],
      ["Engineering", ["material-traders", "raw-sites", "faction-states"]],
      ["Mining", ["mining-hotspots"]],
    ]);
    expect(homeSections(FINDERS.filter((f) => f.section !== "Trade")).map(([s]) => s)).toEqual(["Ship", "Engineering", "Mining"]);
  });

  it("keeps the commodity and range in a Sell or Buy address", () => {
    expect(marketRoute({ side: "sell", symbol: "painite", anywhere: false })).toBe("places/sell/painite");
    expect(marketRoute({ side: "buy", symbol: "lowtemperaturediamond", anywhere: true })).toBe("places/buy/lowtemperaturediamond?range=anywhere");
    expect(parseMarketRoute("#/places/buy/lowtemperaturediamond?range=anywhere")).toEqual({ side: "buy", symbol: "lowtemperaturediamond", anywhere: true });
    expect(parseMarketRoute("#/places/sell/painite?range=far")).toEqual({ side: "sell", symbol: "painite", anywhere: false });
    expect(parseMarketRoute("#/places/sell/unobtainium")).toBeNull();
    expect(parseMarketRoute("#/places/sell")).toBeNull();
    expect(parseMarketRoute("#/places/sell-cargo/painite")).toBeNull();
    expect(parseMarketRoute("#/places/services/painite")).toBeNull();
  });

  it("reads the site kind and faction-state lookup from the address, and nothing from a bad one", () => {
    expect(siteRoute("brain_trees")).toBe("places/raw-sites/brain_trees");
    expect(parseSiteRoute("#/places/raw-sites/geological")).toBe("geological");
    expect(parseSiteRoute("#/places/raw-sites/volcano")).toBeNull();
    expect(parseSiteRoute("#/places/faction-states/geological")).toBeNull();
    expect(factionRoute("famine")).toBe("places/faction-states/famine");
    expect(parseFactionRoute("#/places/faction-states/pharma")).toBe("pharma");
    expect(parseFactionRoute("#/places/faction-states/peace")).toBeNull();
    expect(parseFactionRoute("#/places/raw-sites/pharma")).toBeNull();
  });

  it("keeps the mineral, ring type and reserve in a Mining hotspots address", () => {
    const any = { ore: "painite", ring_type: null, reserve: null } as const;
    expect(hotspotRoute(any)).toBe("places/mining-hotspots/painite");
    expect(hotspotRoute({ ore: "lowtemperaturediamond", ring_type: "Metal Rich", reserve: "pristine" })).toBe(
      "places/mining-hotspots/lowtemperaturediamond?ring=metal-rich&reserve=pristine",
    );
    expect(parseHotspotRoute("#/places/mining-hotspots/lowtemperaturediamond?ring=metal-rich&reserve=pristine")).toEqual({
      ore: "lowtemperaturediamond",
      ring_type: "Metal Rich",
      reserve: "pristine",
    });
    expect(parseHotspotRoute("#/places/mining-hotspots/opal?ring=gold&reserve=rich")).toEqual({ ore: "opal", ring_type: null, reserve: null });
    expect(parseHotspotRoute("#/places/mining-hotspots/gold")).toBeNull();
    expect(parseHotspotRoute("#/places/mining-hotspots")).toBeNull();
    expect(parseHotspotRoute("#/places/services/painite")).toBeNull();
  });

  it("routes each finder at #/places/<finder>[/<input>]", () => {
    expect(placesRoute()).toBe("places");
    expect(placesRoute("shipyard", "Krait MkII")).toBe("places/shipyard/Krait%20MkII");
    expect(serviceRoute("black_market")).toBe("places/services/black_market");
    expect(traderRoute("encoded")).toBe("places/material-traders/encoded");
    expect(parsePlaces("#/places")).toMatchObject({ finder: null, input: "" });
    expect(parsePlaces("#/places/outfitting/Pulse%20Laser?class=2")).toMatchObject({ finder: { id: "outfitting" }, input: "Pulse%20Laser" });
    expect(parsePlaces("#/places/outfitting/Pulse%20Laser?class=2")?.query.get("class")).toBe("2");
    expect(parsePlaces("#/places/nope/x")).toMatchObject({ finder: null, input: "" });
    expect(parsePlaces("#/placesx")).toBeNull();
    expect(parsePlaces("#/services")).toBeNull();
  });

  it("reads the service and trader kind from the address, and nothing from a bad one", () => {
    expect(parseServiceRoute("#/places/services/black_market")).toBe("black_market");
    expect(parseServiceRoute("#/places/services")).toBeNull();
    expect(parseServiceRoute("#/places/services/nope")).toBeNull();
    expect(parseServiceRoute("#/places/services/%E0%A4%A")).toBeNull();
    expect(parseServiceRoute("#/services/black_market")).toBeNull();
    expect(parseTraderRoute("#/places/material-traders/manufactured")).toBe("manufactured");
    expect(parseTraderRoute("#/places/material-traders/rare")).toBeNull();
    expect(parseTraderRoute("#/places/services/raw")).toBeNull();
  });

  it("moves the old Galaxy pages to their finders", () => {
    expect(MOVED_FINDERS).toEqual({ services: "places/services", outfitting: "places/outfitting", shipyard: "places/shipyard" });
  });

  it("keeps each finder's last answer for the session, and forgets it when nothing was found", () => {
    forgetAnswers();
    expect(lastAnswer("services")).toBeUndefined();
    const a = { context: "Refuel", figure: "2", unit: "jumps", route: serviceRoute("refuel") };
    rememberAnswer("services", a);
    expect(lastAnswer("services")).toEqual(a);
    expect(lastAnswer("shipyard")).toBeUndefined();
    rememberAnswer("services", null);
    expect(lastAnswer("services")).toBeUndefined();
  });
});
