import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { DEFAULT_FILTERS, type Filters } from "../search";
import type { Sites as SitesAnswer } from "../sites";
import Sites from "./Sites.svelte";

const asked = vi.fn<(kind: string, origin: string | null, filters: Filters | null) => void>();
vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  loadFilters: () => Promise.resolve(DEFAULT_FILTERS),
  saveFilters: () => Promise.resolve(),
}));
vi.mock("../sites", async (orig) => {
  const real = await orig<typeof import("../sites")>();
  return {
    ...real,
    sites: async (kind: string, origin: string | null, filters: Filters | null): Promise<SitesAnswer> => {
      asked(kind, origin, filters);
      return real.sites(kind, origin, filters);
    },
  };
});


/** Each result row's cells, as text. */
const cells = () =>
  screen
    .getAllByRole("row")
    .slice(1)
    .map((r) => [...r.querySelectorAll('[role="cell"]')].map((c) => c.textContent?.replace(/\s+/g, " ").trim()));
const heads = () => screen.getAllByRole("columnheader").map((h) => h.textContent);

describe("Sites", () => {
  it("lists the nearest crystalline shards with jumps, then switches kind", async () => {
    render(Sites);
    // Ranked by jumps (an estimate off the map); the body without its system, then the system;
    // what the site is; distance and arrival, but no pad.
    expect(await screen.findByText("Oochorrs SB-J d10-5")).toBeTruthy();
    expect(heads()).toEqual(["Jumps", "Body", "Site", "Distance", "Arrival", "Updated", ""]);
    const [first] = cells();
    expect(first?.slice(0, 2)).toEqual(["~44", "B 6 a Oochorrs SB-J d10-5"]);
    expect(first?.[2]).toMatch(/^Crystalline Shards · first at -4\.0, -141\.0/);
    expect(first?.slice(3, 5)).toEqual(["1,426.2ly", "26,249ls"]);
    expect(asked).toHaveBeenLastCalledWith("crystalline_shards", null, DEFAULT_FILTERS);

    await fireEvent.click(screen.getByRole("tab", { name: "Known raw farms" }));
    expect(await screen.findByText("Synuefe BY-F d12-39")).toBeTruthy();
    expect(screen.getByText(/Polonium · brain trees/)).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith("raw_farms", null, DEFAULT_FILTERS);
    expect(screen.getByRole("tab", { name: "Known raw farms" }).getAttribute("aria-selected")).toBe("true");
  });
});
