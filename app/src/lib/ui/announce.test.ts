import { describe, expect, it, vi } from "vitest";
import { announce } from "./announce";

describe("announce", () => {
  it("leaves the region empty past the current task, then fills it, and a newer call wins", async () => {
    vi.useFakeTimers();
    let text = "Saved.";
    const set = (t: string) => (text = t);
    const first = announce(set, "Saved.");
    expect(text).toBe("");
    await vi.advanceTimersByTimeAsync(16); // a frame later: still empty
    expect(text).toBe("");
    const second = announce(set, "Error.");
    await vi.advanceTimersByTimeAsync(200);
    await Promise.all([first, second]);
    expect(text).toBe("Error.");
    vi.useRealTimers();
  });
});
