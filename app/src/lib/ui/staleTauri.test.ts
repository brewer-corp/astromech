import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen, waitFor } from "@testing-library/svelte";
import type { Component } from "svelte";
import fleet from "../fixtures/engineering.json";
import materials from "../fixtures/materials.json";
import eddnSample from "../fixtures/eddn-log.json";
import miningSample from "../fixtures/mining.json";
import Ships from "../../modules/engineering/View.svelte";
import Materials from "../../modules/materials/View.svelte";
import DataSharing from "../../modules/datasharing/View.svelte";
import Mining from "../../modules/mining/View.svelte";

// The production path (astromech-spxx): the real loaders in the app, with the shell's calls
// answering, then failing the way the shell fails (a rejected command, or a core_call reply of
// `{ ok: false }` when the core is down), then answering again. The preview's `?fail=` throws from
// inside the loader, so it cannot show a loader that swallows the failure.
const shell = vi.hoisted(() => {
  // Before any import: `inTauri` is read once, when lib/core.ts loads.
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return { mode: "ok" as "ok" | "reject" | "core_down", invoke: vi.fn() };
});
vi.mock("@tauri-apps/api/core", () => ({ invoke: shell.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: () => Promise.resolve(() => {}) }));

Element.prototype.scrollIntoView = () => {};

const answers: Record<string, unknown> = {
  engineering_fleet: fleet.fleet,
  engineering_headlines: {},
  engineering_engineers: { engineers: [], here: null },
  materials_inventory: materials,
  core_status: { ok: true, mining: miningSample },
};
const coreReplies: Record<string, unknown> = {
  eddn_log: { ok: true, ...eddnSample },
  edsm_status: { ok: true, enabled: true, commander: "Fixture", has_key: true, key_error: null, pending: 0, last_error: null, log: [], skipped: [] },
};

beforeEach(() => {
  shell.mode = "ok";
  shell.invoke.mockReset();
  shell.invoke.mockImplementation(async (cmd: string, args?: { cmd?: string }) => {
    if (shell.mode === "reject") throw "the shell could not reach the core";
    if (cmd === "core_call") {
      if (shell.mode === "core_down") return { ok: false, error: "the core is not running" };
      return coreReplies[args?.cmd ?? ""] ?? { ok: false, error: "no stub" };
    }
    if (!(cmd in answers)) throw `no stub for ${cmd}`;
    return structuredClone(answers[cmd]);
  });
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

const PAGES: [string, "reject" | "core_down", Component, RegExp, number][] = [
  ["Ships", "reject", Ships as unknown as Component, /Heavy Lifter/, 1000],
  ["Materials", "reject", Materials as unknown as Component, /Carbon/, 1000],
  ["Data sharing (EDDN and EDSM)", "core_down", DataSharing as unknown as Component, /Waiting to send/, 2000],
  ["Data sharing (EDDN and EDSM)", "reject", DataSharing as unknown as Component, /Waiting to send/, 2000],
  ["Mining", "reject", Mining as unknown as Component, /Platinum/, 3000],
];

describe.each(PAGES)("%s, in the app, when the shell fails by %s", (_n, how, view, data, every) => {
  it("keeps its data, says the refresh failed, and clears the line on the next good read", async () => {
    render(view);
    await waitFor(() => expect(document.body.textContent).toMatch(data));
    expect(screen.queryByText(/Could not/)).toBeNull();

    shell.mode = how;
    await vi.advanceTimersByTimeAsync(every);
    expect((await screen.findAllByText(/Could not refresh: /)).length).toBeGreaterThan(0);
    expect(document.body.textContent).toMatch(data);
    expect(document.body.textContent).not.toMatch(/isn't running/);

    shell.mode = "ok";
    await vi.advanceTimersByTimeAsync(every);
    await waitFor(() => expect(screen.queryByText(/Could not/)).toBeNull());
    expect(document.body.textContent).toMatch(data);
  });
});
