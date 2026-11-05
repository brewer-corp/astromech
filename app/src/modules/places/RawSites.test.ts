import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { FINDERS } from "../../lib/places";
import { DEFAULT_FILTERS, type Filters } from "../../lib/search";
import type { Sites as SitesAnswer } from "../../lib/sites";
import RawSites from "./RawSites.svelte";

const asked = vi.fn<(kind: string, origin: string | null, filters: Filters | null) => void>();
const loaded = vi.fn<(lookup: string) => void>();
vi.mock("../../lib/search", async (orig) => ({
  ...(await orig<typeof import("../../lib/search")>()),
  loadFilters: (lookup: string) => (loaded(lookup), Promise.resolve(DEFAULT_FILTERS)),
  saveFilters: () => Promise.resolve(),
}));
vi.mock("../../lib/sites", async (orig) => {
  const real = await orig<typeof import("../../lib/sites")>();
  return {
    ...real,
    sites: async (kind: string, origin: string | null, filters: Filters | null): Promise<SitesAnswer> => {
      asked(kind, origin, filters);
      return real.sites(kind, origin, filters);
    },
  };
});

const finder = FINDERS.find((f) => f.id === "raw-sites")!;

/** Each result row's cells, as text. */
const cells = () =>
  screen
    .getAllByRole("row")
    .slice(1)
    .map((r) => [...r.querySelectorAll('[role="cell"]')].map((c) => c.textContent?.replace(/\s+/g, " ").trim()));
const heads = () => screen.getAllByRole("columnheader").map((h) => h.textContent);

afterEach(() => {
  cleanup();
  location.hash = "";
});

describe("Raw sites finder", () => {
  it("lists the nearest crystalline shards with jumps, then switches kind in its main input", async () => {
    location.hash = "#/places/raw-sites";
    render(RawSites, { finder });
    // Ranked by jumps (an estimate off the map); the body without its system, then the system;
    // what the site is; distance and arrival, but no pad.
    expect(await screen.findByText("Oochorrs SB-J d10-5")).toBeTruthy();
    expect(location.hash).toBe("#/places/raw-sites/crystalline_shards");
    expect(heads()).toEqual(["Jumps", "Body", "Site", "Distance", "Arrival", "Updated", ""]);
    const [first] = cells();
    expect(first?.slice(0, 2)).toEqual(["~44", "B 6 a Oochorrs SB-J d10-5"]);
    expect(first?.[2]).toMatch(/^Crystalline Shards · first at -4\.0, -141\.0/);
    expect(first?.slice(3, 5)).toEqual(["1,426.2ly", "26,249ls"]);
    expect(asked).toHaveBeenLastCalledWith("crystalline_shards", null, DEFAULT_FILTERS);
    // The filters Materials remembered for this kind, which its voice query uses too.
    expect(loaded).toHaveBeenLastCalledWith("sites.crystalline_shards");

    const pick = screen.getByRole("combobox", { name: "Site" }) as HTMLSelectElement;
    await fireEvent.change(pick, { target: { value: "raw_farms" } });
    await waitFor(() => expect(location.hash).toBe("#/places/raw-sites/raw_farms"));
    expect(await screen.findByText("Synuefe BY-F d12-39")).toBeTruthy();
    expect(screen.getByText(/Polonium · brain trees/)).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith("raw_farms", null, DEFAULT_FILTERS);
    expect(loaded).toHaveBeenLastCalledWith("sites.raw_farms");
    expect(pick.value).toBe("raw_farms");
    expect(screen.getByText(/^The well-known grade 4 farms/)).toBeTruthy();
  });

  it("opens on the kind in the address", async () => {
    location.hash = "#/places/raw-sites/brain_trees";
    render(RawSites, { finder });
    await waitFor(() => expect(asked).toHaveBeenLastCalledWith("brain_trees", null, DEFAULT_FILTERS));
    expect((screen.getByRole("combobox", { name: "Site" }) as HTMLSelectElement).value).toBe("brain_trees");
  });
});
