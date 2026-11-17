import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import { endsAt, type Overview } from "../../lib/powerplay";
import { answer } from "../../test/transport";
import View from "./View.svelte";

const NOW = Date.parse("2026-10-16T12:30:00Z");

// The page reads the clock for data ages; the test fixes it (the promises and timeouts stay real).
beforeEach(() => {
  vi.useFakeTimers({ toFake: ["Date"] });
  vi.setSystemTime(NOW);
});
afterEach(() => vi.useRealTimers());

const overview = vi.fn<() => Promise<Overview>>();
answer("powerplay_overview", () => overview());

const standing: Overview["standing"] = {
  power: "Li Yong-Rui",
  rank: 9,
  merits: 47129,
  time_pledged_secs: 22680000,
  session_merits: 1329,
  week_merits: 1829,
  week_ends: new Date(NOW + 3 * 86400000).toISOString(),
  next: { rank: 10, from: 47000, merits: 55000, to_go: 7871, reward: "Care package", note: null },
};

const undermine: Overview["system"] = {
  system: "Piscium Sector EB-X b1-3",
  controlling_power: "Pranav Antal",
  state: "Exploited",
  control_progress: 0.076543,
  reinforcement: 0,
  undermining: 9,
  contesting: ["Li Yong-Rui"],
  acquisition: [],
  work: "undermine",
  summary: "",
  suggestions: [
    { activity: "commit-crimes", name: "Crimes", how: "Murder ships or crew of the controlling power here.", odyssey: false, favoured: true, weak_spot: false, uncertain: null },
    { activity: "salvage", name: "Salvage", how: "Collect salvage here.", odyssey: false, favoured: false, weak_spot: false, uncertain: "Hand-in place not documented." },
  ],
  activity_rules: { version: 1, fetched: "2026-10-16" },
};

describe("Powerplay page", () => {
  it("links the current system to Spansh and Inara", async () => {
    const opened = vi.spyOn(window, "open").mockImplementation(() => null);
    overview.mockResolvedValue({ standing, here: "Deciat", system: null });
    render(View);
    await fireEvent.click(await screen.findByRole("button", { description: "Open Deciat on Spansh" }));
    await fireEvent.click(screen.getByRole("button", { description: "Open Deciat on Inara" }));
    expect(opened.mock.calls.map((c) => c[0])).toEqual([
      "https://spansh.co.uk/search/Deciat",
      "https://inara.cz/elite/starsystem/?search=Deciat",
    ]);
    opened.mockRestore();
  });

  it("shows the standing from the journal", async () => {
    overview.mockResolvedValue({ standing, here: null, system: null });
    render(View);
    expect(await screen.findByText("Li Yong-Rui")).toBeTruthy();
    expect(screen.getByText("47,129")).toBeTruthy();
    expect(screen.getByText("1,829")).toBeTruthy();
    expect(screen.getByText("1,329")).toBeTruthy();
    expect(screen.getByText("7,871")).toBeTruthy();
    expect(screen.getByText(/Care package/)).toBeTruthy();
    expect(screen.getByText(/Pledged 37 weeks/)).toBeTruthy();
  });

  it("after the recorded cycle end, counts to the next one and leaves out the ended cycle's merits", async () => {
    overview.mockResolvedValue({ standing: { ...standing, week_ends: new Date(NOW - 60000).toISOString() }, here: null, system: null });
    render(View);
    const cycle = within(await screen.findByRole("region", { name: "This cycle" }));
    expect(await cycle.findByText("6 d 23 h")).toBeTruthy();
    expect(cycle.getByText(`Ends ${endsAt(new Date(NOW + 7 * 86400000 - 60000).toISOString())}`)).toBeTruthy();
    const merits = within(screen.getByRole("region", { name: "Your merits" }));
    expect(merits.queryByText("1,829")).toBeNull();
    expect(merits.getByText("–")).toBeTruthy();
  });

  it("says so when not pledged", async () => {
    overview.mockResolvedValue({ standing: { ...standing, power: null, rank: null, merits: null, next: null }, here: null, system: null });
    render(View);
    expect(await screen.findByRole("heading", { name: "Not pledged" })).toBeTruthy();
  });

  it("shows the current system's Powerplay state", async () => {
    overview.mockResolvedValue({ standing, here: undermine.system, system: undermine });
    render(View);
    expect(await screen.findByText("Undermining target")).toBeTruthy();
    expect(screen.getByText("Pranav Antal")).toBeTruthy();
    expect(screen.getByText("Exploited")).toBeTruthy();
    expect(screen.getByText("8%")).toBeTruthy();
    expect(within(screen.getByRole("region", { name: "This system" })).getByText(/Li Yong-Rui/)).toBeTruthy();
  });

  it("says nothing about who is ahead while a figure is missing", async () => {
    overview.mockResolvedValue({ standing, here: undermine.system, system: { ...undermine, reinforcement: null } });
    render(View);
    const here = within(await screen.findByRole("region", { name: "This system" }));
    expect(await here.findByText("9")).toBeTruthy();
    expect(here.queryByText(/ahead|Level/)).toBeNull();
  });

  it("shows acquisition progress in an unoccupied system", async () => {
    const acquire = { ...undermine, controlling_power: null, state: "Unoccupied", contesting: ["Li Yong-Rui", "Pranav Antal"], work: "acquire" as const, acquisition: [{ power: "Li Yong-Rui", progress: 0.086375 }, { power: "Pranav Antal", progress: 0.003175 }] };
    overview.mockResolvedValue({ standing, here: acquire.system, system: acquire });
    render(View);
    expect(await screen.findByText("Acquisition target")).toBeTruthy();
    expect(screen.getByText("9%")).toBeTruthy();
    expect(screen.getByText("0%")).toBeTruthy();
  });

  it("says when nothing here earns the power merits", async () => {
    const other = { ...undermine, controlling_power: null, state: "Unoccupied", contesting: ["Felicia Winters"], work: null, acquisition: [{ power: "Felicia Winters", progress: 0.2 }], suggestions: [] };
    overview.mockResolvedValue({ standing, here: other.system, system: other });
    render(View);
    expect(await screen.findByText(/Nothing here earns Li Yong-Rui merits/)).toBeTruthy();
  });

  it("says when the system has no Powerplay", async () => {
    overview.mockResolvedValue({ standing, here: "Test Empty", system: null });
    render(View);
    expect(await screen.findByText("No Powerplay activity here.")).toBeTruthy();
    expect(screen.getByText("Nothing: no Powerplay activity here.")).toBeTruthy();
  });
});
