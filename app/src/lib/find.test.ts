import { describe, expect, it } from "vitest";
import { parseServiceRoute, suggestFind } from "./find";

describe("Galaxy find box", () => {
  it("recognises a service, a module and a ship from what was typed", () => {
    expect(suggestFind("factors")[0]).toMatchObject({ label: "Interstellar Factors", detail: "Service", route: "services/interstellar_factors" });
    expect(suggestFind("frame shift")[0]).toMatchObject({ label: "Frame Shift Drive", detail: "Module · Core internals" });
    expect(suggestFind("anaconda")[0]).toMatchObject({ label: "Anaconda", detail: "Ship", route: "shipyard/Anaconda" });
  });

  it("reads a service from the address", () => {
    expect(parseServiceRoute("#/services/black_market")).toBe("black_market");
    expect(parseServiceRoute("#/services")).toBeNull();
    expect(parseServiceRoute("#/services/nope")).toBeNull();
  });
});
