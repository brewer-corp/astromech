import { screen } from "@testing-library/svelte";

/** A stat tile's value with its unit ("9 t"), found by the tile's label (lib/ui/Stat.svelte). */
export function statValue(label: string): string | undefined {
  return screen.getByText(label).nextElementSibling?.textContent ?? undefined;
}
