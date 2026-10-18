import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import type { Component } from "svelte";
import { resetFakeCore } from "../fakecore";
import Ships from "../../modules/engineering/View.svelte";
import Materials from "../../modules/materials/View.svelte";
import OnFoot from "../../modules/odyssey/View.svelte";
import Engineers from "../../modules/unlocks/View.svelte";
import Logbook from "../../modules/logbook/View.svelte";
import Session from "../../modules/session/View.svelte";
import Mining from "../../modules/mining/View.svelte";
import Trade from "../../modules/trade/View.svelte";
import Station from "../../modules/station/View.svelte";
import Colonisation from "../../modules/colonisation/View.svelte";
import Exploration from "../../modules/exploration/View.svelte";
import Powerplay from "../../modules/powerplay/View.svelte";
import DataSharing from "../../modules/datasharing/View.svelte";
import Diagnostics from "../../modules/diagnostics/View.svelte";

// Each fixture-backed page, seen the way the browser preview shows it: with no history
// (`?core=empty`), with a call that never answers (`?slow=never`) and with every call failing
// (`?fail=all`). A page shows exactly one of those, never the sample, a stuck "Reading" or a
// bare error.
Element.prototype.scrollIntoView = () => {};

const at = (query: string) => {
  history.replaceState(null, "", `/${query}`);
  resetFakeCore();
};
afterEach(() => {
  cleanup();
  at("");
});

interface Page {
  name: string;
  view: Component;
  /** The no-data text, under ?core=empty. */
  empty: RegExp;
  /** The waiting text, while the call is pending. */
  loading: RegExp;
  /** What a failed call says. */
  failed: RegExp;
}

const PAGES: Page[] = [
  { name: "Ships", view: Ships as unknown as Component, empty: /No ships in the journal yet/, loading: /Reading the journal…/, failed: /Could not read the fleet: / },
  { name: "Materials", view: Materials as unknown as Component, empty: /0 \/ 6,300 · No journal yet/, loading: /Reading the journal…/, failed: /Could not read the materials: / },
  { name: "On foot", view: OnFoot as unknown as Component, empty: /Nothing held here/, loading: /Reading the ShipLocker…/, failed: /Could not read the inventory: / },
  { name: "Engineers", view: Engineers as unknown as Component, empty: /^0 of \d+ unlocked/, loading: /Reading the journal…/, failed: /Could not read the engineer list: / },
  { name: "Logbook", view: Logbook as unknown as Component, empty: /No commander in the journal history yet/, loading: /Reading the journal history…/, failed: /Could not read the journal history: / },
  { name: "Session", view: Session as unknown as Component, empty: /No game session in the journal history yet/, loading: /Reading the journal history…/, failed: /Could not read the session: / },
  { name: "Mining", view: Mining as unknown as Component, empty: /No mining run yet/, loading: /Reading the run…/, failed: /Could not read the mining run: / },
  { name: "Trade", view: Trade as unknown as Component, empty: /None found/, loading: /Asking Ardent Insight…/, failed: /trade_search failed on purpose/ },
  { name: "Station", view: Station as unknown as Component, empty: /Not docked/, loading: /Reading the journal…/, failed: /Could not read the docked station: / },
  { name: "Colonisation", view: Colonisation as unknown as Component, empty: /No construction sites in your journals yet/, loading: /Reading the journals…/, failed: /Could not read the construction sites: / },
  { name: "Exploration", view: Exploration as unknown as Component, empty: /No system in the journal yet/, loading: /Reading the journal…/, failed: /Could not read the system: / },
  { name: "Powerplay", view: Powerplay as unknown as Component, empty: /Not pledged to a power/, loading: /Reading the journal…/, failed: /Could not read your Powerplay standing: / },
  { name: "Data sharing", view: DataSharing as unknown as Component, empty: /Nothing yet/, loading: /Reading the sharing log…/, failed: /Could not read the sharing log: / },
  { name: "Diagnostics", view: Diagnostics as unknown as Component, empty: /Nothing sent to EDDN yet/, loading: /^—$/, failed: /Could not read the EDDN log: / },
];

describe.each(PAGES)("$name", (p) => {
  it("shows its no-data text under ?core=empty", async () => {
    at("?core=empty");
    render(p.view);
    expect((await screen.findAllByText(p.empty)).length).toBeGreaterThan(0);
    expect(screen.queryByText(/Sample|CMDR Fixture/)).toBeNull();
  });

  it("says it is waiting while a call never answers", async () => {
    at("?slow=never");
    render(p.view);
    expect((await screen.findAllByText(p.loading)).length).toBeGreaterThan(0);
    // Still waiting once the first read has had time to answer (Logbook debounces 200 ms): the call
    // really is pending. Diagnostics shows its dash by default, so this cannot fail for it.
    await new Promise((r) => setTimeout(r, 300));
    expect(screen.queryAllByText(p.loading).length).toBeGreaterThan(0);
    expect(screen.queryByText(p.empty)).toBeNull();
    expect(screen.queryByText(/Could not|failed on purpose/)).toBeNull();
  });

  it("says what failed, with no stuck waiting text", async () => {
    at("?fail=all");
    render(p.view);
    expect((await screen.findAllByText(p.failed)).length).toBeGreaterThan(0);
    // The failure is the answer: neither the sample nor a waiting line stays on screen.
    if (p.name !== "Diagnostics") expect(screen.queryAllByText(p.loading).filter((el) => !p.failed.test(el.textContent ?? ""))).toHaveLength(0);
  });
});
