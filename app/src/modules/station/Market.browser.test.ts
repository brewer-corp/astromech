import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import type { MarketCompare } from "../../lib/trade";
import { answer, previewAnswer, watch } from "../../test/transport";
import Market from "./Market.svelte";

// Outside the app the comparison is the one the Rust test gets from the fixture Market.json,
// Cargo.json and Ardent's recorded answers (crates/astromech-market/tests/here.rs).
const rows = (name: string) =>
  within(screen.getByRole("table", { name }))
    .getAllByRole("row")
    .slice(1)
    .map((r) => within(r).getAllByRole("cell").map((c) => (c as HTMLElement).innerText.replace(/\s+/g, " ").trim()).join(" | "));

describe("Market against nearby prices", () => {
  it("sets the hold and the goods for sale against the best nearby price", async () => {
    render(Market, { marketId: 128000001 });
    await screen.findByRole("table", { name: "Your cargo" });
    const cargo = rows("Your cargo");
    expect(cargo[0]).toMatch(/^Gold 20 t \| 52,600 56,521 \| \+3,921 ?cr\/t \| Arc City Crucis Sector AL-X b1-5 \| ~1 ?jump 15\.4 ly · \d+ d old$/);
    expect(cargo[1]).toMatch(/^Palladium 10 t \| not bought 59,563 \| – \| Arc City/);
    expect(screen.getByText("Left out: 4 t Gold (stolen), 8 t Limpet (limpets).")).toBeTruthy();

    const sale = rows("For sale here");
    expect(sale.map((r) => r?.split(" ")[0])).toEqual(["Tritium", "Silver", "Hydrogen"]);
    expect(sale[0]).toMatch(/^Tritium 3,000 t \| 50,500 103,006 \| \+52,506 ?cr\/t \| FCX-701 Nuenets/);
    expect(sale[1]).toMatch(/^Silver 800 t \| 46,100 49,421 \| \+3,321 ?cr\/t \| Arc City/);
  });

  it("says how old the market's prices are", async () => {
    render(Market, { marketId: 128000001 });
    const seen = await screen.findByText(/^Market as of /);
    expect(seen.textContent).toMatch(/^Market as of 2026-10-16 20:30 UTC, (\d+ (min|h|d) ago|just now)$/);
  });

  it("says which carried goods weren't looked up", async () => {
    const r = await previewAnswer<MarketCompare>("market_compare");
    const extra = { ...r.carried[1]!, commodity: "gold2", name: "Extra", checked: false, best: null, gain: null };
    answer("market_compare", () => ({ ...r, carried: [...r.carried, extra] }));
    render(Market, { marketId: 1 });
    await screen.findByRole("table", { name: "Your cargo" });
    expect(rows("Your cargo")[2]).toMatch(/^Extra 10 t .*Not looked up$/);
  });

  it("asks again on a new dock or when told to, not on every refresh of the same one", async () => {
    const spy = watch("market_compare");
    const { rerender } = render(Market, { marketId: 1 });
    await screen.findByRole("table", { name: "Your cargo" });
    expect(spy).toHaveBeenCalledTimes(1);
    await rerender({ marketId: 1 });
    expect(spy).toHaveBeenCalledTimes(1);
    await rerender({ marketId: 2 });
    expect(spy).toHaveBeenCalledTimes(2);
    await fireEvent.click(screen.getByRole("button", { name: /Check again|Asking/ }));
    expect(spy).toHaveBeenCalledTimes(3);
  });

  it("says why when there is no market to compare", async () => {
    answer("market_compare", () => Promise.reject("no market seen here yet: open the commodity market once"));
    render(Market, { marketId: 1 });
    expect((await screen.findByRole("alert")).textContent).toContain("Could not compare the market: no market seen here yet: open the commodity market once.");
    expect(screen.getByRole("button", { name: "Check again" })).toBeTruthy();
  });
});
