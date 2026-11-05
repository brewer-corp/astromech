import { describe, expect, it } from "vitest";
import {
  FINDERS,
  MOVED_FINDERS,
  forgetAnswers,
  homeSections,
  hotspotRoute,
  lastAnswer,
  parseHotspotRoute,
  parsePlaces,
  parseServiceRoute,
  parseTraderRoute,
  placesRoute,
  rememberAnswer,
  serviceRoute,
  traderRoute,
} from "./places";

describe("Places finders", () => {
  it("lists the built finders under Ship, Engineering and Mining, leaving out sections with none yet", () => {
    expect(FINDERS.map((f) => [f.id, f.title, f.section, f.input])).toEqual([
      ["services", "Services", "Ship", "Service"],
      ["outfitting", "Outfitting", "Ship", "Module"],
      ["shipyard", "Shipyard", "Ship", "Ship"],
      ["material-traders", "Material traders", "Engineering", "Trader"],
      ["mining-hotspots", "Mining hotspots", "Mining", "Mineral"],
    ]);
    expect(homeSections().map(([s, fs]) => [s, fs.map((f) => f.id)])).toEqual([
      ["Ship", ["services", "outfitting", "shipyard"]],
      ["Engineering", ["material-traders"]],
      ["Mining", ["mining-hotspots"]],
    ]);
    const sell = { id: "services", title: "Sell", section: "Trade", what: "", input: "Commodity" } as const;
    expect(homeSections([sell, ...FINDERS]).map(([s]) => s)).toEqual(["Trade", "Ship", "Engineering", "Mining"]);
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
