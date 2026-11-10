---
status: draft
last_changed: "2026-11-10: first version, from the 10 Nov page audit (Sumeet's direction), revised after the first planning reviews"
---

# Why

The core works, and some features are ones Sumeet won't play without: neutron plotting, and
seeing his binds on his own stick. The window lets them down. An audit of all 24 pages at
1920 px on 10 Nov 2026 found:

- **The pages don't follow DESIGN.md's main rule.** "One main value, largest" is the rule; the
  28 px value size (`--fs-xxl`) is used nowhere, and a material tile shows its count at 13 px.
- **Half the window is empty, or the content is stretched.** Eight pages cap themselves at
  `max-width: 60rem` and sit pinned left; Stored modules and On foot spread three columns over
  1,800 px.
- **Nothing ranks.** Almost all text is one orange at 13 to 15 px. On a 1440p screen it is
  smaller than the game's smallest text.
- **Tables are the universal hammer.** Ten page files are plain tables, most others label and
  value lists.
- **There is no home.** The app opens on My controller, and the live views (Station, Session,
  Mining) are filed under Logbook.
- **Explanations are printed on the page**: 13 paragraphs on Powerplay, footnotes elsewhere.

The Route page redesign agreed on 6 Nov (astromech-ok29) is the model for the fix and has not
been built.

# Outcome

A commander opens the window on a page about what they are doing now. Every page outside
Controls uses the window well at any size from 1100 x 700 to 4K, shows one obvious main value
per element, and uses a screen built for its job where a table was. A sparse page stays
sparse: a layout places content, it never adds any. Text is readable from a seat next to the game.
Worth shipping on its own: the foundation (scale, orange, type scale, layouts, tooltip, status
bar) and the Route page.

# Success Criteria

Both cover every page outside Controls (My controller, All keybinds, Presets, Key tester),
which already read best and get only the frame, the scale and Tip.

- fills-the-window: every page is built on one of the four layout components and passes the
  page-sizing check (no page file sets its own width or font size). Shown by the check
  being on for every page file except the Controls ones and the frame's two parts in the
  route module, and by captures of every page at
  1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 with nothing clipped and no action
  further than one column from what it acts on.
- glance-test: on every page a reader can point to the main value, the thing they can change
  and the thing that needs them, without reading (DESIGN.md's own test). Shown by
  astromech-crxa: expected answers are written per page first ("none" where a page has
  nothing to change or nothing needing attention), a fresh agent answers from the
  2560 x 1440 screenshots alone, and every page passes. A failing page is fixed and asked
  again; one left failing leaves the criterion unmet.

# Sizing

- Complexity: high. It touches every page, which is why it runs as increments reviewed one
  screen at a time.
- Value: high. The pages are in daily use, and the owner named their look as the app's
  weakest part.
- Risk: low. Changes are visual, show at once in the preview, and revert per page. The scale
  rule is the one part that could misbehave in WebView2 only.

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

# Design

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

**Now page (increment 2).** A new module `app/src/modules/now/` in a new first nav group. It
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
| Stored modules, On foot, Places home, Mining, Data sharing, Diagnostics, Voice tester | astromech-gvy0 |
| Materials | astromech-mwh5 (layout), astromech-z5oh (tiles) |
| Engineers | astromech-m6gw |
| Places finders | astromech-mwh5, astromech-drnv |
| Route | astromech-ok29 |
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

# Derisking

The riskiest assumption is that the purpose-built screens read better than the tables they
replace, to Sumeet, on his screen. Taste can't be settled by a test, so the plan gets his
eyes on it early and cheaply:

1. The mockups are part of this brief, so the direction is approved before any page changes.
2. Increment 1 ends with the Route page built for real on the foundation. It is the design
   already agreed in detail, so it tests the foundation (scale, layouts, Tip, icons) without
   also testing a new idea. Its page items are blocked on the foundation items, so it cannot
   finish first.
3. astromech-irtr closes increment 1: an agent that built none of it checks every converted
   page at four sizes and publishes preview links. The mission then stops for Sumeet to
   look. This is its one planned stop, reserved in Authority, and the screens in increments
   2 to 4 are planned from what he says.

The second risk is that the plotters can't feed the timeline as drawn. The planning review
found that already, and astromech-woqv now starts from a contract of what each source knows.

The third is the scale rule in WebView2, which only Windows can show. It is a residual risk,
not a gate: astromech-3340 is a check on Sumeet's PC that waits on a release and blocks
nothing. If it goes wrong, the Text size setting and the cap are the levers, and the scale is
one root rule to change.

# Scope

In: the window's frame, the design system and its tokens, shared UI components, the Now page,
the Route page, and the nine screens above. Core changes only where a screen needs data it
lacks (route data, body parents).

Out:

| Thing | Why | Instead |
|---|---|---|
| New features (system and station pages, missions, bookmarks) | A different outcome | They stay in the backlog and adopt the layouts when built |
| Controls pages | They already read best | Only the frame, scale and Tip reach them |
| VR or in-flight UI, overlays | Deferred (PRODUCT.md) | Unchanged |
| A galaxy map of the route | Most work, least proven | Backlog item, after the timeline |
| Themes, colour customisation, a second hue | DESIGN.md fixed rule | Unchanged |
| Voice, callouts, lookups' behaviour | Not visual | Unchanged |

Environment: Windows with WebView2 next to Elite; designed and tested in Chrome on Linux
through the browser preview.

# Authority

The orchestrator may decide the layout and detail of each screen within the approved mockups
and DESIGN.md, the order of screens, and how work is split. It may cut a visual that fails the
"answers a question the commander acts on" test.

One planned stop: after astromech-irtr the orchestrator sends Sumeet the preview links and
waits. It plans the Now page and the screens only after his answer, and records that answer
as a decision.

Back to Sumeet at any time: a change to navigation or the
default page beyond this brief, a new hue or typeface, removing a feature a page has today,
and any screen whose built result departs from its mockup in kind, not detail.

# Risks and Unknowns

- A screen needs data the core doesn't give (body parents are the known case). It becomes a
  core task in that screen's increment; the screen ships after it.
- Scaling changes how much fits at 1100 x 700. If a page can't fit, its layout drops its
  detail panel below the content at that size; it never shrinks text.
- Sumeet dislikes a built screen. It reverts to its old body per page; the foundation stays.
- The page-sizing check would flag existing pages. Each part is turned on file by file as a
  page is converted, and is on for every page file outside Controls at mission close.
