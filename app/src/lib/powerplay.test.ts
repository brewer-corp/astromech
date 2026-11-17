import { afterEach, describe, expect, it, vi } from "vitest";
import { cycleAt, endsAt, timeLeft } from "./powerplay";

afterEach(() => vi.useRealTimers());

describe("the cycle's time left", () => {
  // The cycle ends Thursday 22 Oct 2026, 07:00 UTC.
  const END = "2026-10-22T07:00:00+00:00";

  it.each([
    ["2026-10-19T16:30:00Z", "2 d 14 h"],
    ["2026-10-21T07:00:00Z", "1 d 0 h"],
    ["2026-10-22T01:47:30Z", "5 h 12 min"],
    ["2026-10-22T06:00:00Z", "1 h 0 min"],
    ["2026-10-22T06:48:00Z", "12 min"],
    ["2026-10-22T06:59:00Z", "1 min"],
    // Once the end has passed, the next Thursday 07:00 UTC: the core's answer is from before it.
    ["2026-10-22T07:00:00Z", "7 d 0 h"],
    ["2026-10-22T07:01:00Z", "6 d 23 h"],
    ["2026-11-12T09:00:00Z", "6 d 22 h"],
  ])("at %s is %s", (now, words) => {
    vi.useFakeTimers({ toFake: ["Date"] });
    vi.setSystemTime(Date.parse(now));
    expect(timeLeft(END)).toBe(words);
  });

  it("says when it ends in UTC", () => {
    expect(endsAt(END)).toBe("Thursday 07:00 UTC");
  });

  it.each([
    ["2026-10-22T06:59:00Z", false],
    ["2026-10-22T07:01:00Z", true],
    ["2026-11-12T09:00:00Z", true],
  ])("at %s, whether the answer's cycle is over: %s", (now, over) => {
    expect(cycleAt(END, Date.parse(now)).over).toBe(over);
  });
});
