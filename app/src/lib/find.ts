/**
 * What the header's Find or plot box (DESIGN.md Find or plot, decision astromech-bi94) lists for
 * a query: the entries (`lib/palette.ts`: pages, finders and the values they open with, materials,
 * the fleet) and the systems Spansh suggests, ranked together.
 *
 * - An entry scores by `score` (the whole label 0, its start 1, its words' starts or initials 2,
 *   anywhere in it 3, every word somewhere in the label, detail or keywords 4).
 * - A finder's value also scores as a phrase: when the query's first or last words name the
 *   finder (each the start, of at least 3 letters, of one of its title's words, in order), the
 *   rest of the query is scored against the value alone. "outfitting fsd" is Frame Shift Drive's
 *   initials (2) in Outfitting; "painite hotspots" is Painite in Mining hotspots (0). The finders
 *   are `FINDERS` in `lib/places.ts`, so one added there is matched with no change here.
 * - A system scores 0 when it is exactly the query, else 2.5: after entries whose label starts
 *   with or has words starting with the query, before weaker matches.
 * - Ties keep entries before systems, each in their own order (Spansh's for systems).
 *
 * Voice has its own grammar for the same finders and URLs (decision astromech-g8d5, item 7).
 */
import { score, words, type PaletteItem } from "./palette";
import { FINDERS, type Finder, type FinderId } from "./places";

export type Match = { kind: "entry"; item: PaletteItem } | { kind: "system"; name: string };

/** A system that is not exactly the query ranks after word-start matches. */
const SYSTEM = 2.5;

const titleWords = new Map<FinderId, string[]>(FINDERS.map((f: Finder) => [f.id, words(f.title)]));

/** Whether `qw` names a finder whose title has words `tw`: each query word starts one of them,
 * in order, and is the whole word or at least 3 letters of it. */
function names(qw: string[], tw: string[]): boolean {
  let t = 0;
  for (const w of qw) {
    while (t < tw.length && !(tw[t]!.startsWith(w) && (w.length >= 3 || w === tw[t]))) t++;
    if (t === tw.length) return false;
    t++;
  }
  return true;
}

/** The best score of a finder's value as "<finder> <value>" or "<value> <finder>", or null. */
function phrase(item: PaletteItem, qw: string[]): number | null {
  const tw = item.finder && titleWords.get(item.finder);
  if (!tw) return null;
  let best: number | null = null;
  for (let n = 1; n < qw.length && n <= tw.length; n++) {
    for (const [lead, rest] of [
      [qw.slice(0, n), qw.slice(n)],
      [qw.slice(-n), qw.slice(0, -n)],
    ]) {
      if (!names(lead!, tw)) continue;
      const s = score(item, rest!.join(" "), rest!);
      if (s !== null && (best === null || s < best)) best = s;
    }
  }
  return best;
}

/** The best `limit` matches for `query` among `items` and the suggested `systems`. */
export function find(items: PaletteItem[], systems: string[], query: string, limit = 10): Match[] {
  const q = query.trim().toLowerCase().replace(/\s+/g, " ");
  const qw = words(q);
  if (!qw.length) return [];
  const best = (a: number | null, b: number | null) => (a === null ? b : b === null ? a : Math.min(a, b));
  const hits: { m: Match; s: number; group: number; i: number }[] = [];
  items.forEach((item, i) => {
    const s = best(score(item, q, qw), phrase(item, qw));
    if (s !== null) hits.push({ m: { kind: "entry", item }, s, group: 0, i });
  });
  systems.forEach((name, i) => hits.push({ m: { kind: "system", name }, s: name.toLowerCase() === q ? 0 : SYSTEM, group: 1, i }));
  return hits
    .sort((a, b) => a.s - b.s || a.group - b.group || a.i - b.i)
    .slice(0, limit)
    .map((h) => h.m);
}
