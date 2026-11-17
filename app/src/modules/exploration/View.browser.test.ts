import { describe, expect, it, vi } from "vitest";
import { render, screen, within } from "@testing-library/svelte";
import { userEvent } from "vitest/browser";
import { answer, previewAnswer } from "../../test/transport";
import { statValue } from "../../test/stat";
import View from "./View.svelte";

// Outside the app the view is the sample system (worlds/preview/explorer: the Rust replay test's
// journal grown into a tree of eleven bodies around two stars, with two landings and two samples on
// A 4 a), dumped from the core.
describe("Exploration system view", () => {
  it("shows every scanned body in the system's tree", async () => {
    render(View);
    await screen.findByText("Earth-like world");
    expect(screen.getByText(/^11 bodies, all scanned · 3 worth mapping/)).toBeTruthy();
    const tree = screen.getByRole("treegrid", { name: "Bodies" });
    const rows = Array.from(tree.querySelectorAll("[role=row][aria-level]"));
    // Stars at the root, each body under its planet or star by distance; the planet circling the
    // barycentre of A and B at the root too.
    expect(rows.map((r) => `${r.getAttribute("aria-level")} ${r.querySelector(".name")?.textContent}`)).toEqual([
      "1 A", "2 A 1", "2 A 2", "2 A 3", "3 A 3 a", "2 A 4", "3 A 4 a", "1 B", "2 B 1", "3 B 1 a", "1 AB 1",
    ]);
    // The high metal content world, the water world and the ammonia world are left to map.
    expect(screen.getAllByText("Worth mapping")).toHaveLength(3);
    expect(screen.getByText("Mapped")).toBeTruthy();
    expect(screen.getAllByText("First footfall likely")).toHaveLength(3);
    expect(screen.getByText("Bio 2")).toBeTruthy();
    // The star's asteroid belt is a belt; the gas giant's ring a ring.
    expect(screen.getByText("Belt")).toBeTruthy();
    expect(screen.getAllByText("Rings")).toHaveLength(1);
    expect(screen.getByText("Landed 2 times")).toBeTruthy();
    // No body is selected until the commander picks one.
    expect(screen.queryByRole("complementary", { name: "Details" })).toBeNull();
  });

  it("selects a moon and shows it under its planet, with its facts in the panel", async () => {
    render(View);
    await screen.findByText("Icy body");
    const tree = screen.getByRole("treegrid", { name: "Bodies" });
    const rows = Array.from(tree.querySelectorAll<HTMLElement>("[role=row][aria-level]"));
    const moon = rows.find((r) => r.querySelector(".name")?.textContent === "A 4 a")!;
    await userEvent.click(moon);
    expect(moon.getAttribute("aria-selected")).toBe("true");
    // Its parent is the nearest row above it one level up.
    const level = Number(moon.getAttribute("aria-level"));
    const parent = rows.slice(0, rows.indexOf(moon)).reverse().find((r) => Number(r.getAttribute("aria-level")) === level - 1);
    expect(parent?.querySelector(".name")?.textContent).toBe("A 4");
    const panel = within(screen.getByRole("complementary", { name: "Details" }));
    expect(panel.getByRole("heading", { name: "A 4 a" })).toBeTruthy();
    expect(panel.getByText("6,344")).toBeTruthy();
    expect(panel.getByText("1,300 cr now · not mapped yet")).toBeTruthy();
    expect(panel.getByText("2 biological · 1 geological")).toBeTruthy();
    expect(panel.getByText("First discovery, never mapped")).toBeTruthy();
    expect(panel.getByText("2 times")).toBeTruthy();
    expect(panel.getByRole("link", { name: "Open in Biology" }).getAttribute("href")).toBe("#/exobiology");
    // The row is the tree's one tab stop; the next stop is the panel's first tag, whose Tip says
    // what it means.
    await userEvent.tab();
    expect(document.activeElement?.textContent).toBe("Landable");
    expect(document.activeElement?.closest("[aria-label=Tags]")).toBeTruthy();
    moon.focus();
    // The keyboard moves the selection up the tree.
    await userEvent.keyboard("{ArrowUp}");
    expect(panel.getByRole("heading", { name: "A 4" })).toBeTruthy();
  });

  it("drops the selection on a jump, so a return to the system starts with none", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    try {
      const here = await previewAnswer("exploration_system");
      let reply = here;
      answer("exploration_system", () => reply);
      render(View);
      await screen.findByText("Icy body");
      await userEvent.click(screen.getByText("Icy body"));
      expect(screen.getByRole("complementary", { name: "Details" })).toBeTruthy();
      reply = { ...here, system: "Next", system_address: 7_000_002, bodies: [], body_count: null, all_found: false, value: 0, value_if_mapped: 0 };
      await vi.advanceTimersByTimeAsync(3000);
      await screen.findByText(/^Nothing scanned here yet/);
      reply = here;
      await vi.advanceTimersByTimeAsync(3000);
      await screen.findByText("Icy body");
      expect(screen.queryByRole("complementary", { name: "Details" })).toBeNull();
    } finally {
      vi.useRealTimers();
    }
  });

  it("says when nothing is scanned yet, with no tree", async () => {
    const sample = await previewAnswer("exploration_system");
    answer("exploration_system", () => ({ ...sample, bodies: [], body_count: null, all_found: false, value: 0, value_if_mapped: 0 }));
    render(View);
    await screen.findByText(/^Nothing scanned here yet/);
    expect(screen.queryByRole("treegrid")).toBeNull();
  });

  it("shows the unsold data, with no reminder while it is under the risky mark", async () => {
    render(View);
    await screen.findByText("Organic");
    expect(statValue("Organic")).toBe("33.3 M cr");
    expect(statValue("Cartographic")).toBe("5.3 M cr");
    expect(screen.getByText("11 bodies in 1 systems")).toBeTruthy();
    expect(statValue("Total")).toBe("38.7 M cr");
    expect(screen.getByText(/unsold since 2026-10-17 11:00 UTC/)).toBeTruthy();
    // Under the 50 M cr the core calls risky, so no reminder.
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("reminds that dying loses unsold data once it is a lot", async () => {
    const unsold = await previewAnswer("exploration_unsold");
    answer("exploration_unsold", () => ({ ...unsold, organic: 74_635_746, total: 79_436_307, risky: true }));
    render(View);
    await screen.findByText("Organic");
    expect(statValue("Total")).toBe("79.4 M cr");
    expect(screen.getByRole("alert").textContent).toContain("79.4 M cr aboard. Dying loses all of it");
  });
});
