import { describe, expect, it, vi } from "vitest";
import { changeFilters, openFilters, type FilterStore } from "./filterstate";
import { DEFAULT_FILTERS, type Filters } from "./search";

const D = DEFAULT_FILTERS;
const fail = () => Promise.reject(new Error("database is locked"));

describe("openFilters", () => {
  it("falls back to the defaults (and logs) when the saved filters can't be read", async () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const st = await openFilters({ load: fail, save: fail }, "faction.famine", D);
    expect(st).toEqual({ filters: D, error: null });
    expect(warn).toHaveBeenCalledOnce();
    warn.mockRestore();
  });

  it("returns the saved filters", async () => {
    const f: Filters = { ...D, max_age_days: 7 };
    expect(await openFilters({ load: async () => f, save: fail }, "x", D)).toEqual({ filters: f, error: null });
  });
});

describe("changeFilters", () => {
  it("keeps the old filters and says why when the save fails", async () => {
    const prev: Filters = { ...D, max_age_days: 7 };
    const next: Filters = { ...D, max_age_days: 1 };
    const st = await changeFilters({ load: async () => next, save: fail }, "x", prev, next);
    expect(st).toEqual({ filters: prev, error: "Couldn't save the filter: database is locked" });
  });

  it("reloads after saving, so the pad goes back to the ship's", async () => {
    const prev: Filters = { ...D, pad: "S", pad_set: true };
    const next: Filters = { ...D, pad: "S", pad_set: false };
    let stored = D;
    const store: FilterStore = {
      load: async () => ({ ...stored, pad: stored.pad_set ? stored.pad : "L" }),
      save: async (_, f) => void (stored = f),
    };
    expect(await changeFilters(store, "x", prev, next)).toEqual({ filters: { ...next, pad: "L" }, error: null });
  });

  it("keeps the new filters when only the reload fails", async () => {
    const next: Filters = { ...D, max_age_days: 3 };
    expect(await changeFilters({ load: fail, save: async () => {} }, "x", D, next)).toEqual({ filters: next, error: null });
  });
});
