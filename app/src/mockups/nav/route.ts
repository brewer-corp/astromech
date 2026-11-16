// The sample route every nav mockup follows: Sol to Maia on Spansh's neutron plotter, six stops,
// the commander partway through the leg to the third. The flown leg lists its jumps (the galaxy
// map's NavRoute); the other legs give only their jump counts, as the neutron plotter does.
import type { StarGlyph } from "../../lib/ui/Glyph.svelte";

export type At = "past" | "here" | "next" | "ahead";
export type Row =
  | { kind: "stop"; at: At; name: string; star: StarGlyph; type: string; note?: string; facts?: string[]; state?: string }
  | { kind: "jump"; at: At; name: string; star: StarGlyph; type: string }
  | { kind: "leg"; at: At; jumps: number };

export const ROUTE = { from: "Sol", to: "Maia", plotter: "Neutron", by: "Spansh", n: 3, of: 6, left: 11, away: 2 };

export const ROWS: Row[] = [
  { kind: "stop", at: "past", name: "Sol", star: "star-scoopable", type: "G star, scoopable" },
  { kind: "leg", at: "past", jumps: 3 },
  { kind: "stop", at: "past", name: "Tascheter Sector LS-T a3-0", star: "star-scoopable", type: "M star, scoopable" },
  { kind: "jump", at: "past", name: "Tascheter Sector BV-O a7-2", star: "star-other", type: "T Tauri star" },
  { kind: "jump", at: "here", name: "Arietis Sector LD-H a11-1", star: "star-scoopable", type: "K star, scoopable" },
  { kind: "jump", at: "ahead", name: "Arietis Sector UE-O a7-1", star: "star-brown-dwarf", type: "Brown dwarf" },
  { kind: "stop", at: "next", name: "Hyades Sector DB-W b2-6", star: "star-scoopable", type: "M star, scoopable", note: "Fuel" },
  { kind: "leg", at: "ahead", jumps: 4 },
  { kind: "stop", at: "ahead", name: "Pleione", star: "star-scoopable", type: "B star, scoopable", facts: ["Independent", "Low security"], state: "Boom" },
  { kind: "leg", at: "ahead", jumps: 3 },
  { kind: "stop", at: "ahead", name: "Pleiades Sector IR-W d1-55", star: "star-neutron", type: "Neutron star", note: "Neutron boost" },
  { kind: "leg", at: "ahead", jumps: 2 },
  { kind: "stop", at: "ahead", name: "Maia", star: "star-scoopable", type: "B star, scoopable" },
];

export const NEXT = "Hyades Sector DB-W b2-6";
