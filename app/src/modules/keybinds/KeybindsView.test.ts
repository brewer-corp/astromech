import { describe, expect, it } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import KeybindsView from "./KeybindsView.svelte";
import { binds } from "../../lib/store.svelte";

// Group headers are the buttons with a +/- box; the browser fallback serves the sample preset.
Element.prototype.scrollIntoView = () => {};
const header = (name: string) => screen.getByText(name, { selector: ".gname" }).closest("button")!;
const isOpen = (name: string) => header(name).classList.contains("open");

describe("All keybinds group panels", () => {
  it("opens groups that have issues, and opening another leaves them open", async () => {
    render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    expect(isOpen("Flight")).toBe(false);
    expect(isOpen("General")).toBe(true);
    expect(isOpen("Interface")).toBe(true);

    await fireEvent.click(header("Flight"));

    expect(isOpen("Flight")).toBe(true);
    expect(isOpen("Ship systems")).toBe(false);
    expect(isOpen("General")).toBe(true);
  });

  it("closes a group on its own", async () => {
    render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    await fireEvent.click(header("Flight"));
    expect(isOpen("Flight")).toBe(true);
    expect(isOpen("General")).toBe(true);

    await fireEvent.click(header("Flight"));
    expect(isOpen("Flight")).toBe(false);
    expect(isOpen("General")).toBe(true);

    await fireEvent.click(header("General"));
    expect(isOpen("General")).toBe(false);
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

  it("ignores a mouse click that arrives as a Mouse_1 press (Raw Input forwards the window's own clicks)", async () => {
    const { container } = render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    await fireEvent.click(header("Flight"));

    binds.press({ device: "Mouse", key: "Mouse_1", kind: "button", down: true } as never);
    await new Promise((r) => setTimeout(r, 10));

    const names = [...container.querySelectorAll(".grp .gname")].map((e) => e.textContent!);
    expect(names.filter(isOpen)).toEqual(["General", "Interface", "Flight"]);
  });

  it("still finds a side mouse button (Mouse_4) bound in the game", async () => {
    render(KeybindsView);
    await screen.findByText("Flight", { selector: ".gname" });
    const slot = binds.rows.find((r) => r.id === "HumanoidPrimaryFireButton")!.slots.find((s) => s.slot === "Secondary")!;
    Object.assign(slot, { device: "Mouse", key: "Mouse_4" });
    try {
      binds.press({ device: "Mouse", key: "Mouse_4", kind: "button", down: true } as never);
      await new Promise((r) => setTimeout(r, 10));
      expect(isOpen("On foot combat")).toBe(true);
      expect(screen.getByText(/Mouse 4.*: Fire/)).toBeTruthy();
    } finally {
      Object.assign(slot, { device: "", key: "" });
    }
  });
});
