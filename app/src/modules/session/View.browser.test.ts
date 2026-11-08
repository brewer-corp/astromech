import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import View from "./View.svelte";

// Outside the app the summary is the sample history's latest session (worlds/preview/history,
// the history crate's synthetic journals), dumped from the core.
describe("Session summary", () => {
  it("shows the session's totals", async () => {
    render(View);
    await screen.findByText("7,221,500 cr");
    expect(screen.getByText(/2026-09-03 10:00 to 2026-09-03 10:30 UTC \(29 min\)/)).toBeTruthy();
    expect(screen.getByText("14,483,231 cr")).toBeTruthy();
    expect(screen.getByText("0.0 ly")).toBeTruthy();
    const rows = screen.getAllByRole("row").map((r) => r.textContent?.replace(/\s+/g, " ").trim());
    expect(rows).toEqual([
      "Organic data5,000,000 cr1 payment",
      "Community goals2,000,000 cr1 payment",
      "Exploration data100,000 cr1 payment",
      "Combat bonds60,000 cr1 payment",
      "Search and rescue40,000 cr1 payment",
      "Odyssey goods15,000 cr1 payment",
      "Other vouchers5,000 cr1 payment",
      "Trade1,500 cr1 payment · profit 1,500 cr",
    ]);
  });
});
