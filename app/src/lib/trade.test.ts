import { describe, expect, it } from "vitest";
import { DEFAULT_FILTERS } from "./search";
import { cargoResults, credits, findCommodity, suggestCommodities, toResults, tradeAnywhere, tradeCargo, tradeSearch } from "./trade";

describe("trade lookups", () => {
  it("finds commodities by name, alias or symbol", () => {
    expect(findCommodity("ltds")?.symbol).toBe("lowtemperaturediamond");
    expect(findCommodity("Void Opal")?.symbol).toBe("opal");
    expect(findCommodity("painite")?.name).toBe("Painite");
    expect(findCommodity("banana")).toBeUndefined();
    expect(suggestCommodities("pla")[0]?.name).toBe("Platinum");
    expect(suggestCommodities("diam")[0]?.name).toBe("Low Temperature Diamonds");
  });

  it("shows a market's price first and its demand or stock", async () => {
    const s = await tradeSearch("sell", ["painite"], null, { ...DEFAULT_FILTERS, max_age_days: 7 });
    const r = toResults(s);
    expect(r.source).toBe("Ardent Insight");
    expect(r).toMatchObject({ place: "station", main: { head: "Price", unit: "cr" }, columns: [{ key: "volume", head: "Demand", unit: "t" }] });
    expect(r.rows[0]).toMatchObject({ station: "Good Landing", system: "LHS 1827", main: "211,587", facts: { volume: "13,884" }, pad: "L" });
    expect(r.rows.every((x, i) => i === 0 || Number(x.main?.replace(/\D/g, "")) <= Number(r.rows[i - 1]?.main?.replace(/\D/g, "")))).toBe(true);
    const buy = toResults(await tradeSearch("buy", ["tritium"], "Shinrarta Dezhra", { ...DEFAULT_FILTERS, exclude_carriers: true, min_volume: 100 }));
    expect(buy.origin).toBe("Shinrarta Dezhra");
    expect(buy.columns?.[0]?.head).toBe("Stock");
    expect(buy.rows[0]).toMatchObject({ station: "Ray Freeport", facts: { volume: "1,872" } });
    expect(credits(2565)).toBe("2,565");
  });

  it("lists the whole-load stations with their total and how much they take", async () => {
    const c = await tradeCargo(null, DEFAULT_FILTERS);
    expect(c.plan.load.map((l) => [l.name, l.tonnes])).toEqual([["Low Temperature Diamonds", 300], ["Painite", 200], ["Platinum", 100]]);
    const r = cargoResults(c);
    expect(r).toMatchObject({ main: { head: "Total", unit: "cr" }, columns: [{ key: "takes", head: "Takes", unit: "t" }] });
    expect(r.rows[0]).toMatchObject({ station: "Good Landing", main: "183,674,700", facts: { takes: "all 600" } });
    expect(r.rows.find((x) => x.station === "Tevis Terminal")?.facts?.takes).toBe("500 of 600");
    expect(c.plan.kept.map((k) => k.reason)).toEqual(["stolen", "limpets", "mission cargo"]);
  });

  it("lists the best prices anywhere with no search radius, cheapest first when buying", async () => {
    const sell = toResults(await tradeAnywhere("sell", "painite", null, { ...DEFAULT_FILTERS, max_distance_ly: 10 }));
    expect(sell.rows[0]).toMatchObject({ station: "FCX-800", system: "Chamaeleon Sector HR-W d1-42", main: "390,780", facts: { volume: "1,632" }, station_type: "Drake-Class Carrier", updated_at: "2026-09-18T09:39:20.000Z" });
    expect(sell.rows[0]?.ly).toBeGreaterThan(500);
    expect(sell.rows).toHaveLength(12);
    const buy = toResults(await tradeAnywhere("buy", "tritium", "Colonia", { ...DEFAULT_FILTERS, exclude_carriers: true }));
    expect(buy.origin).toBe("Colonia");
    expect(buy.rows[0]).toMatchObject({ station: "Piceous Elicitation Ltd", main: "40,596" });
    expect(buy.rows.every((x, i) => i === 0 || Number(x.main?.replace(/\D/g, "")) >= Number(buy.rows[i - 1]?.main?.replace(/\D/g, "")))).toBe(true);
  });
});
