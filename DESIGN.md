---
name: astromech
description: The window beside Elite Dangerous, drawn in the game's own orange. Sparse, scannable, lit where it matters.
colors:
  orange: "#fa6e00"
  orange-lit: "#ffa04a"
  orange-mid: "#c26c32"
  orange-dim: "#8a5834"
  on-orange: "#0c0d0f"
  ember-cell: "#3d220f"
  ember-cell-hover: "#4d2b12"
  ember-empty: "#1c1610"
  hull: "#1a1310"
  hull-raised: "#241811"
  row-hover: "#21170f"
  seam: "#3a2716"
  void: "#0c0d0f"
  white: "#ffffff"
  options-grey: "#e0e0e0"
  prose-grey: "#c4c4c4"
  alarm-red: "#f0412e"
typography:
  wordmark:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "18px"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.12em"
  title:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "24px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
  value-xl:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "28px"
    fontWeight: 500
    lineHeight: 1
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  value-l:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "18px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  nav:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: "0.06em"
  section:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "0.06em"
  name:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "normal"
  body:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 400
    lineHeight: 1.45
    letterSpacing: "normal"
  figures:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "15px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  label:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
  meta:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  action:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "13px"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.06em"
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
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.action}"
    rounded: "{rounded.none}"
    padding: "0 20px"
    height: "34px"
  button-primary-disabled:
    backgroundColor: "{colors.ember-empty}"
    textColor: "{colors.orange-dim}"
    typography: "{typography.action}"
    rounded: "{rounded.none}"
    padding: "0 20px"
    height: "34px"
  button-secondary:
    textColor: "{colors.orange}"
    typography: "{typography.action}"
    padding: "0 8px"
    height: "34px"
  button-secondary-disabled:
    textColor: "{colors.orange-dim}"
    typography: "{typography.action}"
    padding: "0 8px"
    height: "34px"
  icon-button:
    textColor: "{colors.orange-mid}"
    size: "28px"
  icon-button-hover:
    textColor: "{colors.orange-lit}"
    size: "28px"
  input:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange}"
    typography: "{typography.body}"
    rounded: "{rounded.none}"
    padding: "0 12px"
    height: "34px"
  input-hover:
    backgroundColor: "{colors.ember-cell-hover}"
  dropdown:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange}"
    typography: "{typography.body}"
    rounded: "{rounded.none}"
    padding: "0 32px 0 12px"
    height: "34px"
  choice:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange}"
    typography: "{typography.label}"
    padding: "0 12px"
    height: "34px"
  choice-on:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.label}"
    padding: "0 12px"
    height: "34px"
  binding-cell:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.options-grey}"
    typography: "{typography.body}"
    padding: "0 12px"
    height: "34px"
  binding-cell-empty:
    backgroundColor: "{colors.ember-empty}"
    textColor: "{colors.orange-dim}"
    typography: "{typography.body}"
    padding: "0 12px"
    height: "34px"
  binding-cell-listening:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.action}"
    padding: "0 12px"
    height: "34px"
  tab-top:
    textColor: "{colors.orange-mid}"
    typography: "{typography.nav}"
    padding: "0 12px"
    height: "34px"
  tab-top-active:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.nav}"
    padding: "0 12px"
    height: "34px"
  tab-page:
    textColor: "{colors.orange-mid}"
    typography: "{typography.name}"
    padding: "10px 0"
  tab-page-active:
    textColor: "{colors.orange-lit}"
    typography: "{typography.name}"
    padding: "10px 0"
  badge:
    textColor: "{colors.orange-mid}"
    typography: "{typography.label}"
    rounded: "{rounded.none}"
    padding: "0 6px"
    height: "20px"
  tile:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.orange}"
    rounded: "{rounded.none}"
    padding: "10px 12px"
  tile-unavailable:
    backgroundColor: "{colors.void}"
    textColor: "{colors.orange-dim}"
    padding: "10px 12px"
  tile-selected:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    padding: "10px 12px"
  result-row:
    textColor: "{colors.orange}"
    typography: "{typography.figures}"
    padding: "0 12px"
    height: "44px"
  result-row-hover:
    backgroundColor: "{colors.row-hover}"
  result-row-selected:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
  route-strip:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.orange}"
    typography: "{typography.name}"
    padding: "0 40px"
    height: "36px"
  popover:
    backgroundColor: "{colors.hull-raised}"
    textColor: "{colors.orange}"
    rounded: "{rounded.none}"
    padding: "4px 0"
---

# Design System: astromech

<!-- Target system (astromech-c4i0, revised 25 Oct 2026 after Sumeet's review). Tokens:
app/src/design/tokens.css. Pages still to be brought in line: astromech-yxtz (orange palette,
caps), astromech-dfvp (cells mean editable), astromech-pey5 (shared controls and icons),
astromech-sllo (two tab rows), astromech-dzwy (one face). The route strip and Places finders
arrive with astromech-51ue. Mockups: docs/design/design-system-*.png. -->

## Overview

**Creative North Star: "The Station Board"**

Everyone who opens this window plays Elite Dangerous, so they already read its design
language fluently. astromech borrows it on purpose: the orange of the station screens, the
brown tiles and cells, the solid orange fill that means "this one", the white screen titles,
the dim orange of things you cannot have. A commander should feel they opened another panel
of their ship, not a web app. Where we depart from the game, we do it for a stated reason;
those departures are listed in [Deviations from the game](#deviations-from-the-game).

The reference is the Encoded Material Trader. Each tile has one big value (the count); the
name is smaller and the trade ratio is a corner mark. State is shown by brightness: a dim
tile is unavailable, the one solid orange tile is selected. Layout carries meaning: rows are
categories, columns are grades, so nothing needs a label. White appears only in the screen
title and the heading of the section you are in.

We follow that screen's thinking, not its density. It is too sparse for a lookup window, so
we keep its rules and fit more by removing levels, never by shrinking text. The test for any
screen: can a commander find the main value, the thing they can change, and the one thing
that needs them, without reading?

**Key Characteristics:**

- The game's orange is the base palette; rank and state come from its brightness, not from a
  second colour.
- One main value per element, in the brightest orange and the largest size.
- Solid orange fill with dark text means selected, as in the game.
- Brown surfaces: dark hull for read-only tiles, brighter ember cells for what you can change.
- White only for titles, the current section and the selected item's key value.
- Red only for stale reports and hard errors.
- One face (variable Saira), tabular figures, square corners, no shadow, no glow, no motion.

### Principles

1. **Speak the game's language.** Use the game's own visual conventions unless there is a
   reason not to, and write the reason down. *Why:* every user already knows them; a familiar
   signal costs nothing to learn, a new one costs attention between jumps.
2. **One main value.** Name each element's one value (count, price, hotspots, next system)
   and give it the largest size and brightest orange. Everything else steps down at least one
   level in size or brightness. *Why:* the trader screen is readable at a glance because the
   number is the only big thing on the tile.
3. **Brightness is state.** Lit, base, mid and dim orange carry rank and availability; a
   solid fill carries selection. *Why:* this is how the game does it, and it reads faster
   than words.
4. **Cells are controls.** An ember cell invites a click; read-only things sit on the
   darker hull or on the window. *Why:* the 24 Oct audit found brown cells on read-only tiles,
   which made real controls look no different from facts.
5. **Layout carries meaning.** Grids encode two axes; lists line up in columns. *Why:* the
   eye runs down a column faster than it reads wrapped facts.
6. **Hierarchy, not density.** Fit more by removing levels (a tab row, a caps label, a box),
   never by going below 12 px. *Why:* density without rank is what made the old result rows
   unreadable.
7. **Honest and plain.** Status and errors are plain words; an orange "!" when the commander
   should act, red when something failed. *Why:* the copy rules (docs/V1-DESIGN.md §6.5)
   already ask for plain words; colour only says how urgent they are.

### Fixed rules (from PRODUCT.md, binding)

- **The game's orange palette, plus one semantic red.** No other hue, and no colour or theme
  customisation or following the in-game HUD colour. *Why:* PRODUCT.md; the window should
  look like the stock game every commander knows.
- **No Frontier assets.** No Frontier art, icons, fonts, audio, screenshots or game files.
  The face is OFL Saira; icons are drawn for astromech; ship hulls are CMDR Qohen Leth's
  cartoons and engineer marks CMDR Robert Phillips', from EDAssets, credited in NOTICE.md.
  Game screenshots used to study the UI are never committed.
- **English only**, in the copy style of docs/V1-DESIGN.md §6.5.
- **No VR or in-flight UI** in v1; the window is for between jumps.

## Colors

The game's orange in four brightnesses on warm brown surfaces, over a neutral near-black.
Values were sampled from game screenshots (the material trader, the commodity market, the
Options → Controls screen and the right-hand modules panel) and lifted slightly where
anti-aliased samples read darker than the game draws them.

### Primary: the orange ramp

- **Orange Lit** (`orange-lit`, 9.6:1 on Void): the one main value of an element (a tile's
  count, a row's price, the next system), and the active page tab. *Why:* the game draws its
  most important figures and highlighted heads in a lighter, yellower orange.
- **Orange** (`orange`, 6.7:1): the base. Names, available items, facts, section headings,
  action words, cell values, and the fill for anything selected. *Why:* it is the colour of
  ordinary text on every station screen.
- **Orange Mid** (`orange-mid`, 5.1:1 on Void, 4.8:1 on Hull): labels, column heads, units,
  secondary facts such as the system after a station name, inactive tabs, icons at rest.
  Never on an ember cell (3.8:1 there); text in cells is at least Orange. *Why:* one step down
  for what supports the main value, still above 4.5:1 where it is used.
- **Orange Dim** (`orange-dim`, 3.3:1 on Void, 3.1:1 on Hull): unavailable, zero, disabled, stale. *Why:* the
  trader's empty tiles: still legible, plainly absent, no word needed.
- **On Orange** (`on-orange`, 6.7:1 on Orange): text and icons on an orange fill.

### Surfaces: ember and hull

- **Hull** (`hull`): the read-only surface: tiles, the route strip, side-panel blocks.
  *Why:* the game's trader tiles and market row bands are a very dark warm brown.
- **Hull Raised** (`hull-raised`): popovers and menus, with a 1 px Seam border.
- **Row Hover** (`row-hover`): hover on a read-only row or tile.
- **Ember Cell** (`ember-cell`) and **Ember Cell Hover** (`ember-cell-hover`): every value
  the commander can change (input, dropdown, choice, binding slot, Settings value). *Why:*
  the game's binding and option cells are this brown; making it brighter than the hull keeps
  "can change" visible at a glance.
- **Ember Empty** (`ember-empty`): an unbound binding slot. *Why:* the game draws empty slots
  like filled ones; we make them visibly empty (see Deviations).
- **Seam** (`seam`): the only line colour besides orange: section rules, column-head
  hairlines, popover and badge outlines, empty-tile outlines. *Why:* the game's inactive
  section rules are this dim brown.

### Neutrals (each with a reason)

- **Void** (`void`): the window background. *Why:* the game's screens sit on a blurred
  cockpit that reads as neutral near-black; a neutral ground makes the orange look like
  light, and a brown ground would make the brown cells disappear.
- **White** (`white`): page and panel titles, the heading of the current section, the
  selected item's key value in a side panel, the wordmark. *Why:* the game uses white
  exactly there (screen titles, the trader's current section, the market's "91 CR").
- **Options Grey** (`options-grey`): row labels and binding text on Controls and Settings
  lists. *Why:* the game's Options screen, which those pages copy, draws its labels and
  bound keys in this grey-white.
- **Prose Grey** (`prose-grey`): paragraphs (descriptions, the side panel's explanation,
  guide-like text). *Why:* the game's "About this setting" text is grey; long passages in
  orange on black tire the eye.

### Alarm

- **Alarm Red** (`alarm-red`, 5.1:1): only (a) the age of a stale report, and (b) the "!"
  and the words of a hard error (something failed: a lookup, a file read, a write). *Why:*
  the game uses red for danger and malfunction (a failing module turns red), so red already
  means "this is not right". Warnings that ask for action stay orange; see Deviations.

### Named Rules

**The Intensity Rule.** Rank and state are shown by orange brightness and fill, not by
switching to another colour. Lit is the main value, base is ordinary, mid is supporting,
dim is unavailable, fill is selected. *Why:* that is the game's own system; it replaces the
first draft's "orange vs grey" signal, which the game never uses on station screens.

**The One Fill Per Group Rule.** In any group of alternatives (a tab row, a choice, a list,
a grid) at most one item is filled, except multiple choices where each chosen option is
filled. A view has at most one primary button. *Why:* the game fills the selected tab and
the selected row at the same time, but never two rows; a fill means "this one".

**The Narrow Red Rule.** Red appears only on a stale report's age and on a hard error. Never
for warnings, health issues, decoration, numbers going down, or emphasis. *Why:* red earns
its meaning by being rare; once it decorates, the next real failure is missed.

## Typography

**Face:** Saira (OFL), variable, at 87.5 % width (`font-stretch: 87.5%`, the semi-condensed
cut), from `@fontsource-variable/saira`, with Bahnschrift and Segoe UI as fallbacks. One face
for everything, including paths and logs. *Why:* condensed and square-shouldered like the
game's lettering without copying it; the variable file has tabular figures (the static
Semi Condensed files do not) and replaces three files (two Saira cuts and Share Tech Mono).

### Hierarchy

| Role | Size / weight / line | Case | Colour | Use |
|---|---|---|---|---|
| Wordmark | 18 / 600 / 1, tracking 0.12em | caps | white | "ASTROMECH" only |
| Title | 24 / 600 / 1.2 | mixed | white | page, finder and side-panel titles |
| Value XL | 28 / 500 / 1, tabular | figures | orange lit | the one value of a tile |
| Value L | 18 / 600 / 1.2, tabular | figures | orange lit | the one value of a row or strip |
| Nav | 15 / 500, tracking 0.06em | caps | orange mid; active on orange fill | top tabs |
| Section | 13 / 600, tracking 0.06em | caps | orange; current section white | section headings, with a Seam rule to the right |
| Name | 15 / 500 / 1.3 | mixed | orange | station, material, ship, engineer names; page tabs |
| Body | 15 / 400 / 1.45, max 68ch | sentence | prose grey | paragraphs |
| Figures | 15 / 400, tabular | figures | orange | numbers in columns and cells |
| Label | 13 / 400 / 1.3 | sentence | orange mid | column heads, field labels, units, secondary facts, badges |
| Meta | 12 / 400, tabular | figures | orange mid | corner marks in tiles |
| Action | 13 / 600, tracking 0.06em | caps | orange; on orange on a fill | buttons and text actions |

Units follow their number in Label size and Orange Mid: **390,780** cr, **7** jumps.
Thousands use commas. *Why:* the unit supports the number; it should not compete with it.

### Named Rules

**The Caps Are Wayfinding Rule.** Uppercase with 0.06em tracking only for the wordmark, top
tabs, section headings and action words. Names, column heads, labels, badges, status, errors
and prose are mixed or sentence case. *Why:* when everything is caps (the audit counted about
140 caps rules), caps signal nothing; mixed case also reads faster, and procedural system
names carry meaning in their case ("Synuefe GW-Z a54-3", "HR-W d1-42").

**The Tabular Rule.** Every number in a column, tile, cell or strip uses tabular figures and
is right-aligned. *Why:* the ones digit sits under the ones digit, so a column can be
compared by eye, as the game's market columns can.

## Layout

The window is a fixed frame: header, route strip, at most one page tab row, then the page.

### Spacing scale

`hair` 2 px · `xs` 4 · `sm` 8 · `md` 12 · `lg` 16 · `xl` 24 · `2xl` 32 · `gutter` 40 · `3xl`
48. Rows inside a section are 2 px apart (the game's cell gap), sections 32 px apart, a
heading 24 px below the last section and 12 px above its content. *Why:* tight groups and
wide separations let grouping be seen without boxes.

### Frame

| Part | Height | Notes |
|---|---|---|
| Header | 64 px | wordmark, top tabs, Find or plot, status, Help; 1 px orange rule under it |
| Route strip | 36 px | on every page |
| Page tabs | 44 px | one row, only when the area has more than one page |
| Page | rest | scrolls; the frame does not |

Side margins 40 px; minimum window width 1100 px, and nothing in the frame wraps above it.

### Content widths and rows

- Prose: 68ch. Settings and forms: 200 px label + up to 700 px cell. Result lists: up to
  1200 px, actions right after the last fact column. Tile grids: 200–240 px columns, 8 px
  gaps. Side panel: 320 px after a 1 px orange line. *Why:* a 1600 px row put the old GO
  button 1400 px from the name it acted on.
- Control row 34 px; result row 44 px, one line; material tile 76 px. *Why:* one-line rows
  fit about 18 results on a 1000 px window and keep columns aligned; the old 65–100 px rows
  fit 8.

### Named Rules

**The Two Tab Rows Rule.** Top tabs plus one row of page tabs, never more. A third level
becomes a choice, a section or a finder of its own. *Why:* the audit found up to four
stacked tab rows; the game never stacks more than two either.

**The Aligned Columns Rule.** Repeated facts are fixed-width columns with a header row. If a
fact would wrap, the column is wrong or the fact should go.

## Elevation & Depth

Flat and tonal: Void, then Hull, then Hull Raised for popovers. No shadows, blur or glow;
state changes are instant. *Why:* the game's panels are flat planes; its glow comes from the
cockpit and the HUD, which a desktop window does not need, and motion would draw the eye
away from the game on the other screen.

## Shapes

Square corners everywhere (the scrollbar thumb excepted). Borders are rare: Seam for
popovers, badges, column heads, empty tiles and section rules; 1 px orange for focus, the
header rule and the side-panel separator. No coloured side borders on cards. *Why:* the game
is square-cornered and nearly line-free; the trader's only lines are the section rules.

### Icons

One set of one-stroke icons drawn for astromech: 16 px on a 16 px grid, 1 px stroke,
`currentColor`, no fill, square ends. They live with the existing glyphs in
`app/src/lib/ui/Glyph.svelte` (copy, station kinds, ship, factions) and add **add stop**
(a route with a plus), **set destination** (a flag; replaces the route), **open external**
(a box with an arrow out) and **chevron** (dropdowns). *Why:* own-drawn keeps Frontier's
icons out and needs no licence; one stroke matches the game's thin line icons.

**The Icon Or Word Rule.**

- *Icons* for frequent, repeated, recognisable actions and facts: copy, add stop, set
  destination, open external, station type. Every icon button has a tooltip and an
  accessible label naming the action ("Add Diaguandri as a stop"). *Why:* a column of
  repeated words ("ADD STOP REPLACE ROUTE" on every row) is noise; an icon repeated down a
  column is learned once.
- *Words* for the primary action, one-off actions and anything destructive (NEXT, SAVE
  PRESET, RUN AGAIN, CHANGE). *Why:* those are read once and must not be guessed.
- *Icon plus word* where the icon alone is not obvious (the first time a new action
  appears, toolbar actions such as "Copy route").
- Pad size is a letter (S, M, L) in its own column, not an icon. *Why:* the letter is what
  the game shows and is already shorter than any icon.

Row action icons are visible at rest in Orange Mid and turn Orange Lit on the hovered row
(On Orange on a selected row). *Why:* hiding them until hover makes them undiscoverable and
moves the row under the pointer; at mid brightness they stay quiet, which was the real
problem with the old GO cell. Copy stays visible beside every system name.

## Components

### Surfaces and cells

**A cell means editable.** Ember cells hold every changeable value; read-only things use the
darker Hull or no surface at all. Material tiles, specs, engineer cards, results, chips and
summaries are never ember cells. *Why:* in the game a brighter brown slot is something you
set; the audit showed what happens when facts wear the same brown.

### Settings row (kept)

Label at 200 px in Options Grey · an ember cell with the current value in Orange · one action
word at the cell's right end in Orange (CHANGE, RUN AGAIN, PREVIEW). A choice row (voice
colour Natural / Cockpit / Harsh; callouts On / Off) uses choice cells, the current option
filled. Group description below in Label, Prose Grey. Section headings Orange caps with a
Seam rule. *Why:* this is the game's Options screen: grey labels, brown cells, orange values,
and the selected option filled like the selected landing-gear cell.

### Input

Ember cell, 34 px, Body text in Orange, placeholder Orange Mid, no border or underline at
rest, 1 px orange outline on focus. An error puts a red "!" and plain words directly under
it. *Why:* the game's text fields are brown boxes with orange text; the orange underline the
old header box had at rest looked like focus all the time.

### Dropdown

Ember cell like the input, value in Orange, an Orange chevron at the right. The open list is
a popover: Hull Raised, Seam border, 34 px rows, the current option filled orange. *Why:*
the game's dropdown cells are exactly this, with an orange ▼; the old dropdowns had no
chevron and did not look openable.

### Choice (segmented and filter options)

A row of ember cells, 2 px apart, Label size: unselected Orange; selected solid orange with
On Orange text. Single and multiple choice look the same; a multiple choice may have
several filled. "On" is always the filled one. *Why:* the game fills the chosen option; the
old Carriers/Odyssey toggles were drawn inverted.

### Buttons

- **Primary:** solid orange, On Orange text, Action type, 34 px, 20 px padding. At most one
  per view, for the step forward (Next, Save preset, Plot route). Disabled: Ember Empty with
  Orange Dim text. *Why:* the game fills the item under the cursor, and its cursor starts on
  the main action; a mouse-driven window has no such cursor, so the primary is filled
  instead (see Deviations).
- **Secondary (text action):** an orange caps verb, no fill, 34 px hit area. Hover: a Row
  Hover background. Disabled: Orange Dim.
- **Icon button:** a 28 px square holding one icon, Orange Mid at rest, Orange Lit on hover,
  tooltip and label always.
- **Links** to somewhere else (Help, a guide page): Orange text with a 1 px Orange Mid
  underline.

### Tabs

- **Top tabs:** Nav type, caps, Orange Mid; active is a solid orange box with On Orange text.
  *Why:* the game's panel tabs are filled when active.
- **Page tabs:** Name type, mixed case, Orange Mid; active Orange Lit with a 2 px orange
  underline. *Why:* one step down from the top row, so the two levels never look alike, and
  only one tab fill per view.
- Maximum two rows (The Two Tab Rows Rule).

### Badge

Label type, mixed case, Orange Mid, 1 px Seam outline, 20 px high, no fill. For a fact with
no column of its own ("Scoopable", "Permit", "Odyssey only"). *Why:* a fill would make it
look like a control or a selection.

### Material tile (picker grid)

A Hull tile, 76 px. The count in Value XL, Orange Lit, top left; the name below in Label,
Orange. The storage cap is the same for a whole grade, so it is stated once in the column
head ("Grade 1 · max 300"). A 2 px Seam track along the bottom edge fills Orange Mid with
count / cap, Orange Lit at the cap. The top right corner holds at most one Meta mark.

- **Have some:** as above.
- **Have none:** Void inside a 1 px Seam outline, all text Orange Dim.
- **Selected:** solid orange, On Orange text, no track. One per grid. The side panel explains
  it.
- **Wanted by a plan:** a "need 12" corner mark in Orange; a short amount gets an orange "!".

*Why:* this is the trader screen's tile, with its corner ratio replaced by what the
commander's plan needs, since the window does not trade.

### Lookup result row

One grammar for every lookup and every Places finder; finders differ only in their lead and
fact columns.

1. **Main value** (first, 112 px, right-aligned): Value L, Orange Lit, unit in Label, Orange
   Mid. It is what the list is ranked by: price (Sell), hotspots (Mining), jumps (Services),
   detour (Find along route).
2. **Place** (flexible, min 240 px): name in Name, Orange; system in Label, Orange Mid; the
   copy icon. One line, truncated before it wraps.
3. **Fact columns** (fixed widths, right-aligned figures in Orange, Label heads in Orange
   Mid): demand, distance, jumps, pad (always shown for stations), arrival, report age.
4. **Actions:** icon buttons for add stop and set destination, after the facts.

Rows are 44 px, no zebra, no separators. Hover: Row Hover. Selected: solid orange fill with
On Orange text (one per list). Column heads sit above a Seam hairline. A one-line summary of
the constraints sits above them in Label ("Within 100 ly of Sol · large pad · reports up to
30 days old"). *Why:* the market screen's grammar (columns, one filled row), with the ranked
value moved first and made largest, which the audit found was the smallest text in a row.

### Route strip

A 36 px Hull band under the header on every page: "Next" in Label · the next system in Name,
Orange Lit · copy icon · "3 of 41" in Figures, Orange Mid · next-jump flags as badges · on the
right the destination and "38 jumps left". An orange "!" with plain words when the commander
should act ("Refuel before the next jump", "Off route"). With no route: "No route" in Orange
Mid and the Find or plot shortcut. Never a second line. *Why:* users of every competing tool
ask for next stop, n of N and refuel on one line they can keep their eyes on.

### Find or plot

The header's one search box: an input, 320 px, placeholder "Find or plot", "Ctrl+K" at its
right end in Orange Mid. A system plots or adds a stop; a phrase opens a finder. Results in a
popover.

### Places home

Finder tiles under section headings (Trade, Ship, Engineering, Mining, Route). Each tile is a
Hull link, 220 px: finder name in Name, Orange Lit; one line in Label, Orange Mid; a live
answer in Figures where known. Hover: Row Hover. *Why:* a visible grid of what can be found,
like the station services board, so search and voice are shortcuts, not the only way in.

### Side panel (kept)

320 px after a 1 px orange line: title in white, facts in Orange with Label heads, the key
value in white, prose in Prose Grey, actions as words or icon buttons. *Why:* the game's
right-hand detail panel (white item name, orange facts, white key price) and the Controls
"About this setting" panel.

### States

| State | Treatment | Why |
|---|---|---|
| Selected row, tile, option, tab | solid orange, On Orange text | the game's selection |
| Active page tab | Orange Lit + 2 px orange underline | one step below the top tab |
| Current section | heading white, its rule orange | the trader screen |
| Hover | Row Hover on read-only things; Ember Cell Hover on cells; icons Orange Lit | a response without a new colour |
| Focus | 1 px orange outline, 2 px offset | keyboard users need to see where they are |
| Unavailable, none, zero, disabled | Orange Dim; no surface lift | the trader's empty tiles |
| Stale report | the row or tile in Orange Dim; the age in Alarm Red ("212 d") | dim says "do not trust this", red says why |
| Warning (act on it) | orange "!" + plain words, at the place it applies | the game's orange warnings; urgent but not failed |
| Hard error (it failed) | red "!" + plain words + a recovery action ("! Could not read the route. TRY AGAIN"); a whole-page error is a white title, a prose line and the action. Never a filled block | red is the game's malfunction colour |
| Empty | one prose line saying what is empty and what to relax, plus one action ("No buyers within 100 ly. WIDEN TO 250 LY") | an empty list with no way forward is a dead end |
| Loading | one Orange Mid line naming what is fetched ("Asking Spansh…"); a refresh keeps old rows Dim until replaced. No spinners or skeletons | motion pulls the eye; dim old rows keep the layout still |
| Listening (press to bind) | binding cell filled orange, On Orange caps | the game's listening cell |
| Conflict, ignored binding | orange "!" + "Also: <action>"; ignored text struck through, Dim | a warning, not a failure |

A report is stale when it is older than the finder's age filter (30 days by default). With the
filter at "Any age", stale rows are shown dimmed with the red age; otherwise they are hidden
and the summary line says how many.

## Deviations from the game

Each place we depart from Elite's UI, what we do instead, and why.

| The game | astromech | Why |
|---|---|---|
| All text in caps | Caps only for the wordmark, top tabs, section headings and action words; mixed case elsewhere | Caps everywhere signals nothing and reads slower; system names carry meaning in their case. Decided for row labels on 11 Oct (V1-DESIGN §9) |
| Red, green and blue for status (green supply bars, green/red price arrows, blue mission goods) | Only one extra colour: red, for stale report ages and hard errors | One semantic colour is enough for a lookup window; green and blue would bring a second meaning system to learn |
| Red for warnings and damage alike | Warnings stay orange "!"; red only when something failed or data is untrustworthy | Keeps red rare enough to notice; an orange "!" already reads as "look here" in the game |
| The item under the gamepad cursor is filled, so the default action starts filled | The primary button is always filled; other buttons are text actions | A mouse window has no cursor highlight; the primary needs the fill to be found |
| Empty binding slots look like filled ones | Empty slots are the dimmer Ember Empty with dim "Unbound" text | Finding unbound actions is a main job of the Controls pages |
| Blurred cockpit behind every panel | Flat neutral Void | Calmer and more legible; no Frontier art behind the window |
| Glow, scan lines and animated panel transitions | Flat, no glow, no motion | A desktop window beside the game should not pull the eye |
| Prose in orange on station screens | Paragraphs in Prose Grey | Long orange passages tire the eye; the game's own Options screen uses grey for its explanations |
| Long single-screen lists with no search | Search, Find or plot, aligned columns and one-line rows | The window is a lookup tool; the game's lists are a known pain (PRODUCT.md) |
| Every value cell has a corner ratio or icon | Corner marks only for what the commander's plan needs | The window does not trade; the plan is the useful corner fact |
| Game fonts and icons | OFL Saira and astromech's own icons | No Frontier assets (PRODUCT.md) |
| One face per role in places (the Options screen mixes cuts) | One face, Saira variable, for everything including paths | Fewer files, one rhythm, tabular figures everywhere |
| No route strip | A route strip on every page | The most requested missing piece in companion tools (research, 24 Oct) |

## Do's and Don'ts

### Do:

- **Do** start from what the game does and write down why when you depart from it.
- **Do** give each element one main value in Orange Lit and the largest size.
- **Do** show rank and state with the orange ramp and the fill: lit, base, mid, dim, filled.
- **Do** fill the selected tab, row, tile or option with solid orange and dark text.
- **Do** keep changeable values in ember cells and read-only things on Hull or Void.
- **Do** use white only for titles, the current section and the selected item's key value.
- **Do** use icons for repeated row actions, with a tooltip and a label, and words for
  primary and one-off actions.
- **Do** set numbers in tabular figures, right-aligned, units in Orange Mid.
- **Do** keep the route strip on every page and a copy icon beside every system name.
- **Do** keep the patterns that work: the Settings row, the Controls side panel, ship
  cartoons, onboarding's step list and primary button, plain-word status and errors.

### Don't:

- **Don't** use grey where the game uses orange; neutral greys only for the window, Options
  labels and prose.
- **Don't** put read-only tiles, specs, chips or cards in ember cells.
- **Don't** fill more than one item in a group, or show more than one primary button.
- **Don't** use red for warnings, health issues, decoration or emphasis.
- **Don't** stack more than two tab rows.
- **Don't** put repeated words like ADD STOP on every row, or hide row actions until hover.
- **Don't** make the ranked value the smallest text in a row.
- **Don't** draw a warning or error as a solid colour block.
- **Don't** use a second typeface, a radius, a shadow, a glow or an animation.
- **Don't** use Frontier art, fonts, icons or screenshots, or any language but English.
