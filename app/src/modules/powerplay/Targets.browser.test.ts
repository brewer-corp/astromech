import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import sample from "../../preview/fixtures/powerplay-targets.json";
import type { Target, Targets as TargetsResult } from "../../lib/powerplay";
import Targets from "./Targets.svelte";

const load = vi.fn<(t: Target, origin: string | null, filters: unknown, within: number) => Promise<TargetsResult>>();
vi.mock("../../lib/powerplay", async (orig) => ({
  ...(await orig<typeof import("../../lib/powerplay")>()),
  loadTargets: (t: Target, o: string | null, f: unknown, w: number) => load(t, o, f, w),
}));

const fixtures = sample as unknown as Record<Target, TargetsResult>;

afterEach(() => vi.useRealTimers());

describe("Where to work", () => {
  it("lists the pledged power's targets nearest first and re-asks on kind and range", async () => {
    load.mockImplementation(async (t) => fixtures[t]);
    render(Targets);
    expect(await screen.findByText("Piscium Sector YJ-A b2")).toBeTruthy();
    // The pledged power isn't listed as "also" in its own targets.
    expect(screen.getByText(/Unoccupied, acquisition 32%/).textContent).not.toContain("other");
    // The other powers are counted on the row, so its line stays whole, and named in its Tip.
    const contested = screen.getByText("Unoccupied, acquisition 9% · 1 other power");
    expect(document.getElementById(contested.getAttribute("aria-describedby") ?? "")?.textContent).toBe("Also active: Pranav Antal");
    expect(load).toHaveBeenLastCalledWith("acquire", null, expect.anything(), 40);

    await fireEvent.click(screen.getByRole("button", { name: "Reinforce" }));
    expect(await screen.findByText("Lembava")).toBeTruthy();
    const lembava = screen.getByText("Lembava").closest('[role="row"]') as HTMLElement;
    expect(within(lembava).getByText("Stronghold 25%")).toBeTruthy();
    expect(within(lembava).getByText("166")).toBeTruthy();
    expect(load).toHaveBeenLastCalledWith("reinforce", null, expect.anything(), 40);

    // The radius is a filter: named in the summary, its control behind CHANGE.
    expect(await screen.findByText(/^Within 40 ly of /)).toBeTruthy();
    expect(screen.queryByRole("combobox", { name: /Within/ })).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    await fireEvent.change(screen.getByRole("combobox", { name: /Within/ }), { target: { value: "80" } });
    await waitFor(() => expect(load).toHaveBeenLastCalledWith("reinforce", null, expect.anything(), 80));
    expect(await screen.findByText(/^Within 80 ly of /)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Done" }));
    expect(screen.queryByRole("combobox", { name: /Within/ })).toBeNull();

    await fireEvent.click(screen.getByRole("button", { name: "Undermine" }));
    expect(await screen.findByText("Piscium Sector EB-X b1-3")).toBeTruthy();
    expect(screen.getByText(/Pranav Antal exploited 8%/)).toBeTruthy();
  });

  it("keeps each report's age on the row, which has no side panel to hold it", async () => {
    // Twelve days after the first target's report.
    vi.useFakeTimers({ toFake: ["Date"] });
    vi.setSystemTime(Date.parse(fixtures.acquire.hits[0]!.updated_at!) + 12 * 86400000);
    load.mockImplementation(async (t) => fixtures[t]);
    render(Targets);
    const row = (await screen.findByText("Piscium Sector YJ-A b2")).closest('[role="row"]') as HTMLElement;
    const age = within(row).getByText("12 d old");
    // Fresh: grey, not the Orange of a report past 30 days.
    expect(age.classList.contains("old")).toBe(false);
    expect(row.classList.contains("stale")).toBe(false);
  });
});
