---
name: astromech
description: The window beside Elite Dangerous. Sparse, scannable, and lit only where it matters.
colors:
  elite-orange: "#fa6e00"
  orange-ember: "#8a3d00"
  void: "#0c0d0f"
  void-deep: "#08090b"
  hull: "#131417"
  hull-raised: "#1a1b1e"
  seam: "#26282b"
  row-hover: "#16171a"
  option-cell: "#2b2d2e"
  option-cell-hover: "#35383a"
  option-on: "#45484b"
  binding-cell: "#3d220f"
  binding-empty: "#1c1610"
  text-lit: "#ffffff"
  text: "#e8e8e8"
  text-lo: "#a0a0a0"
  text-dim: "#5f6266"
  on-orange: "#0c0d0f"
typography:
  wordmark:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "18px"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.12em"
  title:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "24px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
  value-xl:
    fontFamily: "Saira Variable, Saira Semi Condensed, Bahnschrift, sans-serif"
    fontSize: "28px"
    fontWeight: 500
    lineHeight: 1
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  value-l:
    fontFamily: "Saira Variable, Saira Semi Condensed, Bahnschrift, sans-serif"
    fontSize: "18px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  nav:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: "0.06em"
  section:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "0.06em"
  name:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "normal"
  body:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 400
    lineHeight: 1.45
    letterSpacing: "normal"
  figures:
    fontFamily: "Saira Variable, Saira Semi Condensed, Bahnschrift, sans-serif"
    fontSize: "15px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  label:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
  meta:
    fontFamily: "Saira Variable, Saira Semi Condensed, Bahnschrift, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  action:
    fontFamily: "Saira Semi Condensed, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.06em"
  machine:
    fontFamily: "Share Tech Mono, Consolas, monospace"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.4
    letterSpacing: "normal"
rounded:
  none: "0px"
spacing:
  hair: "2px"
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
  2xl: "32px"
  gutter: "40px"
  3xl: "48px"
components:
  button-primary:
    backgroundColor: "{colors.elite-orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.action}"
    rounded: "{rounded.none}"
    padding: "0 20px"
    height: "34px"
  button-primary-disabled:
    backgroundColor: "{colors.option-cell}"
    textColor: "{colors.text-dim}"
    typography: "{typography.action}"
    rounded: "{rounded.none}"
    padding: "0 20px"
    height: "34px"
  button-secondary:
    textColor: "{colors.elite-orange}"
    typography: "{typography.action}"
    padding: "0 8px"
    height: "34px"
  button-secondary-disabled:
    textColor: "{colors.text-dim}"
    typography: "{typography.action}"
    padding: "0 8px"
    height: "34px"
  input:
    backgroundColor: "{colors.option-cell}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.none}"
    padding: "0 12px"
    height: "34px"
  input-hover:
    backgroundColor: "{colors.option-cell-hover}"
  dropdown:
    backgroundColor: "{colors.option-cell}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    rounded: "{rounded.none}"
    padding: "0 32px 0 12px"
    height: "34px"
  choice:
    backgroundColor: "{colors.option-cell}"
    textColor: "{colors.text-lo}"
    typography: "{typography.label}"
    padding: "0 12px"
    height: "34px"
  choice-on:
    backgroundColor: "{colors.option-on}"
    textColor: "{colors.text-lit}"
    typography: "{typography.label}"
    padding: "0 12px"
    height: "34px"
  binding-cell:
    backgroundColor: "{colors.binding-cell}"
    textColor: "{colors.text}"
    typography: "{typography.body}"
    padding: "0 12px"
    height: "34px"
  binding-cell-empty:
    backgroundColor: "{colors.binding-empty}"
    textColor: "{colors.text-dim}"
    typography: "{typography.body}"
    padding: "0 12px"
    height: "34px"
  binding-cell-listening:
    backgroundColor: "{colors.elite-orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.action}"
    padding: "0 12px"
    height: "34px"
  tab-top:
    textColor: "{colors.text-lo}"
    typography: "{typography.nav}"
    padding: "0"
  tab-top-active:
    textColor: "{colors.elite-orange}"
    typography: "{typography.nav}"
    padding: "0"
  tab-page:
    textColor: "{colors.text-lo}"
    typography: "{typography.name}"
    padding: "10px 0"
  tab-page-active:
    textColor: "{colors.text-lit}"
    typography: "{typography.name}"
    padding: "10px 0"
  badge:
    textColor: "{colors.text-lo}"
    typography: "{typography.label}"
    rounded: "{rounded.none}"
    padding: "0 6px"
    height: "20px"
  tile:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.text}"
    rounded: "{rounded.none}"
    padding: "10px 12px"
  tile-unavailable:
    backgroundColor: "{colors.void}"
    textColor: "{colors.text-dim}"
    padding: "10px 12px"
  tile-selected:
    backgroundColor: "{colors.elite-orange}"
    textColor: "{colors.on-orange}"
    padding: "10px 12px"
  result-row:
    textColor: "{colors.text}"
    typography: "{typography.figures}"
    padding: "0 12px"
    height: "44px"
  result-row-hover:
    backgroundColor: "{colors.row-hover}"
  route-strip:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.text}"
    typography: "{typography.name}"
    padding: "0 40px"
    height: "36px"
  popover:
    backgroundColor: "{colors.hull-raised}"
    textColor: "{colors.text}"
    rounded: "{rounded.none}"
    padding: "4px 0"
---

# Design System: astromech

<!-- This is the target system (astromech-c4i0, 24 Oct 2026). Its tokens are in
app/src/design/tokens.css; pages still to be brought in line: astromech-pey5 (shared
controls), astromech-dfvp (cells mean editable), astromech-yxtz (caps and orange),
astromech-sllo (two tab rows), astromech-dzwy (figures face); the route strip and Places
finders arrive with astromech-51ue. Mockups: docs/design/design-system-*.png. -->

## Overview

**Creative North Star: "The Station Board"**

The window reads like one of Elite's own station screens: the Encoded Material Trader, the
Controls list, the station services board. Those screens are dark, nearly silent, and lit
only where something matters. Each tile has one big value. Names are smaller, ratios are a
corner mark. State is shown by brightness, not words: a dimmed tile is unavailable, the one
solid orange tile is selected. Layout carries meaning: on the trader, rows are categories and
columns are grades, so nothing needs a label to be understood. Orange means something because
almost nothing is orange.

astromech follows that thinking, not its density. The trader screen is too sparse for a
lookup window. The goal is **hierarchy, not density**: every element has one value that
outranks everything around it, and everything else steps down in size and brightness. A
commander glancing over between jumps should find the answer in under two seconds and never
have to read a caps label to find it.

The register is still the game's **Controls dialect**: near-black, one stock orange, light
grey text, filled value cells, very few lines, square corners, no ornament. What is new is
that the dialect now has rules a page can be reviewed against. The test for any screen is
not "does it look like Elite" but "can a commander find the main value, the thing they can
change, and the one thing that needs them, without reading".

**Key Characteristics:**

- One main value per element: the largest and brightest text in its tile, row or strip.
- State by brightness: lit, normal, secondary, dim. Words only when brightness is not enough.
- One orange per meaning: selected, action, warning, focus. Never decoration.
- A filled cell means you can change it. Read-only things never sit in a cell.
- Caps for navigation and verbs only. Content is mixed case.
- Numbers in tabular figures, right-aligned, in columns that line up.
- Square, flat, silent: no radius, no shadow, no gradient, no glow, no motion.

### Principles

1. **One main value.** Before drawing an element, name its one value: the count of a
   material, the price at a station, the hotspots in a ring, the next system on the route.
   That value gets the largest size and the brightest text in the element. Everything else is
   at least one step down in size *and* brightness.
2. **Brightness is state.** Four text levels (lit, normal, secondary, dim) carry rank and
   availability. Something unavailable is dimmed, not labelled "unavailable". Something
   chosen is lit. A solid orange fill means "this one, now".
3. **Orange is a signal.** It marks the selected thing, the thing to press, the thing that
   needs the commander, and keyboard focus. If two orange things on screen do not mean one
   of those, one of them is wrong.
4. **Cells are controls.** A filled cell (grey option cell, brown binding cell) invites a
   click. Read-only facts sit on the window or on a hull surface and never look pressable.
5. **Layout carries meaning.** Grids encode two axes (category × grade). Lists line up in
   columns so a commander can run their eye down one fact. Don't wrap facts into prose.
6. **Hierarchy, not density.** Fit more by removing levels (a tab row, a caps label, a band,
   a box), not by shrinking text. Never go below 12 px.
7. **Honest and plain.** Status and errors are plain words in sentence case, with an orange
   "!" when the commander must act. No blocks of colour, no alarm styling.

### Fixed rules (from PRODUCT.md, binding)

- **Stock Elite orange only** (`elite-orange`). No other hue anywhere in the window: no red
  for errors, no green for success, no blue for links, no theme or HUD-colour option.
- **No Frontier assets.** No Frontier art, icons, fonts, audio, screenshots or game files.
  The faces are OFL lookalikes (Saira, Share Tech Mono). Ship hulls are CMDR Qohen Leth's
  cartoons and engineer marks are CMDR Robert Phillips', both from EDAssets and credited in
  NOTICE.md.
- **English only.** Copy follows docs/V1-DESIGN.md §6.5: short, plain, sentence case,
  no "I", nothing that pretends astromech typed into the game.
- **The Controls dialect stays**, defined by this file.
- **No VR or in-flight UI** in v1; the window is for between jumps.

## Colors

One warm signal on a cold, neutral black. Every colour except `elite-orange` and
`orange-ember` is a neutral grey; the two browns are orange at low strength and exist only
for binding slots.

### Primary

- **Elite Orange** (`elite-orange`): the only colour. Allowed in exactly these roles:
  1. **Selected:** the active top tab's text; the 2 px underline under the active page tab;
     the 1 px outline and lead text of the selected row; the solid fill of the one selected
     tile in a picker grid.
  2. **Action:** the primary button's fill; the text of secondary (text) actions such as
     CHANGE, ADD STOP, RUN AGAIN; row actions only on the hovered or selected row.
  3. **Needs you:** the "!" mark before a warning or error; the listening fill of a binding
     cell waiting for a press.
  4. **Focus and structure:** the keyboard focus ring; the header rule; the 1 px separator
     between a list and its side panel; text selection.
- **Orange Ember** (`orange-ember`): dim orange for a pressed scrollbar thumb and an
  inactive structural rule. Never text, never fill.

**The One Signal Rule.** Orange is never used for headings, section titles, values,
numbers, units, prices, dropdown or input text, badges, chips, icons at rest, card borders,
row separators or decoration. If it is orange, it is selected, pressable, urgent or focused.

**The One Fill Rule.** A view shows at most one solid orange fill. When a primary button and
a selected tile would both be filled, the tile uses the 1 px outline instead.

### Neutral

- **Void** (`void`): the window background. Pages sit directly on it.
- **Void Deep** (`void-deep`): a sunken list column, used rarely.
- **Hull** (`hull`): the read-only surface: tiles, the route strip, side-panel blocks, a
  grouped fact block. One step above Void, deliberately far below any cell, so it never looks
  pressable.
- **Hull Raised** (`hull-raised`): popovers and menus (dropdown lists, Ctrl+K, the Find or
  plot results), always with a 1 px Seam border.
- **Seam** (`seam`): the only line colour: the hairline under column heads, a popover border,
  a tile's progress track, a badge outline.
- **Row Hover** (`row-hover`): hover background for a read-only row.
- **Option Cell** (`option-cell`), **Option Cell Hover** (`option-cell-hover`), **Option On**
  (`option-on`): editable values. Option On is the chosen option inside a choice cell.
- **Binding Cell** (`binding-cell`) and **Binding Empty** (`binding-empty`): a key or button
  slot, bound and unbound. Controls pages only.

### Text levels

Four levels, and they carry state:

- **Lit** (`text-lit`, 19.6:1 on Void): the main value of an element; the selected item;
  titles.
- **Normal** (`text`, 16.4:1): names, row text, body copy, values in cells.
- **Secondary** (`text-lo`, 7.6:1): labels, column heads, units, supporting facts, inactive
  tabs, icons at rest.
- **Dim** (`text-dim`, 3.2:1): unavailable, zero, not applicable, stale, disabled. Never used
  for anything the commander must read to proceed.
- **On Orange** (`on-orange`, 6.7:1 on orange): text on an orange fill.

**The Brightness Ladder Rule.** Inside one element, each step down in importance is a step
down the ladder. Two adjacent pieces of text at the same level must differ in size or weight,
or one of them should go.

## Typography

**Text face:** Saira Semi Condensed (OFL), with Bahnschrift and Segoe UI as fallbacks.
**Figures face:** Saira Variable (OFL, `@fontsource-variable/saira`) at 87.5 % width
(`font-stretch: 87.5%`, which is semi-condensed) with tabular figures (`"tnum" 1`). It is the
same design as the text face; it exists because the static Saira Semi Condensed files carry
no tabular figures.
**Machine face:** Share Tech Mono (OFL), only for machine text.

**Character:** condensed and square-shouldered like the game's own lettering, without copying
it. Mixed case for content keeps it readable; caps are kept for the few places the game uses
them as wayfinding.

### Hierarchy

| Role | Size / weight / line | Case | Level | Use |
|---|---|---|---|---|
| Wordmark | 18 / 600 / 1, tracking 0.12em | caps | lit | "ASTROMECH" only |
| Title | 24 / 600 / 1.2 | mixed | lit | page title, finder title, side-panel title |
| Value XL | 28 / 500 / 1, tabular | figures | lit | the one value of a tile (material count) |
| Value L | 18 / 600 / 1.2, tabular | figures | lit | the one value of a result row or strip |
| Nav | 15 / 500, tracking 0.06em | caps | secondary, active orange | top tabs only |
| Section | 13 / 600, tracking 0.06em | caps | lit | a section heading inside a page |
| Name | 15 / 500 / 1.3 | mixed | normal | station, material, ship, engineer names; page tabs |
| Body | 15 / 400 / 1.45, max 68ch | sentence | normal | prose, descriptions, panel text |
| Figures | 15 / 400, tabular | figures | normal | numbers in columns and cells |
| Label | 13 / 400 / 1.3 | sentence | secondary | column heads, field labels, units, facts, badges |
| Meta | 12 / 400, tabular | figures | secondary | corner marks in tiles (cap, trade ratio, age) |
| Action | 13 / 600, tracking 0.06em | caps | orange or on-orange | buttons and text actions |
| Machine | 13 / 400 / 1.4 | as written | normal | file paths, raw bind tokens, journal event names, logs |

Units follow their number in Label size and Secondary level: **390,780** cr, **7** jumps,
**127.3** ly. Thousands are separated with commas.

### Named Rules

**The Caps Are Wayfinding Rule.** Uppercase with 0.06em tracking is allowed for exactly four
roles: the wordmark, top tabs, section headings, and action words (buttons and text
actions). Everything else is mixed or sentence case: page tabs, column heads, field labels,
row labels, names, badges, status, empty states, errors, prose. Tracking never appears
without caps, and caps never appear without tracking.

**The One Caps Line Rule.** An element (a card, a row, a tile) carries at most one caps line,
and it is an action. Status and categories are not caps.

**The Machine Text Rule.** Share Tech Mono is allowed only for text a commander might compare
character by character or type somewhere else: file and folder paths, raw bind tokens,
journal event names, log lines, version hashes. Never for names, dates, prices, distances,
counts or prose; those are Saira figures.

**The Tabular Rule.** Every number that sits in a column, a tile, a cell or a strip uses the
figures face with tabular figures and is right-aligned in its column, so the ones digit sits
under the ones digit.

## Layout

The window is a fixed frame: a header, the route strip, at most one page tab row, then the
page. Pages are calm columns on Void, not boxes inside boxes.

### Spacing scale

`hair` 2 px (between adjacent cells, as in the game) · `xs` 4 · `sm` 8 · `md` 12 · `lg` 16 ·
`xl` 24 · `2xl` 32 · `gutter` 40 (page side margins) · `3xl` 48. Use only these. Group with
the small steps, separate with the large ones: rows inside a section are 2 px apart,
sections are 32 px apart, and a heading sits 24 px below the previous section and 12 px above
its content.

### Frame

| Part | Height | Notes |
|---|---|---|
| Header | 64 px | wordmark, top tabs, Find or plot box, status, Help; 1 px orange rule under it |
| Route strip | 36 px | on every page, under the header rule |
| Page tabs | 44 px | one row, only when the area has more than one page |
| Page | rest | scrolls; the frame does not |

Side margins are 40 px (`gutter`). The window's minimum width is 1100 px; nothing in the
frame wraps to two lines above that.

### Content widths

- **Prose:** 68ch maximum.
- **Settings and forms:** label column 200 px + value cell up to 700 px (940 px total), as on
  Settings today.
- **Result lists:** up to 1200 px wide, left-aligned with the page. Row actions sit after the
  last fact column, never pinned to a distant window edge.
- **Grids of tiles:** columns of 200–240 px with 8 px gaps; the grid's axes must mean
  something (category × grade) or it should be a list.
- **Side panel:** 320 px on the right, separated by one 1 px orange line, explaining the
  selected row or tile (the Controls pattern).

### Row heights

- **Control row** (settings, keybinds, any row of cells): 34 px, 2 px apart.
- **Result row** (lookups): 44 px, one line. A second line is allowed only for a single
  secondary fact under the name, never for wrapping facts. About 18 rows fit at 1000 px.
- **Tile:** 76 px for a material tile; other tiles size to their content on the same scale.

### Named Rules

**The Two Tab Rows Rule.** Navigation is at most two rows deep: top tabs and one row of page
tabs. A third level becomes a choice cell inside the page (Raw / Manufactured / Encoded), a
section heading, or a finder of its own. Never stack tab rows, never add a band between them.

**The Aligned Columns Rule.** Facts that repeat down a list are columns with fixed widths and
a header row. If a fact would wrap, the column is too narrow or the fact should go.

## Elevation & Depth

Flat. Depth is tonal, in three steps: Void (the window), Hull (read-only surfaces and the
route strip), Hull Raised (popovers). There are no shadows, no blur and no glow anywhere. A
popover is separated by its Seam border, not a shadow. Motion is limited to instant state
changes; nothing animates in, so `prefers-reduced-motion` needs no special case.

**The No Glow Rule.** Nothing casts light. Selection is an outline or a fill, never a halo.

## Shapes

Square. Every surface, cell, button, tile, badge and popover has 0 px corners; only the
scrollbar thumb is rounded, as in the game's lists. Borders are rare: the 1 px Seam for
popovers, badges, column-head hairlines and empty tiles, and the 1 px orange for focus, selection and the
side-panel separator. No coloured side borders on cards or rows. Icons are the one-stroke
16 px glyphs in `app/src/lib/ui/Glyph.svelte` (1 px stroke, `currentColor`, no fill),
Secondary at rest and Normal or orange on hover. The "!" mark is text in the Action weight,
not an icon.

## Components

### Surfaces and cells

The rule that most pages broke: **a cell means editable.**

- **Option cell** (grey, `option-cell`): any value the commander can change that is not a
  binding: text input, dropdown, choice, number, the value of a Settings row. Hover lifts it
  to `option-cell-hover`.
- **Binding cell** (brown, `binding-cell`): a key or button slot on the Controls pages,
  and nothing else. Unbound is `binding-empty` with Dim "Unbound" text. Listening fills it
  orange with "PRESS A BUTTON · ESC TO CANCEL" in On Orange.
- **Hull surface** (`hull`): read-only things that need grouping: tiles, the route strip,
  side-panel blocks. No border, no hover unless the whole tile is a link or selectable.
- **Void:** everything else. Most read-only content needs no surface at all.

Material tiles, ship specs, engineer cards, results, chips and summaries are read-only and
are never brown or grey cells.

### Settings row (kept)

Label at 200 px (Name size, Secondary) · option cell holding the current value (Normal) · one
orange action word at the right end of the cell (CHANGE, RUN AGAIN, PREVIEW). A description
for the group sits under it in Label size, Secondary. This is the model for any editable
value with one action.

### Input

One style. Option cell, 34 px high, 12 px side padding, Body text in Normal, placeholder in
Secondary. No border or underline at rest. Focus: a 1 px orange outline. Error: the cell
keeps its look; an orange "!" and the plain-words problem sit directly under it in Label
size. A search input may show its shortcut ("Ctrl+F") at the right end in Label, Secondary.

### Dropdown

One style. Option cell like the input, value text in Normal (never orange), and a 16 px
chevron glyph in Secondary at the right, 12 px from the edge. The open list is a popover:
Hull Raised, 1 px Seam border, 34 px rows, the current option Lit with an orange 2 px
marker on its left, hover `row-hover`.

### Choice (segmented and filter chips)

A row of options inside option cells, 2 px apart, Label size: unselected in Secondary on
`option-cell`, selected in Lit on `option-on`. Single choice (Natural / Cockpit / Harsh;
Raw / Manufactured / Encoded) and multiple choice (pad S M L; Carriers; Odyssey) look the
same; a multiple choice may have several lit. An "on" state is always the brighter one;
"off" is never drawn brighter than "on". No orange inside choices.

### Buttons

- **Primary:** solid orange, On Orange text, Action type, 34 px high, 20 px side padding,
  square. At most one per view, for the step that moves the commander forward (Next, Save
  preset, Plot route). Hover lightens nothing; the cursor and focus ring are enough.
  Disabled: option-cell fill with Dim text.
- **Secondary (text action):** an orange caps verb with no fill or border, Action type,
  34 px hit area with 8 px side padding. Used for everything else: CHANGE, COPY, ADD STOP,
  REPLACE ROUTE, RUN AGAIN. Disabled: Dim text.
- **Links** that go somewhere rather than do something (Help, a guide page, "Show in all
  keybinds") are Normal text with a 1 px Secondary underline offset 3 px; orange on hover.

### Tabs

- **Top tabs:** Nav type, caps, Secondary; hover Normal; active orange. No underline, no
  fill.
- **Page tabs:** Name type, mixed case, Secondary; hover Normal; active Lit with a 2 px
  orange underline. 24 px apart.
- **Maximum depth: two rows.** See the Two Tab Rows Rule.

### Badge

One style, used sparingly: Label type, mixed case, Secondary text, 1 px Seam outline, 20 px
high, 6 px padding, square, no fill (a fill would make it look editable). For a fact that has
no column of its own: "Permit", "Carrier", "Odyssey only". A warning is not a badge; it is
an orange "!" and words.

### Material tile (picker grid)

A Hull tile, 76 px high. The count in Value XL, Lit, at the top left. The name below it in
Label size, Normal. The storage cap is the same for a whole grade, so it is stated once in
the column head ("Grade 1 · max 300"), not in every tile. A 2 px Seam track along the bottom
edge with a Secondary fill shows count / cap; at the cap the fill turns Lit. The top right
corner holds at most one Meta mark. State by brightness:

- **Have some:** as above.
- **Have none:** Void background inside a 1 px Seam outline (so the grid keeps its shape),
  every text Dim. Readable, clearly absent.
- **Selected:** solid orange fill, On Orange text, the track hidden (the One Fill Rule
  applies). The side panel explains it.
- **Wanted by a plan:** a Meta "need 12" corner mark in Normal; a short amount gets the
  orange "!".

### Lookup result row

Every Places finder and every lookup list uses one row grammar; finders differ only in
their lead column and fact columns.

1. **Main value** (lead column, first, 112 px, right-aligned): Value L, Lit, with its unit
   in Label, Secondary. It is the value the list is ranked by: price for Sell, hotspot count
   for Mining, jumps for Services, detour for Find along route.
2. **Place** (flexible, min 280 px): the station or body name in Name, Normal; the system
   after it in Label, Secondary, with the copy glyph. One line; it truncates with an
   ellipsis before it wraps.
3. **Fact columns** (fixed widths, right-aligned figures, Label heads): the facts the
   commander compares, such as demand, distance, jumps, pad, arrival and data age. Pad size
   is always shown when the place is a station.
4. **Actions** (after the facts): ADD STOP and REPLACE ROUTE as text actions in Secondary at
   rest, orange on the hovered or selected row only. Never a filled GO cell.

Rows are 44 px, no zebra, no separators; hover is `row-hover`; selected is a 1 px orange
outline with the main value staying Lit. A column head row in Label, Secondary sits above,
with a 1 px Seam hairline under it. A one-line summary of the active constraints sits above
the heads in Label, Secondary ("Within 100 ly of Sol · large pad · reports up to 30 days
old"), so defaults are stated in words.

### Route strip

A 36 px Hull band under the header rule on every page, with 40 px side padding. Left to
right: "Next" in Label, Secondary · the next system in Name, Lit, with the copy glyph ·
"3 of 41" in Figures, Secondary · flags for the next jump as badges ("Scoopable",
"Neutron") · on the right, the destination in Label, Normal, and "38 jumps left" in Figures,
Secondary. When the commander must act ("! Refuel before the next jump", "! Off route"), the
"!" is orange and the words are Normal. With no route it reads "No route" in Secondary with
the Find or plot shortcut. The strip never grows a second line.

### Find or plot

The header's one search box: an input (option cell, 320 px) with the placeholder "Find or
plot" and "Ctrl+K" at its right end. A system name plots or adds a stop; a phrase ("sell
painite", "5A FSD") opens the matching finder pre-filled. Its results are a popover.

### Places home

A grid of finder tiles under section headings (Trade, Ship, Engineering, Mining,
Exploration, Route). Each tile is a Hull link, 220 px wide: the finder name in Name, Lit;
one line in Label, Secondary saying what it finds; and, where known, a live answer in
Figures ("Best for your cargo: 390,780 cr"). Hover lifts to `row-hover`. No icons needed.

### Side panel (kept)

320 px, right, after one 1 px orange separator. Title in Title type for the selected row or
tile, then facts and Body prose, then its actions as text actions. No eyebrow label above
the title.

### States

| State | Treatment |
|---|---|
| Selected (row) | 1 px orange outline; lead text orange (lists) or Lit (results) |
| Selected (tile in a picker) | solid orange fill, On Orange text; one per view |
| Active tab | top: orange text; page: Lit + 2 px orange underline |
| Hover | read-only row: `row-hover`; cell: `option-cell-hover`; glyph: Normal |
| Focus | 1 px orange outline, 2 px offset, on every focusable thing |
| Unavailable / none / zero | Dim text, no surface lift; no word needed |
| Disabled control | Dim text; a primary button drops to an option-cell fill |
| Stale data | the row or tile drops to Dim; the age column still states the age ("212 d"). Data older than the finder's age filter is hidden unless the commander widens it |
| Warning | orange "!" + plain words in Normal, at the place it applies, Label or Body size |
| Error | as a warning, plus the recovery as a text action ("! Could not read the route. TRY AGAIN"). A whole-page error replaces the content with a Title line, a Body explanation and the action. Never a filled orange block |
| Empty | one Body line in Secondary saying what is empty and what to relax, plus one action ("No buyers within 100 ly. WIDEN TO 250 LY") |
| Loading | one Body line in Secondary naming what is being fetched ("Asking Spansh…") where the results will appear. A refresh keeps the old rows at Dim until the new ones replace them. No spinners, no skeletons |
| Listening (press to bind) | binding cell filled orange, On Orange caps |
| Conflict / ignored binding | "!" + "Also: <action>"; ignored binding text struck through and Dim |

## Do's and Don'ts

### Do:

- **Do** name each element's one main value before styling it, and give it Value L or XL in
  Lit.
- **Do** show availability by brightness: Dim for none, unavailable, stale or disabled.
- **Do** keep orange to selected, action, needs-you and focus, and keep to one solid orange
  fill per view.
- **Do** put every editable value in an option cell and every read-only fact outside one.
- **Do** use mixed case for content and keep caps to the wordmark, top tabs, section heads
  and action words.
- **Do** set numbers in the figures face with tabular figures, right-aligned, units after
  in Label size.
- **Do** line facts up in fixed columns with a Label header row.
- **Do** keep the route strip on every page and the copy glyph beside every system name.
- **Do** state defaults and constraints in words above a result list.
- **Do** keep the patterns that work: the Settings row, the Controls side panel, mixed-case
  row text, ship cartoons, the onboarding step list and primary button, plain-word status and
  errors.

### Don't:

- **Don't** fill read-only tiles, specs, materials, chips or cards with the brown or grey
  cell colours.
- **Don't** use orange for headings, values, numbers, dropdown text, chips, badges or
  decoration, and don't draw the active top tab in white while deeper tabs are orange.
- **Don't** stack more than two tab rows, or put a band between them.
- **Don't** use Share Tech Mono for names, dates, numbers or prose.
- **Don't** add a caps line to a card or row for status or category; Engineers' three caps
  lines per card become one name, mixed-case facts and brightness.
- **Don't** make the ranked value the smallest text in a row, or put a loud action far from
  the thing it acts on.
- **Don't** draw a warning or error as a solid orange block; use the "!" and words.
- **Don't** draw an off toggle brighter than an on toggle.
- **Don't** use another hue, a gradient, a shadow, a glow, a radius or an animation.
- **Don't** put an eyebrow label above a title, or a coloured side border on a card.
- **Don't** use Frontier art, fonts, icons or screenshots, or any language but English.
