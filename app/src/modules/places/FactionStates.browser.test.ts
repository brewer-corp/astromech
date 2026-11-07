import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { FINDERS } from "../../lib/places";
import { DEFAULT_FILTERS } from "../../lib/search";
import { answer, watch } from "../../test/transport";
import FactionStates from "./FactionStates.svelte";

const asked = watch("galaxy_nearby");
const loaded = watch("search_filters");
answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

const finder = FINDERS.find((f) => f.id === "faction-states")!;
const heads = () => screen.getAllByRole("columnheader").map((h) => h.textContent);

afterEach(() => {
  cleanup();
  location.hash = "";
});

describe("Faction states finder", () => {
  it("opens on Boom, ranked by the HGE score, and switches to Famine, ranked by jumps", async () => {
    location.hash = "#/places/faction-states";
    render(FactionStates, { finder });
    expect(await screen.findByText("Tau Ceti")).toBeTruthy();
    expect(location.hash).toBe("#/places/faction-states/proto");
    expect(asked).toHaveBeenLastCalledWith({ query: "proto", origin: null, filters: DEFAULT_FILTERS });
    // The filters Materials remembered for the lookup, which its voice query uses too.
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "faction.proto" });
    expect(heads()).toEqual(["Score", "System", "Population", "Faction", "Distance", "Jumps", "Updated", ""]);
    expect(screen.getByText("High Grade Emissions for Proto materials.")).toBeTruthy();

    const pick = screen.getByRole("combobox", { name: "State" }) as HTMLSelectElement;
    expect([...pick.options].map((o) => o.value)).toEqual(["fed_composites", "empire_shielding", "proto", "military", "improvised", "pharma", "famine"]);
    await fireEvent.change(pick, { target: { value: "famine" } });
    await waitFor(() => expect(location.hash).toBe("#/places/faction-states/famine"));
    expect(await screen.findByText("WISE 0855-0714")).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith({ query: "famine", origin: null, filters: DEFAULT_FILTERS });
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "faction.famine" });
    await waitFor(() => expect(heads()).toEqual(["Jumps", "System", "Faction", "Distance", "Updated", ""]));
  });

  it("opens on the lookup in the address, and an unknown one as Boom", async () => {
    location.hash = "#/places/faction-states/military";
    render(FactionStates, { finder });
    await waitFor(() => expect(asked).toHaveBeenLastCalledWith({ query: "military", origin: null, filters: DEFAULT_FILTERS }));
    cleanup();
    location.hash = "#/places/faction-states/nope";
    render(FactionStates, { finder });
    await waitFor(() => expect(location.hash).toBe("#/places/faction-states/proto"));
  });
});
