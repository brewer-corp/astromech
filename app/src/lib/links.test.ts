import { describe, expect, it } from "vitest";
import { inaraStation, inaraSystem, spanshStation, spanshSystem } from "./links";

describe("Spansh links", () => {
  it("uses the system address when there is one", () => {
    expect(spanshSystem({ id64: 10477373803, name: "Sol" })).toBe("https://spansh.co.uk/system/10477373803");
    expect(spanshSystem({ id64: "10477373803", name: "Sol" })).toBe("https://spansh.co.uk/system/10477373803");
  });
  it("falls back to search by name, encoded", () => {
    expect(spanshSystem({ name: "Col 70 Sector FY-N c21-3" })).toBe("https://spansh.co.uk/search/Col%2070%20Sector%20FY-N%20c21-3");
    expect(spanshStation({ marketId: null, name: "Jameson Memorial" })).toBe("https://spansh.co.uk/search/Jameson%20Memorial");
  });
  it("uses the market id for a station", () => {
    expect(spanshStation({ marketId: 128016384, name: "Abraham Lincoln" })).toBe("https://spansh.co.uk/station/128016384");
  });
});

describe("Inara links", () => {
  it("searches by name", () => {
    expect(inaraSystem("Sol")).toBe("https://inara.cz/elite/starsystem/?search=Sol");
    expect(inaraStation("Abraham Lincoln")).toBe("https://inara.cz/elite/stations/?search=Abraham%20Lincoln");
    expect(inaraSystem("HIP 1/2 & co")).toBe("https://inara.cz/elite/starsystem/?search=HIP%201%2F2%20%26%20co");
  });
});
