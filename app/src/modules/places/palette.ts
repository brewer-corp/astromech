// Find or plot entries, read from the finders' own lists (FINDERS in lib/places.ts): every finder,
// and every value each opens with (a commodity, service, module, ship, trader, site kind,
// faction state or mineral), opening that finder filled in. An entry keeps its finder's id, so
// the box also matches it after the finder's name ("outfitting fsd", "painite hotspots").
import type { PaletteItem } from "../../lib/palette";
import { FINDERS, placesRoute } from "../../lib/places";

export default async function entries(): Promise<PaletteItem[]> {
  const inputs = await Promise.all(
    FINDERS.map(async (f) => ((await f.inputs?.()) ?? []).map((i): PaletteItem => ({ label: i.label, detail: f.title, route: i.route, keywords: i.keywords, finder: f.id }))),
  );
  return [...FINDERS.map((f) => ({ label: f.title, detail: "Places", route: placesRoute(f.id), keywords: f.what })), ...inputs.flat()];
}
