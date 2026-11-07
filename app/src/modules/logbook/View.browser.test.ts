import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the logbook reads the sample history (the Rust fixture set), searched the
// way the core searches it.
describe("Logbook", () => {
  it("finds a visited station with its counts", async () => {
    render(View);
    await screen.findByText(/4 systems · 3 stations · 6 arrivals · 4 dockings/);

    await fireEvent.input(screen.getByPlaceholderText("System or station"), { target: { value: "hutton" } });
    await screen.findByText(/Places matching “hutton”/);

    const rows = within(screen.getByRole("table")).getAllByRole("row").slice(1);
    expect(rows).toHaveLength(1);
    const cells = within(rows[0]!).getAllByRole("cell").map((c) => c.textContent?.replace(/\s+/g, " ").trim());
    expect(cells).toEqual(["Hutton Orbital Alpha Centauri", "2", "2026-09-03 10:10", "2026-09-03 10:25"]);
    // The timeline narrows to those two dockings.
    expect(screen.getAllByText(/Docked at/)).toHaveLength(2);
  });

  it("says so when nothing matches", async () => {
    render(View);
    await fireEvent.input(screen.getByPlaceholderText("System or station"), { target: { value: "Colonia" } });
    expect(await screen.findByText("No visited system or station matches “Colonia”.")).toBeTruthy();
  });
});
