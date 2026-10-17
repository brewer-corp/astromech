// Command palette entries: every module stations sell, opening its nearby outfitting search
// for any size.
import { CATEGORIES, MODULES, moduleRoute } from "../../lib/outfitting";
import type { PaletteItem } from "../../lib/palette";

export default async function entries(): Promise<PaletteItem[]> {
  return MODULES.map((m) => ({
    label: m.name,
    detail: `Outfitting · ${CATEGORIES.find((c) => c.id === m.category)?.title ?? m.category}`,
    route: moduleRoute({ name: m.name, class: null, rating: null, mount: null }),
  }));
}
