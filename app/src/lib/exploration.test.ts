import { describe, expect, it } from "vitest";
import { bodyGlyph, bodyTree, type Body } from "./exploration";
import fixture from "../preview/fixtures/exploration.json";

/** A body with only what the tree reads; `parents` as the journal lists them, nearest first. */
function body(short_name: string, body_id: number | null, distance_ls: number | null, parents: [string, number][] = [], star = false): Body {
  return { short_name, body_id, distance_ls, star, parents: parents.map(([kind, id]) => ({ kind, body_id: id })) } as Body;
}

/** The tree as "name at depth" lines, with the elbow and through lines drawn: "  └ A 1 a". */
function drawn(bodies: Body[]): string[] {
  return bodyTree(bodies).map((r) => {
    const through = r.through.map((t) => (t ? "│ " : "  ")).join("");
    return r.depth === 0 ? r.body.short_name : `${through}${r.last ? "└" : "├"} ${r.body.short_name}`;
  });
}

describe("bodyTree", () => {
  const A = body("A", 1, 0, [["Null", 0]], true);
  const B = body("B", 7, 18_000, [["Null", 0]], true);
  const cases: [string, Body[], string[]][] = [
    ["a moon of a planet of a star", [body("A 1 a", 3, 12, [["Planet", 2], ["Star", 1]]), body("A 1", 2, 10, [["Star", 1]]), A], ["A", "└ A 1", "  └ A 1 a"]],
    ["a planet of a barycentre sits at the root, by distance among the stars", [B, body("AB 1", 9, 25_000, [["Null", 0]]), A], ["A", "B", "AB 1"]],
    ["a body whose parent is not scanned hangs from its nearest scanned ancestor", [A, body("A 1 a", 3, 12, [["Planet", 2], ["Star", 1]])], ["A", "└ A 1 a"]],
    ["a body with no scanned ancestor goes to the root", [body("A 1 a", 3, 12, [["Planet", 2], ["Star", 1]])], ["A 1 a"]],
    ["a ring between a body and its planet is skipped", [A, body("A 1", 2, 10, [["Star", 1]]), body("A 1 b", 4, 11, [["Ring", 5], ["Planet", 2], ["Star", 1]])], ["A", "└ A 1", "  └ A 1 b"]],
    ["a star is at the root even when it orbits another", [A, body("A 1", 2, 10, [["Star", 1]]), body("B", 7, 20, [["Star", 1]], true)], ["A", "└ A 1", "B"]],
    [
      "siblings go by distance, the line passing the earlier ones' children",
      [A, body("A 2", 4, 200, [["Star", 1]]), body("A 1", 2, 10, [["Star", 1]]), body("A 1 a", 3, 11, [["Planet", 2], ["Star", 1]])],
      ["A", "├ A 1", "│ └ A 1 a", "└ A 2"],
    ],
    ["equal distances go by name, and an unknown distance last", [A, body("A 3", 5, null, [["Star", 1]]), body("A 2", 4, 10, [["Star", 1]]), body("A 1", 2, 10, [["Star", 1]])], ["A", "├ A 1", "├ A 2", "└ A 3"]],
    ["a body with no id has no children but is still drawn", [A, body("A 1", null, 10, [["Star", 1]]), body("A 1 a", 3, 11, [["Planet", 2], ["Star", 1]])], ["A", "├ A 1", "└ A 1 a"]],
    ["a loop in the parents still draws every body, once", [body("X", 1, 5, [["Planet", 2]]), body("Y", 2, 6, [["Planet", 1]])], ["X", "└ Y"]],
  ];
  it.each(cases)("%s", (_, bodies, want) => {
    expect(drawn(bodies)).toEqual(want);
  });

  it("says which rows have children", () => {
    const rows = bodyTree([A, body("A 1", 2, 10, [["Star", 1]])]);
    expect(rows.map((r) => r.kids)).toEqual([true, false]);
  });
});

describe("bodyGlyph", () => {
  // Every class in the preview's sample system, and the star classes the route's table tells apart.
  const cases: [string, boolean, string][] = [
    ...fixture.system.bodies.map((b): [string, boolean, string] => [b.class, b.star, ""]),
    ["DA", true, "star-white-dwarf"],
    ["N", true, "star-neutron"],
    ["H", true, "star-black-hole"],
    ["T", true, "star-brown-dwarf"],
    ["M_RedGiant", true, "star-scoopable"],
    ["AeBe", true, "star-other"],
    ["Gas giant with water based life", false, "body-gas-giant"],
    ["Helium rich gas giant", false, "body-gas-giant"],
    ["Water giant", false, "body-water-giant"],
  ];
  const want: Record<string, string> = {
    K: "star-scoopable",
    M: "star-scoopable",
    "Earthlike body": "body-earth-like",
    "High metal content body": "body-high-metal",
    "Sudarsky class I gas giant": "body-gas-giant",
    "Icy body": "body-icy",
    "Water world": "body-water-world",
    "Rocky ice body": "body-rocky-ice",
    "Ammonia world": "body-ammonia",
    "Metal rich body": "body-metal-rich",
    "Rocky body": "body-rocky",
  };
  it.each(cases)("%s", (cls, star, glyph) => {
    expect(bodyGlyph({ class: cls, star })).toBe(glyph || want[cls]);
  });

  it("draws a planet class it does not know as a rocky body", () => {
    expect(bodyGlyph({ class: "Something new", star: false })).toBe("body-rocky");
  });
});
