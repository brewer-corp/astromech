import { afterEach, describe, expect, it } from "vitest";
import { tradeRouteDefaults } from "./trade";
import { fakeCall, previewData, previewGate, resetFakeCore } from "./fakecore";

// The preview's switches for seeing a page empty, loading or failing (top of fakecore.ts).
const at = (query: string) => {
  history.replaceState(null, "", `/${query}`);
  resetFakeCore();
};
const sample = () => "sample";
const empty = () => "empty";
const settled = (p: Promise<unknown>) => Promise.race([p.then(() => "settled", () => "settled"), new Promise((r) => setTimeout(() => r("pending"), 60))]);

afterEach(() => at(""));

describe("previewData", () => {
  it("answers the sample without a scenario, the empty answer under ?core=empty", async () => {
    at("");
    expect(await previewData("logbook", sample, empty)).toBe("sample");
    at("?core=ready");
    expect(await previewData("logbook", sample, empty)).toBe("sample");
    at("?core=empty");
    expect(await previewData("logbook", sample, empty)).toBe("empty");
  });
});

describe("sample-backed calls under ?core=empty", () => {
  it("know no ship, cash or station for the trade route", async () => {
    at("?core=ready");
    expect((await tradeRouteDefaults()).station).toBe("Abraham Lincoln");
    at("?core=empty");
    expect(await tradeRouteDefaults()).toEqual({ capacity: null, credits: null, range: null, station: null, system: null });
  });
});

describe("?fail=", () => {
  it("rejects only the named calls", async () => {
    at("?fail=logbook,session_summary");
    await expect(previewData("logbook", sample, empty)).rejects.toMatch(/logbook failed on purpose/);
    await expect(previewData("session_summary", sample, empty)).rejects.toMatch(/session_summary/);
    expect(await previewData("materials_inventory", sample, empty)).toBe("sample");
  });

  it("rejects everything for all, core commands included", async () => {
    at("?core=ready&fail=all");
    await expect(previewData("engineering_fleet", sample, empty)).rejects.toBeTruthy();
    await expect(fakeCall("eddn_log")).rejects.toMatch(/eddn_log failed on purpose/);
  });
});

describe("?slow=", () => {
  it("never answers for never, for every call or only the named ones", async () => {
    at("?slow=never");
    expect(await settled(previewGate("logbook"))).toBe("pending");
    at("?slow=never:logbook");
    expect(await settled(previewGate("logbook"))).toBe("pending");
    expect(await settled(previewGate("session_summary"))).toBe("settled");
  });

  it("waits the given milliseconds, then answers", async () => {
    at("?slow=30");
    const t = Date.now();
    expect(await previewData("logbook", sample, empty)).toBe("sample");
    expect(Date.now() - t).toBeGreaterThanOrEqual(25);
  });

  it("can be slow and then fail", async () => {
    at("?slow=20&fail=logbook");
    await expect(previewGate("logbook")).rejects.toBeTruthy();
  });
});
