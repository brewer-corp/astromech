import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the page shows the sample run (the JSON the core's mining_status test builds
// from the Rust mining fixture).
describe("Mining run", () => {
  it("shows tons refined by ore and tons per hour", async () => {
    render(View);
    await screen.findByText("9 t");
    expect(screen.getByText("18 t/h")).toBeTruthy();
    const rows = screen.getAllByRole("row").map((r) => r.textContent?.replace(/\s+/g, " ").trim());
    expect(rows).toEqual(["Platinum7 t", "Painite2 t"]);
  });
});
