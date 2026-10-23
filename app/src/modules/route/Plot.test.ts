import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import type { PlotSetup, Plotted } from "../../lib/route";
import Plot from "./Plot.svelte";

// The range line shows the ship and cargo a plot would use now (astromech-83og): a ship swap or
// a cargo change after the page opened shows up, and the result says the range it was plotted at.
const calls = vi.hoisted(() => ({ setup: vi.fn(), plot: vi.fn() }));
vi.mock("../../lib/route", async (orig) => ({ ...(await orig<typeof import("../../lib/route")>()), plotSetup: calls.setup, plotRoute: calls.plot }));

const asp: PlotSetup = { here: "Sol", ship: { name: "Asp Explorer", range: 39.87, cargo: 0, unresolved: [] }, ship_error: null };
const laden: PlotSetup = { here: "Sol", ship: { name: "Asp Explorer", range: 31.2, cargo: 64, unresolved: [] }, ship_error: null };
const plotted: Plotted = { name: "Sol to Maia", source: "exact plotter", waypoints: [], jumps: 12, ly: 452, neutrons: 0, refuels: 2, range: 31.2, local: false };
const VIEW = { route: null, here: "Sol", next: null, remaining: 0, clipboard_error: null, parked: null };
const TO = "Maia";

beforeEach(() => {
  calls.setup.mockReset();
  calls.plot.mockReset();
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

describe("Route plotter range line", () => {
  it("follows a cargo change while the page is open", async () => {
    calls.setup.mockResolvedValue(asp);
    render(Plot, { to: TO, onfollow: () => {} });
    expect(await screen.findByText(/Asp Explorer: 39\.87 ly a jump with a full tank, from/)).toBeTruthy();

    calls.setup.mockResolvedValue(laden);
    await vi.advanceTimersByTimeAsync(5000);
    expect(await screen.findByText(/Asp Explorer: 31\.20 ly a jump with a full tank and 64 t of cargo/)).toBeTruthy();
  });

  it("re-reads the ship after a plot and says the range the route was plotted at", async () => {
    calls.setup.mockResolvedValueOnce(asp).mockResolvedValue(laden);
    calls.plot.mockResolvedValue({ plotted, view: VIEW });
    render(Plot, { to: TO, onfollow: () => {} });
    await screen.findByText(/39\.87 ly a jump/);
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText("Following Sol to Maia: 12 jumps · 452 ly · 2 refuel stops, at 31.20 ly a jump. Plotted by Spansh.")).toBeTruthy();
    expect(await screen.findByText(/31\.20 ly a jump with a full tank and 64 t of cargo/)).toBeTruthy();
  });
});

describe("Route plotter, where the route was plotted", () => {
  it("says Spansh plotted a neutron route because the neutron star file can't be read", async () => {
    calls.setup.mockResolvedValue(asp);
    const fallback: Plotted = {
      ...plotted,
      name: "Sol to Colonia",
      source: "neutron plotter",
      jumps: 128,
      ly: 22000,
      neutrons: 1,
      refuels: 0,
      range: 39.87,
      fallback: "neutron star file is cut short (100 bytes)",
    };
    calls.plot.mockResolvedValue({ plotted: fallback, view: VIEW });
    render(Plot, { to: TO, onfollow: () => {} });
    await screen.findByText(/39\.87 ly a jump/);
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText("Following Sol to Colonia: 128 jumps · 22,000 ly · 1 neutron boost, at 39.87 ly a jump. Plotted by Spansh.")).toBeTruthy();
    expect(screen.getByText("The neutron star file can't be read, so Spansh plotted this route: neutron star file is cut short (100 bytes)")).toBeTruthy();
  });

  it("says a local route is near the fewest jumps when the search didn't prove it", async () => {
    calls.setup.mockResolvedValue(asp);
    const local: Plotted = { ...plotted, name: "Sol to Colonia", source: "neutron plotter (this PC)", jumps: 180, ly: 22000, neutrons: 149, refuels: 0, range: 39.87, local: true, fewest: false };
    calls.plot.mockResolvedValueOnce({ plotted: local, view: VIEW }).mockResolvedValueOnce({ plotted: { ...local, fewest: true }, view: VIEW });
    render(Plot, { to: TO, onfollow: () => {} });
    await screen.findByText(/39\.87 ly a jump/);
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText(/at 39\.87 ly a jump\. Plotted on this PC, near the fewest jumps\.$/)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Plot and follow" }));
    expect(await screen.findByText(/at 39\.87 ly a jump\. Plotted on this PC\.$/)).toBeTruthy();
  });
});
