import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import Market from "./Market.svelte";
import * as trade from "../../lib/trade";

// Outside the app the comparison is the one the Rust test gets from the fixture Market.json,
// Cargo.json and Ardent's recorded answers (crates/astromech-market/tests/here.rs).
const rows = (name: string) =>
  within(screen.getByRole("table", { name }))
    .getAllByRole("row")
    .slice(1)
    .map((r) => within(r).getAllByRole("cell").map((c) => c.textContent?.replace(/\s+/g, " ").trim()).join(" | "));

describe("Market against nearby prices", () => {
  it("sets the hold and the goods for sale against the best nearby price", async () => {
    render(Market, { marketId: 128000001 });
    await screen.findByRole("table", { name: "Your cargo" });
    const cargo = rows("Your cargo");
    expect(cargo[0]).toMatch(/^20 t Gold \| 52,600 \| 56,521 \| \+3,921\/t \| Arc City Crucis Sector AL-X b1-5 .*~1 jump · 15\.4 ly · demand 10,210/);
    expect(cargo[1]).toMatch(/^10 t Palladium \| not bought \| 59,563 \| {2}\| Arc City/);
    expect(screen.getByText("Left out: 4 t Gold (stolen), 8 t Limpet (limpets).")).toBeTruthy();

    const sale = rows("For sale here");
    expect(sale.map((r) => r?.split(" ")[0])).toEqual(["Tritium", "Silver", "Hydrogen"]);
    expect(sale[0]).toContain("50,500 | 103,006 | +52,506/t | FCX-701");
    expect(sale[1]).toContain("46,100 | 49,421 | +3,321/t | Arc City");
  });

  it("asks again on a new dock or when told to, not on every refresh of the same one", async () => {
    const spy = vi.spyOn(trade, "compareMarket");
    const { rerender } = render(Market, { marketId: 1 });
    await screen.findByRole("table", { name: "Your cargo" });
    expect(spy).toHaveBeenCalledTimes(1);
    await rerender({ marketId: 1 });
    expect(spy).toHaveBeenCalledTimes(1);
    await rerender({ marketId: 2 });
    expect(spy).toHaveBeenCalledTimes(2);
    await fireEvent.click(screen.getByRole("button", { name: /Check again|Asking/ }));
    expect(spy).toHaveBeenCalledTimes(3);
    spy.mockRestore();
  });

  it("says why when there is no market to compare", async () => {
    const spy = vi.spyOn(trade, "compareMarket").mockRejectedValue("no market seen here yet: open the commodity market once");
    render(Market, { marketId: 1 });
    expect(await screen.findByText("no market seen here yet: open the commodity market once")).toBeTruthy();
    spy.mockRestore();
  });
});
