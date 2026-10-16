import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import type { DryRun, Poll } from "../../lib/voice";
import View from "./View.svelte";

// The real dry run (the browser fallback serves results recorded from the core's tester);
// polling and the live run are stubbed so the test can see whether a live run was asked for.
const poll = vi.fn<() => Promise<Poll | null>>();
const live = vi.fn<(command: string) => Promise<string>>();
vi.mock("../../lib/voice", async (orig) => ({
  ...(await orig<typeof import("../../lib/voice")>()),
  voicePoll: () => poll(),
  voiceStop: async () => {},
  voiceLive: (c: string) => live(c),
}));

const heard: DryRun = {
  source: "voice",
  transcript: "Lights on.",
  intent: { command: "lights.on", phrase: "lights on", score: 1, decision: "execute" },
  would: "press",
  message: "Lights on.",
  action: "ShipSpotLightToggle",
  keys: { key: "Key_K", modifiers: ["Key_LeftShift"], slot: "Primary" },
  live: "lights.on",
};

async function typePhrase(text: string) {
  await fireEvent.input(screen.getByLabelText("Phrase"), { target: { value: text } });
  await fireEvent.click(screen.getByRole("button", { name: "Test" }));
}

describe("Voice tester", () => {
  beforeEach(() => {
    poll.mockReset().mockResolvedValue({ listening: true, last: null });
    live.mockReset().mockResolvedValue("Pressed; the game confirmed it in 120 ms.");
  });

  it("shows a typed phrase's intent and keys and runs nothing live", async () => {
    render(View);
    await typePhrase("Gear down");

    expect(await screen.findByText("landing_gear.down", { exact: false })).toBeTruthy();
    expect(screen.getByText("L").className).toContain("key");
    expect(screen.getByText(/Press the keys, then say/).textContent).toContain("Landing gear deployed.");
    expect(screen.getByText("Nothing was sent to the game.")).toBeTruthy();
    expect(live).not.toHaveBeenCalled();

    await fireEvent.click(screen.getByRole("button", { name: "Run live" }));
    expect(live).toHaveBeenCalledExactlyOnceWith("landing_gear.down");
    expect(await screen.findByText("Pressed; the game confirmed it in 120 ms.")).toBeTruthy();
  });

  it("shows a new push-to-talk result, not one from an earlier visit", async () => {
    const old = { ...heard, transcript: "Gear up." };
    poll.mockResolvedValueOnce({ listening: true, last: { seq: 4, result: old } }).mockResolvedValue({ listening: true, last: { seq: 5, result: heard } });
    render(View);

    expect(await screen.findByText("lights.on", { exact: false }, { timeout: 2500 })).toBeTruthy();
    const heardRow = screen.getByText(/· push-to-talk/).parentElement!;
    expect(heardRow.textContent).toContain("Lights on.");
    expect(screen.queryByText("Gear up.", { exact: false })).toBeNull();
    expect(screen.getByText("Left shift + K")).toBeTruthy();
    expect(live).not.toHaveBeenCalled();
  });

  it("can't run live without Elite", async () => {
    poll.mockResolvedValue({ listening: false, last: null });
    render(View);
    await typePhrase("lights on");
    expect(await screen.findByText("Live runs need Elite running.")).toBeTruthy();
    expect((screen.getByRole("button", { name: "Run live" }) as HTMLButtonElement).disabled).toBe(true);
  });
});
