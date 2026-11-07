import { describe, expect, it } from "vitest";
import { render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";
import { millions, shortName, valueRange } from "../../lib/exobiology";

// Outside the app the page shows the synthetic sample system (the core's view of a replayed
// system: two bodies, one mapped, a Stratum being sampled, and what others reported).
describe("Biology", () => {
  it("ranks the system's bodies by value with their genera", async () => {
    render(View);
    const table = await screen.findByRole("table", { name: "Bodies with biology" });
    const rows = within(table).getAllByRole("row").slice(1);
    expect(rows).toHaveLength(2);
    expect(rows[0]!.textContent).toContain("A 2");
    expect(rows[0]!.textContent).toContain("Stratum");
    expect(rows[0]!.textContent).toContain("Tectonicas");
    expect(rows[0]!.textContent).toContain("20 M");
    expect(rows[0]!.textContent).toContain("From the DSS");
    expect(rows[1]!.textContent).toContain("B 3");
    expect(rows[1]!.textContent).toContain("Predicted from the scan");
    expect(screen.getByText(/biology on 2 bodies/)).toBeTruthy();
  });

  it("shows the sampling in progress against the colony range", async () => {
    render(View);
    const tiles = await screen.findByLabelText("Sampling");
    expect(tiles.textContent).toContain("Stratum Tectonicas");
    expect(tiles.textContent).toContain("1 of 3");
    expect(tiles.textContent).toContain("231 m of 500 m");
    expect(tiles.textContent).toContain("270 m to go");
  });

  it("lists what other players reported", async () => {
    render(View);
    const table = await screen.findByRole("table", { name: "Reported by other players" });
    expect(within(table).getAllByRole("row")).toHaveLength(3);
    expect(table.textContent).toContain("Stratum Tectonicas 19 M");
  });

  it("formats values and names", () => {
    expect(millions(19_010_800)).toBe("19 M");
    expect(millions(1_362_000)).toBe("1.4 M");
    expect(valueRange([1_000_000, 19_010_800])).toBe("1–19 M");
    expect(valueRange([6_284_600, 6_284_600])).toBe("6.3 M");
    expect(shortName("Synthetic Sector AB-C d1 A 2", "Synthetic Sector AB-C d1")).toBe("A 2");
    expect(shortName("Other 1", "Synthetic")).toBe("Other 1");
  });
});
