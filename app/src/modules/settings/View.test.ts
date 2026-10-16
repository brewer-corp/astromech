import { describe, expect, it } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import View from "./View.svelte";
import App from "../../App.svelte";
import { resetFakeCore, type Scenario } from "../../lib/fakecore";

function on(scenario: Scenario, hash = "") {
  history.replaceState(null, "", `/?core=${scenario}${hash}`);
  resetFakeCore();
}

describe("Settings → Microphone", () => {
  it("shows the chosen mic and its stored gate", async () => {
    on("ready");
    render(View);
    expect(await screen.findByText("Microphone (Example USB Mic)")).toBeTruthy();
    expect(screen.getByText("Ignores clips quieter than -51 dBFS")).toBeTruthy();
  });

  it("calibrates from Settings and shows the new gate", async () => {
    on("new");
    render(View);
    await fireEvent.click(await screen.findByRole("button", { name: "Calibrate" }));
    expect(screen.getByRole("dialog", { name: "Calibrate the microphone" })).toBeTruthy();
    await fireEvent.click(await screen.findByRole("button", { name: "Record silence" }));
    for (let i = 0; i < 3; i++) {
      await screen.findByText(/as you would in the cockpit/);
      await fireEvent.click(screen.getByRole("button", { name: "Record" }));
    }
    await screen.findByText("Ignore clips quieter than");
    await fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(await screen.findByText("Ignores clips quieter than -51 dBFS")).toBeTruthy();
  });
});

describe("first launch", () => {
  it("opens onboarding until it's finished", async () => {
    on("new");
    render(App);
    expect(await screen.findByRole("heading", { name: "Find Elite" })).toBeTruthy();
  });

  it("opens the app once onboarding is done, and #/setup/<step> re-runs a step", async () => {
    on("ready", "#/settings");
    render(App);
    expect(await screen.findByText("Calibration", { selector: "dt" })).toBeTruthy();
    expect(screen.queryByRole("heading", { name: "Find Elite" })).toBeNull();
    location.hash = "#/setup/mic";
    expect(await screen.findByRole("heading", { name: "Microphone" })).toBeTruthy();
  });
});
