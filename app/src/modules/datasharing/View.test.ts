import { describe, expect, it } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import View from "./View.svelte";
import { fakeCall, resetFakeCore } from "../../lib/fakecore";

function on(hash = "") {
  history.replaceState(null, "", `/?core=ready${hash}`);
  resetFakeCore();
}

describe("Settings → Data sharing", () => {
  it("is off until switched on, and saves the switch", async () => {
    on();
    render(View);
    expect(await screen.findByText("Off")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Turn on" }));
    expect(await screen.findByText(/Sharing is on/)).toBeTruthy();
    expect((await fakeCall("eddn_log")).enabled).toBe(true);
    expect(await screen.findByText("On")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Turn off" }));
    expect(await screen.findByText(/Sharing is off/)).toBeTruthy();
    expect((await fakeCall("eddn_log")).enabled).toBe(false);
  });

  it("shows the queue, the last send and the last error from a failed send", async () => {
    on();
    render(View);
    await fireEvent.click(await screen.findByRole("button", { name: "Turn on" }));
    expect(await screen.findByText(/journal\/1 · HTTP 503: Service Unavailable; retry in 60 s/)).toBeTruthy();
    expect(screen.getByText(/commodity\/3$/)).toBeTruthy();
    expect(await screen.findByText("1")).toBeTruthy();
  });

  it("says plainly what gets sent", async () => {
    on();
    render(View);
    expect(await screen.findByText(/Your credits, reputation, cargo, missions/)).toBeTruthy();
  });
});
