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

describe("Powerplay page", () => {
  it("shows the standing from the journal", async () => {
    overview.mockResolvedValue({ standing });
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
    overview.mockResolvedValue({ standing: { ...standing, power: null, rank: null, merits: null, next: null } });
    render(View);
    expect(await screen.findByText(/Not pledged to a power/)).toBeTruthy();
  });
});
