import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the summary is the sample history's first session (the Rust fixture set).
describe("Session summary", () => {
  it("shows the session's totals", async () => {
    render(View);
    await screen.findByText("870,000 cr");
    expect(screen.getByText(/2026-09-01 10:00 to 2026-09-01 10:30 UTC \(29 min\)/)).toBeTruthy();
    expect(screen.getByText("1,744,847 cr")).toBeTruthy();
    expect(screen.getByText("10.9 ly")).toBeTruthy();
    const rows = screen.getAllByRole("row").map((r) => r.textContent?.replace(/\s+/g, " ").trim());
    expect(rows).toEqual([
      "Trade500,000 cr1 payment · profit 50,000 cr",
      "Missions250,000 cr1 payment",
      "Bounty vouchers120,000 cr1 payment",
    ]);
  });
});
