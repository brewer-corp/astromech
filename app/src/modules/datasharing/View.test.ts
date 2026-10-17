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

  it("EDSM: the name and key are saved, the key is never shown, and upload needs both", async () => {
    on();
    render(View);
    await screen.findByText("Upload to EDSM");
    const toggles = await screen.findAllByRole("button", { name: "Turn on" });
    await fireEvent.click(toggles[1]);
    expect(await screen.findByText("On, but no commander name")).toBeTruthy();
    await fireEvent.input(screen.getByLabelText("Commander name"), { target: { value: "Jameson" } });
    await fireEvent.click(screen.getByRole("button", { name: "Save commander name" }));
    expect(await screen.findByText("On, but no API key")).toBeTruthy();
    expect((await fakeCall("edsm_status")).commander).toBe("Jameson");
    const key = screen.getByLabelText("API key") as HTMLInputElement;
    expect(key.type).toBe("password");
    await fireEvent.input(key, { target: { value: "s3cret" } });
    await fireEvent.click(screen.getByRole("button", { name: "Save API key" }));
    expect(await screen.findByText(/API key saved/)).toBeTruthy();
    expect(key.value).toBe("");
    expect((await fakeCall("edsm_status")).uploading).toBe(true);
    expect(document.body.textContent).not.toContain("s3cret");
    await fireEvent.click(screen.getByRole("button", { name: "Remove API key" }));
    expect(await screen.findByText("On, but no API key")).toBeTruthy();
  });

  it("EDSM: shows the flight log", async () => {
    on();
    render(View);
    await fireEvent.input(await screen.findByLabelText("Commander name"), { target: { value: "Jameson" } });
    await fireEvent.click(screen.getByRole("button", { name: "Save commander name" }));
    await fireEvent.input(screen.getByLabelText("API key"), { target: { value: "k" } });
    await fireEvent.click(screen.getByRole("button", { name: "Save API key" }));
    await screen.findByText(/API key saved/);
    await fireEvent.click(screen.getByRole("button", { name: "Show flight log" }));
    expect(await screen.findByText("2 systems in the last week")).toBeTruthy();
    expect(screen.getByText("Achenar")).toBeTruthy();
    expect(screen.getByText("First to report")).toBeTruthy();
  });
});
