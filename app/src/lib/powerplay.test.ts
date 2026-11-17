import { afterEach, describe, expect, it, vi } from "vitest";
import { endsAt, timeLeft } from "./powerplay";

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
    ["2026-10-22T07:00:00Z", "0 min"],
    ["2026-10-23T09:00:00Z", "0 min"],
  ])("at %s is %s", (now, words) => {
    vi.useFakeTimers({ toFake: ["Date"] });
    vi.setSystemTime(Date.parse(now));
    expect(timeLeft(END)).toBe(words);
  });

  it("says when it ends in UTC", () => {
    expect(endsAt(END)).toBe("Thursday 07:00 UTC");
  });
});
