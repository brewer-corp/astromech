import { describe, expect, it } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import KeybindsView from "./KeybindsView.svelte";

// Group headers are the buttons with a +/- box; the browser fallback serves the sample preset.
const header = (name: string) => screen.getByText(name, { selector: ".gname" }).closest("button")!;
const isOpen = (name: string) => header(name).classList.contains("open");

describe("All keybinds group panels", () => {
  it("opens one group without opening the others", async () => {
    render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    expect(isOpen("Flight")).toBe(false);

    await fireEvent.click(header("Flight"));

    expect(isOpen("Flight")).toBe(true);
    expect(isOpen("Ship systems")).toBe(false);
    expect(isOpen("General")).toBe(false);
  });

  it("closes a group on its own", async () => {
    render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    await fireEvent.click(header("Flight"));
    await fireEvent.click(header("General"));
    await fireEvent.click(header("Flight"));

    expect(isOpen("Flight")).toBe(false);
    expect(isOpen("General")).toBe(true);
  });

  it.each(["SRV", "On foot"])("keeps panels independent when the %s filter leaves three groups", async (filter) => {
    const { container } = render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    await fireEvent.click(screen.getByText(filter, { selector: ".filters button" }));
    const names = () => [...container.querySelectorAll(".grp .gname")].map((e) => e.textContent!);
    const first = names()[0]!;
    expect(names()).toHaveLength(3);
    // Nothing is open until you open it.
    expect(names().filter(isOpen)).toEqual([]);

    await fireEvent.click(header(first));

    expect(names().filter(isOpen)).toEqual([first]);
  });
});
