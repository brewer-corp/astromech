---
status: draft
last_changed: "2026-11-17: amended from Sumeet's answer at the increment 1 stop: route sidebar, SHIPS tab, engineer search, calmer colours, text size in place of the zoom"
---

# Sizing

- Complexity: high. It touches every page, which is why it runs as increments reviewed one
  screen at a time.
- Value: high. The pages are in daily use, and the owner named their look as the app's
  weakest part.
- Risk: low for the visual changes, which show at once in the preview and revert per page.
  Two parts of increment 2 can be wrong without looking wrong, and carry tests for it: the
  engineer search's grade advice (astromech-7did) and the demand colours (astromech-3ca6).
  Text size must not shrink an existing install (astromech-b2cz).

It is worth its cost because features that already work are being undersold by pages nobody
enjoys reading. The cheaper alternative, fixing only the scale and orange (astromech-k6pl),
makes the same tables bigger.

# Shape

Sumeet approved all eight proposals on 10 Nov 2026 (decision astromech-ci0o). The mockups
in [Mockups](#mockups) are the direction; DESIGN.md stays the binding rulebook and is amended
first.

- **The game's language still wins.** DESIGN.md's Station Board stays: one hue plus alarm red,
  square corners, brightness as state, cells mean editable. This mission makes the pages obey
  it and amends it in the few places below.
- **Hierarchy over density** (Sumeet's sparse-UI rule). An empty area is not a reason to add
  content. A screen earns a visual only when the visual answers a question the commander
  acts on; the Route thread cut bars, distance columns and jump counts on that test.
- **Purpose-built over generic.** A table stays where the job is comparing rows on shared
  columns (finder results, the market). It goes where the data has a shape of its own: a
  tree, a route, a loadout, progress to a goal.
- **Design on the real page.** Each screen is built on its branch against the browser preview
  and shared with `scripts/preview-publish` (CONTRIBUTING.md). The static mockups are only
  the brief's reference.
- **When qualities conflict:** readable beats dense, the game's convention beats a new one,
  and a page that says less beats one that explains itself in prose.

## DESIGN.md amendments (revision 4)

1. Scale with window height: `clamp(1, min(height / 1000, width / 1100), 1.8)`, with a Text
   size setting. Text never shrinks below its size today, and a narrow window is not zoomed
   past its width.
2. Orange `#ff7100`, label orange `#d97a34`, body weight 500; labels used less.
3. Frame: a 32 px status bar at the foot (ship, range, fuel, cargo, app health); the header
   loses its status text; a NOW tab first, and the app opens on it.
4. Four page layouts replace the content-width caps: Board, List with detail, Workbench, Form.
   The width rule becomes "an action sits next to what it acts on".
5. Tip, an instant tooltip, as a component; explanations live in Tips, not in page prose.
6. White or Options Grey is allowed for entity names outside Controls where it ranks them.
7. Brief state feedback is allowed (a copy icon becomes a check for 1.5 s). No decorative
   motion, no transitions between pages.
8. Icon sets for star types, body types and station types, drawn for astromech.
9. Collapsed filters: a finder shows its one-line constraint summary with CHANGE; the controls
   open on demand.

## DESIGN.md amendments (revision 5, 17 Nov 2026)

From Sumeet's answer at the stop, each approved as a mockup on `spike/nav-rework` at
`dea5f0bb` (captures in `docs/design/nav-spike/`, brought to main by astromech-n41x).
They replace amendments 1 and 6 above and change 2 and 3.

1. Text size is the commander's choice: sizes in rem, one root size, six steps. The zoom
   from window height goes. A first run picks the step from the display's height; nothing
   changes it after that but the commander.
2. Read-only data is white or grey. Orange is for the value a list is ranked by, the main
   value, and what the commander can act on. Colour beyond that needs a stated meaning and
   an entry in DESIGN.md's list (demand against the hold, a pad too small, a stale report,
   star icons, security, engineer state, hexagons).
3. A selected row is raised with lit text, not filled orange.
4. Result rows are two lines in groups, not a column per field.
5. Where Elite has a symbol players know, use it (the engineer hexagon).
6. The route is a left sidebar on the frame's surface, joined to the status bar; collapsed
   or in a narrow window it is the route strip. The Route page goes and GALAXY becomes PLACES.
7. Top tabs: NOW, SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK,
   CONTROLS, SETTINGS. SHIPS holds Ships, Stored modules and Suits; ENGINEERING holds
   Materials and Engineers. On foot is not a page.

# Design

**The reworked foundation (increment 2).** In order, because the first three touch every
component: DESIGN.md revision 5 (astromech-n41x), text size (astromech-b2cz), colours
(astromech-3kf5). Then in parallel: the lint for raw colours and sizes (astromech-4wi1), the
route sidebar (astromech-uc7i) and after it the tabs (astromech-5a01), which both edit the
tab groups, then the engineer search (astromech-7did); and the grouped result rows
(astromech-3ca6). astromech-8e22 closes the
increment. The spike is reference only: items write fresh against the real components,
except the hexagon component, which astromech-7did may bring over and rework.

- *What the spike showed.* The sidebar fits at 320 px with the star type carried by its
  icon. It can dock only where a List with detail page keeps about 1,300 px beside it. Ten
  top tabs need more header width than eight. Every figure was measured under the old
  zoom, so astromech-b2cz, astromech-uc7i and astromech-5a01 measure again.
- *What it did not show.* The engineer search's main answer was wrong in the mockup (it
  named an engineer the commander's rank cannot yet use at that grade); astromech-7did
  states the rule. Distances were from Sol. No window command returns grades by module type.
- *Narrow-window rules.* Ten page files measure the whole frame (`@container win`). The
  container moves to the page column with the sidebar, and its breakpoints to rem with the
  text size.

**The screens (increments 3 and 4)** are below as first planned, now blocked on
astromech-8e22 and built to revision 5. The Now page no longer carries the next route stop,
which the sidebar shows. Suits (astromech-nnn0) joins them.

**Foundation (increment 1).** Order: DESIGN.md revision 4 (astromech-b2k3) and the scale
(astromech-k6pl) first; then layouts (astromech-mwh5), then type roles (astromech-z5oh);
the Route page's two page items wait on all three. astromech-irtr is the gate that closes
the increment.

- *Scale and colour* (astromech-k6pl): one root `zoom` set from window height in
  `app/src/App.svelte`, tokens in `app/src/design/tokens.css`. The Text size setting lives in
  the core's config behind commands in the command layer, as the voice settings do; the
  window has no settings store of its own.
- *Layouts*: `app/src/lib/ui/layout/` holds `Board`, `ListDetail`, `Workbench` and `Form`.
  Each owns its widths, gutters and the 320 px detail panel. A page module picks one and sets
  no width of its own.
- *The page-sizing check*: one check in `pnpm lint`, built by astromech-mwh5, with two parts
  switched on per file from one list in `app/eslint.config.js`, so a module enrols a page at
  a time: width (a `max-width` or
  fixed width in a page module; media-query conditions and the prose measure are allowed) and
  font size (any `font-size` in a page module). Each item enrols the files it converts;
  astromech-gvy0 enrols whatever is left. The frame's parts that live in the route module
  (`Strip.svelte`, `PlotBar.svelte`) are not pages and are exempt, like Controls. The Controls modules are exempt for good.
- *Type roles*: the roles in DESIGN.md's Hierarchy table become classes in `tokens.css`
  (`.value-xl`, `.value-l`, `.name`, `.label`, `.meta`), so a page names a role, not a size
  (astromech-z5oh, after k6pl has set the values).
- *Tip* (astromech-sixj): `app/src/lib/ui/Tip.svelte`, used by `IconButton` and every former
  `title=`.
- *Status bar* (astromech-q79q): `app/src/lib/ui/StatusBar.svelte` in the frame. The ship and
  its range come from the `engineering_fleet` reply; fuel and cargo need a new window command,
  `ship_status`, since the `status` reply carries neither. At 1100 x 700 the frame's fixed
  parts must leave the page 480 px; if not, the bar shares the route strip's row there.
- *Icons*: the new sets go in `app/src/lib/ui/Glyph.svelte` with the existing ones.
- *Filter summary* (astromech-drnv): `app/src/lib/ui/SearchFilters.svelte` owns the collapsed
  state and shows the summary its parent passes. Edits apply live and the panel closes only
  on DONE, Escape or CHANGE, whatever a search does, so several filters change in one visit.

**Route page (increment 1, astromech-ok29).** The data in the core (astromech-woqv), the title
as the plotter (astromech-irrv), the timeline (astromech-qwvs). It is the first screen built
on the foundation, and its items require the layouts, type roles, Tip and icons.

One thing changes from the mockup agreed on 6 Nov. That mockup drew the stars passed on every
leg. The neutron plotters return only the stops and a jump count per leg, so the stars of a
leg are known only where a source lists them: the leg being flown, from the game's
`NavRoute.json` once it is plotted in the galaxy map, and every leg of an exact route. A leg
with unknown jumps shows its jump count. A leg's jumps come from NavRoute first, then the
plotter's own list, then not at all; the page says there is nowhere to scoop only when every
jump is listed and known not to be scoopable.

**Now page (increment 3, after astromech-8e22).** A new module `app/src/modules/now/` in a new first nav group. It
composes replies the window already has: `DockedHere` (station, services), the route view, the
cargo sale comparison, Powerplay standing, and `Session`. It follows the game's state: docked,
in a system, or not running. Station, Session and Mining leave the Logbook group; their old
links keep working through `App.svelte`'s `MOVED` map. No new core command is expected; if one
is needed it goes through the command layer like any other.

**Screens (increments 3 and 4).** Each replaces one page's body inside its existing module,
on existing replies unless noted:

| Screen | Module | Data |
|---|---|---|
| System tree | `exploration` | `BodyView` lacks a body's parents; the core adds them from the journal's `Scan` |
| Biology cards | `exobiology` | existing |
| Ship loadout | `engineering` | existing (`Ship.modules`); slot grouping from the module's slot name |
| Engineer tree | `unlocks` | existing (`referred_by`, `refers`, `next`) |
| Powerplay | `powerplay` | existing (`NextRank`, system view); cycle end from the clock |
| Colonisation | `colonisation` | existing |
| Session and Logbook | `session`, `logbook` | existing |
| Station board | `station` (shown on Now) | existing (`DockedHere.services`) |
| Trade route legs | `trade` | existing |

**Which item converts which page.** Controls pages are exempt.

| Page | Item |
|---|---|
| Ships | astromech-8wr4 |
| Stored modules, Places home, Mining, Data sharing, Diagnostics, Voice tester | astromech-gvy0 |
| Suits (new, from On foot) | astromech-5a01 (moved in), astromech-nnn0 (screen) |
| Materials | astromech-mwh5 (layout), astromech-z5oh (tiles) |
| Engineers | astromech-m6gw |
| Places finders | astromech-mwh5, astromech-drnv |
| Route | astromech-ok29, then the frame's sidebar by astromech-uc7i |
| Commodities | astromech-mwh5 (layout), astromech-el3i (result) |
| Colonisation | astromech-m07p |
| System, Biology | astromech-dpaz, astromech-evj3 |
| Powerplay | astromech-rj6i |
| Logbook, Session | astromech-fzhp |
| Station | astromech-7vxi |
| Settings > General | astromech-mwh5 |
| Now (new) | astromech-coq6 |

The planned pages already in the backlog (System page astromech-glrl, Station page
astromech-0t4i, Missions astromech-ak4v, Bookmarks astromech-mg25, part diagrams
astromech-rgrh) are not in this mission. When they are built they use these layouts and
components.

## Mockups

Static mockups in `app/src/mockups/glow/`, rendered by `scripts/mockup-shots.sh` at
2560 x 1440:

- Now, docked: [glow-up-now.png](../design/glow-up-now.png), and at the 1100 x 700 minimum
  [glow-up-now-1100.png](../design/glow-up-now-1100.png)
- System tree: [glow-up-system.png](../design/glow-up-system.png)
- Ship loadout: [glow-up-ship.png](../design/glow-up-ship.png)
- Engineer tree: [glow-up-engineers.png](../design/glow-up-engineers.png)
- Powerplay: [glow-up-powerplay.png](../design/glow-up-powerplay.png)
- Materials with the real type scale: [glow-up-materials.png](../design/glow-up-materials.png)
- Sell finder with collapsed filters: [glow-up-sell.png](../design/glow-up-sell.png)
- Route timeline: the 6 Nov mockup, https://claude.ai/artifact/JduR92Bxotyi7hd6Jg8nN7
- The files above are on the branch `mockups/glow-up` only; astromech-n41x brings the
  captures to main.
- Increment 2, on `spike/nav-rework` at `dea5f0bb`, in `docs/design/nav-spike/`: the frame
  `hier-B-sell-2560.png`, `hier-B-collapsed-2560.png`, `hier-A-over-page-1100.png`,
  `4-no-route-2560.png`; colours and rows `tone5-sell-2560.png`, `tone5-sell-1100.png`;
  Engineers `tone5-engineers-laser-2560.png`, `tone5-engineers-rest-2560.png`; Suits
  `tone5-suits-2560.png`; tabs `5-ships-2560.png`, `7-materials-on-foot-2560.png`.

# Derisking

The riskiest assumption is that the purpose-built screens read better than the tables they
replace, to Sumeet, on his screen. Taste can't be settled by a test, so the plan gets his
eyes on it early and cheaply:

1. The mockups are part of this brief, so the direction is approved before any page changes.
2. Increment 1 ends with the Route page built for real on the foundation. It is the design
   already agreed in detail, so it tests the foundation (scale, layouts, Tip, icons) without
   also testing a new idea. Its page items are blocked on the foundation items, so it cannot
   finish first.
3. astromech-irtr closed increment 1: an agent that built none of it checked every converted
   page at four sizes and published preview links, and the mission stopped for Sumeet to
   look. That was its one planned stop. His answer (16 and 17 Nov 2026) became increment 2,
   each change seen as a mockup on `spike/nav-rework` before it was planned.
4. astromech-8e22 closes increment 2 the same way but does not stop the mission: the
   screens start when it merges.

The second risk is that the plotters can't feed the timeline as drawn. The planning review
found that already, and astromech-woqv now starts from a contract of what each source knows.

The third is how the window reads in WebView2 on Sumeet's monitor, which only Windows can
show: the text size a first run picks, and the new colours. It is a residual risk, not a
gate: astromech-3340 is a check on his PC that waits on a release and blocks nothing. The
height zoom this risk was first written about is removed by astromech-b2cz; the lever now is
the Text size setting.

# Risks and Unknowns

- The text size a display starts with is a guess from its height. If it is wrong on
  Sumeet's monitor he changes it in Settings; astromech-3340 checks it on Windows.
- The colour change touches every built page at once (astromech-3kf5). It is one revert,
  and the checkpoint looks at eight pages before any screen builds on it.

- A screen needs data the core doesn't give (body parents are the known case). It becomes a
  core task in that screen's increment; the screen ships after it.
- A larger Text size changes how much fits at 1100 x 700. If a page can't fit, its layout drops its
  detail panel below the content at that size; it never shrinks text.
- Sumeet dislikes a built screen. It reverts to its old body per page; the foundation stays.
- The page-sizing check would flag existing pages. Each part is turned on file by file as a
  page is converted, and is on for every page file outside Controls at mission close.
