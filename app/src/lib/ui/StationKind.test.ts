import { describe, expect, it } from "vitest";
import { stationGlyph } from "./StationKind.svelte";

// docked_here gives the journal's StationType, the finders Spansh's type name: both draw the same icon.
describe("the station type icon", () => {
  it.each([
    ["Coriolis", "Coriolis Starport", "coriolis"],
    ["Orbis", "Orbis Starport", "orbis"],
    ["Bernal", "Ocellus Starport", "ocellus"],
    ["Outpost", "Outpost", "outpost"],
    ["CraterOutpost", "Planetary Outpost", "planetary"],
    ["CraterPort", "Planetary Port", "planetary"],
    ["AsteroidBase", "Asteroid base", "asteroid-base"],
    ["FleetCarrier", "Drake-Class Carrier", "carrier"],
    ["MegaShip", "Mega ship", "megaship"],
    ["OnFootSettlement", "Odyssey Settlement", "settlement"],
  ])("%s and %s: %s", (journal, spansh, glyph) => {
    expect(stationGlyph(journal)).toBe(glyph);
    expect(stationGlyph(spansh)).toBe(glyph);
  });
});
