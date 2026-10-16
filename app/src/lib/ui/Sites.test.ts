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

describe("Sites", () => {
  it("lists the nearest crystalline shards with jumps, then switches kind", async () => {
    render(Sites);
    // the body without its system, then the system chip; jumps are an estimate off the map
    expect(await screen.findByText("Oochorrs SB-J d10-5")).toBeTruthy();
    expect(screen.getAllByText(/^B 6 a ·/).length).toBe(1);
    expect(screen.getAllByText("~44 jumps · 1426.2 ly").length).toBe(1);
    expect(screen.getByText(/26k ls · Crystalline Shards · first at -4\.0, -141\.0/)).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith("crystalline_shards", null, DEFAULT_FILTERS);

    await fireEvent.click(screen.getByRole("tab", { name: "Known raw farms" }));
    expect(await screen.findByText("Synuefe BY-F d12-39")).toBeTruthy();
    expect(screen.getByText(/Polonium · brain trees/)).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith("raw_farms", null, DEFAULT_FILTERS);
    expect(screen.getByRole("tab", { name: "Known raw farms" }).getAttribute("aria-selected")).toBe("true");
  });
});
