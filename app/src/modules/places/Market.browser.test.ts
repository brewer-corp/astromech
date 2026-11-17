import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import { FINDERS, forgetAnswers, lastAnswer, type FinderId } from "../../lib/places";
import { DEFAULT_FILTERS, type Filters } from "../../lib/search";
import type { CargoSearch, Side, TradeSearch } from "../../lib/trade";
import type { Args } from "../../lib/transport";
import { answer, idle, previewAnswer, watch } from "../../test/transport";
import trade from "../../preview/fixtures/trade.json";
import shipWorld from "../../preview/fixtures/ship-status.json";
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
  it("opens Sell on painite near you, and switches to the best prices anywhere, in price order with no radius filter", async () => {
    location.hash = "#/places/sell";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(location.hash).toBe("#/places/sell/painite");
    expect((screen.getByRole("combobox", { name: "Commodity" }) as HTMLInputElement).value).toBe("Painite");
    expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodities: ["painite"], origin: null }));
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.sell" });
    expect(screen.getByText(/^Within 100 ly of Sol/)).toBeTruthy();

    // The range is a filter: CHANGE opens it, and the summary names the new range while Ardent
    // is still asked, with the controls left open.
    let release: () => void = () => {};
    answer("trade_anywhere", async (args: { side: Side }) => {
      await new Promise<void>((r) => (release = r));
      return previewAnswer("trade_anywhere", args);
    });
    expect(screen.queryByRole("button", { name: "Anywhere" })).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(location.hash).toBe("#/places/sell/painite?range=anywhere");
    expect(await screen.findByText("Asking Ardent Insight…")).toBeTruthy();
    expect(screen.getByText(/^Best prices anywhere · distances from Sol/)).toBeTruthy();
    await waitFor(() => expect(anywhere).toHaveBeenCalled());
    release();
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(screen.getByRole("button", { name: "Anywhere" }).getAttribute("aria-pressed")).toBe("true"); // still open
    expect(anywhere).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodity: "painite", origin: null }));
    // Each range keeps the filters Trade remembered for it, which voice questions use too.
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.sell.anywhere" });
    expect(screen.getByText(/Best prices anywhere/)).toBeTruthy();
    expect(screen.queryByText(/^Within \d+ ly of /)).toBeNull();

    // The price over its demand leads each row, highest first.
    expect(screen.getAllByRole("columnheader")[0]?.textContent).toBe("Price · demand");
    const prices = [...document.querySelectorAll(".main .v")].map((e) => Number((e.textContent ?? "").replace(/,/g, "")));
    expect(prices[0]).toBe(390780);
    expect(prices.every((p, i) => i === 0 || p <= (prices[i - 1] ?? 0))).toBe(true);

    // Its tile keeps the answer and reopens the same search.
    expect(lastAnswer("sell")).toMatchObject({ figure: "390,780", unit: "cr", context: "Painite", route: "places/sell/painite?range=anywhere" });
  });

  it("keeps the filters open while two change and the list follows each, until DONE", async () => {
    let stored: Filters = DEFAULT_FILTERS;
    answer("search_filters", () => stored);
    answer("set_search_filters", ({ filters }: { filters: Filters }) => void (stored = filters));
    location.hash = "#/places/sell/painite";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByRole("combobox", { name: "Within" })).toBeNull();
    /** Each row's figure in its group: light years in the trip, tonnes under the price. */
    const facts = (what: "ly" | "t") =>
      screen
        .getAllByRole("row")
        .slice(1)
        .map((r) => Number((r.querySelector(what === "ly" ? ".trip .ly" : ".tonnes")?.textContent ?? "").replace(/[^\d.]/g, "")));
    const before = facts("ly");
    expect(before.some((ly) => ly > 20)).toBe(true);

    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    await fireEvent.change(screen.getByRole("combobox", { name: "Within" }), { target: { value: "20" } });
    await waitFor(() => expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ filters: expect.objectContaining({ max_distance_ly: 20 }) })));
    await waitFor(() => expect(facts("ly").every((ly) => ly <= 20)).toBe(true));
    const within20 = facts("ly").length;
    await fireEvent.change(screen.getByRole("combobox", { name: "Demand" }), { target: { value: "5000" } });
    await waitFor(() => expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ filters: expect.objectContaining({ max_distance_ly: 20, min_volume: 5000 }) })));
    await waitFor(() => expect(facts("t").length).toBeLessThan(within20));
    expect(facts("t").every((t) => t >= 5000)).toBe(true);
    expect(screen.getByText(/^Within 20 ly of Sol · demand at least 5,000 t/)).toBeTruthy();
    expect(screen.getByRole("combobox", { name: "Within" })).toBeTruthy(); // still open

    await fireEvent.click(screen.getByRole("button", { name: "Done" }));
    expect(screen.queryByRole("combobox", { name: "Within" })).toBeNull();
    expect(stored).toMatchObject({ max_distance_ly: 20, min_volume: 5000 });
  });

  it("changes where Sell searches from inside the filters, which stay open while the summary names it", async () => {
    location.hash = "#/places/sell/painite";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    expect(screen.queryByLabelText("Search from system")).toBeNull();
    let release: () => void = () => {};
    answer("trade_search", async (args: { side: Side }) => {
      await new Promise<void>((r) => (release = r));
      return previewAnswer("trade_search", args);
    });
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    const box = screen.getByLabelText("Search from system");
    await fireEvent.input(box, { target: { value: "Deciat" } });
    await fireEvent.keyDown(box, { key: "Enter" });
    await waitFor(() => expect(near).toHaveBeenLastCalledWith(expect.objectContaining({ side: "sell", commodities: ["painite"], origin: "Deciat" })));
    expect(screen.getByText(/^Within 100 ly of Deciat/)).toBeTruthy();
    release();
    await waitFor(() => expect(screen.getByRole("table").getAttribute("aria-busy")).toBe("false"));
    expect(screen.getByLabelText("Search from system")).toBeTruthy(); // still open

    // Escape in the box closes the filters; the summary keeps the origin.
    await fireEvent.keyDown(screen.getByLabelText("Search from system"), { key: "Escape" });
    expect(screen.queryByLabelText("Search from system")).toBeNull();
  });

  it("leaves the filters open when the search they started fails, and over TRY AGAIN", async () => {
    let stored: Filters = DEFAULT_FILTERS;
    answer("search_filters", () => stored);
    answer("set_search_filters", ({ filters }: { filters: Filters }) => void (stored = filters));
    let fail = false;
    answer("trade_search", async (args: { side: Side }) => {
      if (fail) throw "Ardent took too long to answer";
      return previewAnswer("trade_search", args);
    });
    location.hash = "#/places/sell/painite";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Good Landing", { exact: false })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    fail = true;
    await fireEvent.change(screen.getByRole("combobox", { name: "Data age" }), { target: { value: "7" } });
    expect((await screen.findByRole("alert")).textContent).toMatch(/Ardent took too long to answer/);
    expect(screen.getByRole("combobox", { name: "Data age" })).toBeTruthy();
    expect(screen.getByText(/· reports up to 7 days old$/)).toBeTruthy();

    fail = false;
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    await waitFor(() => expect(screen.queryByRole("alert")).toBeNull());
    expect(screen.getByRole("combobox", { name: "Data age" })).toBeTruthy();
  });

  it("opens Buy from its address, cheapest first, near or anywhere", async () => {
    location.hash = "#/places/buy/lowtemperaturediamond?range=anywhere";
    render(Market, { finder: finder("buy") });
    expect(await screen.findByText(/Cheapest anywhere/)).toBeTruthy();
    expect(anywhere).toHaveBeenLastCalledWith(expect.objectContaining({ side: "buy", commodity: "lowtemperaturediamond", origin: null }));
    expect(loaded).toHaveBeenLastCalledWith({ lookup: "trade.buy.anywhere" });
    expect((screen.getByRole("combobox", { name: "Commodity" }) as HTMLInputElement).value).toBe("Low Temperature Diamonds");

    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
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
    await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
    await fireEvent.click(screen.getByRole("button", { name: "Anywhere" }));
    expect(await screen.findByText(/FCX-800/)).toBeTruthy();
    expect(screen.queryByText(/narrow the radius/)).toBeNull();
  });
});

// The sol world's hold, which the recorded markets are measured against: 200 t of Painite among
// 600 t to sell; then the same Asp with its hold emptied (200 t free).
const solHold = trade.ship;
const emptied = trade.ship_emptied;
/** Each row's demand or stock: its state ("short", "tight", "plenty", "plain") and tonnes. */
const tonnes = () =>
  [...document.querySelectorAll(".result .tonnes")].map((e) => [[...e.classList].find((c) => c !== "tonnes"), (e.textContent ?? "").replace(/\D/g, "")]);

describe("Demand against the hold", () => {
  afterEach(() => answer("ship_status", () => previewAnswer("ship_status")));

  it("colours Sell's demand against the commodity aboard: short, tight and plenty, with a Tip on a short row", async () => {
    answer("ship_status", () => solHold);
    location.hash = "#/places/sell/painite?range=anywhere";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("200 t of Painite aboard")).toBeTruthy();
    await waitFor(() => expect(tonnes()).toContainEqual(["short", "15"]));
    // 193 t takes less than the 200 t aboard; 220 t takes it all, but under five times it.
    expect(tonnes()).toEqual(expect.arrayContaining([["short", "193"], ["tight", "220"], ["plenty", "1070"]]));
    const short = screen.getByRole("img", { name: "Demand 15 t, short of the 200 t of Painite aboard" });
    expect(short.querySelectorAll("rect.on")).toHaveLength(1);
    await fireEvent.pointerEnter(short);
    expect(document.querySelector(".tip")?.textContent).toMatch(/^15 t demand, less than the 200 t of Painite aboard\./);
    expect(screen.getByRole("img", { name: "Demand 220 t, tight for the 200 t of Painite aboard" }).querySelectorAll("rect.on")).toHaveLength(2);
  });

  it("colours Buy's stock against the free space of an empty hold", async () => {
    answer("ship_status", () => emptied);
    location.hash = "#/places/buy/tritium";
    render(Market, { finder: finder("buy") });
    expect(await screen.findByText("200 t free")).toBeTruthy();
    await waitFor(() => expect(tonnes()).toEqual(expect.arrayContaining([["short", "31"], ["tight", "385"], ["plenty", "1872"]])));
  });

  it("warns of nothing with none aboard or with the hold not known, and says which", async () => {
    answer("ship_status", () => solHold);
    location.hash = "#/places/sell/gold";
    render(Market, { finder: finder("sell") });
    // The sol world's gold is stolen or for a mission: none of it is the commander's to sell.
    expect(await screen.findByText("No Gold aboard")).toBeTruthy();
    cleanup();

    answer("ship_status", () => ({ ...solHold, sellable: null, cargo_free: null, cargo: null, cargo_capacity: null }));
    location.hash = "#/places/sell/painite?range=anywhere";
    render(Market, { finder: finder("sell") });
    expect(await screen.findByText("Cargo not known: Elite isn't running")).toBeTruthy();
    await waitFor(() => expect(tonnes().length).toBeGreaterThan(0));
    expect(tonnes().every(([state]) => state === "plain")).toBe(true);
  });

  it("follows the hold as it changes, without searching again", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    try {
      // The sol world's Asp after its hold is emptied, then with its mixed hold aboard.
      let hold: typeof solHold = emptied;
      answer("ship_status", () => hold);
      location.hash = "#/places/sell/painite?range=anywhere";
      render(Market, { finder: finder("sell") });
      expect(await screen.findByText("No Painite aboard")).toBeTruthy();
      await waitFor(() => expect(tonnes().length).toBeGreaterThan(0));
      expect(tonnes().every(([state]) => state === "plain")).toBe(true);
      await idle();
      // One search: opening the page from its address doesn't ask twice.
      expect(anywhere).toHaveBeenCalledTimes(1);

      hold = solHold;
      await vi.advanceTimersByTimeAsync(2000);
      expect(await screen.findByText("200 t of Painite aboard")).toBeTruthy();
      await waitFor(() => expect(tonnes()).toContainEqual(["short", "15"]));
      expect(anywhere).toHaveBeenCalledTimes(1);
    } finally {
      vi.useRealTimers();
    }
  });

  it("strikes through a pad too small for the ship, and says why", async () => {
    // The ship world's Anaconda needs a large pad; Cousteau Vista has a medium one.
    answer("ship_status", () => shipWorld.flying);
    location.hash = "#/places/sell/painite";
    render(Market, { finder: finder("sell") });
    const row = (await screen.findByText("Cousteau Vista")).closest('[role="row"]') as HTMLElement;
    const pad = await within(row).findByRole("img", { name: "Medium pad: too small for your ship" });
    expect(pad.classList.contains("unfit")).toBe(true);
    expect(getComputedStyle(pad).textDecorationLine).toBe("line-through");
    const good = (await screen.findByText("Good Landing")).closest('[role="row"]') as HTMLElement;
    expect(within(good).getByRole("img", { name: "Large pad" }).classList.contains("unfit")).toBe(false);
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
