import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { fakeCall, resetFakeCore } from "../fakecore";
import { followRoute } from "../route";
import { DEFAULT_FILTERS, type Filters, type Origin, type Results } from "../search";
import SearchKit from "./SearchKit.svelte";

// Storage fails: the kit must still search (with the defaults) instead of showing nothing.
const loadFilters = vi.fn<(lookup: string) => Promise<Filters>>();
const saveFilters = vi.fn<(lookup: string, f: Filters) => Promise<void>>();
vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  loadFilters: (l: string) => loadFilters(l),
  saveFilters: (l: string, f: Filters) => saveFilters(l, f),
  systemNames: async () => [],
}));

const results: Results = {
  origin: "Sol",
  range: 20,
  source: "Spansh",
  fetched: "2026-10-16T12:00:00Z",
  place: "system",
  columns: [{ key: "faction", head: "Faction", text: true }],
  rows: [{ key: 1, system: "Ross 154", ly: 9.7, jumps: 1, exact: true, updated_at: "2026-10-16T11:00:00Z", facts: { faction: "Ross 154 Boys" } }],
};

const DAY = 86400000;
const daysAgo = (d: number) => new Date(Date.now() - d * DAY).toISOString();
/** Each result row's cells, as text. */
const cells = () =>
  screen
    .getAllByRole("row")
    .slice(1)
    .map((r) => [...r.querySelectorAll('[role="cell"]')].map((c) => c.textContent?.replace(/\s+/g, " ").trim()));
const heads = () => screen.getAllByRole("columnheader").map((h) => h.textContent);
const kit = (run: (o: Origin, f: Filters) => Promise<Results>, props: Record<string, unknown> = {}) =>
  render(SearchKit, { lookup: "test.lookup", offers: [], run, ...props });

describe("SearchKit", () => {
  it("searches with the default filters when the saved ones can't be read", async () => {
    vi.spyOn(console, "warn").mockImplementation(() => {});
    loadFilters.mockRejectedValue("couldn't open astromech.sqlite");
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(results);
    render(SearchKit, { lookup: "faction.famine", offers: ["age"], run });

    expect(await screen.findByText("Ross 154")).toBeTruthy();
    expect(run).toHaveBeenCalledExactlyOnceWith(null, DEFAULT_FILTERS);
    expect(screen.getByRole("combobox")).toBeTruthy(); // the data-age filter still shows
  });

  it("keeps the later filter change when an earlier save fails after it", async () => {
    vi.spyOn(console, "warn").mockImplementation(() => {});
    const saves: Array<{ f: Filters; fail: (e: Error) => void; ok: () => void }> = [];
    let stored = DEFAULT_FILTERS;
    loadFilters.mockImplementation(async () => stored);
    saveFilters.mockImplementation(
      (_, f) =>
        new Promise<void>((ok, fail) => {
          saves.push({ f, ok: () => ((stored = f), ok()), fail });
        }),
    );
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(results);
    render(SearchKit, { lookup: "faction.famine", offers: ["age"], run });
    const select = (await screen.findByRole("combobox")) as HTMLSelectElement;

    await fireEvent.change(select, { target: { value: "7" } });
    await fireEvent.change(select, { target: { value: "30" } });
    expect(saves.map((s) => s.f.max_age_days)).toEqual([7, 30]);
    // the second save lands, then the first fails
    saves[1]!.ok();
    await waitFor(() => expect(select.value).toBe("30"));
    saves[0]!.fail(new Error("database is locked"));
    await new Promise((r) => setTimeout(r, 20));
    expect(select.value).toBe("30");
    expect(screen.queryByText(/Couldn't save/)).toBeNull();
  });

  it("labels the ship's pad with the current ship's, not the one the filters loaded with", async () => {
    // Filters as the shell resolves them: not chosen, so the pad is the current ship's.
    let pad: Filters["pad"] = "M";
    loadFilters.mockImplementation(async () => ({ ...DEFAULT_FILTERS, pad }));
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(results);
    render(SearchKit, { lookup: "stations.service", offers: ["pad", "carriers", "odyssey"], run });
    expect(await screen.findByRole("button", { name: "Pad M, current ship" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Carriers" }).getAttribute("aria-pressed")).toBe("true");
    expect(screen.getByRole("button", { name: "Odyssey" }).getAttribute("aria-pressed")).toBe("true");
    expect(screen.getByRole("button", { name: "Pad M, current ship" }).getAttribute("aria-pressed")).toBe("true");

    pad = "L"; // the commander swaps to an Anaconda, then searches from another system
    const box = screen.getByLabelText("Search from system");
    await fireEvent.input(box, { target: { value: "Deciat" } });
    await fireEvent.keyDown(box, { key: "Enter" });
    await waitFor(() => expect(run).toHaveBeenLastCalledWith("Deciat", expect.anything()));
    expect(await screen.findByRole("button", { name: "Pad L, current ship" })).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Pad M, current ship" })).toBeNull();
  });

  it("lays out a result as DESIGN.md's row: the ranked value first, the place, the facts under heads, the actions", async () => {
    loadFilters.mockResolvedValue({ ...DEFAULT_FILTERS, pad: "L" });
    const sell: Results = {
      ...results,
      place: "station",
      main: { head: "Price", unit: "cr" },
      columns: [{ key: "volume", head: "Demand", unit: "t", width: 88 }],
      rows: [
        { key: 2, system: "Diaguandri", station: "Parekh Chemical Holdings", ly: 127.3, jumps: 7, exact: true, pad: "L", arrival_ls: 775, station_type: "Coriolis Starport", updated_at: daysAgo(10), main: "313,239", facts: { volume: "15" } },
      ],
    };
    kit(vi.fn().mockResolvedValue(sell), { offers: ["distance", "pad", "age"], hints: { distance: 100, age: 30 } });
    expect(await screen.findByText("Parekh Chemical Holdings")).toBeTruthy();
    expect(heads()).toEqual(["Price", "Station", "Demand", "Distance", "Jumps", "Pad", "Arrival", "Updated", ""]);
    expect(cells()).toEqual([["313,239cr", "Parekh Chemical Holdings Diaguandri", "15t", "127.3ly", "7", "L", "775ls", "10d", ""]]);
    // The ranked value is the row's one Value L figure, in Orange Lit.
    expect(screen.getByText("313,239").closest(".main")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Copy Diaguandri" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Add Diaguandri as a stop" })).toBeTruthy();
    expect(screen.getByText("Within 100 ly of Sol · large pad · reports up to 30 days old")).toBeTruthy();
  });

  it("keeps the ranked value, the place and the actions in a panel too narrow for the facts", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    // A 320 px side panel: the row needs 112 + 240 + 64 px plus its fact columns.
    const width = vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(297);
    vi.stubGlobal(
      "ResizeObserver",
      class {
        constructor(private cb: () => void) {}
        observe() {
          this.cb();
        }
        disconnect() {}
      },
    );
    try {
      kit(vi.fn().mockResolvedValue(results), { offers: ["age"] });
      expect(await screen.findByText("Ross 154")).toBeTruthy();
      await waitFor(() => expect(heads()).toEqual(["Jumps", "System", ""]));
      expect(cells()).toEqual([["1", "Ross 154", ""]]);
      expect(screen.getByRole("button", { name: "Set Ross 154 as the destination" })).toBeTruthy();
    } finally {
      width.mockRestore();
      vi.unstubAllGlobals();
    }
  });

  it("ranks by jumps when nothing else ranks, and gives a system lookup no pad or arrival column", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const r = { ...results, rows: [{ ...results.rows[0]!, updated_at: daysAgo(2), exact: false }] };
    kit(vi.fn().mockResolvedValue(r), { offers: ["age"] });
    expect(await screen.findByText("Ross 154")).toBeTruthy();
    expect(heads()).toEqual(["Jumps", "System", "Faction", "Distance", "Updated", ""]);
    expect(cells()).toEqual([["~1", "Ross 154", "Ross 154 Boys", "9.7ly", "2d", ""]]);
    expect(screen.getByText("Nearest to Sol · reports of any age")).toBeTruthy();
    expect(screen.getByText(/~ = straight-line estimate/)).toBeTruthy();
  });

  it("steps a stale report down with its age in Orange when there is no age filter", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const r: Results = {
      ...results,
      fetched: new Date().toISOString(),
      rows: [
        { key: 1, system: "Fresh", ly: 1, jumps: 1, exact: true, updated_at: daysAgo(3) },
        { key: 2, system: "Old", ly: 2, jumps: 1, exact: true, updated_at: daysAgo(212) },
        { key: 3, system: "Undated", ly: 3, jumps: 1, exact: true, updated_at: null },
      ],
    };
    kit(vi.fn().mockResolvedValue(r), { offers: ["age"] });
    expect(await screen.findByText("Old")).toBeTruthy();
    const stale = screen.getAllByRole("row").filter((row) => row.classList.contains("stale"));
    expect(stale.map((row) => row.textContent)).toEqual([expect.stringContaining("Old")]);
    expect(stale[0]!.querySelector(".age")?.textContent).toBe("212d");
  });

  it("hides reports older than the age filter and counts the ones that came back", async () => {
    loadFilters.mockResolvedValue({ ...DEFAULT_FILTERS, max_age_days: 7 });
    const r: Results = {
      ...results,
      fetched: new Date().toISOString(),
      rows: [
        { key: 1, system: "Fresh", ly: 1, jumps: 1, exact: true, updated_at: daysAgo(3) },
        { key: 2, system: "Old", ly: 2, jumps: 1, exact: true, updated_at: daysAgo(10) },
      ],
    };
    kit(vi.fn().mockResolvedValue(r), { offers: ["age"] });
    expect(await screen.findByText("Fresh")).toBeTruthy();
    expect(screen.queryByText("Old")).toBeNull();
    expect(screen.getByText("Nearest to Sol · reports up to 7 days old · 1 older report hidden")).toBeTruthy();
    expect(document.querySelector(".stale")).toBeNull();
  });

  it("says what is empty and widens the search in one action", async () => {
    let stored: Filters = DEFAULT_FILTERS;
    loadFilters.mockImplementation(async () => stored);
    saveFilters.mockImplementation(async (_, f) => void (stored = f));
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockImplementation(async (_, f) => (f.max_distance_ly === 200 ? results : { ...results, rows: [] }));
    kit(run, { offers: ["distance"], hints: { distance: 100 }, noun: "buyers" });
    expect(await screen.findByText(/No buyers within 100 ly\./)).toBeTruthy();
    expect(screen.queryByRole("table")).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Widen to 200 ly" }));
    expect(await screen.findByText("Ross 154")).toBeTruthy();
    expect(run).toHaveBeenLastCalledWith(null, expect.objectContaining({ max_distance_ly: 200 }));
    expect(stored.max_distance_ly).toBe(200);
  });

  it("keeps the old rows, dim, under one line naming the source while a new search runs", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValueOnce(results).mockReturnValue(new Promise(() => {}));
    kit(run, { busyText: "Asking Spansh…" });
    expect(await screen.findByText("Ross 154")).toBeTruthy();
    expect(screen.getByRole("table").getAttribute("aria-busy")).toBe("false");
    const box = screen.getByLabelText("Search from system");
    await fireEvent.input(box, { target: { value: "Deciat" } });
    await fireEvent.keyDown(box, { key: "Enter" });
    expect(await screen.findByText("Asking Spansh…")).toBeTruthy();
    expect(screen.getByText("Ross 154")).toBeTruthy();
    expect(screen.getByRole("table").getAttribute("aria-busy")).toBe("true");
    expect(screen.queryByText(/^Nearest to/)).toBeNull();
  });

  it("shows a failed search as a hard error, and TRY AGAIN runs it again", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockRejectedValueOnce("Spansh took too long to answer").mockResolvedValue(results);
    kit(run);
    const alert = await screen.findByRole("alert");
    expect(alert.textContent).toMatch(/^! Spansh took too long to answer\s*Try again$/);
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(await screen.findByText("Ross 154")).toBeTruthy();
    expect(run).toHaveBeenCalledTimes(2);
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("shows a filter that couldn't be saved as a hard error, and TRY AGAIN saves it again", async () => {
    vi.spyOn(console, "warn").mockImplementation(() => {});
    let stored = DEFAULT_FILTERS;
    loadFilters.mockImplementation(async () => stored);
    saveFilters.mockRejectedValueOnce(new Error("database is locked")).mockImplementation(async (_, f) => void (stored = f));
    kit(vi.fn().mockResolvedValue(results), { offers: ["age"] });
    const select = (await screen.findByRole("combobox")) as HTMLSelectElement;
    await fireEvent.change(select, { target: { value: "7" } });
    expect((await screen.findByRole("alert")).textContent).toMatch(/Couldn't save the filter: database is locked/);
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    await waitFor(() => expect(stored.max_age_days).toBe(7));
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("Add stop and Set destination hand the core the row's system and label, and say when it can't", async () => {
    loadFilters.mockResolvedValue(DEFAULT_FILTERS);
    const station: Results = {
      ...results,
      rows: [{ key: 2, system: "Deciat", station: "Garay Terminal", ly: 12, jumps: 1, exact: true, updated_at: null }],
    };
    const run = vi.fn<(o: Origin, f: Filters) => Promise<Results>>().mockResolvedValue(station);
    // Outside the app with no preview core, the core isn't there to take it.
    history.replaceState(null, "", "/");
    const { unmount } = render(SearchKit, { lookup: "stations.service", offers: [], run });
    await fireEvent.click(await screen.findByRole("button", { name: "Add Deciat as a stop" }));
    // A hard error: a red "!", why, and TRY AGAIN, which tries the same action again.
    expect((await screen.findByRole("alert")).textContent).toMatch(/^! Couldn't add it: not running inside the app\s*Try again$/);
    await fireEvent.click(screen.getByRole("button", { name: "Set Deciat as the destination" }));
    expect(await screen.findByText(/Couldn't follow it: not running inside the app/)).toBeTruthy();
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    await fireEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(await screen.findByText("Following. Deciat is on your clipboard.")).toBeTruthy();
    expect(screen.queryByRole("alert")).toBeNull();
    unmount();

    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    await followRoute("Engineer trip", "engineer trip", [{ system: "Kuk", label: "Ram Tah" }, { system: "Sol" }]);
    render(SearchKit, { lookup: "stations.service", offers: [], run });
    const route = async () => (await fakeCall("route")).view;

    // Add stop: the next stop of the route followed, which stays.
    await fireEvent.click(await screen.findByRole("button", { name: "Add Deciat as a stop" }));
    expect(await screen.findByText("Deciat is the next stop. Deciat is on your clipboard.")).toBeTruthy();
    let v = await route();
    expect(v.route).toMatchObject({ name: "Engineer trip", next: 0 });
    expect(v.route.waypoints).toEqual([{ system: "Deciat", label: "Garay Terminal" }, { system: "Kuk", label: "Ram Tah" }, { system: "Sol" }]);
    expect(screen.queryByRole("button", { name: "Resume" })).toBeNull();

    // Set destination: a one-stop route; the trip is parked, and the note offers it back.
    await fireEvent.click(screen.getByRole("button", { name: "Set Deciat as the destination" }));
    expect(await screen.findByText("Replaced Engineer trip.")).toBeTruthy();
    expect(screen.getByText(/Following\. Deciat is on your clipboard\./)).toBeTruthy();
    v = await route();
    expect(v.route).toMatchObject({ name: "Deciat", source: "search" });
    expect(v.route.waypoints).toEqual([{ system: "Deciat", label: "Garay Terminal" }]);
    expect(v.parked.name).toBe("Engineer trip");

    await fireEvent.click(screen.getByRole("button", { name: "Resume" }));
    await waitFor(() => expect(screen.queryByText("Replaced Engineer trip.")).toBeNull());
    v = await route();
    expect(v.route.name).toBe("Engineer trip");
    expect(v.parked).toBeNull();
    history.replaceState(null, "", "/");
  });
});
