import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import type { Component } from "svelte";
import Route from "../../modules/route/View.svelte";
import Edsm from "../../modules/datasharing/Edsm.svelte";
import { pollFailure } from "./errors";

// The two polling pages the preview's recorded core cannot drive (no route; EDSM status is a fixed
// sample): their calls are mocked, good once and then failing, then good again (astromech-spxx).
const route = vi.hoisted(() => ({ get: vi.fn(), skip: vi.fn() }));
const edsm = vi.hoisted(() => ({ status: vi.fn() }));
vi.mock("../route", async (orig) => ({ ...(await orig<typeof import("../route")>()), getRoute: route.get, skipWaypoint: route.skip }));
vi.mock("../edsm", async (orig) => ({ ...(await orig<typeof import("../edsm")>()), edsmStatus: edsm.status }));

const VIEW = { route: { name: "Colonia run", source: "test", waypoints: [{ system: "Sol" }, { system: "Colonia" }] }, here: "Sol", next: { system: "Colonia", label: null }, remaining: 2, clipboard_error: null };
const STATUS = { ok: true, enabled: true, commander: "Fixture", has_key: true, key_error: null, pending: 0, last: null, last_error: null };

beforeEach(() => vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] }));
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

describe("EDSM, first read failing", () => {
  it("says it could not read the status", async () => {
    edsm.status.mockReset();
    edsm.status.mockRejectedValue("the core is not running");
    render(Edsm as unknown as Component);
    expect(await screen.findByText("Could not read the EDSM status: the core is not running.")).toBeTruthy();
  });
});

describe("EDSM, first read failing then working", () => {
  it("fills the commander name from the first read that works, so Save cannot write an empty name", async () => {
    edsm.status.mockReset();
    edsm.status.mockRejectedValueOnce("the core is not running");
    edsm.status.mockResolvedValue({ ...STATUS, commander: "Jameson" });
    render(Edsm as unknown as Component);
    await screen.findByText(/Could not read the EDSM status/);
    await vi.advanceTimersByTimeAsync(2000);
    const field = (await screen.findByLabelText("Commander name")) as HTMLInputElement;
    expect(field.value).toBe("Jameson");
  });
});

describe("Route, an action failing", () => {
  it("keeps the action's error through good polls until the commander acts again", async () => {
    route.get.mockReset();
    route.get.mockResolvedValue(VIEW);
    route.skip.mockReset();
    route.skip.mockRejectedValueOnce("the clipboard is busy");
    render(Route as unknown as Component);
    await screen.findByText("Colonia run");

    await fireEvent.click(screen.getByRole("button", { name: "Skip" }));
    expect(await screen.findByText("the clipboard is busy")).toBeTruthy();
    await vi.advanceTimersByTimeAsync(6000);
    expect(route.get.mock.calls.length).toBeGreaterThan(3);
    expect(screen.getByText("the clipboard is busy")).toBeTruthy();

    route.skip.mockResolvedValueOnce(VIEW);
    await fireEvent.click(screen.getByRole("button", { name: "Skip" }));
    await waitFor(() => expect(screen.queryByText("the clipboard is busy")).toBeNull());
  });
});

describe("pollFailure", () => {
  it("is the full error with nothing shown and 'Could not refresh' once there is data", () => {
    expect(pollFailure("read the fleet", "journal folder not found: C:\\x", false)).toBe("Could not read the fleet: journal folder not found: C:\\x.");
    expect(pollFailure("read the fleet", new Error("Error: busy."), true)).toBe("Could not refresh: busy.");
  });
});

describe.each([
  ["Route", Route as unknown as Component, route.get, VIEW, /Colonia run/],
  ["EDSM", Edsm as unknown as Component, edsm.status, STATUS, /Upload to EDSM/],
] as const)("%s", (_n, view, call, good, data) => {
  it("keeps its data and says the refresh failed, then clears the line on the next good read", async () => {
    call.mockReset();
    call.mockResolvedValue(good);
    render(view);
    await waitFor(() => expect(document.body.textContent).toMatch(data));

    call.mockRejectedValue("the core did not answer");
    await vi.advanceTimersByTimeAsync(2000);
    expect((await screen.findByText(/Could not refresh: the core did not answer\./)).textContent).toMatch(/^! /);
    expect(document.body.textContent).toMatch(data);

    call.mockResolvedValue(good);
    await vi.advanceTimersByTimeAsync(2000);
    await waitFor(() => expect(screen.queryByText(/Could not/)).toBeNull());
    expect(document.body.textContent).toMatch(data);
  });
});
