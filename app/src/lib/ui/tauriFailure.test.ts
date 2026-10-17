import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import type { Component } from "svelte";
import { loadFleet } from "../engineering";
import { loadMaterials } from "../materials";
import Ships from "../../modules/engineering/View.svelte";
import Materials from "../../modules/materials/View.svelte";

// The real app (inTauri) with the shell's calls failing the way it fails with no journal folder.
// The pages must say so, never fall back to the sample fleet or inventory (astromech-32i6).
const invoke = vi.hoisted(() => {
  // Before any import: `inTauri` is read once, when lib/core.ts loads.
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  return vi.fn();
});
vi.mock("@tauri-apps/api/core", () => ({ invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: () => Promise.resolve(() => {}) }));

Element.prototype.scrollIntoView = () => {};
const REASON = "journal folder not found: C:\\Users\\cmdr\\Saved Games\\Frontier Developments\\Elite Dangerous";

beforeEach(() => {
  invoke.mockReset();
  invoke.mockRejectedValue(REASON);
});
afterEach(cleanup);

describe("no readable journal folder, in the app", () => {
  it("loadFleet rejects with the reason instead of returning the sample", async () => {
    await expect(loadFleet()).rejects.toBe(REASON);
  });

  it("loadMaterials rejects with the reason instead of returning the sample", async () => {
    await expect(loadMaterials()).rejects.toBe(REASON);
  });

  it.each([
    ["Ships", Ships, /Could not read the fleet: journal folder not found/],
    ["Materials", Materials, /Could not read the materials: journal folder not found/],
  ])("%s says the journal could not be read, with no sample data", async (_n, view, msg) => {
    render(view as unknown as Component);
    expect(await screen.findByText(msg)).toBeTruthy();
    expect(screen.queryByText(/Sample journal/)).toBeNull();
    expect(document.body.textContent).not.toMatch(/Sample journal|CMDR Fixture/);
    // One error line, not the same reason again for the ship stats.
    expect(screen.queryAllByText(/journal folder not found/)).toHaveLength(1);
  });
});
