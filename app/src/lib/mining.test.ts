import { beforeEach, describe, expect, it, vi } from "vitest";

// The real app (Tauri) path: the core's status carries the run; it is null without a session.
const status = vi.hoisted(() => vi.fn());
vi.mock("./core", () => ({ inTauri: true, coreStatus: status }));

import { loadMining } from "./mining";

beforeEach(() => status.mockReset());

describe("loadMining in the app", () => {
  it("rejects while the core does not answer, and is null while no session has a run", async () => {
    status.mockResolvedValue(null);
    await expect(loadMining()).rejects.toThrow("the core did not answer");
    status.mockResolvedValue({ mining: null });
    expect(await loadMining()).toBeNull();
  });

  it("is the core's run when there is one", async () => {
    const run = { started: "2026-10-16T20:00:00Z", refined: { painite: 2 }, total: 2, per_hour: null };
    status.mockResolvedValue({ mining: run });
    expect(await loadMining()).toEqual(run);
  });
});
