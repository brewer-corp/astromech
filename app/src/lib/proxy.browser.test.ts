import { beforeEach, describe, expect, it, vi } from "vitest";
import { coreCall, coreStatus } from "./core";
import { downloadNeutron, resetNeutronDownload, type NeutronProgress } from "./neutron";
import { loadTargets } from "./powerplay";

// In the app every call goes through the shell's one proxy, `call(cmd, args, progress)`
// (app/src-tauri/src/window.rs): what the window hands it, and what it makes of the answers.
const tauri = vi.hoisted(() => {
  // Before any import: the transport checks for Tauri on each call.
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return { invoke: vi.fn() };
});
vi.mock("@tauri-apps/api/core", () => ({
  invoke: tauri.invoke,
  // The shell sends a command's progress values down this channel.
  Channel: class {
    constructor(readonly onmessage: (p: unknown) => void) {}
  },
}));
vi.mock("@tauri-apps/api/event", () => ({ listen: () => Promise.resolve(() => {}) }));

type Sent = { cmd: string; args: unknown; progress: { onmessage: (p: unknown) => void } };
const sent = (): Sent[] => tauri.invoke.mock.calls.map(([proxy, a]) => (expect(proxy).toBe("call"), a as Sent));

beforeEach(() => {
  tauri.invoke.mockReset();
  resetNeutronDownload();
});

describe("the window's calls, through the shell's proxy", () => {
  it("sends a command's args under the names the core takes", async () => {
    tauri.invoke.mockResolvedValue({ hits: [] });
    await loadTargets("acquire", null, null, 30);
    expect(sent()).toMatchObject([{ cmd: "powerplay_targets", args: { target: "acquire", origin: null, filters: null, within_ly: 30 } }]);
  });

  it("sends a string as it is (the proxy makes it the request's text) and no args as null", async () => {
    tauri.invoke.mockResolvedValue({ ok: true });
    await coreCall("voice_test", "gear down");
    await coreCall("find_elite");
    expect(sent()).toMatchObject([
      { cmd: "voice_test", args: "gear down" },
      { cmd: "find_elite", args: null },
    ]);
  });

  it("gives a core command's failed call as its reply's error, and a failed status as no core", async () => {
    tauri.invoke.mockRejectedValue("the core is not running");
    expect(await coreCall("eddn_log")).toEqual({ ok: false, error: "the core is not running" });
    expect(await coreStatus()).toBeNull();
    expect(sent().map((s) => s.cmd)).toEqual(["eddn_log", "status"]);
  });

  it("publishes a command's progress on the command's topic", async () => {
    let answer!: (v: unknown) => void;
    tauri.invoke.mockReturnValue(new Promise((r) => (answer = r)));
    const seen: NeutronProgress[] = [];
    const w = downloadNeutron((p) => seen.push(p));
    await vi.waitFor(() => expect(sent()).toHaveLength(1));
    const [{ cmd, progress }] = sent() as [Sent];
    expect(cmd).toBe("neutron_download");
    progress.onmessage({ done: 1, total: 4 });
    progress.onmessage({ done: 4, total: 4 });
    answer({ file: null, error: null });
    await w.promise;
    expect(seen).toEqual([
      { done: 1, total: 4 },
      { done: 4, total: 4 },
    ]);
  });
});
