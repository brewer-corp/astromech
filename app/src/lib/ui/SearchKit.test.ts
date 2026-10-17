import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { DEFAULT_FILTERS, type Filters, type Origin, type Results } from "../search";
import SearchKit from "./SearchKit.svelte";

// Storage fails: the kit must still search (with the defaults) instead of showing nothing.
const loadFilters = vi.fn<(lookup: string) => Promise<Filters>>();
const saveFilters = vi.fn<(lookup: string, f: Filters) => Promise<void>>();
vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  loadFilters: (l: string) => loadFilters(l),
  saveFilters: (l: string, f: Filters) => saveFilters(l, f),
}));

const results: Results = {
  origin: "Sol",
  range: 20,
  source: "Spansh",
  fetched: "2026-10-16T12:00:00Z",
  rows: [{ key: 1, system: "Ross 154", ly: 9.7, jumps: 1, exact: true, updated_at: "2026-10-16T11:00:00Z", detail: "Ross 154 Boys" }],
};

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
});
