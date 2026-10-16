import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import sample from "../../lib/fixtures/powerplay-targets.json";
import type { Target, Targets as TargetsResult } from "../../lib/powerplay";
import Targets from "./Targets.svelte";

const load = vi.fn<(t: Target, origin: string | null, filters: unknown, within: number) => Promise<TargetsResult>>();
vi.mock("../../lib/powerplay", async (orig) => ({
  ...(await orig<typeof import("../../lib/powerplay")>()),
  loadTargets: (t: Target, o: string | null, f: unknown, w: number) => load(t, o, f, w),
}));

const fixtures = sample as unknown as Record<Target, TargetsResult>;

describe("Where to work", () => {
  it("lists the pledged power's targets nearest first and re-asks on kind and range", async () => {
    load.mockImplementation(async (t) => fixtures[t]);
    render(Targets);
    expect(await screen.findByText("Piscium Sector YJ-A b2")).toBeTruthy();
    // The pledged power isn't listed as "also" in its own targets.
    expect(screen.getByText(/Unoccupied, acquisition 32%/).textContent).not.toContain("also");
    expect(screen.getByText(/Unoccupied, acquisition 9% · also Pranav Antal/)).toBeTruthy();
    expect(load).toHaveBeenLastCalledWith("acquire", null, expect.anything(), 40);

    await fireEvent.click(screen.getByRole("button", { name: "Reinforce" }));
    expect(await screen.findByText("Lembava")).toBeTruthy();
    expect(screen.getByText(/Li Yong-Rui stronghold 25% · reinforcement 0 \/ undermining 166/)).toBeTruthy();
    expect(load).toHaveBeenLastCalledWith("reinforce", null, expect.anything(), 40);

    await fireEvent.change(screen.getByRole("combobox", { name: /Within/ }), { target: { value: "80" } });
    await waitFor(() => expect(load).toHaveBeenLastCalledWith("reinforce", null, expect.anything(), 80));

    await fireEvent.click(screen.getByRole("button", { name: "Undermine" }));
    expect(await screen.findByText("Piscium Sector EB-X b1-3")).toBeTruthy();
    expect(screen.getByText(/Pranav Antal exploited 8%/)).toBeTruthy();
  });
});
