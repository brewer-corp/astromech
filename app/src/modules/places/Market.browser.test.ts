import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { FINDERS, forgetAnswers, lastAnswer, type FinderId } from "../../lib/places";
import { DEFAULT_FILTERS } from "../../lib/search";
import type { CargoSearch, Side, TradeSearch } from "../../lib/trade";
import type { Args } from "../../lib/transport";
import { answer, previewAnswer, watch } from "../../test/transport";
import Cargo from "./Cargo.svelte";
import Market from "./Market.svelte";

const near = watch("trade_search");
const anywhere = watch("trade_anywhere");
let cut: Side | "all" | null = null;
let extraMission = false;
/** The preview's answer, every market marked cut when `cut` says so. */
const cutting = (cmd: string) => async (args: { side: Side }) => {
  const r = await previewAnswer<TradeSearch>(cmd, args);
  return cut === "all" || cut === args.side ? { ...r, results: r.results.map((p) => ({ ...p, truncated: true })) } : r;
};
answer("trade_search", cutting("trade_search"));
answer("trade_anywhere", cutting("trade_anywhere"));
answer("trade_cargo", async (args: Args<"trade_cargo">) => {
  const r = await previewAnswer<CargoSearch>("trade_cargo", args);
  if (!extraMission) return r;
  return { ...r, plan: { ...r.plan, kept: [...r.plan.kept, { name: "Gold", reason: "mission cargo", tonnes: 4 }] } };
});

const loaded = watch("search_filters");
answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

const finder = (id: FinderId) => FINDERS.find((f) => f.id === id)!;

beforeEach(() => {
  near.mockReset();
  anywhere.mockReset();
  loaded.mockReset();
  cut = null;
  extraMission = false;
  forgetAnswers();
});

afterEach(() => {
  cleanup();
  location.hash = "";
});

describe("Sell and Buy finders", () => {
  it("opens Sell on painite near you, and switches to the best prices anywhere, in price order with data age and no radius filter", async () => {
    location.hash = "#/places/sell";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(location.hash).toBe("#/places/sell/painite");
    expect((screen.getByRole("combobox", { name: "Commodity" }) as HTMLInputElement).value).toBe("Painite");
    expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodities: ["painite"], origin: null }));
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.sell" });
    expect(screen.getByText(/^Within 100 ly of Sol/)).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(location.hash).toBe("#/places/sell/painite?range=anywhere");
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodity: "painite", origin: null }));
    // Each range keeps the filters Trade remembered for it, which voice questions use too.
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.sell.anywhere" });
    expect(screen.getByText(/Best prices anywhere/)).toBeTruthy();
    expect(screen.queryByText(/^Within/)).toBeNull();

    // The price leads each row, highest first, and every row says how old its report is.
    expect(screen.getAllByRole("columnheader")[0]?.textContent).toBe("Price");
    const prices = [...document.querySelectorAll(".main .v")].map((e) => Number((e.textContent ?? "").replace(/,/g, "")));
    expect(prices[0]).toBe(390780);
    expect(prices.every((p, i) => i === 0 || p <= (prices[i - 1] ?? 0))).toBe(true);
    expect([...document.querySelectorAll(".fact.age")].every((e) => /^[\d,]+(d|h|min)$/.test(e.textContent ?? ""))).toBe(true);

    // Its tile keeps the answer and reopens the same search.
    expect(lastAnswer("sell")).toMatchObject({ figure: "390,780", unit: "cr", context: "Painite", route: "places/sell/painite?range=anywhere" });
  });

  it("opens Buy from its address, cheapest first, near or anywhere", async () => {
    location.hash = "#/places/buy/lowtemperaturediamond?range=anywhere";
    render(Market, { finder: finder("buy") });
    expect(await screen.findByText(/Cheapest anywhere/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith(expect.objectContaining({ side: "buy", commodity: "lowtemperaturediamond", origin: null }));
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.buy.anywhere" });
    expect((screen.getByRole("combobox", { name: "Commodity" }) as HTMLInputElement).value).toBe("Low Temperature Diamonds");

    await fireEvent.click(screen.getByRole("button", { name: "Near" }));
    expect(location.hash).toBe("#/places/buy/lowtemperaturediamond");
    expect(await screen.findByText(/^Within 100 ly of Sol/)).toBeTruthy();
    expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ side: "buy", commodities: ["lowtemperaturediamond"], origin: null }));
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.buy" });
  });

  it("picks a commodity by a short name in the main input", async () => {
    location.hash = "#/places/sell/painite";
    render(Market, { finder: finder("sell") });
    await screen.findByText("Good Landing", { exact: false });
    const box = screen.getByRole("combobox", { name: "Commodity" });
    await fireEvent.input(box, { target: { value: "LTDs" } });
    await screen.findByText("Low Temperature Diamonds");
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(location.hash).toBe("#/places/sell/lowtemperaturediamond");
    await waitFor(() => expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodities: ["lowtemperaturediamond"], origin: null })));
  });

  it("reads an unknown commodity in the address as painite", async () => {
    location.hash = "#/places/sell/unobtainium";
    render(Market, { finder: finder("sell") });
    await waitFor(() => expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodities: ["painite"], origin: null })));
    expect(location.hash).toBe("#/places/sell/painite");
  });

  const hint = /Too many markets for Painite: narrow the radius to see them all/;

  it("a full Sell answer has no narrow-the-radius hint", async () => {
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });

  it("a cut Sell answer says to narrow the radius", async () => {
    cut = "sell";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText(hint)).toBeTruthy();
  });

  it("a cut Buy answer says to narrow the radius, and a Sell-only cut does not reach Buy", async () => {
    cut = "buy";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
    cleanup();
    render(Market, { finder: finder("buy") });
    expect(await screen.findByText(hint)).toBeTruthy();
  });

  it("an Anywhere answer, always cut at 100, shows no hint", async () => {
    cut = "all";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.getByText(hint)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });
});

describe("Sell my cargo finder", () => {
  it("lists the hold, and two mission lots of the same commodity do not crash the page", async () => {
    extraMission = true;
    location.hash = "#/places/sell-cargo";
    render(Cargo, { finder: finder("sell-cargo") });
    expect(await screen.findByText("Low Temperature Diamonds")).toBeTruthy();
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.cargo" });
    expect(screen.getByRole("heading", { level: 1, name: "Sell my cargo" })).toBeTruthy();
    expect(document.querySelector(".finder .head .main"), "no main input").toBeNull();
    expect(screen.getByText("300 t")).toBeTruthy();
    expect(screen.getByText("stolen")).toBeTruthy();
    expect(screen.getAllByText("mission cargo")).toHaveLength(2);
    expect(screen.getByText("4 t")).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Best for each on its own" })).toBeTruthy();
    expect(lastAnswer("sell-cargo")).toMatchObject({ unit: "cr", route: "places/sell-cargo" });
  });
});
