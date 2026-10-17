// Command palette entries: every material the window knows, opening it on the Materials page
// (with its nearby search when it has one).
import { loadMaterials } from "../../lib/materials";
import type { PaletteItem } from "../../lib/palette";

const KIND: Record<string, string> = { raw: "Raw", manufactured: "Manufactured", encoded: "Encoded" };

export default async function entries(): Promise<PaletteItem[]> {
  const inv = await loadMaterials();
  const cells = [...inv.kinds.flatMap((k) => k.groups.flatMap((g) => g.cells)), ...inv.other];
  return cells.map((c) => ({
    label: c.name,
    detail: `Material · ${KIND[c.kind] ?? c.kind} G${c.grade}`,
    route: `materials/${encodeURIComponent(c.id)}`,
    keywords: c.group,
  }));
}
