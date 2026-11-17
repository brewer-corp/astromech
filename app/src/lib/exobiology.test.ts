import { describe, expect, it } from "vitest";
import { bodyCards, type BodyBio, type GenusBio, type KnownBody } from "./exobiology";

// What other players reported (Spansh) merged into the commander's body cards. The replies here
// set up cases the recorded fixture lacks: a body or genus only others reported, ids missing.

const genus = (name: string, species: [string, number][], known = true): GenusBio => {
  const vs = species.map(([, v]) => v);
  return {
    genus: name,
    known,
    species: species.map(([s, value]) => ({ genus: name, species: s, value, fit: 1 })),
    value: [Math.min(...vs), Math.max(...vs)],
    colony_range_m: 500,
    analysed: false,
  };
};
const body = (name: string, body_id: number | null, genera: GenusBio[], value: [number, number]): BodyBio => ({
  body_id,
  name,
  signals: genera.length,
  mapped: genera.every((g) => g.known),
  genera,
  value,
});
const report = (name: string, body_id: number | null, genera: string[], species: [string, number][], value: [number, number]): KnownBody => ({
  name,
  body_id,
  signals: genera.length,
  genera,
  species: species.map(([n, v]) => ({ name: n, value: v })),
  value,
});

const A2 = body("S A 2", 5, [genus("Stratum", [["Stratum Tectonicas", 19_010_800]])], [19_010_800, 19_010_800]);
const B3 = body("S B 3", 7, [genus("Bacterium", [["Bacterium Vesicula", 1_000_000]], false)], [1_000_000, 1_000_000]);

/** A card as the test reads it: its name, whether only others reported it, its ranges, and its genera. */
const read = (bodies: BodyBio[], known: KnownBody[] | null) =>
  bodyCards(bodies, known).map((c) => ({
    name: c.name,
    reported: c.reported,
    value: c.value,
    reported_value: c.reported_value,
    genera: c.genera.map((g) => `${g.genus}${g.reported ? " (reported)" : g.others ? " (also reported)" : ""}: ${g.species.map((s) => `${s.name}${s.reported ? "*" : ""}`).join(", ")}`),
  }));

describe("bodyCards", () => {
  it.each([
    {
      case: "known null: the commander's cards alone",
      bodies: [A2, B3],
      known: null,
      cards: [
        { name: "S A 2", reported: false, value: [19_010_800, 19_010_800], reported_value: null, genera: ["Stratum: Stratum Tectonicas"] },
        { name: "S B 3", reported: false, value: [1_000_000, 1_000_000], reported_value: null, genera: ["Bacterium: Bacterium Vesicula"] },
      ],
    },
    {
      case: "known empty: the same",
      bodies: [A2],
      known: [],
      cards: [{ name: "S A 2", reported: false, value: [19_010_800, 19_010_800], reported_value: null, genera: ["Stratum: Stratum Tectonicas"] }],
    },
    {
      case: "a report on the commander's body adds its range, a new genus and a new species",
      bodies: [A2],
      known: [report("S A 2", 5, ["Stratum", "Tussock"], [["Stratum Excutitur", 2_448_900], ["Tussock Pennata", 5_853_800]], [20_000_000, 28_000_000])],
      cards: [
        {
          name: "S A 2",
          reported: false,
          value: [19_010_800, 19_010_800],
          reported_value: [20_000_000, 28_000_000],
          genera: ["Stratum (also reported): Stratum Tectonicas, Stratum Excutitur*", "Tussock (reported): Tussock Pennata*"],
        },
      ],
    },
    {
      case: "a report that says what the commander has: the species marked reported, the same range not repeated",
      bodies: [A2],
      known: [report("S A 2", 5, ["Stratum"], [["Stratum Tectonicas", 19_010_800]], [19_010_800, 19_010_800])],
      cards: [{ name: "S A 2", reported: false, value: [19_010_800, 19_010_800], reported_value: null, genera: ["Stratum (also reported): Stratum Tectonicas*"] }],
    },
    {
      case: "a reported species among the commander's predicted candidates keeps its reported value",
      bodies: [body("S B 3", 7, [genus("Electricae", [["Electricae Pluma", 6_284_600], ["Electricae Radialem", 6_284_600]], false)], [6_284_600, 6_284_600])],
      known: [report("S B 3", 7, ["Electricae"], [["Electricae Radialem", 6_284_600]], [6_284_600, 6_284_600])],
      cards: [{ name: "S B 3", reported: false, value: [6_284_600, 6_284_600], reported_value: null, genera: ["Electricae (also reported): Electricae Pluma, Electricae Radialem*"] }],
    },
    {
      case: "species named before their genus go under it (Roseum Brain Tree is a Brain Tree)",
      bodies: [],
      known: [report("S D 4", 11, ["Brain Tree", "Sinuous Tuber"], [["Roseum Brain Tree", 1_593_700], ["Roseum Sinuous Tuber", 1_514_500]], [3_108_200, 3_108_200])],
      cards: [
        { name: "S D 4", reported: true, value: [3_108_200, 3_108_200], reported_value: null, genera: ["Brain Tree (reported): Roseum Brain Tree*", "Sinuous Tuber (reported): Roseum Sinuous Tuber*"] },
      ],
    },
    {
      case: "a body only others reported: its own card after the commander's",
      bodies: [B3],
      known: [report("S C 1", 9, ["Osseus"], [["Osseus Discus", 12_934_900]], [12_934_900, 12_934_900])],
      cards: [
        { name: "S B 3", reported: false, value: [1_000_000, 1_000_000], reported_value: null, genera: ["Bacterium: Bacterium Vesicula"] },
        { name: "S C 1", reported: true, value: [12_934_900, 12_934_900], reported_value: null, genera: ["Osseus (reported): Osseus Discus*"] },
      ],
    },
    {
      case: "no bodies of the commander's: every report is a card",
      bodies: [],
      known: [report("S A 2", 5, ["Stratum", "Bacterium"], [], [1_000_000, 20_000_000])],
      cards: [{ name: "S A 2", reported: true, value: [1_000_000, 20_000_000], reported_value: null, genera: ["Stratum (reported): ", "Bacterium (reported): "] }],
    },
    {
      case: "ids missing: matched by full name, and two null ids never match each other",
      bodies: [body("S A 2", null, [genus("Stratum", [["Stratum Tectonicas", 19_010_800]])], [19_010_800, 19_010_800])],
      known: [report("S B 3", null, ["Fonticulua"], [], [1_000_000, 1_000_000]), report("S A 2", null, ["Stratum"], [], [19_010_800, 19_010_800])],
      cards: [
        { name: "S A 2", reported: false, value: [19_010_800, 19_010_800], reported_value: null, genera: ["Stratum (also reported): Stratum Tectonicas"] },
        { name: "S B 3", reported: true, value: [1_000_000, 1_000_000], reported_value: null, genera: ["Fonticulua (reported): "] },
      ],
    },
  ])("$case", ({ bodies, known, cards }) => {
    expect(read(bodies, known)).toEqual(cards);
  });
});
