import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { DEFAULT_FILTERS } from "../search";
import Nearby from "./Nearby.svelte";
import { answer, previewAnswer, startPreview } from "../../test/transport";

answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

/** Each result row's cells, as text. */
const cells = () =>
  screen
    .getAllByRole("row")
    .slice(1)
    .map((r) => [...r.querySelectorAll('[role="cell"]')].map((c) => c.textContent?.replace(/\s+/g, " ").trim()));
const heads = () => screen.getAllByRole("columnheader").map((h) => h.textContent);

describe("Nearby", () => {
  it("leads an HGE lookup with the score it is ranked by, then the population it weighs", async () => {
    render(Nearby, { query: "fed_composites" });
    expect(await screen.findByText("Tau Ceti")).toBeTruthy();
    expect(heads()).toEqual(["Score", "System", "Population", "Faction", "Distance", "Jumps", "Updated", ""]);
    expect(cells()[0]?.slice(0, 3)).toEqual(["109", "Tau Ceti", "5.0B"]);
    expect(cells()[1]?.[0]).toBe("0");
    expect(cells()[1]?.[2]).toBe("2M");
    expect(screen.getByText(/ranked by population and distance/)).toBeTruthy();
  });

  it("ranks faction-state lookups by jumps, with no score", async () => {
    render(Nearby, { query: "famine" });
    expect(await screen.findByText("WISE 0855-0714")).toBeTruthy();
    expect(heads()).toEqual(["Jumps", "System", "Faction", "Distance", "Updated", ""]);
  });

  it("says when a system's states come from the commander's own visit", async () => {
    render(Nearby, { query: "famine" });
    expect(await screen.findByText("Ross 154")).toBeTruthy();
    const visited = screen.getAllByText(/seen on your visit/);
    expect(visited).toHaveLength(1);
    expect(visited[0]?.textContent).toMatch(/Ross 154 Boys \(Independent, Famine, Civil Unrest\) · seen on your visit/);
  });

  it("Set destination puts a faction-state system on the route follower", async () => {
    startPreview("/?core=ready");
    render(Nearby, { query: "famine" });
    await fireEvent.click(await screen.findByRole("button", { name: "Set Ross 154 as the destination" }));
    expect(await screen.findByText("Following. Ross 154 is on your clipboard.")).toBeTruthy();
    const { view } = await previewAnswer("route");
    expect(view.route).toMatchObject({ name: "Ross 154", source: "search", next: 0 });
    expect(view.route.waypoints).toEqual([{ system: "Ross 154", label: "Famine (distribution centres)" }]);
    history.replaceState(null, "", "/");
  });
});
