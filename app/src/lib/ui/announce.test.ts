import { afterEach, describe, expect, it, vi } from "vitest";
import { announcer } from "./announce";

afterEach(() => vi.useRealTimers());

describe("announcer", () => {
  it("leaves the region empty past the current task, then fills it, and a newer call wins", async () => {
    vi.useFakeTimers();
    let text = "Saved.";
    const say = announcer((t) => (text = t));
    const first = say("Saved.");
    expect(text).toBe("");
    await vi.advanceTimersByTimeAsync(16); // a frame later: still empty
    expect(text).toBe("");
    const second = say("Error.");
    await vi.advanceTimersByTimeAsync(200);
    await Promise.all([first, second]);
    expect(text).toBe("Error.");
  });

  it("keeps two regions apart: announcing in one doesn't cancel the other", async () => {
    vi.useFakeTimers();
    const shown = { callouts: "", prospector: "" };
    const a = announcer((t) => (shown.callouts = t));
    const b = announcer((t) => (shown.prospector = t));
    const both = Promise.all([a("the core did not answer"), b("the core did not answer")]);
    await vi.advanceTimersByTimeAsync(200);
    await both;
    expect(shown).toEqual({ callouts: "the core did not answer", prospector: "the core did not answer" });
  });
});
