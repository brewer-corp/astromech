import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import type { Component } from "svelte";
import { FINDERS, forgetAnswers, type Finder, type FinderId } from "../../lib/places";
import { DEFAULT_FILTERS } from "../../lib/search";
import { answer, idle, watch } from "../../test/transport";
import FactionStates from "./FactionStates.svelte";
import Hotspots from "./Hotspots.svelte";
import Outfitting from "./Outfitting.svelte";
import RawSites from "./RawSites.svelte";
import Services from "./Services.svelte";
import Shipyard from "./Shipyard.svelte";
import Traders from "./Traders.svelte";

answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

// Each finder opened from a link that names a query other than its default, and the command
// its search sends. Sell and Buy are pinned in Market.browser.test.ts.
const LINKS: { id: FinderId; page: Component<{ finder: Finder }>; hash: string; cmd: string }[] = [
  { id: "outfitting", page: Outfitting, hash: "#/places/outfitting/Pulse%20Laser?class=1&rating=F&mount=Fixed", cmd: "stations_nearby" },
  { id: "services", page: Services, hash: "#/places/services/raw_trader", cmd: "stations_nearby" },
  { id: "shipyard", page: Shipyard, hash: "#/places/shipyard/Anaconda", cmd: "stations_nearby" },
  { id: "material-traders", page: Traders, hash: "#/places/material-traders/encoded", cmd: "stations_nearby" },
  { id: "raw-sites", page: RawSites, hash: "#/places/raw-sites/brain_trees", cmd: "galaxy_sites" },
  { id: "faction-states", page: FactionStates, hash: "#/places/faction-states/military", cmd: "galaxy_nearby" },
  { id: "mining-hotspots", page: Hotspots, hash: "#/places/mining-hotspots/platinum?ring=metallic&reserve=pristine", cmd: "mining_hotspots" },
];

beforeEach(() => forgetAnswers());

afterEach(() => {
  cleanup();
  location.hash = "";
});

const finder = (id: FinderId) => FINDERS.find((f) => f.id === id)!;
const hashChanged = () => new Promise<void>((done) => window.addEventListener("hashchange", () => done(), { once: true }));

describe("A finder opened from a link", () => {
  it.each(LINKS)("$id asks its source once", async ({ id, page, hash, cmd }) => {
    const asked = watch(cmd);
    asked.mockReset();
    location.hash = hash;
    render(page, { finder: finder(id) });
    // The hash change the link made lands after the page has mounted and listens for it.
    await hashChanged();
    await idle();
    expect(asked).toHaveBeenCalledTimes(1);
  });
});

// The two finders whose query is more than one choice set it, then the hash, on a pick.
describe("A pick on a finder's page", () => {
  it("outfitting asks its source once for a new rating", async () => {
    const asked = watch("stations_nearby");
    history.replaceState(null, "", "#/places/outfitting/Frame%20Shift%20Drive?class=5&rating=A");
    render(Outfitting, { finder: finder("outfitting") });
    await idle();
    asked.mockReset();
    const landed = hashChanged();
    await fireEvent.click(screen.getByRole("button", { name: "Rating B" }));
    await landed;
    await idle();
    expect(asked).toHaveBeenCalledTimes(1);
  });

  it("mining-hotspots asks its source once for a new reserve", async () => {
    const asked = watch("mining_hotspots");
    history.replaceState(null, "", "#/places/mining-hotspots/platinum");
    render(Hotspots, { finder: finder("mining-hotspots") });
    await idle();
    asked.mockReset();
    await fireEvent.click(await screen.findByRole("button", { name: "Change filters" }));
    const landed = hashChanged();
    await fireEvent.click(screen.getByRole("button", { name: "Pristine" }));
    await landed;
    await idle();
    expect(asked).toHaveBeenCalledTimes(1);
  });
});
