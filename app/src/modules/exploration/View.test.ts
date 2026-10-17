import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the view is the sample system (the Rust replay test's journal, folded).
describe("Exploration system view", () => {
  it("shows every scanned body with its value", async () => {
    render(View);
    await screen.findByText("Earth-like world");
    expect(screen.getByText("5 of 5 bodies scanned · all found")).toBeTruthy();
    const rows = screen.getAllByRole("row").slice(1).map((r) => r.querySelector("td")?.textContent);
    expect(rows).toEqual(["A", "A 2", "A 3", "A 4", "A 4 a"]);
    // The terraformable high metal content world is the one left to map.
    expect(screen.getAllByText("Worth mapping")).toHaveLength(1);
    expect(screen.getByText("Mapped, efficiently")).toBeTruthy();
    expect(screen.getByText("First footfall likely")).toBeTruthy();
    expect(screen.getByText("Bio 2")).toBeTruthy();
  });
});
