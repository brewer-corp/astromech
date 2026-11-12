// The page layouts' shared numbers (DESIGN.md Page layouts). A page never uses them for a width of
// its own; it hands its layout a count (tile columns) and the layout works out the width.

/** The side panel's content width (DESIGN.md Side panel), as --panel in tokens.css. */
export const PANEL = 320;
/** Between the content and the side panel, or the inputs and the result. */
export const GUTTER = 48;
/** A tile column's width when a side panel sits beside the tiles (DESIGN.md: 200–240 px). */
export const TILE = 200;
/** A tile grid's row-head column, budgeted ("Category 1", "Mechanical components"). */
export const HEAD = 128;
/** Between tiles. */
export const TILE_GAP = 8;

/** The narrowest a tile grid of `cols` columns keeps its tiles at TILE, so a panel fits beside it. */
export function tilesMin(cols: number, heads = false): number {
  return cols * TILE + (cols - 1) * TILE_GAP + (heads ? HEAD + TILE_GAP : 0);
}

/** Whether a side panel goes below the content: a `width` too narrow for `min` beside the panel. */
export function stacks(width: number, min: number): boolean {
  return width > 0 && width < min + GUTTER + PANEL;
}
