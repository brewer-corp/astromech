import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import type { Candidates, Interest } from "../colonies";
import { DEFAULT_FILTERS, type Filters } from "../search";
import Colonies from "./Colonies.svelte";

const asked = vi.fn<(interest: Interest, origin: string | null, filters: Filters | null) => void>();
vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  loadFilters: () => Promise.resolve(DEFAULT_FILTERS),
  saveFilters: () => Promise.resolve(),
}));
vi.mock("../colonies", async (orig) => {
  const real = await orig<typeof import("../colonies")>();
  return {
    ...real,
    candidates: async (interest: Interest, origin: string | null, filters: Filters | null): Promise<Candidates> => {
      asked(interest, origin, filters);
      return real.candidates(interest, origin, filters);
    },
  };
});

const rows = () => screen.getAllByRole("listitem").map((li) => li.textContent ?? "");

describe("Colonies", () => {
  it("lists unclaimed systems in claim range with their bodies, then narrows to rings and Earth-like worlds", async () => {
    render(Colonies);
    // Nearest first, from the recorded search around Greae Phoea EM-K d9-675.
    expect(await screen.findByText("Greae Phoea VS-M b39-29")).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith("any", null, DEFAULT_FILTERS);
    expect(rows()).toHaveLength(10);
    expect(rows()[0]).toContain("1 jump · 9.5 ly");
    expect(rows()[0]).toContain("M (Red dwarf) star · 4 landable · 9.5 ly from Greae Phoea EM-K d9-675");
    // Measured from the colony that puts it in range; no bodies scanned yet.
    expect(screen.getByText(/bodies not on Spansh · 6\.2 ly from Greae Phoea EM-K d9-373/)).toBeTruthy();
    // The radius defaults to 30 ly.
    expect(screen.getByRole("option", { name: "30 ly" })).toBeTruthy();

    await fireEvent.click(screen.getByRole("tab", { name: "Rings" }));
    await waitFor(() => expect(asked).toHaveBeenLastCalledWith("rings", null, DEFAULT_FILTERS));
    await waitFor(() => expect(rows()).toHaveLength(1));
    expect(rows()[0]).toContain("Greae Phoea KJ-T c19-51");
    expect(rows()[0]).toContain("3 ringed");
    expect(screen.getByRole("tab", { name: "Rings" }).getAttribute("aria-selected")).toBe("true");

    await fireEvent.click(screen.getByRole("tab", { name: "Earth-like world" }));
    expect(await screen.findByText("None found.")).toBeTruthy();
  });
});
