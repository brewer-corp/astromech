import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen, waitFor } from "@testing-library/svelte";
import type { Component } from "svelte";
import { resetFakeCore } from "../fakecore";
import Ships from "../../modules/engineering/View.svelte";
import Materials from "../../modules/materials/View.svelte";
import OnFoot from "../../modules/odyssey/View.svelte";
import Logbook from "../../modules/logbook/View.svelte";
import Session from "../../modules/session/View.svelte";
import Mining from "../../modules/mining/View.svelte";
import Station from "../../modules/station/View.svelte";
import Colonisation from "../../modules/colonisation/View.svelte";
import Exploration from "../../modules/exploration/View.svelte";
import Exobiology from "../../modules/exobiology/View.svelte";
import Powerplay from "../../modules/powerplay/View.svelte";
import DataSharing from "../../modules/datasharing/View.svelte";
import Diagnostics from "../../modules/diagnostics/View.svelte";

// Every polling page, after a first good read (astromech-spxx): a later failed poll keeps the data
// and adds "Could not refresh: <reason>."; the next good poll clears the line. The preview reads
// `?fail=` on each call, so changing the URL between polls makes the same page fail and recover.
Element.prototype.scrollIntoView = () => {};

const at = (query: string) => history.replaceState(null, "", `/${query}`);

beforeEach(() => {
  at("");
  resetFakeCore();
  // Only the poll timers: the pages' own timeouts and the promises stay real.
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
  at("");
});

interface Page {
  name: string;
  view: Component;
  /** Text of the sample data, on screen after the first read. */
  data: RegExp;
  /** The call the page polls. */
  call: string;
  /** The longest poll interval on the page, ms. */
  every: number;
}

const PAGES: Page[] = [
  { name: "Ships", view: Ships as unknown as Component, data: /Heavy Lifter/, call: "engineering_fleet", every: 1000 },
  { name: "Materials", view: Materials as unknown as Component, data: /Carbon/, call: "materials_inventory", every: 1000 },
  { name: "On foot", view: OnFoot as unknown as Component, data: /Sample ShipLocker/, call: "odyssey_view", every: 2000 },
  { name: "Logbook", view: Logbook as unknown as Component, data: /CMDR Fixture/, call: "logbook", every: 5000 },
  { name: "Session", view: Session as unknown as Component, data: /CMDR Fixture/, call: "session_summary", every: 5000 },
  { name: "Mining", view: Mining as unknown as Component, data: /Platinum/, call: "core_status", every: 3000 },
  { name: "Station", view: Station as unknown as Component, data: /Test Orbital/, call: "docked_here", every: 5000 },
  { name: "Colonisation", view: Colonisation as unknown as Component, data: /Test Hub/, call: "colonisation_projects", every: 2000 },
  { name: "Exploration", view: Exploration as unknown as Component, data: /bodies scanned/, call: "exploration_system", every: 15000 },
  { name: "Exobiology", view: Exobiology as unknown as Component, data: /Stratum Tectonicas/, call: "exobiology", every: 2000 },
  { name: "Powerplay", view: Powerplay as unknown as Component, data: /Li Yong-Rui/, call: "powerplay_overview", every: 2000 },
  { name: "Data sharing", view: DataSharing as unknown as Component, data: /Waiting to send/, call: "eddn_log", every: 2000 },
  { name: "Diagnostics", view: Diagnostics as unknown as Component, data: /Waiting to send|1 waiting/, call: "eddn_log", every: 2000 },
];

describe.each(PAGES)("$name", (p) => {
  it("keeps its data and says the refresh failed, then clears the line on the next good read", async () => {
    render(p.view);
    await waitFor(() => expect(document.body.textContent).toMatch(p.data));
    expect(screen.queryByText(/Could not/)).toBeNull();

    at(`?fail=${p.call}`);
    await vi.advanceTimersByTimeAsync(p.every);
    const line = await screen.findByText(/Could not refresh: .*failed on purpose/);
    expect(line.textContent).toMatch(/^! Could not refresh: .+\.$/);
    // The last good data is still there, and the page did not fall back to its first-read error.
    expect(document.body.textContent).toMatch(p.data);
    expect(screen.queryByText(/Could not read/)).toBeNull();

    at("");
    await vi.advanceTimersByTimeAsync(p.every);
    await waitFor(() => expect(screen.queryByText(/Could not/)).toBeNull());
    expect(document.body.textContent).toMatch(p.data);
  });
});
