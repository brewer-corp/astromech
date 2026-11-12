import { describe, expect, it } from "vitest";
import { starKind } from "./route";

describe("starKind", () => {
  it("tells a star by its NavRoute class first, then by the plotter's flags", () => {
    const of = (star_class: string | null, scoopable: "yes" | "no" | "unknown" = "unknown", neutron: boolean | null = null) =>
      starKind({ star_class, scoopable, neutron });
    expect(of("DAB", "no")).toMatchObject({ glyph: "star-white-dwarf", name: "White dwarf" });
    expect(of("T", "no")).toMatchObject({ glyph: "star-brown-dwarf", name: "Class T brown dwarf" });
    expect(of("K_OrangeGiant", "yes")).toMatchObject({ glyph: "star-scoopable", name: "Orange giant" });
    expect(of("H", "no").glyph).toBe("star-black-hole");
    expect(of("WC", "no")).toMatchObject({ glyph: "star-other", name: "Wolf-Rayet star" });
    expect(of(null, "no", true).glyph).toBe("star-neutron");
    expect(of(null, "no").glyph).toBe("star-other");
    expect(of(null).glyph).toBe("star-unknown");
  });
});
