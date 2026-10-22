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
  // `only`: the core_call commands that fail (the rest answer); null: every call fails.
  return { mode: "ok" as "ok" | "reject" | "core_down", only: null as Set<string> | null, invoke: vi.fn() };
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
  shell.only = null;
  answers.core_status = { ok: true, mining: miningSample };
  shell.invoke.mockReset();
  shell.invoke.mockImplementation(async (cmd: string, args?: { cmd?: string }) => {
    const failing = shell.only === null || (cmd === "core_call" && shell.only.has(args?.cmd ?? ""));
    if (shell.mode === "reject" && failing) throw "the shell could not reach the core";
    // The shell's core_status is `call("status").ok()`: null while the core is down.
    if (cmd === "core_status" && shell.mode === "core_down") return null;
    if (cmd === "core_call") {
      if (shell.mode === "core_down" && failing) return { ok: false, error: "the core is not running" };
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
  ["Mining", "core_down", Mining as unknown as Component, /Platinum/, 3000],
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

// The "Could not refresh" line under the EDSM heading, and the one at the top of the EDDN section.
const edsmLine = () => screen.getByRole("heading", { name: "EDSM" }).nextElementSibling?.textContent ?? "";
const eddnLine = () => screen.getByRole("heading", { name: "EDDN" }).previousElementSibling?.textContent ?? "";

describe.each(["reject", "core_down"] as const)("Data sharing, in the app, when only EDSM's call fails by %s", (how) => {
  it("shows the refresh failure under the EDSM heading and none in the EDDN section", async () => {
    render(DataSharing as unknown as Component);
    await screen.findByRole("heading", { name: "EDSM" });
    expect(edsmLine()).not.toMatch(/Could not/);

    shell.mode = how;
    shell.only = new Set(["edsm_status"]);
    await vi.advanceTimersByTimeAsync(2000);
    await waitFor(() => expect(edsmLine()).toMatch(/^! Could not refresh: the (shell could not reach the core|core is not running)\.$/));
    expect(eddnLine()).toBe("");
    expect(screen.getAllByText(/Could not/)).toHaveLength(1);

    shell.mode = "ok";
    await vi.advanceTimersByTimeAsync(2000);
    await waitFor(() => expect(edsmLine()).not.toMatch(/Could not/));
  });
});

// A first read that fails leaves nothing to show, so the page says what it could not read. On Data
// sharing a failed EDDN read stands for the page (EDSM's section waits for it); `only` fails EDSM's
// call alone.
const FIRST: [string, "reject" | "core_down", string[] | null, Component, string][] = [
  ["Ships", "reject", null, Ships as unknown as Component, "Could not read the fleet: the shell could not reach the core."],
  ["Materials", "reject", null, Materials as unknown as Component, "Could not read the materials: the shell could not reach the core."],
  ["Data sharing", "reject", null, DataSharing as unknown as Component, "Could not read the sharing log: the shell could not reach the core."],
  ["Data sharing", "core_down", null, DataSharing as unknown as Component, "Could not read the sharing log: the core is not running."],
  ["EDSM", "reject", ["edsm_status"], DataSharing as unknown as Component, "Could not read the EDSM status: the shell could not reach the core."],
  ["EDSM", "core_down", ["edsm_status"], DataSharing as unknown as Component, "Could not read the EDSM status: the core is not running."],
  ["Mining", "reject", null, Mining as unknown as Component, "Could not read the mining run: the core did not answer."],
  ["Mining", "core_down", null, Mining as unknown as Component, "Could not read the mining run: the core did not answer."],
];

describe.each(FIRST)("%s, in the app, when the first read fails by %s", (_n, how, only, view, line) => {
  it("says what it could not read", async () => {
    shell.mode = how;
    shell.only = only && new Set(only);
    render(view);
    expect(await screen.findByText(line)).toBeTruthy();
    expect(screen.getAllByText(/Could not/)).toHaveLength(1);
  });
});

describe("Mining, in the app, with the core up and no session", () => {
  it("says there is no run yet, not that the core is down", async () => {
    answers.core_status = { ok: true, mining: null };
    render(Mining as unknown as Component);
    expect(await screen.findByText("No mining run yet.")).toBeTruthy();
    expect(document.body.textContent).not.toMatch(/isn't running|Could not/);
  });
});
