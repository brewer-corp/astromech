import { describe, expect, it } from "vitest";
import { failure } from "./errors";

describe("failure", () => {
  it("names what was being read and keeps the reason, from a shell string or an Error", () => {
    expect(failure("read the fleet", "journal folder not found: C:\\nowhere")).toBe("Could not read the fleet: journal folder not found: C:\\nowhere.");
    expect(failure("read the fleet", new Error("busy"))).toBe("Could not read the fleet: busy.");
  });

  it("drops an 'Error:' prefix and a doubled full stop, and copes with no reason", () => {
    expect(failure("read the fleet", "Error: busy.")).toBe("Could not read the fleet: busy.");
    expect(failure("read the fleet", "")).toBe("Could not read the fleet.");
  });
});
