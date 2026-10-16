import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import type { Overview } from "../../lib/powerplay";
import View from "./View.svelte";

const overview = vi.fn<() => Promise<Overview>>();
vi.mock("../../lib/powerplay", async (orig) => ({
  ...(await orig<typeof import("../../lib/powerplay")>()),
  loadOverview: () => overview(),
}));

const standing: Overview["standing"] = {
  power: "Li Yong-Rui",
  rank: 9,
  merits: 47129,
  time_pledged_secs: 22680000,
  session_merits: 1329,
  week_merits: 1829,
  week_ends: new Date(Date.now() + 3 * 86400000).toISOString(),
  next: { rank: 10, merits: 55000, to_go: 7871, reward: "Care package", note: null },
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
};

describe("Powerplay page", () => {
  it("shows the standing from the journal", async () => {
    overview.mockResolvedValue({ standing, here: null, system: null });
    render(View);
    expect(await screen.findByText("Li Yong-Rui")).toBeTruthy();
    expect(screen.getByText("47,129")).toBeTruthy();
    expect(screen.getByText("1,829")).toBeTruthy();
    expect(screen.getByText("1,329")).toBeTruthy();
    expect(screen.getByText("7,871")).toBeTruthy();
    expect(screen.getByText(/Care package/)).toBeTruthy();
    expect(screen.getByText("262 days")).toBeTruthy();
  });

  it("says so when not pledged", async () => {
    overview.mockResolvedValue({ standing: { ...standing, power: null, rank: null, merits: null, next: null }, here: null, system: null });
    render(View);
    expect(await screen.findByText(/Not pledged to a power/)).toBeTruthy();
  });

  it("shows the current system's Powerplay state", async () => {
    overview.mockResolvedValue({ standing, here: undermine.system, system: undermine });
    render(View);
    expect(await screen.findByText("Undermining target")).toBeTruthy();
    expect(screen.getByText("Pranav Antal")).toBeTruthy();
    expect(screen.getByText("Exploited")).toBeTruthy();
    expect(screen.getByText("8%")).toBeTruthy();
    expect(screen.getByText("Li Yong-Rui", { selector: "dd" })).toBeTruthy();
  });

  it("shows acquisition progress in an unoccupied system", async () => {
    const acquire = { ...undermine, controlling_power: null, state: "Unoccupied", contesting: ["Li Yong-Rui", "Pranav Antal"], work: "acquire" as const, acquisition: [{ power: "Li Yong-Rui", progress: 0.086375 }, { power: "Pranav Antal", progress: 0.003175 }] };
    overview.mockResolvedValue({ standing, here: acquire.system, system: acquire });
    render(View);
    expect(await screen.findByText("Acquisition target")).toBeTruthy();
    expect(screen.getByText("9%")).toBeTruthy();
    expect(screen.getByText("0%")).toBeTruthy();
  });

  it("says when the system has no Powerplay", async () => {
    overview.mockResolvedValue({ standing, here: "Test Empty", system: null });
    render(View);
    expect(await screen.findByText("No Powerplay activity here.")).toBeTruthy();
  });
});
