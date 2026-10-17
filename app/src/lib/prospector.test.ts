import { describe, expect, it } from "vitest";
import { parseThreshold } from "./prospector";

describe("parseThreshold", () => {
  it("reads a percentage, with or without the sign", () => {
    expect(parseThreshold("25")).toBe(25);
    expect(parseThreshold(" 22.5 % ")).toBe(22.5);
    expect(parseThreshold("0")).toBe(0);
    expect(parseThreshold("100")).toBe(100);
  });
  it("treats an empty box as off", () => {
    expect(parseThreshold("")).toBeNull();
    expect(parseThreshold("  ")).toBeNull();
  });
  it("refuses anything that isn't 0 to 100", () => {
    expect(parseThreshold("150")).toBe("invalid");
    expect(parseThreshold("-1")).toBe("invalid");
    expect(parseThreshold("lots")).toBe("invalid");
  });
});
