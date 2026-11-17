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
  label-grey: "#8c8c8c"
  grey-dim: "#6a6a6a"
  mark-off: "#4f4a45"
  red-low: "#ca6657"
  alarm-red: "#f0412e"
typography:
  wordmark:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.12em"
  title:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "1.5rem"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
  value-xl:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "1.75rem"
    fontWeight: 500
    lineHeight: 1
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  value-l:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  nav:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: "0.06em"
  section:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "0.06em"
  name:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "normal"
  body:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 400
    lineHeight: 1.45
    letterSpacing: "normal"
  figures:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  cell:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.9375rem"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "normal"
  label:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
  meta:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: "normal"
    fontFeature: "\"tnum\" 1"
  action:
    fontFamily: "Saira Variable, Bahnschrift, Segoe UI, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 600
    lineHeight: 1
    letterSpacing: "0.06em"
rounded:
  none: "0px"
spacing:
  hair: "0.125rem"
  xs: "0.25rem"
  sm: "0.5rem"
  md: "0.75rem"
  lg: "1rem"
  xl: "1.5rem"
  2xl: "2rem"
  gutter: "2.5rem"
  3xl: "3rem"
components:
  button-primary:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.action}"
    rounded: "{rounded.none}"
    padding: "0 1.25rem"
    height: "2.125rem"
  button-primary-disabled:
    backgroundColor: "{colors.ember-empty}"
    textColor: "{colors.orange-dim}"
    typography: "{typography.action}"
    rounded: "{rounded.none}"
    padding: "0 1.25rem"
    height: "2.125rem"
  button-secondary:
    textColor: "{colors.orange}"
    typography: "{typography.action}"
    padding: "0 0.5rem"
    height: "2.125rem"
  button-secondary-disabled:
    textColor: "{colors.orange-dim}"
    typography: "{typography.action}"
    padding: "0 0.5rem"
    height: "2.125rem"
  icon-button:
    textColor: "{colors.label-grey}"
    size: "1.75rem"
  icon-button-hover:
    textColor: "{colors.orange-lit}"
    size: "1.75rem"
  input:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange}"
    typography: "{typography.cell}"
    rounded: "{rounded.none}"
    padding: "0 0.75rem"
    height: "2.125rem"
  input-hover:
    backgroundColor: "{colors.ember-cell-hover}"
  dropdown:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange}"
    typography: "{typography.cell}"
    rounded: "{rounded.none}"
    padding: "0 2rem 0 0.75rem"
    height: "2.125rem"
  choice:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange}"
    typography: "{typography.label}"
    padding: "0 0.75rem"
    height: "2.125rem"
  choice-on:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.orange-lit}"
    typography: "{typography.label}"
    padding: "0 0.75rem"
    height: "2.125rem"
  binding-cell:
    backgroundColor: "{colors.ember-cell}"
    textColor: "{colors.options-grey}"
    typography: "{typography.body}"
    padding: "0 0.75rem"
    height: "2.125rem"
  binding-cell-empty:
    backgroundColor: "{colors.ember-empty}"
    textColor: "{colors.orange-mid}"
    typography: "{typography.body}"
    padding: "0 0.75rem"
    height: "2.125rem"
  binding-cell-listening:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.action}"
    padding: "0 0.75rem"
    height: "2.125rem"
  tab-top:
    textColor: "{colors.label-grey}"
    typography: "{typography.nav}"
    padding: "0 0.75rem"
    height: "2.125rem"
  tab-top-active:
    backgroundColor: "{colors.orange}"
    textColor: "{colors.on-orange}"
    typography: "{typography.nav}"
    padding: "0 0.75rem"
    height: "2.125rem"
  tab-page:
    textColor: "{colors.label-grey}"
    typography: "{typography.name}"
    padding: "0.625rem 0"
  tab-page-active:
    textColor: "{colors.options-grey}"
    typography: "{typography.name}"
    padding: "0.625rem 0"
  badge:
    textColor: "{colors.label-grey}"
    typography: "{typography.label}"
    rounded: "{rounded.none}"
    padding: "0 0.375rem"
    height: "1.25rem"
  tile:
    textColor: "{colors.prose-grey}"
    rounded: "{rounded.none}"
    padding: "0.625rem 0.75rem"
  tile-unavailable:
    backgroundColor: "{colors.void}"
    textColor: "{colors.grey-dim}"
    padding: "0.625rem 0.75rem"
  tile-selected:
    backgroundColor: "{colors.hull-raised}"
    textColor: "{colors.white}"
    padding: "0.625rem 0.75rem"
  result-row:
    textColor: "{colors.prose-grey}"
    typography: "{typography.figures}"
    padding: "0 0.75rem"
    height: "3.75rem"
  result-row-hover:
    backgroundColor: "{colors.row-hover}"
  result-row-selected:
    backgroundColor: "{colors.hull-raised}"
    textColor: "{colors.white}"
  route-strip:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.prose-grey}"
    typography: "{typography.name}"
    padding: "0 2.5rem"
    height: "2.25rem"
  route-sidebar:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.prose-grey}"
    typography: "{typography.name}"
    padding: "0 1.5rem"
    width: "20rem"
  route-rail:
    textColor: "{colors.label-grey}"
    width: "3rem"
  status-bar:
    backgroundColor: "{colors.hull}"
    textColor: "{colors.prose-grey}"
    typography: "{typography.label}"
    height: "2rem"
  popover:
    backgroundColor: "{colors.hull-raised}"
    textColor: "{colors.orange}"
    rounded: "{rounded.none}"
    padding: "0.25rem 0"
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
docs/design/glow-up-*.png, brought to main by astromech-n41x): page layouts in place of the width
caps, lifted entity names, brief state feedback, the star, body and station icons, collapsed
filters and the NOW tab. The rest of that list lands with its own
item, which writes it here: the height scale and the new orange values (astromech-k6pl), Tip
(astromech-sixj) and the status bar (astromech-q79q).
Revision 5, 17 Nov 2026 (astromech-n41x): what Sumeet approved at the increment 1 stop
(mission astromech-xajc, notes of 17 Nov), each first seen as a mockup on `spike/nav-rework`
at dea5f0bb, with the captures in docs/design/nav-spike/: orange means act and data is white
and grey, with the list of colours that carry a meaning; selection as a raised row; result
rows in two-line groups; Elite's own symbols; the route sidebar in the frame; the new top
tabs; text size in rem in place of the height zoom; input values Orange at 500. The app
catches up item by item (text size landed with astromech-b2cz; colours, selection and tiles
off Hull with astromech-3kf5), and until each lands the app differs from this document there: the lint
for raw colours and sizes (astromech-4wi1), the route sidebar and PLACES (astromech-uc7i),
the tabs (astromech-5a01), the grouped result rows and demand colours (astromech-3ca6), the
hexagon component (astromech-7did). -->

## Overview

**Creative North Star: "The Station Board"**

Everyone who opens this window plays Elite Dangerous, so they already read its design
language fluently. astromech borrows it on purpose: the orange of the station screens, the
brown cells of its Options screen, the white screen titles, the game's own symbols (the
engineer hexagon, the market's demand bars), the dim marks of things you cannot have. A
commander should feel they opened another panel of their ship, not a web app. Where we
depart from the game, we do it for a stated reason; those departures are listed in
[Deviations from the game](#deviations-from-the-game).

The reference is the Encoded Material Trader. Each tile has one big value (the count); the
name is smaller and the trade ratio is a corner mark. State is shown by brightness: a dim
tile is unavailable. Layout carries meaning: rows are categories, columns are grades, so
nothing needs a label. White appears only in the screen title and the heading of the section
you are in.

We follow that screen's thinking, not its density or its colour. It is too sparse for a
lookup window, so we keep its rules and fit more by removing levels, never by shrinking
text. It is also all orange, which in a window full of data means nothing ranks: "if
everything draws the eye, then nothing can" (Sumeet, 16 Nov 2026). So data is white and grey,
and orange is spent on what matters. The test for any screen: can a commander find the main
value, the thing they can change, and the one thing that needs them, without reading?

**Key Characteristics:**

- Read-only data is white or grey, in three steps. Orange is for the value a list is ranked
  by, the one main value, and what the commander can act on or change.
- Colour beyond that only where it carries a stated meaning, from
  [the list](#colours-that-carry-a-meaning).
- One main value per element, in the largest size.
- A selected row or tile is raised (Hull Raised) with lit text. The filled orange button is
  the one fill on a page; the active top tab is the frame's.
- Hull is the frame's surface (the route sidebar, the strip, the status bar). Ember cells are
  what you can change; page tiles and blocks are outlines or Hull Raised.
- Red for hard errors; a duller red for a fact that rules a result out ("won't fit", "won't
  sell").
- Elite's own symbols where players know one: the engineer hexagon, star and station icons,
  the market's demand bars.
- One face (variable Saira), tabular figures, square corners, no shadow, no glow, and no
  motion beyond a control's brief confirmation that it worked.
- Text size is the commander's choice; a bigger window shows more, never bigger text.

### Principles

1. **Speak the game's language.** Use the game's own visual conventions unless there is a
   reason not to, and write the reason down. *Why:* every user already knows them; a familiar
   signal costs nothing to learn, a new one costs attention between jumps.
2. **One main value.** Name each element's one value (count, price, hotspots, next system)
   and give it the largest size. Everything else steps down at least one level in size or
   brightness. Its colour follows [Orange Means Act](#named-rules): Orange where it ranks a
   list or is the one main value of the page or panel, white or grey otherwise. *Why:* the
   trader screen is readable at a glance because the number is the only big thing on the tile.
3. **Brightness is state.** Within the greys and within the oranges, a brighter step ranks
   higher and a dim step is unavailable; a raised surface with lit text carries selection.
   *Why:* this is how the game does it, and it reads faster than words.
4. **Cells are controls.** An ember cell invites a click; read-only things sit on the
   window, in an outline or on Hull Raised. *Why:* the 24 Oct audit found brown cells on
   read-only tiles, which made real controls look no different from facts.
5. **Layout carries meaning.** Grids encode two axes; lists line up in columns. *Why:* the
   eye runs down a column faster than it reads wrapped facts.
6. **Hierarchy, not density.** Fit more by removing levels (a tab row, a caps label, a box),
   never by going below 12 px. *Why:* density without rank is what made the old result rows
   unreadable.
7. **Honest and plain.** Status and errors are plain words; an orange "!" when the commander
   should act, red when something failed. *Why:* the copy rules (docs/V1-DESIGN.md §6.5)
   already ask for plain words; colour only says how urgent they are.

### Fixed rules (from PRODUCT.md, binding)

- **The game's orange, neutral greys and red.** No other hue: no green, blue or theme, no
  colour customisation and no following the in-game HUD colour. Red has two steps, alarm red
  for failures and a duller red for a fact that rules a result out; every other colour use is
  on [the list](#colours-that-carry-a-meaning). *Why:* PRODUCT.md; the window should look
  like the stock game every commander knows, and greys are what the game's own Options screen
  uses for what is not orange.
- **No Frontier assets.** No Frontier art, icons, fonts, audio, screenshots or game files.
  The face is OFL Saira; icons are drawn for astromech; ship hulls are CMDR Qohen Leth's
  cartoons and engineer marks CMDR Robert Phillips', from EDAssets, credited in NOTICE.md.
  Game screenshots used to study the UI are never committed. Using a symbol players know
  ([The Game's Symbols Rule](#named-rules)) means its shape, drawn for astromech or from a
  credited EDAssets file, never Frontier's own.
- **English only**, in the copy style of docs/V1-DESIGN.md §6.5.
- **No VR or in-flight UI** in v1; the window is for between jumps.

### Reference captures

The approved mockups, at 2560 x 1440 unless named. They show the direction; where a capture
and a rule here differ, the rule wins, and the differences are said where the capture is cited.

- Revision 5, from `spike/nav-rework` (17 Nov 2026), in `docs/design/nav-spike/`: the frame,
  [hier-B-sell-2560.png](docs/design/nav-spike/hier-B-sell-2560.png),
  [hier-B-collapsed-2560.png](docs/design/nav-spike/hier-B-collapsed-2560.png),
  [hier-B-engineers-2560.png](docs/design/nav-spike/hier-B-engineers-2560.png), the route over
  the page at 1100 x 700 [hier-A-over-page-1100.png](docs/design/nav-spike/hier-A-over-page-1100.png),
  no route [4-no-route-2560.png](docs/design/nav-spike/4-no-route-2560.png), the stacked header
  [header-stacked-1590.png](docs/design/nav-spike/header-stacked-1590.png); colours and result
  rows, [tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png),
  [tone5-sell-1100.png](docs/design/nav-spike/tone5-sell-1100.png),
  [tone5-compare-sell.png](docs/design/nav-spike/tone5-compare-sell.png); Engineers and its
  search, [tone5-engineers-rest-2560.png](docs/design/nav-spike/tone5-engineers-rest-2560.png),
  [tone5-engineers-laser-2560.png](docs/design/nav-spike/tone5-engineers-laser-2560.png); Suits,
  [tone5-suits-2560.png](docs/design/nav-spike/tone5-suits-2560.png); the tabs,
  [5-ships-2560.png](docs/design/nav-spike/5-ships-2560.png),
  [7-materials-on-foot-2560.png](docs/design/nav-spike/7-materials-on-foot-2560.png). The `hier-*`,
  `4-*`, `5-*`, `7-*` and `header-*` captures predate the calmer colours: read them for layout,
  the `tone5-*` ones for colour. The engineer search's main answer in
  `tone5-engineers-laser-2560.png` names an engineer the commander's rank cannot yet use at that
  grade; astromech-7did states the rule.
- Revision 4's screen mockups (10 Nov 2026), in `docs/design/`: Now
  [glow-up-now.png](docs/design/glow-up-now.png) and at 1100 x 700
  [glow-up-now-1100.png](docs/design/glow-up-now-1100.png), the system tree
  [glow-up-system.png](docs/design/glow-up-system.png), the ship loadout
  [glow-up-ship.png](docs/design/glow-up-ship.png), the engineer tree
  [glow-up-engineers.png](docs/design/glow-up-engineers.png), Powerplay
  [glow-up-powerplay.png](docs/design/glow-up-powerplay.png), Materials
  [glow-up-materials.png](docs/design/glow-up-materials.png), the Sell finder
  [glow-up-sell.png](docs/design/glow-up-sell.png) and the icons
  [glow-up-icons.png](docs/design/glow-up-icons.png). They predate revision 5: their layouts
  stand, their colours, fills and one-line rows give way to it.

## Colors

Neutral greys for data, the game's orange in four brightnesses for what to act on, and red in
two steps, over a neutral near-black. Orange values were sampled from game screenshots (the
material trader, the commodity market, the Options → Controls screen and the right-hand modules
panel) and lifted slightly where anti-aliased samples read darker than the game draws them.
Revision 4 (astromech-k6pl, 12 Nov 2026) made Orange and Orange Mid stronger: the sampled
`#fa6e00` and `#c26c32` looked washed out next to the game, which draws its orange as light.
Revision 5 (17 Nov 2026) moved read-only data from orange to grey, from Sumeet's choice of the
`tone5` mockups ([hier-B-sell-2560.png](docs/design/nav-spike/hier-B-sell-2560.png) before,
[tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png) after;
[tone5-compare-sell.png](docs/design/nav-spike/tone5-compare-sell.png) sets the chosen version
beside concept 4).

Contrast is WCAG's, given on Void (the page), Hull (the frame) and Hull Raised (a selected
row), in that order. Text the commander reads is 4.5:1 or better on every surface it sits on;
the two dim steps and Mark Off are state, not text to read, and say so.

### Greys: read-only data

Data is grey in three readable steps, with White above them for titles.

- **White** (`white`, 19.4 / 18.3 / 17.3): page and panel titles, the heading of the current
  section, the selected row's name, the selected item's key name in a side panel, the
  wordmark, and the few names lifted under [The Lifted Name Rule](#named-rules). The **current
  section** is the one holding the item a side panel describes (the selected row, tile or
  binding); a page with no such selection has none. Focus and scroll position do not make a
  section current. *Why:* the game uses white exactly there (screen titles, the trader's
  current section, the market's "91 CR").
- **Options Grey** (`options-grey`, 14.7 / 13.9 / 13.1), step 1: names (station, system,
  material, module, engineer, ship), the facts that lead a row or block, the active page tab,
  and the row labels and binding text of Controls and Settings. *Why:* the game's Options
  screen draws its labels and bound keys in this grey-white.
- **Prose Grey** (`prose-grey`, 11.1 / 10.5 / 9.9), step 2: ordinary data (the figures in a
  row, facts in a fact list, counts in a tile) and paragraphs. *Why:* the game's "About this
  setting" text is grey; long passages in orange on black tire the eye.
- **Label Grey** (`label-grey`, 5.8 / 5.5 / 5.1), step 3: labels, units, column heads,
  secondary facts (the system under a station's name, the light years under the jumps),
  inactive tabs, icon buttons at rest, and the status bar's names for its stats. Use labels
  sparingly: a value that says what it is (a system name, "7 jumps", "27.4 / 32 t") needs no
  label in front of it. Never on an ember cell (4.4:1 there). *Why:* one step down for what
  supports the data, far enough above 4.5:1 to read from the seat beside the game.
- **Grey Dim** (`grey-dim`, 3.6 / 3.4 / 3.2): read-only things that are absent or over: a
  past route stop, an engineer not yet introduced, a tile with none. It is state, not
  content, and knowingly below WCAG AA, like Orange Dim; it never carries text the commander
  needs in order to act.
- **Mark Off** (`mark-off`, 2.2 / 2.1 / 2.0): the unlit part of a mark, such as an unreached
  hexagon, an unlit demand bar or a past stop's star. Never text. The lit part beside it
  carries the count, so the mark reads by how many are lit.

### Orange: what to act on

[Orange Means Act](#named-rules) says where orange goes; these are its steps.

- **Orange Lit** (`orange-lit`, 9.6 / 9.1 / 8.6): the one main value of the page or panel
  (the side panel's price, the distance to the engineer to go to), the ranked value on the
  selected row, the next stop in the route. *Why:* the game draws its most important figures
  in a lighter, yellower orange.
- **Orange** (`orange`, 7.0 / 6.7 / 6.3): the value a list is ranked by (one column), and
  everything the commander can act on or change: values in cells, action words, links, the
  primary button's fill, the active top tab's fill, a warning's "!", a plan (a planned grade,
  a "need 12" mark), a stale report's age. *Why:* it is the colour the game draws what you
  can select and use in.
- **Orange Mid** (`orange-mid`, 6.3 / 5.9 / 5.6): the ranked value of a stale row, an icon
  button on a hovered or selected row, a link's underline, the Tip's border, and the list's
  middle meanings (tight demand, a scoopable star, low security, an engineer you can work
  toward, a planned grade not yet reached). Never on an ember cell (4.7:1 there): text in
  cells is at least Orange, the placeholder excepted. The darker Ember Empty is the exception
  (5.8:1): an empty binding slot's word is Orange Mid.
- **Orange Dim** (`orange-dim`, 3.3 / 3.1 / 2.9): a control that is disabled or a choice
  that is unavailable. It is state, not content, and knowingly below WCAG AA: it says "not
  this one", and it never carries text the commander needs in order to act. Absent read-only
  data is Grey Dim. *Why:* the trader's empty tiles: still legible, plainly absent.
- **On Orange** (`on-orange`, 7.0:1 on Orange): text and icons on an orange fill (the primary
  button, the active top tab, a binding cell while it listens), and the focus outline on one.
- **Placeholder** (`placeholder`, Orange at 75 % opacity, 3.6:1 on an ember cell): the hint
  text in an empty input, and nothing else. It disappears as soon as there is a value, and the
  input always has a label of its own (visible, or an accessible name for the header's Find
  or plot). *Why:* one step below the value it stands in for, without a second hue in a cell.

### Red: it failed, or it rules this out

- **Alarm Red** (`alarm-red`, 5.1 / 4.8 / 4.53): only the "!" and the words of a hard error
  (something failed: a lookup, a file read, a write). *Why:* the game uses red for danger and
  malfunction (a failing module turns red), so red already means "this is not right".
- **Red Low** (`red-low`, `#ca6657`, 5.1 / 4.9 / 4.57): a fact that rules a result out for
  this commander: demand short of what they carry ("won't sell"), a pad too small for their
  ship ("won't fit"), an engineer who has barred them, an anarchy stop. Always with a second
  cue that is not colour: one demand bar, a struck-through pad letter, the reason in words.
  *Why:* the game colours the same facts (its market's red demand, a refused docking), and
  the commander should see a row they cannot use before they fly to it. The spike drew it
  `#c86455`, which is 4.46:1 on Hull Raised; it is lifted to `#ca6657` so a short or unfit
  selected row still reads.

[The Narrow Red Rule](#named-rules) says how the two differ.

### Surfaces: frame, cells and the page

- **Void** (`void`): the window background and the page. *Why:* the game's screens sit on a
  blurred cockpit that reads as neutral near-black; a neutral ground makes the orange look
  like light, and a brown ground would make the brown cells disappear.
- **Hull** (`hull`): the frame's surface, and only the frame's: the route sidebar, the route
  strip and the status bar. Page tiles and blocks are not Hull: they are a 1 px Seam outline
  on Void, or Hull Raised. *Why:* once the route became frame on Hull, a Hull tile on the page
  would read as part of the frame; the approved engineer tree draws its nodes as outlines on
  Void ([tone5-engineers-rest-2560.png](docs/design/nav-spike/tone5-engineers-rest-2560.png)).
- **Hull Raised** (`hull-raised`): a selected row or tile, the page's one leading block (the
  engineer search's main answer), and popovers, menus and the Tip, with a 1 px Seam border.
- **Row Hover** (`row-hover`): hover on a read-only row or tile.
- **Ember Cell** (`ember-cell`) and **Ember Cell Hover** (`ember-cell-hover`): every value
  the commander can change (input, dropdown, choice, binding slot, Settings value). *Why:*
  the game's binding and option cells are this brown; making it brighter than the hull keeps
  "can change" visible at a glance. The hover is only as bright as keeps Orange text at 4.5:1
  (4.8:1); the placeholder there is 3.4:1, a hint the input's own label backs up.
- **Ember Empty** (`ember-empty`): an unbound binding slot. *Why:* the game draws empty slots
  like filled ones; we make them visibly empty (see Deviations).
- **Seam** (`seam`): the only line colour besides orange: section rules, column-head
  hairlines, tile and block outlines, popover and badge outlines, and the edge of the page
  where it meets the frame. (The scrollbar thumb is Orange Dim at rest and Orange Mid under
  the pointer or while dragged: Seam is too faint to find on Void.) *Why:* the game's
  inactive section rules are this dim brown.

### Colours that carry a meaning

Beyond the greys, the orange steps and the two reds as above, colour is used only for these.
A colour not on this list needs adding to it, with its meaning and its second cue, before a
page uses it. *Why:* colour that means something is how the game's market and engineer screens
are read; colour that means nothing is what made every page loud.

| Thing | Colour | Meaning | Also shown by |
|---|---|---|---|
| Demand on Sell, against the tonnes of that commodity aboard that are the commander's to sell | Red Low, short: below that figure. Orange Mid, tight: below five times it. Prose Grey, plenty | will it take your cargo | the market's bars: one, two or three lit; a Tip on a short row |
| Supply on Buy, against the free cargo space | as demand | will it fill your hold | as demand |
| Demand on Sell with none of the commodity aboard, or the sellable tonnes unknown (Elite not running) | Prose Grey bars, no warning colour | the figure, with nothing to measure it against | the page says what it measures against ("200 t of Painite aboard"), and tells a known zero ("none aboard") from an unknown |
| Supply on Buy when free space is unknown (cargo or capacity not known) | Prose Grey bars, no warning colour | as above | as above; an empty hold with a known capacity is not this case: it is measured against its free space like any other ("256 t free") |
| Supply on Buy with no free space (a full hold) | Prose Grey bars, no warning colour | nothing more fits, so no seller is short | the page says the hold is full |
| A pad too small for the current ship | Red Low letter and outline | won't fit | struck through; its accessible name says why |
| A stale report (older than the finder's age filter) | its age in Orange; the ranked value steps to Orange Mid | do not trust this | the age in words ("34 d old") |
| A star in the route | scoopable Orange Mid; neutron star White; any other Label Grey; past stops Mark Off | what you do there: refuel, boost, nothing | the star type icon and its Tip |
| The next stop in the route | Orange Lit name; its note (Fuel) in Orange | where to go now, and what to do there | raised, and "Next stop" above it |
| Security of an inhabited stop | low Orange Mid; anarchy Red Low; others Prose Grey | how risky the stop is | the word ("Low security", "Anarchy") |
| An engineer's state | unlocked: name Options Grey, rank hexagons White. Known: name Orange Mid, the step to unlock in Prose Grey. Barred: name and reason Red Low. Not introduced: Grey Dim | can you use them, work toward them, or not | the hexagons, or the step or reason in words |
| Engineer rank and blueprint grade | reached rank White; a grade the commander can get Orange; a planned grade not reached Orange Mid; unreached Mark Off | how far you are, and what you can get | the count of lit hexagons, and the accessible name ("Rank 3 of 5") |
| A short amount (need more than have) | "!" and the have figure in Orange | collect this | the "!" |
| A material at its storage cap | the tile's track Orange Lit | trade or use some before you collect more | the full track |
| A binding conflict | "!" and "Also: <action>" in Orange; an ignored binding struck through, Orange Dim | a warning, not a failure | the words |
| The commander's position in the route | White pin and name, raised | you are here | the pin icon |

### Named Rules

**The Orange Means Act Rule.** Read-only data is white or grey. Orange is for three things:
what the commander can act on or change (cells, actions, links, the primary button, a warning's
"!", a plan), the value a list is ranked by (one column), and the one main value of the page or
panel (in Orange Lit). Anything else coloured is on [the list](#colours-that-carry-a-meaning).
The test: cover the orange, and what is left is reading; look only at the orange, and it says
what ranks, what to read first and what to press. *Why:* Sumeet, 16 Nov 2026, on increment 1:
almost every word was saturated orange at weight 500, so nothing ranked. It replaces revision
4's Intensity Rule (rank by orange brightness alone), which decision astromech-bi94's "orange
only where it means something" now applies to directly.

**The One Fill Per Page Rule.** A page has at most one solid orange fill: its primary button.
The frame has one more, the active top tab, and a binding cell is filled while it listens.
Nothing else is filled: a selected row or tile is Hull Raised with lit text, a chosen option
is an ember cell with Orange Lit text and a 2 px orange underline, a highlighted popover row is
an ember cell. A list or grid shows a selection only when something shows what is selected: a
side panel about it, or keyboard focus in the list. There is no default selected row. *Why:*
the game fills the selected row, but its rows are orange; with grey rows, an orange slab is
the loudest thing on the page and outshouts the main value (compare the filled row in
[hier-B-sell-2560.png](docs/design/nav-spike/hier-B-sell-2560.png) with the raised one in
[tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png)). A fill means "press
this", so it stays on the one thing to press.

**The Lifted Name Rule.** Entity names (a station, body, module, system, engineer or ship) are
white or grey, never orange; the value a row is ranked by is orange, so a name never competes
with it. Options Grey is the default. Lift a name to White where it leads: the selected row's
name, a side panel's title, the station a Now answer names, the engineer to go to. Leave the
rest Options Grey, and step a secondary name (the system under a station) to Label Grey. A
lifted name keeps its role's size and never outranks the element's main value. The exceptions
are on [the list](#colours-that-carry-a-meaning): an engineer you can work toward is Orange
Mid, one who barred you Red Low. *Why:* names are what the commander reads to recognise a row;
the game draws its few key names (screen titles, the selected item) in white, so a lift reads
as "this is the thing" without a new hue.

**The Narrow Red Rule.** Alarm Red is for a hard error only: the "!" and the words of
something that failed. Never for stale data, warnings, health issues, decoration, numbers
going down, or emphasis. Red Low is for a fact that rules a result out for this commander,
from [the list](#colours-that-carry-a-meaning), always with a second cue; it never carries the
"!" and never says something failed. The two differ by use and by look, not by brightness
(their contrast is nearly the same): Alarm Red is saturated and appears with a "!" and a
recovery action; Red Low is duller and marks a fact inside a result. *Why:* red earns its
meaning by being rare; once it decorates, the next real failure is missed.

## Typography

**Face:** Saira (OFL), variable, at 87.5 % width (`font-stretch: 87.5%`, the semi-condensed
cut), from `@fontsource-variable/saira`, with Bahnschrift and Segoe UI as fallbacks. One face
for everything, including paths and logs. *Why:* condensed and square-shouldered like the
game's lettering without copying it; the variable file has tabular figures (the static
Semi Condensed files do not) and replaced three files (two Saira cuts and Share Tech Mono).
`tokens.css` sets the face, its width and tabular figures once on the page, so every element
has them; the `font` shorthand resets the width, so set `font-stretch` after it.

**Weight.** Each role below states its weight. Orange at 15 px and over is never below 500:
orange at 400 looked thin and washed out at the game's distance (revision 4). Grey data reads
well at 400, so Body, Figures, Label and Meta are 400, and the grey that leads (names) is 500.
Orange in Label size (an unselected choice, "Ctrl+K", a stale report's age) keeps Label's 400,
as revision 4 had it. A value in a
cell is the Cell role, Orange at 500, because it is what the commander can change; this
settles the conflict astromech-z5oh reported, where the component table gave inputs Body (400)
and this section said orange at 400 looked thin.

The sizes below are CSS px at 100 percent Text size, a 16 px root. `tokens.css` holds them in
rem (the px figure divided by 16), so the commander's Text size scales them all ([The Text Size
Rule](#named-rules-1)).

### Hierarchy

| Role | Size / weight / line | Case | Colour | Use |
|---|---|---|---|---|
| Wordmark | 18 / 600 / 1, tracking 0.12em | caps | white | "ASTROMECH" only |
| Title | 24 / 600 / 1.2 | mixed | white | page, finder and side-panel titles |
| Value XL | 28 / 500 / 1, tabular | figures | orange lit for the one main value of the page or panel; otherwise options grey | the one value of a tile or panel |
| Value L | 18 / 600 / 1.2, tabular | figures | orange for the value a list is ranked by (orange lit on the selected row); otherwise options grey | the one value of a row or strip |
| Nav | 15 / 400, tracking 0.06em | caps | label grey; active 500 on the orange fill | top tabs, and the sidebar's ROUTE heading |
| Section | 13 / 600, tracking 0.06em | caps | prose grey; current section white | section headings, with a Seam rule to the right |
| Name | 15 / 500 / 1.3 | mixed | options grey; white under The Lifted Name Rule | station, material, ship, engineer names; page tabs |
| Body | 15 / 400 / 1.45, max 68ch | sentence | prose grey | paragraphs |
| Figures | 15 / 400, tabular | figures | prose grey | numbers in columns and fact lists |
| Cell | 15 / 500 / 1.3 | as entered | orange | the value in an input, dropdown or Settings cell |
| Label | 13 / 400 / 1.3 | sentence | label grey | column heads, field labels, units, secondary facts, badges |
| Meta | 12 / 400, tabular | figures | label grey | corner marks in tiles |
| Action | 13 / 600, tracking 0.06em | caps | orange; on orange on a fill | buttons and text actions |

The Colour column gives each role's default. A component may override it where its own
spec says so (anything on a fill is On Orange; a stale row's ranked value steps down to Orange
Mid; a colour from [the list](#colours-that-carry-a-meaning)).

Units follow their number in Label size and Label Grey: **390,780** cr, **7** jumps.
Thousands use commas. *Why:* the unit supports the number; it should not compete with it.

### Named Rules

**The Caps Are Wayfinding Rule.** Uppercase with 0.06em tracking only for the wordmark, top
tabs, the route sidebar's heading, section headings and action words. Names, column heads,
labels, badges, status, errors and prose are mixed or sentence case. *Why:* when everything is
caps (the audit counted about 140 caps rules), caps signal nothing; mixed case also reads
faster, and procedural system names carry meaning in their case ("Synuefe GW-Z a54-3",
"HR-W d1-42").

**The Text Size Rule.** Text size is the commander's choice, set the standard way: one root
font size, and every size that should grow with the text in rem. A bigger window shows more;
it never changes the size of the text.

- **Text size** in Settings → General has six steps: 100, 115, 130, 145, 160 and 180 percent of
  a 16 px root. It sets the `html` font size and nothing else does. The core's config stores
  it as a percentage (`[window] text_size`).
- **The first run picks a step** from the display's height (the screen's, never the
  window's) and saves it once: below 1200 px, 100; below 1300, 115; below 1400, 130; below
  1600, 145; below 1800, 160; otherwise 180. After that only the commander changes it. A
  resize changes nothing. An install from before revision 5 keeps the size it had: its old
  Smaller, Normal or Larger becomes the nearest step (astromech-b2cz).
- **In rem:** font sizes, spacing, row and control heights, the side panel, the route
  sidebar, the Form layout's widths and every narrow-window breakpoint. **In px:** 1 px rules,
  borders and outlines, and icon strokes.
- **Narrow-window rules** are container queries in rem: a page's rules on the page column
  (`@container win (max-width: …rem)`), the header's on the window. A breakpoint in rem moves
  with Text size, so at a larger step the layout switches to its narrow form at a wider window,
  as it should: the content needs the same room in rem. A media query would not, in px or rem:
its rem is the browser's default size, not the `html` font size.

| Window | Step a first run picks | The window in rem |
|---|---|---|
| 1100 × 700, the smallest, on a 1080p display | 100 | 68.75 × 43.75 |
| 1920 × 1080, filling a 1080p display | 100 | 120 × 67.5 |
| 2560 × 1440, filling a 1440p display | 145 | 110.3 × 62.1 |
| 3840 × 2160, filling a 2160p display | 180 | 133.3 × 75 |

*Why:* Sumeet, 16 Nov 2026, on revision 4's zoom from the window's height: "as a web dev,
this sort of solution feels very wrong". A window grown to see more should show more, and the
size of text is a reading choice like any web page's. The first-run step keeps a 1440p display
at the size of the approved captures (145 percent there matches the old 1.44 zoom) without
leaving a 1080p display cramped. It replaces revision 4's Height Scale Rule, its root `zoom`,
`--zoom`, `--win-h` and `--win-w`, which astromech-b2cz removed.

**The Tabular Rule.** Every number uses tabular figures, and a column of quantities is
right-aligned. A tile's value sits where its tile spec puts it (the material tile's count top
left); the values in a fact list and dates in a column read as text and stay left-aligned.
*Why:* the ones digit sits under the ones digit, so a column can be compared by eye, as the
game's market columns can; a fact list or a date has no column of quantities to compare.

## Layout

The window is a fixed frame: the header across the full width; under it, the route sidebar on
the left (or the route strip across the top, collapsed or in a narrow window); the status bar
at the foot; and inside that L of frame, the page with at most one page tab row above it.

### Spacing scale

`hair` 2 px · `xs` 4 · `sm` 8 · `md` 12 · `lg` 16 · `xl` 24 · `2xl` 32 · `gutter` 40 · `3xl`
48, at 100 percent Text size; `tokens.css` holds them in rem. Rows inside a section are 2 px
apart (the game's cell gap), sections 32 px apart, a heading 24 px below the last section and
12 px above its content. *Why:* tight groups and wide separations let grouping be seen
without boxes.

### Frame

The approved frame is Variant B ([hier-B-sell-2560.png](docs/design/nav-spike/hier-B-sell-2560.png),
collapsed in [hier-B-collapsed-2560.png](docs/design/nav-spike/hier-B-collapsed-2560.png), with
the calmer colours in [tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png)).

| Part | Size | Notes |
|---|---|---|
| Header | 64 px high, or 64 + 44 px | wordmark, top tabs, Find or plot, Help, on Void. Its 1 px orange rule runs the full width of the window, over the sidebar ([The Full-Width Header Rule](#named-rules-2)). Below the width where one row fits, the top tabs sit on a 44 px second row under the wordmark. With the nine tabs astromech-5a01 shipped, one row fits from 95.5 rem of window width, measured at every Text size step: 1529 px at 100 percent, 1757 at 115, 1987 at 130, 2216 at 145, 2446 at 160 and 2751 at 180. So 1920 x 1080 and 2560 x 1440 at their starting steps show one row, and 1100 x 700 shows two. Finish setup, while setup is unfinished, adds its own width. astromech-coq6 measures again when NOW joins ([header-stacked-1590.png](docs/design/nav-spike/header-stacked-1590.png) shows ten tabs on the second row in a 1590 px window) |
| Route sidebar | 320 px wide, header rule to status bar | while a route is followed and the window is at or above the dock width; see [Route sidebar](#route-sidebar) |
| Route strip | 36 px high | across the page column when the sidebar is collapsed, or below the dock width; see [Route strip](#route-strip) |
| No-route rail | 48 px wide | with no route, at or above the dock width: two actions, Find or plot and Import a route |
| Page tabs | 44 px | one row, only when the area has more than one page |
| Page | rest | Void; scrolls; the frame does not |
| Status bar | 32 px | on every page, Hull: the ship, its fuel and cargo, and the core's state |

Side margins 16 to 40 px (3 % of the window's width); minimum window 1100 × 700 at 100
percent Text size. Every size here is CSS px at 100 percent and in rem in `tokens.css` ([The
Text Size Rule](#named-rules-1)), so the widths at which the frame changes (the header's
second row, the dock width) are widths in rem and come sooner at a larger step.

**Height budget.** At the smallest window, 1100 x 700 at 100 percent, the fixed parts leave the
page at least 480 px: header 108 (stacked), route strip 36 (below the dock width the strip,
or its No route state, is always there), page tabs 44 and status bar 32 take 220 px, which
leaves exactly 480 (measured 12 Nov 2026). A fixed part that would take more moves the status
bar onto the route strip's row at that size instead of shrinking the page. At a larger Text
size the page gets less, and scrolls.

**Top tabs.** NOW, SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS,
SETTINGS, in that order. SHIPS holds Ships, Stored modules and Suits; ENGINEERING holds
Materials and Engineers; PLACES (once GALAXY) is one page with no page tab row. On foot is not
a page: its loadouts, blueprints and shopping list go to Ships > Suits, its inventory to
Materials as an On foot kind, its engineers to Engineers beside the ship engineers. There is
no Route page: the route is frame. *Why:* Sumeet, 16 Nov 2026: Ships and Stored modules are what
the commander owns, not engineering, and On foot was a copy of the whole section cut by mode
([5-ships-2560.png](docs/design/nav-spike/5-ships-2560.png),
[7-materials-on-foot-2560.png](docs/design/nav-spike/7-materials-on-foot-2560.png), both drawn
before the calmer colours).

The window opens on the Now page: what the commander is doing at the moment (docked, in a
system, or Elite not running). *Why:* the window used to open on My controller and filed the
live views (Station, Session, Mining) under Logbook, so it had no home page (audit, 10 Nov
2026).

### Route sidebar

The route is part of the frame, on every page, and no longer a page of its own. *Why:*
Sumeet, 16 Nov 2026: the route does not deserve a full page; he chose a persistent sidebar,
and of two mockups, Variant B.

- **Docked** (a route is followed, the window at or above the dock width): a 320 px column of
  Hull, flush to the window's left edge, from the header's rule to the status bar. The status
  bar is Hull too and the sidebar runs into it with no rule between them, so the two are one L
  of frame; the page is the inset, edged by one Seam L (a rule on the sidebar's right and over
  the status bar). Its heading is ROUTE in the Nav role, Label Grey, with a collapse chevron at
  its right; there is no page Title. Under it, what the Route page held: the plotter title
  (from and the destination, editable in place; Plotter and Worked out as cells; Replot from
  here as an action; ? and the menu), then the timeline with the next stop raised and the
  commander's row marked. Star types are carried by their icons, coloured by what you do there
  ([the list](#colours-that-carry-a-meaning)).
- **Collapsed:** the route strip across the top of the page column, with a chevron that opens
  the sidebar again. The choice persists.
- **No route:** a 48 px rail of Void with a Seam rule on its right and two icon buttons: one
  takes focus to Find or plot, one opens the Import a route dialog. It reserves no more width.
  [4-no-route-2560.png](docs/design/nav-spike/4-no-route-2560.png) shows the rail, drawn with
  one action and the old colours; the rule is two.
- **Below the dock width** the route is the strip. Its chevron opens the same sidebar over the
  page: Hull, with a 1 px orange line on its open edge, the page behind it dimmed ([The Dimmed
  Page Rule](#named-rules-2)). It closes on Escape, on the chevron and on a click outside. The
  strip's No route state offers the same two actions as the rail. See
  [hier-A-over-page-1100.png](docs/design/nav-spike/hier-A-over-page-1100.png) (Variant A's
  surfaces, the old colours; the overlay is the same in B).
- **The dock width** is the narrowest window at which a List with detail page (the Sell finder
  with a row selected) keeps its rows clear of its side panel beside the sidebar. The spike
  measured about 1,700 px under the old zoom: the Sell row at its narrowest (about 930 px), the
  320 px panel, the sidebar's 320 px and two gutters. astromech-uc7i measured it at every Text
  size step: 1,756 px at 100 percent (109.75rem), so a 1920 x 1080 display at 100 percent and a
  2560 x 1440 one at its 145 percent both dock. It is `DOCK` in
  `app/src/lib/ui/layout/layout.ts`, and a browser test fails when the Sell row outgrows it.

### Page layouts

Every page outside Controls is built on one of four layouts. The layout owns the page's
widths, gutters and detail panel; a page picks one and sets no width of its own. A layout
places content, it never adds any: a sparse page stays sparse.

| Layout | What it holds | Use it when | For example |
|---|---|---|---|
| Board | blocks or tiles laid out across the width, each with its own main value; where a tile can be selected, the 320 px side panel after a 1 px Seam rule describes it | the page is answers or tiles read at a glance, laid out by their own shape (a grid of categories and grades, a row of answers) | Now, Powerplay, Places home; Materials, with its side panel |
| List with detail | a list (rows or a tree) and the 320 px side panel after a 1 px Seam rule, which describes the selected item | the commander picks one item from many and then wants its details | Places finders, System, Ships, Engineers |
| Workbench | the inputs on the left, the result on the right | the commander sets up a question and reads one answer that changes as the inputs do | Trade > Commodities |
| Form | Settings rows in one column: a 200 px label and a cell up to 700 px, 900 px in all | the page is values to set | Settings |

Pick by the commander's job on the page, not by how much there is to show. When the page
column is too narrow for a side panel (the 1100 x 700 minimum, or a larger Text size), the
layout moves the panel below the content. It never shrinks text to make it fit. The layouts
measure the page column, not the window, so the sidebar's width is already taken out.

Inside a layout: prose 68ch; tile grids 200–240 px columns with 8 px gaps; control row 34 px;
result row 60 px, two lines in groups ([Lookup result row](#lookup-result-row)); material tile
76 px. Rows visible: at 1100 x 700 at 100 percent, 5 whole rows show
([tone5-sell-1100.png](docs/design/nav-spike/tone5-sell-1100.png)); at 2560 x 1440 at 145
percent the capture has 8 rows and room below them, which works out at about 11
([tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png)), a figure computed from the
free space, not counted. Revision 4's one-line 44 px rows fitted about 18 on a 1000 px window,
and the 65 to 100 px rows before them 8. astromech-3ca6 measures the built row. *Why:* fewer rows that each read as an answer beat more rows that read as a spreadsheet;
the commander picks from the top few, and the list scrolls.

### Named Rules

**The Two Tab Rows Rule.** Top tabs plus one row of page tabs, never more. A third level
becomes a choice, a section or a finder of its own. *Why:* the audit found up to four
stacked tab rows; the game never stacks more than two either.

**The Aligned Columns Rule.** Repeated facts line up down a list, in fixed-width groups under a
header row that names each group's facts ("Price · demand", "Station · pad · arrival",
"Trip"). A group is two lines: the fact the commander reads first, over what qualifies it.
Each line of a group aligns down the list: figures right-aligned, text left. A fact sits in
the group it is read with (demand with price, a station's pad and arrival with the station);
a fact the commander does not choose a row by goes to the side panel. If a fact would wrap,
the group is wrong or the fact should go. *Why:* Sumeet, 17 Nov 2026, on a row with a column
per field: "table carcinisation again". Grouped, a row reads as a result; aligned, the list can
still be compared by eye.

**The Next To It Rule.** An action sits next to what it acts on. A row is as wide as its
columns, not as the window, and its actions come right after its last fact column; a side
panel's actions act on the item the panel describes; a form's action word ends its cell. The
test: at any window size, no action is more than one column from the thing it acts on.
Spare width stays empty or goes to the side panel; it never stretches a row. *Why:* a 1600 px
row put the old GO button 1400 px from the name it acted on, and the fixed width caps that
followed (60rem on eight pages) left half of a big window empty, with the page pinned left.

**The Full-Width Header Rule.** The header and its 1 px orange rule run the full width of the
window, over the route sidebar; the sidebar starts under the rule. *Why:* the first sidebar
mockup ended the header at the sidebar and read as a column of the page; a rule across the
whole window says the sidebar is frame, like the header itself.

**The Dimmed Page Rule.** Anything opened over the page dims the page behind it, so the page
reads at about Dim brightness. The route sidebar opened below the dock width uses a Void
scrim at 72 % opacity, a 1 px orange line on its edge toward the page, and closes on Escape
and on a click on the dimmed page. A dialog keeps its own scrim and closing rules
(`app/src/lib/ui/Dialog.svelte`, black at 70 %), which already dim the page. *Why:* an overlay at
full brightness over a full-brightness page reads as part of it; dimming says what has focus,
the way the game dims its cockpit behind a panel.

## Elevation & Depth

Flat and tonal: Void for the page, Hull for the frame, then Hull Raised for a selected row or
tile, the page's one leading block, popovers and the Tip. The one translucent layer is the
Void scrim behind an overlay ([The Dimmed Page Rule](#named-rules-2)). No shadows, blur or
glow; state changes are instant. *Why:* the game's panels are flat planes; its glow comes from the
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
popovers, badges, column heads, tiles and page blocks, section rules, the side-panel
separator and the page's edge against the frame; 1 px orange for focus, the header rule and
the open edge of an overlay; Orange Mid for the Tip. No coloured side borders on
cards. *Why:* the game is square-cornered and nearly line-free; the trader's only lines are the
section rules.

### Icons

One set of one-stroke icons drawn for astromech: 16 px on a 16 px grid, 1 px stroke,
`currentColor`, no fill, square ends. They live in `app/src/lib/ui/Glyph.svelte`: copy and
copied, **add stop** (a route with a plus), **set destination** (a flag; replaces the route),
**open external** (a box with an arrow out), **you are here** (a map pin, the commander's row in the route), **chevron** (dropdowns), **clear** (an ×, the end
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

A type icon is a fact, not a control: it sits before the name it describes, in Label Grey (a
star in the route takes its colour from [the list](#colours-that-carry-a-meaning)), at 16 px
in a row and 24 px beside a title, and its accessible name is the type
("Orbis starport", "Neutron star"). A type the set does not draw, such as a rare star class,
uses the nearest shape and keeps its own name.

**The Game's Symbols Rule.** Where Elite has a symbol players know, use it, not a generic
shape: the engineer hexagon for an engineer's rank and a blueprint's grade (one hexagon per
step, the reached ones lit), the star and station type icons, and the market's three demand
bars. Each is drawn for astromech or comes from a credited EDAssets file (the hexagon is
`app/public/engineers/access.svg`, CMDR Robert Phillips' mark), never from Frontier's files
(Fixed rules). A rank shown as "Rank 3" alone, or as pips or dots, is wrong where the hexagon
fits; the words stay as the accessible name. *Why:* Sumeet, 16 Nov 2026, asked for the hexagon
back on Engineers and Suits; a commander reads a row of lit hexagons as a rank without reading
([tone5-engineers-rest-2560.png](docs/design/nav-spike/tone5-engineers-rest-2560.png),
[tone5-suits-2560.png](docs/design/nav-spike/tone5-suits-2560.png)).

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
- Pad size is a letter (S, M, L) in a small Seam outline after the station's name, not an
  icon. *Why:* the letter is what the game shows and is already shorter than any icon.

Row action icons are visible at rest in Label Grey, turn Orange Mid on the hovered or
selected row, and Orange Lit under the pointer. *Why:* hiding them until hover makes them
undiscoverable and moves the row under the pointer; quiet at rest, they leave the ranked value
the loudest thing on the row, which was the real problem with the old GO cell. Copy stays visible beside every system name.

## Components

### Surfaces and cells

**A cell means editable.** Ember cells hold every changeable value; read-only things sit on
the window, in a Seam outline, or on Hull Raised. Material tiles, specs, engineer cards,
results, chips and summaries are never ember cells. Hull is the frame's, never a page's.
*Why:* in the game a brighter brown slot is something you set; the audit showed what happens
when facts wear the same brown.

A read-only card or tile that holds controls (a ship module card with its blueprint dropdowns)
is a Seam outline; only the controls inside it are cells. A read-only state on a card is shown
by brightness or by [the list](#colours-that-carry-a-meaning) (an unlocked engineer's name in
Options Grey, one you can work toward in Orange Mid, one not yet introduced in Grey Dim),
never by a cell, a fill or a coloured side border.

### Facts, stats and meters

- **Fact list:** read-only label and value pairs (ship specs, Diagnostics, a test's result, the
  side panel's facts). The label in Label, Label Grey (Options Grey on Controls and
  Settings-style pages), the value in Figures or Name, Prose Grey or Options Grey; on the
  window, or grouped in a Seam outline. On a Settings-style page a fact lines up with the cell
  text beside it but has no cell.
- **Multi-value stat:** one label naming the values in order ("Jump min / current / max"), then
  the values in Figures, separated by a Label Grey "|", with one unit after the last.
- **Meter:** a Mark Off track, 2 to 6 px high, filled from the left in Prose Grey; the material
  tile's track turns Orange Lit at its cap, and a shopping list's fill turns Orange Lit when
  complete. No label inside the track; the figure beside it says how much. *Why:* the trader
  tile's bar, kept thin so the number stays the main value.
- **Hexagons:** a rank or grade is a row of engineer hexagons, one per step, the reached ones
  lit, coloured by [the list](#colours-that-carry-a-meaning); 12 to 14 px in a row, its words
  ("Rank 3 of 5") as its accessible name. One shared component draws them (astromech-7did).
- **Demand bars:** three bars rising left to right, 14 × 12 px, the lit ones in the demand's
  colour and the rest Mark Off, before the figure they qualify.

### Settings row (kept)

Label at 200 px in Options Grey · an ember cell with the current value in the Cell role
(Orange at 500) · one action word at the cell's right end in Orange (CHANGE, RUN AGAIN,
PREVIEW). A choice row (voice colour Natural / Cockpit / Harsh; callouts On / Off) uses choice
cells, the current option marked as [Choice](#choice-segmented-and-filter-options) says. Group
description below in Label, Prose Grey. Section headings in the Section role with a Seam rule.
*Why:* this is the game's Options screen: grey labels, brown cells, orange values.

A status the commander changes with its one action is a changeable value, so it sits in a
cell with that action: "Setup: Done · RUN AGAIN", "Voice: Listening while Elite runs ·
PAUSE". A status with no action is a fact and sits outside a cell ("Speech models:
Installed"). *Why:* the cell still means "you can change this here"; the action word says
how.

### Input

Ember cell, 34 px, the value in the Cell role (Orange at 500), placeholder in Placeholder
(Orange at 75 %), no border or underline at rest, 1 px orange outline on focus. A page's own
search box also keeps the outline while it holds the search the page is answering, as in
[tone5-engineers-laser-2560.png](docs/design/nav-spike/tone5-engineers-laser-2560.png): it
says what the answer is for, which is not at rest. An error puts a red "!" and plain words directly under
it. *Why:* the game's text fields are brown boxes with orange text; the orange underline the
old header box had at rest looked like focus all the time.

### Dropdown

Ember cell like the input, value in the Cell role, an Orange chevron at the right. The open
list is a popover: Hull Raised, Seam border, 34 px rows in Orange, the current option in
Orange Lit, the highlighted row (pointer or keyboard) an ember cell. *Why:* the game's
dropdown cells are exactly this, with an orange ▼; the old dropdowns had no chevron and did not
look openable.

### Choice (segmented and filter options)

A row of ember cells, 2 px apart, Label size: unselected Orange; chosen Orange Lit at 600 with
a 2 px orange underline along the cell's foot. Single and multiple choice look the same; a
multiple choice may have several chosen. The chosen option is always the marked one; never
draw it inverted. *Why:* the game marks the chosen option, and a mark brighter than its
neighbours reads at once; revision 5 keeps solid fills for the one thing to press ([The One
Fill Per Page Rule](#named-rules)).

### Buttons

- **Primary:** solid orange, On Orange text, Action type, 34 px, 20 px padding. At most one
  per page (the frame's Find or plot results and an open dialog count as their own page), for
  the step forward (Next, Save preset, Plot route, Set destination). It is the page's one
  fill. Disabled: Ember Empty with Orange Dim text. *Why:* the game fills the item under the
  cursor, and its cursor starts on the main action; a mouse-driven window has no such cursor,
  so the primary is filled instead (see Deviations).
- **Secondary (text action):** an orange caps verb, no fill, 34 px hit area. Hover: a Row
  Hover background. Disabled: Orange Dim.
- **Icon button:** a 28 px square holding one icon, Label Grey at rest, Orange Lit on hover,
  a Tip and a label always. On a hovered or selected row, Orange Mid.
- **Links** to somewhere else (Help, a guide page, the crumb back to a finder's home): Orange
  text with a 1 px Orange Mid underline.

### Tabs

- **Top tabs:** Nav type, caps, Label Grey at 400; active is a solid orange box with On
  Orange text at 500. *Why:* the game's panel tabs are filled when active, and the active
  top tab is the frame's one fill.
- **Page tabs:** Name type, mixed case, Label Grey; active Options Grey with a 2 px orange
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
- **Plain:** one line in Body, Prose Grey. **Rich:** a title in white, a fact line in Prose
  Grey and a note in Label, Label Grey.
- **Where:** every icon button (its label is its Tip unless it says more), and anything whose
  explanation is not on screen: a value's conditions ("Full tank and full cargo"), a warning's
  detail, a short demand ("96 t demand, less than the 120 t you carry"), a station or star type
  icon. Not on text that only repeats itself, such as a name cut short by an ellipsis.
- **Access:** screen readers get the same words through `aria-describedby` (not when they only
  repeat the target's label). A target the keyboard can't reach gets a tab stop, except a mark
  repeated down a column whose words are already its accessible name.

A `title=` on an interactive element fails `app/src/lib/ui/tip.test.ts`. *Why:* the commander
glances over from the game; an explanation has to be there the moment the pointer is.

### Badge

Label type, mixed case, Label Grey, 1 px Seam outline, 20 px high, no fill. For a fact with
no place of its own in a row ("Scoopable", "Permit", "Odyssey only"); a pad letter is a badge.
*Why:* a fill would make it look like a control or a selection.

### Material tile (picker grid)

A 76 px tile in a 1 px Seam outline on Void. The count in Value XL, Options Grey, top left;
the name below in Label, Prose Grey. The storage cap is the same for a whole grade, so it is
stated once in the column head ("Grade 1 · max 300"). A 2 px Mark Off track along the bottom
edge fills Prose Grey with count / cap, Orange Lit at the cap. The top right corner holds at
most one Meta mark.

- **Have some:** as above.
- **Have none:** all text Grey Dim, the track empty.
- **Selected:** Hull Raised, the count and name White. One per grid. The side panel explains
  it.
- **Wanted by a plan:** a "need 12" corner mark in Orange; a short amount gets an orange "!".

*Why:* this is the trader screen's tile, with its corner ratio replaced by what the
commander's plan needs, since the window does not trade. The counts are grey because a grid
of them is read, not ranked; the plan's marks are what to act on.

### Lookup result row

One grammar for every lookup and every Places finder: a result reads as a result, not a
record. A row is two lines in three groups, then its actions; finders differ only in what
their groups hold. See [tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png) and,
at the smallest window, [tone5-sell-1100.png](docs/design/nav-spike/tone5-sell-1100.png).

1. **The value and what qualifies it** (first, about 136 px, right-aligned): the value the list
   is ranked by in Value L, Orange, its unit in Label, Label Grey; under it, in Label, the fact
   that says whether it holds for this commander. On Sell, price over demand with the demand
   bars, coloured by [the list](#colours-that-carry-a-meaning). Each other finder's groups are
   settled when it adopts the row (astromech-3ca6).
2. **The place and what you need to dock** (flexible, about 280 to 400 px): the station's type
   icon, its name in Name, Options Grey, its pad as a badge (always shown for stations; struck
   through in Red Low when too small for the ship) and its arrival distance in Label, Prose
   Grey; under it, indented to the name, the system in Label, Label Grey, with the copy icon.
   Each line truncates before it wraps.
3. **The trip** (about 96 px, right-aligned): jumps in Figures, Prose Grey, over light years in
   Label, Label Grey. A report older than the age limit puts its age here in Orange in place of
   the light years ("34 d old"). Where the list has no side panel, the age stays here at any
   age, in Label Grey when fresh.
4. **Actions:** icon buttons for add stop and set destination, at the row's end.

Facts the commander does not choose a row by (when the report was made, the faction, the
market's other figures) go to the side panel, and stay on the row only where the list has
none. A text fact that is the same on every row is left out and said once in the summary line.

Rows are 60 px, no zebra, no separators. Group heads ("Price · demand", "Station · pad ·
arrival", "Trip") sit above a Seam hairline in Label, Label Grey. Hover: Row Hover. Selected:
Hull Raised, the ranked value Orange Lit, the name White, the row's icon buttons Orange Mid;
one per list, and only while something shows the selection (a side panel about the row, or
keyboard focus in the list). A stale row steps its ranked value down to Orange Mid and shows
its age in Orange, so the age is the brightest fact on it. *Why:* the market screen's grammar
(the ranked figure, then the place, one selection), grouped the way a commander reads a result:
price with demand, a station with its distance from the star (Sumeet, 17 Nov 2026).

**Collapsed filters.** Above the group heads a finder shows its filters as one summary line in
Label, Label Grey ("Within 100 ly of Sol · large pad · reports up to 30 days old") with CHANGE
after it, and no filter controls. The thing searched for (the commodity, module or service) is
not a filter: it stays an input above the summary, with what the demand colours measure against
beside it ("120 t of Painite aboard"). CHANGE opens the controls in place; each edit applies at
once and searches again, and the controls stay open across edits, so several filters change in
one visit. They close on DONE, on Escape or on CHANGE again, never because a search started,
finished or failed. A filter set away from its default is always named in the summary. *Why:*
up to five rows of filter controls stood above the first result, though the commander changes
them rarely and reads the results every time, and the summary line above the results already
named them.

Where a group's line has no room, it truncates with an ellipsis rather than wrap. Where even
that leaves no room, as in a side panel, the row is **compact**: the first two groups' first
lines and the actions, one line, no heads. *Why:* the Aligned Columns Rule drops the fact
before it wraps the row; a 320 px panel cannot hold the groups, and its lookups become links
into their finder later (decision astromech-g8d5, item 5).

### Route strip

A 36 px Hull band across the top of the page column, where the route sidebar is collapsed or
the window is below the dock width: a chevron at the left end that opens the sidebar · "Next"
in Label, Label Grey · the next system in Name, Orange Lit · copy icon · "3 of 41" in Figures,
Prose Grey · next-jump notes as badges · on the right the destination and "38 jumps left". An
orange "!" with plain words when the commander should act ("Refuel before the next jump", "Off
route"). With no route: "No route" in Label Grey and the same two actions as the
[no-route rail](#route-sidebar), Find or plot and Import a route. Never a second line. *Why:*
users of every competing tool ask for next stop, n of N and refuel on one line they can keep
their eyes on.

### Status bar

A 32 px Hull band at the foot of the window, Label size, one line. It joins the route sidebar
with no rule between them; where it meets the page, the page's Seam edge runs along its top.
On the left the current ship's name in Options Grey and its type in Label Grey, then "Jump"
with its unladen maximum, "Fuel" (main tank | capacity) and "Cargo" (tonnes aboard | capacity)
as multi-value stats, their names in Label Grey and figures in Prose Grey. On the right the
core's state ("Elite running · Voice ready") in Label Grey, or Orange when Elite or the core
isn't running. Fuel and cargo show only while Elite runs; a failed read drops them silently,
since the next read on the 2 s tick is the recovery. *Why:* the ship and the app's health
belong to no one page, and taking the state out of the header gives it room.

### Find or plot

The header's one search box: an input, 320 px, placeholder "Find or plot", "Ctrl+K" at its
right end in Label size, Orange (it is text in a cell, so not a grey). A system plots or adds
a stop; a phrase opens a finder. Results in a popover: one 34 px row each, the name in Options
Grey and what it is in Label, Label Grey ("System", "Material traders", "Page · Trade"),
systems and entries ranked together. The highlighted row is an ember cell, and on it the
kind steps up to Prose Grey (Label Grey is 4.4:1 on a cell). A system row ends with
PLOT (a word: it opens the plot panel) and the add stop and set destination icon buttons. With
text in the box the clear icon replaces "Ctrl+K".

### Places home

Finder tiles under section headings (Trade, Ship, Engineering, Mining, Route). Each tile is a
link in a Seam outline, 220 px: finder name in Name, Orange (it is what you press); one line
in Label, Label Grey; where a live answer is known, its figure in Value L, Options Grey, with
its context in Label, Label Grey. Hover: Row Hover. *Why:* a visible grid of what can be
found, like the station services board, so search and voice are shortcuts, not the only way
in.

### Side panel (kept)

320 px after a 1 px Seam rule: title in white, its place in Prose Grey, the key value in Value
XL, Orange Lit (the panel's one main value), facts in Prose Grey with Label Grey heads, prose in
Prose Grey, actions as the primary button and text actions. *Why:* the game's right-hand detail
panel (white item name, its facts, the key price brightest) and the Controls "About this
setting" panel.

### States

| State | Treatment | Why |
|---|---|---|
| Selected row or tile | Hull Raised; the ranked value Orange Lit, the name White, other text one step brighter; only while a side panel or keyboard focus shows the selection | a selection that does not outshout the page's one fill and its main value ([tone5-sell-2560.png](docs/design/nav-spike/tone5-sell-2560.png)) |
| Chosen option | ember cell, Orange Lit at 600, a 2 px orange underline | marked, not filled: the fill is kept for the one thing to press |
| Active top tab | solid orange, On Orange text | the game's panel tabs; the frame's one fill |
| Active page tab | Options Grey + 2 px orange underline | one step below the top tab |
| Current section | heading white, its rule orange; only the section holding the item a side panel describes | the trader screen, where the section the cursor is in is white |
| Hover | Row Hover on read-only things; Ember Cell Hover on cells; icons Orange Lit | a response without a new colour |
| Focus | 1 px orange outline, 2 px offset. On a filled element (the primary button, the active top tab, a listening cell) a 1 px On Orange outline, inset 2 px | keyboard users need to see where they are; an orange outline vanishes against an orange fill |
| Disabled, unavailable choice | Orange Dim; no surface lift | the trader's empty tiles |
| None, zero, over (read-only) | Grey Dim; marks Mark Off | absent, still legible |
| Stale report | the ranked value steps down to Orange Mid; the age in Orange in the trip group ("212 d old") | a step down says "do not trust this" and the age, now the brightest fact, says why; red stays for failures |
| Rules this result out | Red Low with its second cue (one demand bar, a struck-through pad, the reason in words) | the commander sees a row they cannot use before flying to it ([The Narrow Red Rule](#named-rules)) |
| Warning (act on it) | orange "!" + plain words, at the place it applies | the game's orange warnings; urgent but not failed |
| Hard error (it failed) | Alarm Red "!" + plain words + a recovery action ("! Could not read the route. TRY AGAIN"); a whole-page error is a white title, a prose line and the action. A page that re-reads on its own every few seconds offers no action: the next read is the recovery. Never a filled block | red is the game's malfunction colour |
| Empty | one prose line saying what is empty and what to relax, plus one action ("No buyers within 100 ly. WIDEN TO 250 LY") | an empty list with no way forward is a dead end |
| Loading | one Label Grey line naming what is fetched ("Asking Spansh…"); a refresh keeps old rows Grey Dim until replaced. No spinners or skeletons | motion pulls the eye; dim old rows keep the layout still |
| Done (a copy, say) | the control's own icon swaps for 1.5 s (copy to the `copied` check), then back; instant both ways | the one brief feedback allowed (The Brief Feedback Rule): a copy leaves no other trace |
| Listening (press to bind) | binding cell filled orange, On Orange caps, while it listens | the game's listening cell; it is the one thing to press at that moment |
| Conflict, ignored binding | orange "!" + "Also: <action>"; ignored text struck through, Orange Dim | a warning, not a failure |
| Overlay open | the page behind under a Void scrim ([The Dimmed Page Rule](#named-rules-2)) | says what has focus |

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

Sizes are listed in px at 100 percent Text size; `tokens.css` holds each in rem (px / 16),
except 1 px rules and borders. Rows marked "(new)" are revision 5's, which `tokens.css` does
not have yet: the item named adds them, and the rows it replaces stay until then.

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
| scrim | `--scrim` | #000000b3 (black at 70 %), behind a dialog only |
| scrim, route | `--scrim-route` | Void at 72 %, the page behind the route sidebar open over it ([The Dimmed Page Rule](#named-rules-2)) |
| white | `--text-hi` | #ffffff |
| options-grey | `--options-grey` | #e0e0e0 |
| prose-grey | `--prose` | #c4c4c4 |
| label-grey | `--label-grey` | #8c8c8c |
| grey-dim | `--grey-dim` | #6a6a6a |
| mark-off | `--mark-off` | #4f4a45 |
| red-low | `--red-low` | #ca6657 |
| alarm-red | `--alarm` | #f0412e |
| hair | `--gap` | 2 px |
| xs, sm, md, lg | `--sp-1`, `--sp-2`, `--sp-3`, `--sp-4` | 4, 8, 12, 16 px |
| xl, 2xl, 3xl | `--sp-6`, `--sp-8`, `--sp-12` | 24, 32, 48 px |
| gutter | `--gutter` | 40 px |
| Prose width | `--measure` | 68ch |
| Form layout width | `--form-w` | 900 px |
| Side panel (List with detail) | `--panel` | 320 px |
| Control row, result row | `--row`, `--row-result` | 34, 60 px (44 until astromech-3ca6) |
| Route sidebar, no-route rail (new, astromech-uc7i) | `--sidebar`, `--rail` | 320, 48 px |
| Route strip, status bar, material tile | `--strip`, `--status-bar`, `--tile` | 36, 32, 76 px |
| Meta, Label/Section/Action, Body/Name/Nav/Figures/Cell | `--fs-xs`, `--fs-s`, `--fs` | 12, 13, 15 px |
| Value L and wordmark, Title, Value XL | `--fs-l`, `--fs-xl`, `--fs-xxl` | 18, 24, 28 px |
| Caps tracking | `--caps` | 0.06em |
| Each Hierarchy role (Typography) | `.type-wordmark`, `.type-title`, `.type-value-xl`, `.type-value-l`, `.type-nav`, `.type-name`, `.type-body`, `.type-figures`, `.type-cell`, `.type-label`, `.type-meta`, `.type-action`; Section is `.section` | a class setting the role's size, weight, line height, case and colour; a page names the role and sets no font size or colour. A unit after its number is `.type-unit`. The one main value of a page or panel adds `.main` to Value XL or Value L (Orange Lit) |
| Text size (The Text Size Rule) | the `html` font size | 100 to 180 % of 16 px, set by `lib/scale.svelte.ts` from the commander's choice. Revision 4's `--zoom`, `--win-h` and `--win-w` went with astromech-b2cz |
| The frame's side margins | `--side` | 16 to 40 px: 3 % of the window's width |
| The face | `--face`, `--face-stretch` | Saira Variable at 87.5 % |

## Deviations from the game

Each place we depart from Elite's UI, what we do instead, and why.

| The game | astromech | Why |
|---|---|---|
| All text in caps | Caps only for the wordmark, top tabs, the sidebar's heading, section headings and action words; mixed case elsewhere | Caps everywhere signals nothing and reads slower; system names carry meaning in their case. Decided for row labels on 11 Oct (V1-DESIGN §9) |
| Every word on a station screen in orange | Read-only data in white and grey; orange for the ranked value, the main value and what can be acted on (The Orange Means Act Rule) | In a window full of data, all-orange ranks nothing (Sumeet, 16 Nov 2026); the game's own Options screen already uses grey for what is not orange |
| The selected row filled solid orange | Selection is Hull Raised with lit text; the one fill on a page is its primary button (The One Fill Per Page Rule) | With grey rows, an orange slab outshouts the main value |
| Red, green and blue for status (green supply bars, green/red price arrows, blue mission goods) | No green or blue. The market's three bars stay, coloured by what they mean for this commander: Red Low short, Orange Mid tight, grey plenty | The bars are a symbol players know; a second hue system would be one more to learn, so the colours come from the palette already here |
| Red for warnings and damage alike | Warnings and stale data stay orange; Alarm Red only when something failed; Red Low for a fact that rules a result out, with a second cue | Keeps alarm red rare enough to notice; an orange "!" already reads as "look here" in the game |
| The item under the gamepad cursor is filled, so the default action starts filled | The primary button is always filled; other buttons are text actions | A mouse window has no cursor highlight; the primary needs the fill to be found |
| Empty binding slots look like filled ones | Empty slots are the dimmer Ember Empty with the word "Set" in Orange Mid | Finding unbound actions is a main job of the Controls pages; the word is what the commander clicks, so it stays readable |
| Blurred cockpit behind every panel | Flat neutral Void; a Void scrim behind an overlay | Calmer and more legible; no Frontier art behind the window |
| Glow, scan lines and animated panel transitions | Flat, no glow, no motion; the one exception is a control's brief confirmation (copy to a check for 1.5 s) | A desktop window beside the game should not pull the eye; a copy leaves no other trace on screen |
| The UI scales with the screen | Text size is the commander's choice, picked once from the display; the window never zooms with its size (The Text Size Rule) | A desktop window is resized to see more, not bigger text |
| Prose in orange on station screens | Paragraphs in Prose Grey | Long orange passages tire the eye; the game's own Options screen uses grey for its explanations |
| Long single-screen lists with no search | Search, Find or plot, and grouped two-line result rows | The window is a lookup tool; the game's lists are a known pain (PRODUCT.md) |
| Every value cell has a corner ratio or icon | Corner marks only for what the commander's plan needs | The window does not trade; the plan is the useful corner fact |
| Game fonts and icons | OFL Saira and astromech's own icons; the engineer hexagon from a credited EDAssets file | No Frontier assets (PRODUCT.md) |
| One face per role in places (the Options screen mixes cuts) | One face, Saira variable, for everything including paths | Fewer files, one rhythm, tabular figures everywhere |
| No route panel outside the galaxy map | The route sidebar, part of the frame on every page, or the route strip | The most requested missing piece in companion tools (research, 24 Oct); the route belongs beside whatever page the commander is on |

## Do's and Don'ts

### Do:

- **Do** start from what the game does and write down why when you depart from it.
- **Do** keep read-only data white and grey, and spend orange on the ranked value, the main
  value and what the commander can act on or change.
- **Do** give each element one main value in the largest size.
- **Do** add a colour to [the list](#colours-that-carry-a-meaning), with its meaning and a
  second cue, before a page uses it.
- **Do** show a selected row or tile as Hull Raised with lit text.
- **Do** keep changeable values in ember cells (Orange at 500) and read-only things on Void,
  in a Seam outline or on Hull Raised.
- **Do** use white only for titles, the current section, the selected row's name and names
  lifted under The Lifted Name Rule.
- **Do** use Elite's own symbols where players know one: the engineer hexagon, star and station
  icons, the market's demand bars.
- **Do** build every page outside Controls on Board, List with detail, Workbench or Form, and
  keep each action next to what it acts on.
- **Do** group a result row's facts the way they are read: the value and what qualifies it;
  the place and what you need to dock; the trip.
- **Do** show a finder's filters as their summary line with CHANGE, and open the controls only
  on demand.
- **Do** name star, body and station types with their icons, the type as the accessible name.
- **Do** use icons for repeated row actions, with a Tip and a label, and words for
  primary and one-off actions.
- **Do** set numbers in tabular figures, right-aligned, units in Label Grey.
- **Do** give sizes in rem, so the commander's Text size scales them.
- **Do** keep the route in the frame on every page and a copy icon beside every system name.
- **Do** dim the page behind anything opened over it.
- **Do** keep the patterns that work: the Settings row, the Controls side panel, ship
  cartoons, onboarding's step list and primary button, plain-word status and errors.

### Don't:

- **Don't** colour read-only data orange, or use a colour that is not on the list.
- **Don't** set a page's own width or stretch a row across the window; the layout places it.
- **Don't** colour a name orange, or lift every name on a page.
- **Don't** put read-only tiles, specs, chips or cards in ember cells, or a page tile on Hull.
- **Don't** fill anything on a page but its primary button, or show more than one primary
  button.
- **Don't** use red for stale data, warnings, health issues, decoration or emphasis; Red Low
  only for a fact on the list, with its second cue.
- **Don't** put text the commander needs in order to act in Orange Dim or Grey Dim.
- **Don't** select a row or tile by default; a selection shows only with its side panel or
  keyboard focus.
- **Don't** stack more than two tab rows.
- **Don't** give a result row a column per field.
- **Don't** put repeated words like ADD STOP on every row, or hide row actions until hover.
- **Don't** make the ranked value the smallest text in a row.
- **Don't** draw a warning or error as a solid colour block.
- **Don't** scale text with the window, or write a size in px that should follow Text size.
- **Don't** use a second typeface, a radius, a shadow, a glow, an animation or a page
  transition. A control's 1.5 s confirmation is the one timed change allowed.
- **Don't** use Frontier art, fonts, icons or screenshots, or any language but English.
