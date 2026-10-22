import { describe, expect, it } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/svelte";
import View from "./View.svelte";
import App from "../../App.svelte";
import { fakeCall, fakePreviews, resetFakeCore, type Scenario } from "../../lib/fakecore";

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

describe("Settings → Voice responses", () => {
  it("lists British voices first and saves the speaker, colour and speed", async () => {
    on("ready");
    render(View);
    const speaker = (await screen.findByLabelText("Speaker")) as HTMLSelectElement;
    expect(speaker.value).toBe("bf_emma");
    const groups = [...speaker.querySelectorAll("optgroup")].map((g) => g.label);
    expect(groups).toEqual(["British", "American"]);
    await fireEvent.change(speaker, { target: { value: "bm_george" } });
    expect(await screen.findByText("Saved. The next reply uses it.")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Natural" }));
    await fireEvent.change(screen.getByLabelText("Speed"), { target: { value: "0.9" } });
    const saved = await fakeCall("voice_settings");
    expect([saved.speaker, saved.colour, saved.speed]).toEqual(["bm_george", "natural", 0.9]);
    expect(screen.getByRole("button", { name: "Natural" }).getAttribute("aria-pressed")).toBe("true");
  });

  it("previews the chosen voice", async () => {
    on("ready");
    render(View);
    await fireEvent.change(await screen.findByLabelText("Speaker"), { target: { value: "bf_lily" } });
    await fireEvent.click(screen.getByRole("button", { name: "Preview" }));
    expect(await screen.findByText("“Landing gear deployed.”")).toBeTruthy();
    expect(fakePreviews()).toEqual(["bf_lily"]);
  });

  it.each([
    [0.94, "0.9"],
    [2.0, "1.3"],
    [0.5, "0.7"],
  ])("shows a hand-edited speed %s as the nearest step %s", async (speed, shown) => {
    on("ready");
    await fakeCall("set_voice", { speed });
    render(View);
    const select = (await screen.findByLabelText("Speed")) as HTMLSelectElement;
    expect(select.value).toBe(shown);
  });

  it("refuses a preview before the speech models are downloaded", async () => {
    on("new");
    render(View);
    await fireEvent.click(await screen.findByRole("button", { name: "Preview" }));
    expect(await screen.findByText(/download the speech models first/)).toBeTruthy();
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

// The poll reads the core's status every second; a failed read is null (lib/core.ts coreStatus).
describe("Settings, a status read failing after a good one (astromech-cvxf)", () => {
  it("keeps the settings shown, says the refresh failed, and clears the line on the next good read", async () => {
    on("ready");
    render(View);
    await screen.findByText("Start with Windows");
    history.replaceState(null, "", "/?core=ready&fail=core_status");
    expect(await screen.findByText("Could not refresh: the core did not answer.", {}, { timeout: 2500 })).toBeTruthy();
    expect(screen.getByText("Start with Windows")).toBeTruthy();
    expect(screen.queryByText(/isn't running/)).toBeNull();
    history.replaceState(null, "", "/?core=ready");
    await waitFor(() => expect(screen.queryByText(/Could not refresh/)).toBeNull(), { timeout: 2500 });
  });
});
