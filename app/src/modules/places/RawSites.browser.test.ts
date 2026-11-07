import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { FINDERS } from "../../lib/places";
import { DEFAULT_FILTERS } from "../../lib/search";
import { answer, watch } from "../../test/transport";
import RawSites from "./RawSites.svelte";

const asked = watch("galaxy_sites");
const loaded = watch("search_filters");
answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

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
    expect(first?.slice(0, 2)).toEqual(["~46", "B 6 a Oochorrs SB-J d10-5"]);
    expect(first?.[2]).toMatch(/^Crystalline Shards · first at -4\.0, -141\.0/);
    expect(first?.slice(3, 5)).toEqual(["1,426.2ly", "26,249ls"]);
    expect(asked).toHaveBeenLastCalledWith({ kind: "crystalline_shards", origin: null, filters: DEFAULT_FILTERS });
    // The filters Materials remembered for this kind, which its voice query uses too.
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "sites.crystalline_shards" });

    const pick = screen.getByRole("combobox", { name: "Site" }) as HTMLSelectElement;
    await fireEvent.change(pick, { target: { value: "raw_farms" } });
    await waitFor(() => expect(location.hash).toBe("#/places/raw-sites/raw_farms"));
    expect(await screen.findByText("Synuefe BY-F d12-39")).toBeTruthy();
    expect(await screen.findByText(/Polonium · brain trees/)).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith({ kind: "raw_farms", origin: null, filters: DEFAULT_FILTERS });
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "sites.raw_farms" });
    expect(pick.value).toBe("raw_farms");
    expect(screen.getByText(/^The well-known grade 4 farms/)).toBeTruthy();
  });

  it("opens on the kind in the address", async () => {
    location.hash = "#/places/raw-sites/brain_trees";
    render(RawSites, { finder });
    await waitFor(() => expect(asked).toHaveBeenLastCalledWith({ kind: "brain_trees", origin: null, filters: DEFAULT_FILTERS }));
    expect((screen.getByRole("combobox", { name: "Site" }) as HTMLSelectElement).value).toBe("brain_trees");
  });
});
