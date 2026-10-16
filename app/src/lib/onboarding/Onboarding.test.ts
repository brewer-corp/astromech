import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent, within } from "@testing-library/svelte";
import Onboarding from "./Onboarding.svelte";
import { fakeStatus, resetFakeCore, type Scenario } from "../fakecore";
import { binds } from "../store.svelte";

// Every step runs against a recorded core (fakecore.ts): `empty` is a PC with no Elite install,
// no controllers and no microphone; `new` is a fresh install on a PC that has them.
function on(scenario: Scenario) {
  history.replaceState(null, "", `/?core=${scenario}`);
  resetFakeCore();
}

const button = (name: string) => screen.getByRole("button", { name });
const step = (start: string) => render(Onboarding, { start, onfinish: vi.fn() });
const press = (key: string) =>
  binds.press({ device: "Keyboard", key, kind: "key", down: true, device_name: "Keyboard", t_ms: performance.now() });

describe("onboarding: find Elite", () => {
  beforeEach(() => on("empty"));

  it("says when Elite isn't installed and accepts a Bindings folder", async () => {
    step("elite");
    expect(await screen.findByText(/hasn't run on this Windows account/)).toBeTruthy();
    expect(screen.getAllByText("Not found")).toHaveLength(3);
    const input = screen.getByLabelText("Bindings folder");
    expect((button("Use this folder") as HTMLButtonElement).disabled).toBe(true);
    await fireEvent.input(input, { target: { value: "D:\\Elite\\Bindings" } });
    await fireEvent.click(button("Use this folder"));
    expect(await screen.findByText("D:\\Elite\\Bindings")).toBeTruthy();
    expect(await screen.findByText(/Found Elite/)).toBeTruthy();
  });

  it("finds Elite's folders on a PC that has it", async () => {
    on("new");
    step("elite");
    expect(await screen.findByText(/Found Elite/)).toBeTruthy();
    expect(screen.getByText("Steam")).toBeTruthy();
  });
});

describe("onboarding: controllers", () => {
  it("says when no controllers are connected, and shows presses", async () => {
    on("empty");
    step("devices");
    expect(await screen.findByText("No sticks, throttles or pads found.")).toBeTruthy();
    press("Key_F13");
    expect(await screen.findByText("Last press: F13")).toBeTruthy();
  });

  it("lists connected sticks with their stock map", async () => {
    on("new");
    step("devices");
    expect(await screen.findByText("VKBsim Gladiator EVO R")).toBeTruthy();
    expect(screen.getAllByText(/stock map/)).toHaveLength(2);
  });
});

describe("onboarding: push-to-talk and kill switch", () => {
  beforeEach(() => on("empty"));

  it("binds push-to-talk by pressing it, and saves it to the config", async () => {
    step("buttons");
    await screen.findAllByText("Not set");
    await fireEvent.click(screen.getAllByRole("button", { name: "Set" })[0]!);
    expect(screen.getByText("Press the button you'll hold to speak.")).toBeTruthy();
    expect((button("Save") as HTMLButtonElement).disabled).toBe(true);
    press("Key_F13");
    expect(await screen.findByText("F13", { selector: ".press" })).toBeTruthy();
    await fireEvent.click(button("Save"));
    expect(await screen.findByText("Saved.")).toBeTruthy();
    expect(fakeStatus().ptt).toBe("Keyboard Key_F13");
  });

  it("won't use the push-to-talk button as the kill switch", async () => {
    step("buttons");
    await screen.findAllByText("Not set");
    await fireEvent.click(screen.getAllByRole("button", { name: "Set" })[0]!);
    press("Key_F13");
    await fireEvent.click(await screen.findByRole("button", { name: "Save" }));
    await screen.findByText("Saved.");
    // Wait for the status poll to show the new button, then bind the kill switch.
    await screen.findByRole("button", { name: "Change" }, { timeout: 2000 });
    await fireEvent.click(button("Set"));
    press("Key_F13");
    expect(await screen.findByText(/already the push-to-talk button/)).toBeTruthy();
    expect((button("Save") as HTMLButtonElement).disabled).toBe(true);
    expect(fakeStatus().kill_switch).toBeNull();
  });
});

describe("onboarding: microphone", () => {
  it("says when there is no microphone", async () => {
    on("empty");
    step("mic");
    expect(await screen.findByText("No microphones found.")).toBeTruthy();
  });

  it("picks a microphone by name and checks its level", async () => {
    on("new");
    step("mic");
    const row = (await screen.findByText("Headset Microphone (Example VR Headset)")).closest("dd")!;
    await fireEvent.click(within(row as HTMLElement).getByRole("button", { name: "Use" }));
    expect(await within(row as HTMLElement).findByText("Chosen")).toBeTruthy();
    expect(fakeStatus().mic).toBe("Headset Microphone (Example VR Headset)");
    await fireEvent.click(button("Check level"));
    expect(await screen.findByText(/Good level/)).toBeTruthy();
  });
});

describe("onboarding: calibration", () => {
  it("reports a missing microphone instead of calibrating", async () => {
    on("empty");
    step("calibrate");
    await fireEvent.click(await screen.findByRole("button", { name: "Record silence" }));
    expect(await screen.findByText(/no recording device found/)).toBeTruthy();
  });

  it("records silence and three phrases, then shows the stored gate", async () => {
    on("new");
    step("calibrate");
    await fireEvent.click(await screen.findByRole("button", { name: "Record silence" }));
    for (const phrase of ["gear down", "deploy hardpoints", "request docking"]) {
      expect(await screen.findByText(`Say “${phrase}” as you would in the cockpit.`)).toBeTruthy();
      await fireEvent.click(button("Record"));
    }
    expect(await screen.findByText("Ignore clips quieter than")).toBeTruthy();
    expect(screen.getByText("-51 dBFS")).toBeTruthy();
  });
});

describe("onboarding: health check", () => {
  it("lists what's missing on an empty PC, and each fix goes to its step", async () => {
    on("empty");
    step("health");
    expect(await screen.findByText("No preset found to check")).toBeTruthy();
    expect(screen.getByText("Not downloaded (about 800 MB)")).toBeTruthy();
    expect(screen.getByText("No microphone found")).toBeTruthy();
    expect(screen.getByText(/Fix the highlighted rows/)).toBeTruthy();
    const ptt = screen.getByText("Push-to-talk", { selector: "dt" }).nextElementSibling as HTMLElement;
    await fireEvent.click(within(ptt).getByRole("button", { name: "Set" }));
    expect(screen.getByRole("heading", { name: "Push-to-talk" })).toBeTruthy();
  });

  it("fixes a voice action with no keyboard key, then is ready", async () => {
    on("ready");
    step("health");
    await fireEvent.click(await screen.findByRole("button", { name: "Bind a key" }));
    expect(screen.getByRole("dialog", { name: "Bind Night vision" })).toBeTruthy();
    press("Key_N");
    await fireEvent.click(await screen.findByRole("button", { name: "Save" }));
    await fireEvent.click(await screen.findByRole("button", { name: "Close" }));
    expect(await screen.findByText(/Ready\. Start Elite/)).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Bind a key" })).toBeNull();
  });
});

describe("onboarding flow", () => {
  it("finishing records it in the core", async () => {
    on("new");
    const onfinish = vi.fn();
    render(Onboarding, { start: "health", onfinish });
    expect(fakeStatus().onboarded).toBe(false);
    await fireEvent.click(await screen.findByRole("button", { name: "Finish" }));
    expect(onfinish).toHaveBeenCalled();
    expect(fakeStatus().onboarded).toBe(true);
  });

  it("walks the steps in order and can be skipped", async () => {
    on("new");
    const onfinish = vi.fn();
    render(Onboarding, { onfinish });
    const titles = [];
    for (let n = 0; n < 6; n++) {
      titles.push(screen.getByRole("heading", { level: 1 }).textContent);
      if (n < 5) await fireEvent.click(button("Next"));
    }
    expect(titles).toEqual(["Find Elite", "Controllers", "Push-to-talk", "Microphone", "Calibration", "Health check"]);
    await fireEvent.click(button("Back"));
    await fireEvent.click(button("Skip setup"));
    expect(onfinish).toHaveBeenCalled();
    expect(fakeStatus().onboarded).toBe(true);
  });
});
