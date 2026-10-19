import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { fakeCall, resetFakeCore } from "../fakecore";
import { DEFAULT_FILTERS } from "../search";
import Nearby from "./Nearby.svelte";

vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  loadFilters: () => Promise.resolve(DEFAULT_FILTERS),
  saveFilters: () => Promise.resolve(),
}));

describe("Nearby", () => {
  it("shows the score and population an HGE lookup is ranked by", async () => {
    render(Nearby, { query: "fed_composites" });
    expect(await screen.findByText("Tau Ceti")).toBeTruthy();
    const lines = screen.getAllByText(/^score \d+ · pop /).map((e) => e.textContent ?? "");
    expect(lines[0]).toMatch(/^score 109 · pop 5\.0B · /);
    expect(lines[1]).toMatch(/^score 0 · pop 2M · /);
    expect(screen.getByText(/ranked by population and distance/)).toBeTruthy();
  });

  it("keeps faction-state lookups without a score", async () => {
    render(Nearby, { query: "famine" });
    expect(await screen.findByText("WISE 0855-0714")).toBeTruthy();
    expect(screen.queryByText(/^score /)).toBeNull();
  });

  it("says when a system's states come from the commander's own visit", async () => {
    render(Nearby, { query: "famine" });
    expect(await screen.findByText("Ross 154")).toBeTruthy();
    const visited = screen.getAllByText(/seen on your visit/);
    expect(visited).toHaveLength(1);
    expect(visited[0]?.textContent).toMatch(/Ross 154 Boys \(Independent, Famine\) · seen on your visit/);
  });

  it("Go puts a faction-state system on the route follower", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(Nearby, { query: "famine" });
    await fireEvent.click(await screen.findByRole("button", { name: "Go to Ross 154" }));
    expect(await screen.findByText("Following. Ross 154 is on your clipboard.")).toBeTruthy();
    const { view } = await fakeCall("route");
    expect(view.route).toMatchObject({ name: "Ross 154", source: "search", next: 0 });
    expect(view.route.waypoints).toEqual([{ system: "Ross 154", label: "Famine (distribution centres)" }]);
    history.replaceState(null, "", "/");
  });
});
