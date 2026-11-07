import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { answer, previewAnswer } from "../../test/transport";
import View from "./View.svelte";

// Outside the app the view is the sample system (worlds/preview/explorer: the Rust replay test's
// journal, with two landings and two samples on A 4 a), dumped from the core.
describe("Exploration system view", () => {
  it("shows every scanned body with its value", async () => {
    render(View);
    await screen.findByText("Earth-like world");
    expect(screen.getByText("5 of 5 bodies scanned · all found")).toBeTruthy();
    const bodies = screen.getByRole("table", { name: "Bodies" });
    const rows = Array.from(bodies.querySelectorAll("tbody tr")).map((r) => r.querySelector("td")?.textContent);
    expect(rows).toEqual(["A", "A 2", "A 3", "A 4", "A 4 a"]);
    // The terraformable high metal content world is the one left to map.
    expect(screen.getAllByText("Worth mapping")).toHaveLength(1);
    expect(screen.getByText("Mapped, efficiently")).toBeTruthy();
    expect(screen.getByText("First footfall likely")).toBeTruthy();
    expect(screen.getByText("Bio 2")).toBeTruthy();
    // The star's asteroid belt is a belt; the gas giant's ring a ring.
    expect(screen.getByText("Belt")).toBeTruthy();
    expect(screen.getAllByText("Rings")).toHaveLength(1);
    expect(screen.getByText("Landed 2 times")).toBeTruthy();
  });

  it("shows the unsold data, with no reminder while it is under the risky mark", async () => {
    render(View);
    await screen.findByText("33.3 M cr");
    expect(screen.getAllByText("4.8 M cr").length).toBeGreaterThan(0);
    expect(screen.getByText("5 bodies in 1 systems")).toBeTruthy();
    expect(screen.getByText("38.1 M cr")).toBeTruthy();
    expect(screen.getByText(/unsold since 2026-10-17 11:00 UTC/)).toBeTruthy();
    // Under the 50 M cr the core calls risky, so no reminder.
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("reminds that dying loses unsold data once it is a lot", async () => {
    const unsold = await previewAnswer("exploration_unsold");
    answer("exploration_unsold", () => ({ ...unsold, organic: 74_635_746, total: 79_436_307, risky: true }));
    render(View);
    await screen.findByText("79.4 M cr");
    expect(screen.getByRole("alert").textContent).toContain("79.4 M cr aboard. Dying loses all of it");
  });
});
