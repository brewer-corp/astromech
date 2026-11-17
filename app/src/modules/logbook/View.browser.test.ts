import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the logbook reads the sample history (the Rust fixture set), searched the
// way the core searches it.
const text = (el: Element) => el.textContent?.replace(/\s+/g, " ").trim();

describe("Logbook", () => {
  it("finds a visited station with its counts", async () => {
    render(View);
    await screen.findByText("arrivals");

    await fireEvent.input(screen.getByPlaceholderText("System or station"), { target: { value: "hutton" } });
    const places = await screen.findByRole("region", { name: "Places matching “hutton”" });

    const rows = within(places).getAllByRole("listitem");
    expect(rows.map(text)).toEqual(["Hutton Orbital Alpha Centauri 2 visits · first 2026-09-03 10:10 · last 2026-09-03 10:25"]);
    // The timeline narrows to those two dockings, on their day.
    const day = screen.getByRole("region", { name: /^2026-09-03/ });
    expect(within(day).getAllByRole("listitem").map(text)).toEqual(["10:25 Hutton Orbital Alpha Centauri", "10:10 Hutton Orbital Alpha Centauri"]);
    expect(screen.getAllByRole("region")).toHaveLength(2);
  });

  it("says so when nothing matches", async () => {
    render(View);
    await fireEvent.input(screen.getByPlaceholderText("System or station"), { target: { value: "Colonia" } });
    expect(await screen.findByText("No visited system or station matches “Colonia”.")).toBeTruthy();
  });
});
