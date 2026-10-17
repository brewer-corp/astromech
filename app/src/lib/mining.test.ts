import { beforeEach, describe, expect, it, vi } from "vitest";

// The real app (Tauri) path: the core's status says whether it runs and whether a session has a run.
const status = vi.hoisted(() => vi.fn());
vi.mock("./core", () => ({ inTauri: true, coreStatus: status }));

import { loadMining } from "./mining";

beforeEach(() => status.mockReset());

describe("loadMining in the app", () => {
  it("is null only while the core is not running", async () => {
    status.mockResolvedValue(null);
    expect(await loadMining()).toBeNull();
  });

  it("is an empty run while the core runs without a session", async () => {
    status.mockResolvedValue({ mining: null });
    expect(await loadMining()).toEqual({ started: null, refined: {}, total: 0, per_hour: null });
  });

  it("is the core's run when there is one", async () => {
    const run = { started: "2026-10-16T20:00:00Z", refined: { painite: 2 }, total: 2, per_hour: null };
    status.mockResolvedValue({ mining: run });
    expect(await loadMining()).toEqual(run);
  });
});
