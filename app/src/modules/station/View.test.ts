import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the view shows the sample docking and searches the sample outfitting history
// (the Rust fixture set), the way the core searches it.
describe("Station", () => {
  it("shows the facts of the station docked at and the upgrades it sells", async () => {
    render(View);
    await screen.findByText("Test Orbital");
    expect(screen.getByText("Test Holdings")).toBeTruthy();
    expect(screen.getByText("Boom")).toBeTruthy();
    expect(screen.getByText("High Tech 70% · Refinery 30%")).toBeTruthy();
    expect(screen.getByText("3 large · 6 medium · 4 small")).toBeTruthy();
    expect(screen.getByText("Outfitting")).toBeTruthy(); // the service chip
    expect(screen.getByText("Outfitting for Synthetic")).toBeTruthy();

    const upgrades = within(screen.getByRole("table", { name: "Upgrades on sale" }));
    const cells = upgrades.getAllByRole("row")[1]!.textContent?.replace(/\s+/g, " ").trim();
    expect(cells).toBe("Frame shift drive5C Frame Shift Drive5A Frame Shift Drive9,000,000 cr");

    // The market against nearby prices sits between the facts and the outfitting.
    expect(screen.getByText("Market")).toBeTruthy();
    expect(await screen.findByRole("table", { name: "For sale here" })).toBeTruthy();
  });

  it("links the system and the station to Spansh and Inara", async () => {
    const opened = vi.spyOn(window, "open").mockImplementation(() => null);
    render(View);
    await screen.findByText("Test Orbital");
    await fireEvent.click(screen.getByTitle("Open Test System on Spansh"));
    await fireEvent.click(screen.getByTitle("Open Test System on Inara"));
    await fireEvent.click(screen.getByTitle("Open Test Orbital on Spansh"));
    await fireEvent.click(screen.getByTitle("Open Test Orbital on Inara"));
    expect(opened.mock.calls.map((c) => c[0])).toEqual([
      "https://spansh.co.uk/search/Test%20System",
      "https://inara.cz/elite/starsystem/?search=Test%20System",
      "https://spansh.co.uk/station/128000001",
      "https://inara.cz/elite/stations/?search=Test%20Orbital",
    ]);
    opened.mockRestore();
  });

  it("finds the stations that sold a module", async () => {
    render(View);
    await screen.findByText("Test Orbital");
    await fireEvent.input(screen.getByPlaceholderText(/5a frame shift/), { target: { value: "5a fsd" } });
    const table = await screen.findByRole("table", { name: "Stations that sold it" });
    const rows = within(table).getAllByRole("row").slice(1);
    expect(rows).toHaveLength(2);
    expect(rows[0]!.textContent).toContain("Other Port");
    expect(rows[0]!.textContent).toContain("9,500,000 cr");
    expect(rows[1]!.textContent).toContain("Test Orbital");
  });

  it("says so when no remembered station sold it", async () => {
    render(View);
    await fireEvent.input(screen.getByPlaceholderText(/5a frame shift/), { target: { value: "railgun" } });
    expect(await screen.findByText("No remembered station sold “railgun”.")).toBeTruthy();
  });
});
