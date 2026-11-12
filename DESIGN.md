---
name: astromech
description: The window beside Elite Dangerous, drawn in the game's own orange. Sparse, scannable, lit where it matters.
colors:
  orange: "#ff7100"
  orange-lit: "#ffa04a"
  orange-mid: "#d97a34"
  orange-dim: "#8a5834"
  placeholder: "#ff7100bf"
  on-orange: "#0c0d0f"
  ember-cell: "#3d220f"
  ember-cell-hover: "#482810"
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
    fontWeight: 500
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
    textColor: "{colors.orange-mid}"
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

<!-- Target system (astromech-c4i0), revision 3: revised 25 Oct 2026 after Sumeet's review,
then 4 Nov 2026 with the changes the independent panel asked for (decision astromech-5p9c).
Tokens: app/src/design/tokens.css, mapped in Token map below. The pages moved to the orange
ramp and the caps rule with astromech-yxtz, to cells-mean-editable with astromech-dfvp, and to
the one face with astromech-dzwy, and to two tab rows with astromech-sllo. Mockups:
docs/design/design-system-*.png.
Revision 4, 12 Nov 2026 (astromech-b2k3): the Glow-up amendments Sumeet approved on 10 Nov
(decision astromech-ci0o, mission astromech-xajc; mockups in app/src/mockups/glow/ and
docs/design/glow-up-*.png on the mockups/glow-up branch): page layouts in place of the width
caps, lifted entity names, brief state feedback, the star, body and station icons, collapsed
filters and the NOW tab. The rest of that list lands with its own
item, which writes it here: the height scale and the new orange values (astromech-k6pl), Tip
(astromech-sixj) and the status bar (astromech-q79q). -->

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
- White only for titles, the current section, the selected item's key value and, where it
  ranks them, entity names.
- Red only for hard errors.
- One face (variable Saira), tabular figures, square corners, no shadow, no glow, and no
  motion beyond a control's brief confirmation that it worked.

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
anti-aliased samples read darker than the game draws them. Revision 4 (astromech-k6pl,
12 Nov 2026) made Orange and Orange Mid stronger: the sampled `#fa6e00` and `#c26c32` looked
washed out next to the game, which draws its orange as light.

### Primary: the orange ramp

- **Orange Lit** (`orange-lit`, 9.6:1 on Void): the one main value of an element (a tile's
  count, a row's price, the next system), and the active page tab. *Why:* the game draws its
  most important figures and highlighted heads in a lighter, yellower orange.
- **Orange** (`orange`, 7.1:1): the base. Names, available items, facts, section headings,
  action words, cell values, and the fill for anything selected. *Why:* it is the colour of
  ordinary text on every station screen.
- **Orange Mid** (`orange-mid`, 6.3:1 on Void, 5.9:1 on Hull): units, column heads,
  secondary facts such as the system after a station name, inactive tabs, icons at rest, and
  the few labels a value needs. Use labels sparingly: a value that says what it is (a system
  name, "7 jumps", "27.4 / 32 t") needs no label in front of it, and a label it doesn't need is
  one more Orange Mid word to read past. Never on an ember cell (4.7:1 there): text in cells is
  at least Orange, the placeholder excepted. The darker Ember Empty is the exception (5.8:1): an
  empty binding slot's word is Orange Mid. *Why:* one step down for what supports the main
  value, far enough above 4.5:1 to read from the seat beside the game.
- **Orange Dim** (`orange-dim`, 3.3:1 on Void, 3.1:1 on Hull): unavailable, zero, disabled.
  It is state, not content, and knowingly below WCAG AA: it says "not this one", and it never
  carries text the commander needs in order to act. Anything they must read to decide stays
  at Orange Mid or brighter. *Why:* the trader's empty tiles: still legible, plainly absent,
  no word needed.
- **On Orange** (`on-orange`, 7.1:1 on Orange): text and icons on an orange fill, and the
  focus outline on a filled element.
- **Placeholder** (`placeholder`, Orange at 75 % opacity, 3.6:1 on an ember cell): the hint
  text in an empty input, and nothing else. It disappears as soon as there is a value, and the
  input always has a label of its own (visible, or an accessible name for the header's Find
  or plot). *Why:* one step below the value it stands in for, without a second hue in a cell.

### Surfaces: ember and hull

- **Hull** (`hull`): the read-only surface: tiles, the route strip, side-panel blocks.
  *Why:* the game's trader tiles and market row bands are a very dark warm brown.
- **Hull Raised** (`hull-raised`): popovers and menus, with a 1 px Seam border.
- **Row Hover** (`row-hover`): hover on a read-only row or tile.
- **Ember Cell** (`ember-cell`) and **Ember Cell Hover** (`ember-cell-hover`): every value
  the commander can change (input, dropdown, choice, binding slot, Settings value). *Why:*
  the game's binding and option cells are this brown; making it brighter than the hull keeps
  "can change" visible at a glance. The hover is only as bright as keeps Orange text at 4.5:1
  (4.8:1); the placeholder there is 3.4:1, a hint the input's own label backs up.
- **Ember Empty** (`ember-empty`): an unbound binding slot. *Why:* the game draws empty slots
  like filled ones; we make them visibly empty (see Deviations).
- **Seam** (`seam`): the only line colour besides orange: section rules, column-head
  hairlines, popover and badge outlines, and empty-tile outlines. (The scrollbar thumb is
  Orange Dim at rest and Orange Mid under the pointer or while dragged: Seam is too faint to find
  on Void.) *Why:* the game's inactive
  section rules are this dim brown.

### Neutrals (each with a reason)

- **Void** (`void`): the window background. *Why:* the game's screens sit on a blurred
  cockpit that reads as neutral near-black; a neutral ground makes the orange look like
  light, and a brown ground would make the brown cells disappear.
- **White** (`white`): page and panel titles, the heading of the current section, the
  selected item's key value in a side panel, the wordmark, and entity names lifted under
  [The Lifted Name Rule](#named-rules). The **current section** is the one
  holding the item a side panel describes (the selected row, tile or binding); a page with no
  such selection has none. Focus and scroll position do not make a section current. *Why:* the game uses white
  exactly there (screen titles, the trader's current section, the market's "91 CR").
- **Options Grey** (`options-grey`): row labels and binding text on Controls and Settings
  lists, and a long column of entity names lifted under The Lifted Name Rule. *Why:* the
  game's Options screen, which those pages copy, draws its labels and bound keys in this
  grey-white.
- **Prose Grey** (`prose-grey`): paragraphs (descriptions, the side panel's explanation,
  guide-like text). *Why:* the game's "About this setting" text is grey; long passages in
  orange on black tire the eye.

### Alarm

- **Alarm Red** (`alarm-red`, 5.1:1): only the "!" and the words of a hard error (something
  failed: a lookup, a file read, a write). *Why:* the game uses red for danger and malfunction
  (a failing module turns red), so red already means "this is not right". Warnings that ask
  for action stay orange, and so does staleness, which brightness carries (see
  [States](#states)); each hue keeps one meaning.

### Named Rules

**The Intensity Rule.** Rank and state are shown by orange brightness and fill, not by
switching to another colour. Lit is the main value, base is ordinary, mid is supporting,
dim is unavailable, fill is selected. *Why:* that is the game's own system; it replaces the
first draft's "orange vs grey" signal, which the game never uses on station screens.

Decision astromech-bi94 asked for "orange only where it means something". With orange as the
base text colour, that rule is now carried by brightness and fill: Lit and fill appear only
where they mean something (the one main value, the one selection), never as decoration.

**The One Fill Per Group Rule.** In any group of alternatives (a tab row, a choice, a list,
a grid) at most one item is filled, except multiple choices where each chosen option is
filled. A view has at most one primary button. A list or grid has a filled item only when
something shows the selection: a side panel about it, or keyboard focus in the list. There is
no default selected row. *Why:* the game fills the selected tab and the selected row at the
same time, but never two rows; a fill means "this one", and a fill that nothing explains is
decoration.

**The Lifted Name Rule.** Outside Controls, an entity name (a station, body, module, system
or engineer) may be White or Options Grey instead of Orange where the lift ranks it. Lift a
name when the element is about the named thing and Orange would sink it among facts of the
same size and colour: the station a Now answer names, the bodies down a system tree, the
engineered modules of a loadout, the system a Powerplay page is about. Leave it Orange when
the element is ranked by a value (a finder row's price ranks it, so its station stays Orange),
and when every name on the page would be lifted, since a lift that holds for all of them ranks
none. White for a few names that lead; Options Grey for a long column of them, one step
quieter so the titles still lead. A lifted name keeps its role's size and never outranks the
element's main value. *Why:* with everything in one orange at one size, the 10 Nov audit found
nothing ranked; the game itself draws its few key names (screen titles, the selected item) in
white, so a lift reads as "this is the thing" without a new hue.

**The Narrow Red Rule.** Red appears only on a hard error. Never for stale data, warnings,
health issues, decoration, numbers going down, or emphasis. *Why:* red earns its meaning by
being rare; once it decorates, the next real failure is missed.

## Typography

**Face:** Saira (OFL), variable, at 87.5 % width (`font-stretch: 87.5%`, the semi-condensed
cut), from `@fontsource-variable/saira`, with Bahnschrift and Segoe UI as fallbacks. One face
for everything, including paths and logs. *Why:* condensed and square-shouldered like the
game's lettering without copying it; the variable file has tabular figures (the static
Semi Condensed files do not) and replaced three files (two Saira cuts and Share Tech Mono).
`tokens.css` sets the face, its width and tabular figures once on the page, so every element
has them; the `font` shorthand resets the width, so set `font-stretch` after it. It also sets
weight 500, so text is 500 unless its role below says otherwise; Body, Label and Meta are 400.
*Why:* orange text at 400 looked thin and washed out at the game's distance.

The sizes below are CSS px at zoom 1, a window up to 1000 px tall. The window scales them all with
its height ([The Height Scale Rule](#named-rules-1)).

### Hierarchy

| Role | Size / weight / line | Case | Colour | Use |
|---|---|---|---|---|
| Wordmark | 18 / 600 / 1, tracking 0.12em | caps | white | "ASTROMECH" only |
| Title | 24 / 600 / 1.2 | mixed | white | page, finder and side-panel titles |
| Value XL | 28 / 500 / 1, tabular | figures | orange lit | the one value of a tile |
| Value L | 18 / 600 / 1.2, tabular | figures | orange lit | the one value of a row or strip |
| Nav | 15 / 500, tracking 0.06em | caps | orange mid; active on orange fill | top tabs |
| Section | 13 / 600, tracking 0.06em | caps | orange; current section white | section headings, with a Seam rule to the right |
| Name | 15 / 500 / 1.3 | mixed | orange; white or options grey under The Lifted Name Rule | station, material, ship, engineer names; page tabs |
| Body | 15 / 400 / 1.45, max 68ch | sentence | prose grey | paragraphs |
| Figures | 15 / 500, tabular | figures | orange | numbers in columns and cells |
| Label | 13 / 400 / 1.3 | sentence | orange mid | column heads, field labels, units, secondary facts, badges |
| Meta | 12 / 400, tabular | figures | orange mid | corner marks in tiles |
| Action | 13 / 600, tracking 0.06em | caps | orange; on orange on a fill | buttons and text actions |

The Colour column gives each role's default. A component may override it where its own
spec says so (a material tile's name is Label size in Orange; anything on a fill is On
Orange; a stale row's facts step down to Orange Mid).

Units follow their number in Label size and Orange Mid: **390,780** cr, **7** jumps.
Thousands use commas. *Why:* the unit supports the number; it should not compete with it.

### Named Rules

**The Caps Are Wayfinding Rule.** Uppercase with 0.06em tracking only for the wordmark, top
tabs, section headings and action words. Names, column heads, labels, badges, status, errors
and prose are mixed or sentence case. *Why:* when everything is caps (the audit counted about
140 caps rules), caps signal nothing; mixed case also reads faster, and procedural system
names carry meaning in their case ("Synuefe GW-Z a54-3", "HR-W d1-42").

**The Height Scale Rule.** The whole window scales with its height, as the game scales its UI
with the screen's. One root zoom does it, so every px value in this document and in
`tokens.css` stays as written:

    zoom = clamp(1, min(height / 1000, width / 1100), 1.8) × text size

`height` and `width` are the window's, in CSS px. The width term keeps the layout at least
1100 px wide (see [Frame](#frame)), so a tall, narrow window is not zoomed past its width. The
floor of 1 means text never shrinks below these sizes, and a short window scrolls. The cap is
1.8. The commander's **Text size** (Settings → General: Smaller, Normal, Larger, which is 0.85,
1 and 1.15) multiplies the result; it is `[window] text_size` in the core's config.

| Window | Zoom at Normal |
|---|---|
| 1100 × 700 | 1 (unchanged) |
| 1920 × 1080 | 1.08 |
| 2560 × 1440 | 1.44 |
| 3840 × 2160 | 1.8 (capped) |
| 1100 × 1400 | 1 (the width term) |

`app/src/lib/scale.svelte.ts` sets it on the root as `zoom` and `--zoom`. CSS zoom scales `vh`
and `vw` as well, so a size taken from the window uses `--win-h` (its height) or `--win-w` (1 %
of its width), and a page's narrow-window rule is a container query on the frame,
`@container win (max-width: …)`, which measures the layout. A media query measures the screen,
and would leave a zoomed window on its wide layout. *Why:* fixed px sizes left a 1440p or 4K
window's text about the size of the game's smallest; the game reads the same on any screen.

**The Tabular Rule.** Every number uses tabular figures, and a column of quantities is
right-aligned. A tile's value sits where its tile spec puts it (the material tile's count top
left); the values in a fact list and dates in a column read as text and stay left-aligned.
*Why:* the ones digit sits under the ones digit, so a column can be compared by eye, as the
game's market columns can; a fact list or a date has no column of quantities to compare.

## Layout

The window is a fixed frame: header, route strip, at most one page tab row, the page, then the
status bar.

### Spacing scale

`hair` 2 px · `xs` 4 · `sm` 8 · `md` 12 · `lg` 16 · `xl` 24 · `2xl` 32 · `gutter` 40 · `3xl`
48. Rows inside a section are 2 px apart (the game's cell gap), sections 32 px apart, a
heading 24 px below the last section and 12 px above its content. *Why:* tight groups and
wide separations let grouping be seen without boxes.

### Frame

| Part | Height | Notes |
|---|---|---|
| Header | 64 px, or 64 + 44 px | wordmark, top tabs, Find or plot, Help; 1 px orange rule under it. Below the width where one row fits (about 1456 px with the 320 px Find or plot box, and about 1606 px with Finish setup, remeasured 12 Nov 2026 once the status moved to the status bar) the top tabs sit on a 44 px second row under the wordmark |
| Route strip | 36 px | on every page |
| Page tabs | 44 px | one row, only when the area has more than one page |
| Page | rest | scrolls; the frame does not |
| Status bar | 32 px | on every page: the ship, its fuel and cargo, and the core's state |

Side margins 40 px; minimum window width 1100 px. Every size here is in CSS px before the root
zoom ([The Height Scale Rule](#named-rules-1)), and 1100 px is the narrowest the zoomed layout
gets at Normal text size: the header's widths above are layout widths, the window's divided by
its zoom. The only width-dependent change in the frame is
the header's second tab row (decision astromech-ghyo, 5 Nov 2026).

**Height budget.** At the smallest window, 1100 x 700, the fixed parts leave the page at least
480 px: header 108 (stacked), route strip 36, page tabs 44 and status bar 32 take 220 px, which
leaves exactly 480 (measured 12 Nov 2026). A fixed part that would take more moves the status bar
onto the route strip's row at that size instead of shrinking the page.

The top tabs start with **NOW**, and the window opens on the Now page: what the commander is
doing at the moment (docked, in a system, or Elite not running). *Why:* the window used to open
on My controller and filed the live views (Station, Session, Mining) under Logbook, so it had
no home page (audit, 10 Nov 2026).

### Page layouts

Every page outside Controls is built on one of four layouts. The layout owns the page's
widths, gutters and detail panel; a page picks one and sets no width of its own. A layout
places content, it never adds any: a sparse page stays sparse.

| Layout | What it holds | Use it when | For example |
|---|---|---|---|
| Board | blocks or tiles laid out across the width, each with its own main value; where a tile can be selected, the 320 px side panel after a 1 px orange line describes it | the page is answers or tiles read at a glance, laid out by their own shape (a grid of categories and grades, a row of answers) | Now, Powerplay, Places home; Materials, with its side panel |
| List with detail | a list (rows or a tree) and the 320 px side panel after a 1 px orange line, which describes the selected item | the commander picks one item from many and then wants its details | Places finders, System, Ships, Engineers |
| Workbench | the inputs on the left, the result on the right | the commander sets up a question and reads one answer that changes as the inputs do | Trade > Commodities |
| Form | Settings rows in one column: a 200 px label and a cell up to 700 px, 900 px in all | the page is values to set | Settings |

Pick by the commander's job on the page, not by how much there is to show. When a window is
too narrow for a side panel (the 1100 x 700 minimum), the layout moves the panel below the
content. It never shrinks text to make it fit.

Inside a layout: prose 68ch; tile grids 200–240 px columns with 8 px gaps; control row 34 px;
result row 44 px, one line; material tile 76 px. *Why:* one-line rows fit about 18 results on a
1000 px window and keep columns aligned; the old 65–100 px rows fit 8.

### Named Rules

**The Two Tab Rows Rule.** Top tabs plus one row of page tabs, never more. A third level
becomes a choice, a section or a finder of its own. *Why:* the audit found up to four
stacked tab rows; the game never stacks more than two either.

**The Aligned Columns Rule.** Repeated facts are fixed-width columns with a header row. If a
fact would wrap, the column is wrong or the fact should go.

**The Next To It Rule.** An action sits next to what it acts on. A row is as wide as its
columns, not as the window, and its actions come right after its last fact column; a side
panel's actions act on the item the panel describes; a form's action word ends its cell. The
test: at any window size, no action is more than one column from the thing it acts on.
Spare width stays empty or goes to the side panel; it never stretches a row. *Why:* a 1600 px
row put the old GO button 1400 px from the name it acted on, and the fixed width caps that
followed (60rem on eight pages) left half of a big window empty, with the page pinned left.

## Elevation & Depth

Flat and tonal: Void, then Hull, then Hull Raised for popovers and the Tip. No shadows, blur or glow;
state changes are instant. *Why:* the game's panels are flat planes; its glow comes from the
cockpit and the HUD, which a desktop window does not need, and motion would draw the eye
away from the game on the other screen.

**The Brief Feedback Rule.** The one timed change allowed is a control confirming what it
just did, where nothing else on screen shows it: the copy icon becomes a check (the `copied`
glyph) for 1.5 s, then turns back. The swap is instant both
ways, on the control the commander used, and says only that the action worked. Nothing
fades, slides, pulses, blinks or spins; pages change without a transition; loading is a line
of words (see [States](#states)). Decorative motion stays banned. *Why:* a copy leaves no
trace on screen, so without the check the commander copies twice to be sure; a swap that
lasts a moment and stops answers that without pulling the eye.

## Shapes

Square corners everywhere (the scrollbar thumb excepted). Borders are rare: Seam for
popovers, badges, column heads, empty tiles and section rules; 1 px orange for focus, the
header rule and the side-panel separator; Orange Mid for the Tip. No coloured side borders on
cards. *Why:* the game is square-cornered and nearly line-free; the trader's only lines are the
section rules.

### Icons

One set of one-stroke icons drawn for astromech: 16 px on a 16 px grid, 1 px stroke,
`currentColor`, no fill, square ends. They live in `app/src/lib/ui/Glyph.svelte`: copy and
copied, **add stop** (a route with a plus), **set destination** (a flag; replaces the route),
**open external** (a box with an arrow out), **you are here** (a map pin, the commander's row on the Route page), **chevron** (dropdowns), **clear** (an ×, the end
action that empties an input), **more** (three dots, a menu of one-off actions), **help** (a
question mark, an explanation in its Tip), ship and the factions. *Why:* own-drawn keeps Frontier's icons
out and needs no licence; one stroke matches the game's thin line icons.

Three type sets say what a star, body or station is, so a row or a tree shows it without a
word:

- **Stars:** scoopable (a plain disc), other non-scoopable (a disc with eight rays), brown
  dwarf (a small banded disc, no rays), white dwarf (a small disc with four short rays),
  neutron star (a pulsar: a small core, its two beams and a faint ring), black hole (a disc
  inside a flat ring) and unknown (a dashed circle).
- **Bodies:** a disc for every planet, told apart by what is inside it: rocky, high metal
  content (a metal core), metal-rich, icy, rocky ice, water world, Earth-like, ammonia world,
  gas giant, water giant and ringed (a disc in a tilted ring).
- **Stations:** Coriolis (a many-faced hull with its letterbox), Orbis (a wheel on a spindle),
  Ocellus (a ball with its docking eye), outpost, planetary port (a port on the horizon),
  settlement, asteroid base, fleet carrier and megaship. These replace the old station kinds
  (one starport icon for all three starports).

A type icon is a fact, not a control: it sits before the name it describes, in that name's
colour, at 16 px in a row and 24 px beside a title, and its accessible name is the type
("Orbis starport", "Neutron star"). A type the set does not draw, such as a rare star class,
uses the nearest shape and keeps its own name.

**The Icon Or Word Rule.**

- *Icons* for frequent, repeated, recognisable actions and facts: copy, add stop, set
  destination, open external, star, body and station type. Every icon button has a Tip and
  an accessible label naming the action ("Add Diaguandri as a stop"). *Why:* a column of
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

A read-only card or tile that holds controls (a ship module card with its blueprint dropdowns)
is Hull; only the controls inside it are cells. A read-only state on a card is shown by
brightness (an unlocked engineer's name in Orange Lit, one not yet introduced in Orange Mid),
never by a cell, a fill or a coloured side border.

### Facts, stats and meters

- **Fact list:** read-only label and value pairs (ship specs, Diagnostics, a test's result, the
  side panel's facts). The label in Label, Orange Mid (Options Grey on Controls and Settings-style
  pages), the value in Orange; on the window, or grouped in a Hull block. On a Settings-style page
  a fact lines up with the cell text beside it but has no cell.
- **Multi-value stat:** one label naming the values in order ("Jump min / current / max"), then
  the values in Figures, separated by an Orange Mid "|", with one unit after the last.
- **Meter:** a Seam track, 2 to 6 px high, filled from the left in Orange; the material tile's
  track and the shopping list's fill Orange Mid and turn Orange Lit when full. No label inside the
  track; the figure beside it says how much. *Why:* the trader tile's bar, kept thin so the
  number stays the main value.

### Settings row (kept)

Label at 200 px in Options Grey · an ember cell with the current value in Orange · one action
word at the cell's right end in Orange (CHANGE, RUN AGAIN, PREVIEW). A choice row (voice
colour Natural / Cockpit / Harsh; callouts On / Off) uses choice cells, the current option
filled. Group description below in Label, Prose Grey. Section headings Orange caps with a
Seam rule. *Why:* this is the game's Options screen: grey labels, brown cells, orange values,
and the selected option filled like the selected landing-gear cell.

A status the commander changes with its one action is a changeable value, so it sits in a
cell with that action: "Setup: Done · RUN AGAIN", "Voice: Listening while Elite runs ·
PAUSE". A status with no action is a fact and sits outside a cell ("Speech models:
Installed"). *Why:* the cell still means "you can change this here"; the action word says
how.

### Input

Ember cell, 34 px, Body text in Orange, placeholder in Placeholder (Orange at 75 %), no border
or underline at rest, 1 px orange outline on focus. An error puts a red "!" and plain words
directly under it. *Why:* the game's text fields are brown boxes with orange text; the orange
underline the old header box had at rest looked like focus all the time.

### Dropdown

Ember cell like the input, value in Orange, an Orange chevron at the right. The open list is
a popover: Hull Raised, Seam border, 34 px rows, the current option filled orange. *Why:*
the game's dropdown cells are exactly this, with an orange ▼; the old dropdowns had no
chevron and did not look openable.

### Choice (segmented and filter options)

A row of ember cells, 2 px apart, Label size: unselected Orange; selected solid orange with
On Orange text. Single and multiple choice look the same; a multiple choice may have
several filled. The chosen option is always the filled one; never draw it inverted. *Why:*
the game fills the chosen option; the old Carriers/Odyssey toggles were drawn inverted.

### Buttons

- **Primary:** solid orange, On Orange text, Action type, 34 px, 20 px padding. At most one
  per view, for the step forward (Next, Save preset, Plot route). Disabled: Ember Empty with
  Orange Dim text. *Why:* the game fills the item under the cursor, and its cursor starts on
  the main action; a mouse-driven window has no such cursor, so the primary is filled
  instead (see Deviations).
- **Secondary (text action):** an orange caps verb, no fill, 34 px hit area. Hover: a Row
  Hover background. Disabled: Orange Dim.
- **Icon button:** a 28 px square holding one icon, Orange Mid at rest, Orange Lit on hover,
  a Tip and a label always.
- **Links** to somewhere else (Help, a guide page): Orange text with a 1 px Orange Mid
  underline.

### Tabs

- **Top tabs:** Nav type, caps, Orange Mid; active is a solid orange box with On Orange text.
  *Why:* the game's panel tabs are filled when active.
- **Page tabs:** Name type, mixed case, Orange Mid; active Orange Lit with a 2 px orange
  underline. *Why:* one step down from the top row, so the two levels never look alike, and
  only one tab fill per view.
- Maximum two rows (The Two Tab Rows Rule).

### Tip

The window's one tooltip, in place of the native title, which takes about a second to appear,
is small and is ignored. One shared Tip (`app/src/lib/ui/tip.ts`, styled in `tokens.css`):

- **When:** at once on hover or keyboard focus; its content swaps in place when the pointer
  moves to the next target; gone when the pointer leaves, on scroll or on Escape. No delay or
  fade (Elevation & Depth: state changes are instant).
- **Look:** Hull Raised with a 1 px Orange Mid border and a pointer at the target. Above the
  target, below it near the top edge, always inside the window.
- **Plain:** one line in Body, Orange. **Rich:** a title in white, a fact line in Orange and a
  note in Label, Prose Grey.
- **Where:** every icon button (its label is its Tip unless it says more), and anything whose
  explanation is not on screen: a value's conditions ("Full tank and full cargo"), a warning's
  detail, a station type icon. Not on text that only repeats itself, such as a name cut short by
  an ellipsis.
- **Access:** screen readers get the same words through `aria-describedby` (not when they only
  repeat the target's label). A target the keyboard can't reach gets a tab stop, except a mark
  repeated down a column whose words are already its accessible name.

A `title=` on an interactive element fails `app/src/lib/ui/tip.test.ts`. *Why:* the commander
glances over from the game; an explanation has to be there the moment the pointer is.

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

Rows are 44 px, no zebra, no separators. Hover: Row Hover. Selected: solid orange fill with On
Orange text, one per list, and only while something shows the selection (a side panel about the
row, or keyboard focus in the list); a plain finder list has no filled row. A stale row keeps
its name in Orange, steps its main value and facts down to Orange Mid and shows its report age
in Orange, so the age is the brightest fact on the row. Column heads sit above a Seam hairline.
*Why:* the market screen's grammar (columns, one filled row), with the ranked value moved first
and made largest, which the audit found was the smallest text in a row.

**Collapsed filters.** Above the column heads a finder shows its filters as one summary line in
Label ("Within 100 ly of Sol · large pad · reports up to 30 days old") with CHANGE after it,
and no filter controls. The thing searched for (the commodity, module or service) is not a
filter: it stays an input above the summary. CHANGE opens the controls in place; each edit
applies at once and searches again, and the controls stay open across edits, so several
filters change in one visit. They close on DONE, on Escape or on CHANGE again, never because a
search started, finished or failed. A filter set away from its default is always named in the
summary. *Why:* up to five rows of filter controls stood above the first result, though the
commander changes them rarely and reads the results every time, and the summary line above the
results already named them.

Text fact columns (faction, module, bodies) share the width left after the fixed columns with
the place, and truncate with an ellipsis rather than wrap. Where even that leaves no room, as in
a side panel, the row is **compact**: main value, place and actions only, no fact columns or
heads. *Why:* the Aligned Columns Rule drops the fact before it wraps the row; a 320 px panel
cannot hold the facts, and its lookups become links into their finder later (decision
astromech-g8d5, item 5).

### Route strip

A 36 px Hull band under the header on every page: "Next" in Label · the next system in Name,
Orange Lit · copy icon · "3 of 41" in Figures, Orange Mid · next-jump flags as badges · on the
right the destination and "38 jumps left". An orange "!" with plain words when the commander
should act ("Refuel before the next jump", "Off route"). With no route: "No route" in Orange
Mid and the Find or plot shortcut. Never a second line. *Why:* users of every competing tool
ask for next stop, n of N and refuel on one line they can keep their eyes on.

### Status bar

A 32 px band at the foot of every page with a 1 px Seam rule over it, Label size, one line. On
the left the current ship's name in Orange and its type in Orange Mid, then "Jump" with its
unladen maximum, "Fuel" (main tank | capacity) and "Cargo" (tonnes aboard | capacity) as
multi-value stats. On the right the core's state ("Elite running · Voice ready"), Orange Mid, or
Orange when Elite or the core isn't running. Fuel and cargo show only while Elite runs; a failed
read drops them silently, since the next read on the 2 s tick is the recovery. *Why:* the ship and
the app's health belong to no one page, and taking the state out of the header gives it room.

### Find or plot

The header's one search box: an input, 320 px, placeholder "Find or plot", "Ctrl+K" at its
right end in Label size, Orange (it is text in a cell, so not Orange Mid). A system plots or
adds a stop; a phrase opens a finder. Results in a popover: one 34 px row each, the name in
Orange and what it is in Label, Orange Mid ("System", "Material traders", "Page · Trade"),
systems and entries ranked together, the highlighted row filled. A system row ends with PLOT
(a word: it opens the plot panel) and the add stop and set destination icon buttons. With text in
the box the clear icon replaces "Ctrl+K".

### Places home

Finder tiles under section headings (Trade, Ship, Engineering, Mining, Route). Each tile is a
Hull link, 220 px, with one main value: finder name in Name, Orange; one line in Label, Orange
Mid; where a live answer is known, its figure is the main value in Value L, Orange Lit, with
its context in Label, Orange Mid. A tile with no answer has no Lit text. Hover: Row Hover.
*Why:* a visible grid of what can be found, like the station services board, so search and
voice are shortcuts, not the only way in.

### Side panel (kept)

320 px after a 1 px orange line: title in white, facts in Orange with Label heads, the key
value in white, prose in Prose Grey, actions as words or icon buttons. *Why:* the game's
right-hand detail panel (white item name, orange facts, white key price) and the Controls
"About this setting" panel.

### States

| State | Treatment | Why |
|---|---|---|
| Selected row, tile, option, tab | solid orange, On Orange text; a row or tile only while a side panel or keyboard focus shows the selection | the game's selection |
| Active page tab | Orange Lit + 2 px orange underline | one step below the top tab |
| Current section | heading white, its rule orange; only the section holding the item a side panel describes | the trader screen, where the section the cursor is in is white |
| Hover | Row Hover on read-only things; Ember Cell Hover on cells; icons Orange Lit | a response without a new colour |
| Focus | 1 px orange outline, 2 px offset. On a filled element (selected row, tile or option, active top tab) a 1 px On Orange outline, inset 2 px | keyboard users need to see where they are; an orange outline vanishes against an orange fill |
| Unavailable, none, zero, disabled | Orange Dim; no surface lift | the trader's empty tiles |
| Stale report | name stays Orange; main value and facts step down to Orange Mid; the age in Orange ("212 d") | a step down says "do not trust this" and the age, now the brightest fact, says why; the facts stay readable, and red stays for failures |
| Warning (act on it) | orange "!" + plain words, at the place it applies | the game's orange warnings; urgent but not failed |
| Hard error (it failed) | red "!" + plain words + a recovery action ("! Could not read the route. TRY AGAIN"); a whole-page error is a white title, a prose line and the action. A page that re-reads on its own every few seconds offers no action: the next read is the recovery. Never a filled block | red is the game's malfunction colour |
| Empty | one prose line saying what is empty and what to relax, plus one action ("No buyers within 100 ly. WIDEN TO 250 LY") | an empty list with no way forward is a dead end |
| Loading | one Orange Mid line naming what is fetched ("Asking Spansh…"); a refresh keeps old rows Dim until replaced. No spinners or skeletons | motion pulls the eye; dim old rows keep the layout still |
| Done (a copy, say) | the control's own icon swaps for 1.5 s (copy to the `copied` check), then back; instant both ways | the one brief feedback allowed (The Brief Feedback Rule): a copy leaves no other trace |
| Listening (press to bind) | binding cell filled orange, On Orange caps | the game's listening cell |
| Conflict, ignored binding | orange "!" + "Also: <action>"; ignored text struck through, Dim | a warning, not a failure |

A report is stale when it is older than the finder's age filter (30 days by default). With the
filter at "Any age", stale rows are shown stepped down with their age in Orange; otherwise
they are hidden and the summary line names the limit ("reports up to 30 days old"). Sources
apply the age filter in the query, so the window cannot count what they left out; it counts
only rows that came back past the limit. Staleness is judged as of the answer's fetch time
(when the source applied the filter); the age shown is measured from now.

## Token map

Each name in this document and the `app/src/design/tokens.css` variable that holds it. Use
the variable; never repeat the value. Some variables keep names from before this document
(`--accent`, `--bg`, `--cell`) because the pages already use them. `--row-hover` was a grey
(#16171a) before revision 3; it is now DESIGN.md's warm Row Hover for every page. `--list-w`
(1200 px) is the result-list cap that revision 4 drops: it is not a DESIGN.md name, and it stays
in tokens.css only until the last page using it moves onto a layout.

| DESIGN.md | tokens.css | Value |
|---|---|---|
| void | `--bg` | #0c0d0f |
| hull | `--surface` | #1a1310 |
| hull-raised | `--surface-hi` | #241811 |
| row-hover | `--row-hover` | #21170f |
| seam | `--line` | #3a2716 |
| ember-cell | `--cell` | #3d220f |
| ember-cell-hover | `--cell-hover` | #482810 |
| ember-empty | `--cell-empty` | #1c1610 |
| orange | `--accent` | #ff7100 |
| orange-lit | `--orange-lit` | #ffa04a |
| orange-mid | `--orange-mid` | #d97a34 |
| orange-dim | `--orange-dim` | #8a5834 |
| placeholder | `--placeholder` | #ff7100bf (Orange at 75 %) |
| on-orange | `--on-accent` | #0c0d0f |
| white | `--text-hi` | #ffffff |
| options-grey | `--options-grey` | #e0e0e0 |
| prose-grey | `--prose` | #c4c4c4 |
| alarm-red | `--alarm` | #f0412e |
| hair | `--gap` | 2 px |
| xs, sm, md, lg | `--sp-1`, `--sp-2`, `--sp-3`, `--sp-4` | 4, 8, 12, 16 px |
| xl, 2xl, 3xl | `--sp-6`, `--sp-8`, `--sp-12` | 24, 32, 48 px |
| gutter | `--gutter` | 40 px |
| Prose width | `--measure` | 68ch |
| Form layout width | `--form-w` | 900 px |
| Side panel (List with detail) | `--panel` | 320 px |
| Control row, result row | `--row`, `--row-result` | 34, 44 px |
| Route strip, status bar, material tile | `--strip`, `--status-bar`, `--tile` | 36, 32, 76 px |
| Meta, Label/Section/Action, Body/Name/Nav/Figures | `--fs-xs`, `--fs-s`, `--fs` | 12, 13, 15 px |
| Value L and wordmark, Title, Value XL | `--fs-l`, `--fs-xl`, `--fs-xxl` | 18, 24, 28 px |
| Caps tracking | `--caps` | 0.06em |
| Each Hierarchy role (Typography) | `.type-wordmark`, `.type-title`, `.type-value-xl`, `.type-value-l`, `.type-nav`, `.type-name`, `.type-body`, `.type-figures`, `.type-label`, `.type-meta`, `.type-action`; Section is `.section` | a class setting the role's size, weight, line height, case and colour; a page names the role and sets no font size. A unit after its number is `.type-unit` |
| The root zoom (The Height Scale Rule) | `--zoom` | set by `lib/scale.svelte.ts`; 1 to 1.8, times the text size |
| The window's height, 1 % of its width, under the zoom | `--win-h`, `--win-w` | `100vh / --zoom`, `1vw / --zoom` |
| The frame's side margins | `--side` | 16 to 40 px: 3 % of the window's width |
| The face | `--face`, `--face-stretch` | Saira Variable at 87.5 % |

## Deviations from the game

Each place we depart from Elite's UI, what we do instead, and why.

| The game | astromech | Why |
|---|---|---|
| All text in caps | Caps only for the wordmark, top tabs, section headings and action words; mixed case elsewhere | Caps everywhere signals nothing and reads slower; system names carry meaning in their case. Decided for row labels on 11 Oct (V1-DESIGN §9) |
| Red, green and blue for status (green supply bars, green/red price arrows, blue mission goods) | Only one extra colour: red, for hard errors | One semantic colour is enough for a lookup window; green and blue would bring a second meaning system to learn |
| Red for warnings and damage alike | Warnings and stale data stay orange; red only when something failed | Keeps red rare enough to notice; an orange "!" already reads as "look here" in the game |
| The item under the gamepad cursor is filled, so the default action starts filled | The primary button is always filled; other buttons are text actions | A mouse window has no cursor highlight; the primary needs the fill to be found |
| Empty binding slots look like filled ones | Empty slots are the dimmer Ember Empty with the word "Set" in Orange Mid | Finding unbound actions is a main job of the Controls pages; the word is what the commander clicks, so it stays readable |
| Blurred cockpit behind every panel | Flat neutral Void | Calmer and more legible; no Frontier art behind the window |
| Glow, scan lines and animated panel transitions | Flat, no glow, no motion; the one exception is a control's brief confirmation (copy to a check for 1.5 s) | A desktop window beside the game should not pull the eye; a copy leaves no other trace on screen |
| Names on station screens in orange, like every other fact | White or Options Grey for entity names where the lift ranks them (The Lifted Name Rule) | With one orange at one size nothing ranked (audit, 10 Nov); the game already uses white for its few key names |
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
- **Do** use white only for titles, the current section, the selected item's key value and
  names lifted under The Lifted Name Rule.
- **Do** build every page outside Controls on Board, List with detail, Workbench or Form, and
  keep each action next to what it acts on.
- **Do** show a finder's filters as their summary line with CHANGE, and open the controls only
  on demand.
- **Do** name star, body and station types with their icons, the type as the accessible name.
- **Do** use icons for repeated row actions, with a Tip and a label, and words for
  primary and one-off actions.
- **Do** set numbers in tabular figures, right-aligned, units in Orange Mid.
- **Do** keep the route strip on every page and a copy icon beside every system name.
- **Do** keep the patterns that work: the Settings row, the Controls side panel, ship
  cartoons, onboarding's step list and primary button, plain-word status and errors.

### Don't:

- **Don't** use grey where the game uses orange; neutral greys only for the window, Options
  labels, prose and lifted names.
- **Don't** set a page's own width or stretch a row across the window; the layout places it.
- **Don't** lift every name on a page, or the name in a row ranked by its value.
- **Don't** put read-only tiles, specs, chips or cards in ember cells.
- **Don't** fill more than one item in a group, or show more than one primary button.
- **Don't** use red for stale data, warnings, health issues, decoration or emphasis.
- **Don't** put text the commander needs in order to act in Orange Dim.
- **Don't** fill a row or tile by default; a selection shows only with its side panel or
  keyboard focus.
- **Don't** stack more than two tab rows.
- **Don't** put repeated words like ADD STOP on every row, or hide row actions until hover.
- **Don't** make the ranked value the smallest text in a row.
- **Don't** draw a warning or error as a solid colour block.
- **Don't** use a second typeface, a radius, a shadow, a glow, an animation or a page
  transition. A control's 1.5 s confirmation is the one timed change allowed.
- **Don't** use Frontier art, fonts, icons or screenshots, or any language but English.
