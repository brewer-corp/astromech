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

const rows = () => screen.queryAllByRole("row").slice(1).map((r) => r.textContent ?? "");
const heads = () => screen.getAllByRole("columnheader").map((h) => h.textContent);

describe("Colonies", () => {
  it("lists unclaimed systems in claim range with their bodies, then narrows to rings and Earth-like worlds", async () => {
    render(Colonies);
    // Nearest first, from the recorded search around Greae Phoea EM-K d9-675.
    expect(await screen.findByText("Greae Phoea VS-M b39-29")).toBeTruthy();
    expect(asked).toHaveBeenLastCalledWith("any", null, DEFAULT_FILTERS);
    expect(rows()).toHaveLength(10);
    expect(heads()).toEqual(["Jumps", "System", "Bodies", "Distance", "Updated", ""]);
    expect(rows()[0]).toMatch(/^\s*1\s+Greae Phoea VS-M b39-29/);
    expect(rows()[0]).toContain("9.5ly");
    expect(rows()[0]).toContain("M (Red dwarf) star · 4 landable · 9.5 ly from Greae Phoea EM-K d9-675");
    // Measured from the colony that puts it in range; no bodies scanned yet.
    expect(screen.getByText(/bodies not on Spansh · 6\.2 ly from Greae Phoea EM-K d9-373/)).toBeTruthy();
    // The radius defaults to 30 ly.
    expect(screen.getByRole("option", { name: "30 ly" })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Rings" }));
    await waitFor(() => expect(asked).toHaveBeenLastCalledWith("rings", null, DEFAULT_FILTERS));
    await waitFor(() => expect(rows()).toHaveLength(1));
    expect(rows()[0]).toContain("Greae Phoea KJ-T c19-51");
    expect(rows()[0]).toContain("3 ringed");
    expect(screen.getByRole("button", { name: "Rings", pressed: true })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Earth-like world" }));
    // Nothing within the default 30 ly: say so, and offer the next radius.
    expect(await screen.findByText(/No candidates within 30 ly\./)).toBeTruthy();
    expect(screen.getByRole("button", { name: "Widen to 50 ly" })).toBeTruthy();
  });
});
