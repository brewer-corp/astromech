import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import App from "../../App.svelte";
import { answer, idle, previewAnswer, startPreview } from "../../test/transport";

// The status bar in the window's frame, against the recorded core (`?core=ready`): the engineering
// sample's current ship, and the ship world's fuel and cargo (fixtures/ship-status.json).
const bar = () => screen.getByRole("contentinfo");

beforeEach(() => {
  startPreview("/?core=ready");
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
  location.hash = "";
});

describe("the status bar", () => {
  it("shows the ship, and its fuel and cargo from the next tick after Elite starts", async () => {
    let running = false;
    const outside = { ok: true, ship_id: null, fuel: null, fuel_capacity: null, cargo: null, cargo_capacity: null };
    answer("ship_status", () => (running ? previewAnswer("ship_status") : outside));
    render(App);
    await idle();
    expect(bar().textContent).toContain("Heavy Lifter");
    expect(bar().textContent).toContain("Anaconda");
    expect(bar().textContent).toContain("Jump18.0ly");
    expect(bar().textContent).not.toContain("Fuel");
    expect(bar().textContent).not.toContain("Cargo");

    running = true;
    await vi.advanceTimersByTimeAsync(2000);
    await idle();
    expect(bar().textContent).toContain("Fuel24.6|32t");
    expect(bar().textContent).toContain("Cargo120|256t");
    expect(bar().textContent).toContain("Heavy Lifter");
  });

  it("shows the new jump range after the current ship is refitted, without a ship swap", async () => {
    // The recorded replies, adjusted as a later Loadout of the same ship 3 would change them.
    let refitted = false;
    answer("ship_status", async () => {
      const s = await previewAnswer("ship_status");
      return refitted ? { ...s, max_jump_range: 21.4 } : s;
    });
    answer("engineering_fleet", async () => {
      const f = await previewAnswer("engineering_fleet");
      if (!refitted) return f;
      return { ...f, ships: f.ships.map((s: { current: boolean }) => (s.current ? { ...s, max_jump_range: 21.4 } : s)) };
    });
    render(App);
    await idle();
    expect(bar().textContent).toContain("Jump18.0ly");

    refitted = true;
    await vi.advanceTimersByTimeAsync(2000);
    await idle();
    expect(bar().textContent).toContain("Jump21.4ly");
  });

  it("keeps the ship and drops fuel and cargo when the call fails, with no error", async () => {
    answer("ship_status", () => Promise.reject("the core didn't answer"));
    render(App);
    await idle();
    const text = bar().textContent ?? "";
    expect(text).toContain("Heavy Lifter");
    expect(text).not.toContain("Fuel");
    expect(text).not.toContain("Cargo");
    expect(text).not.toContain("didn't answer");
    expect(bar().querySelector(".failure")).toBeNull();
  });
});
