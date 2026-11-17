---
schema: atelier.item
schema_version: 3
id: astromech-m6gw
title: Engineering > Engineers as the unlock tree
status: building
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-8e22
- astromech-70ts
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Adb36f767-2b14-496f-a905-a66493b4f489
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:41.864039859Z
---

## Goal

The engineers read as the tree they are: who introduces whom, where the commander stands with each, and what to do next, for ship and suit engineers side by side. Today the page shows one kind at a time behind a Ship or Suit choice, as cards in columns that state their prerequisites in words (`app/src/modules/unlocks/View.svelte`), and what an engineer works on is two module types from `app/src/lib/engineerSpecialties.ts`, which is incomplete.

## Done when

- [x] Engineering > Engineers is a List with detail page on the type roles, laid out as `docs/design/nav-spike/tone5-engineers-rest-2560.png`: the search box astromech-7did built on top with the summary line beside it ("10 of 25 ship and 4 of 13 suit engineers unlocked · 15 to work on", counted from the tree), then SHIP ENGINEERS and SUIT ENGINEERS as two trees side by side. The Ship and Suit choice goes; `#/unlocks?kind=suit` (the old On foot link, `App.svelte`'s `ON_FOOT`) still opens the page. Colonia engineers sit in the same trees, where their introducers put them, as the capture shows.
- [x] Each tree's columns follow `Node.depth`, with a Seam line from each engineer to each one in `Node.introduced_by` (`engineering_unlocks`). A node is a Seam outline on Void, never a cell: its name in Name, then one line. By DESIGN.md's Engineer state entry: unlocked, the name Options Grey with the rank as White hexagons (`Hexes.svelte`, `tone="rank"`; a suit engineer, who has no ranks, shows one) and the system in Label; one the commander can work on now (`Node.next`, not unlocked), the name Orange Mid and the step left (`Node.left`) in Prose Grey; barred, the name and reason Red Low; not introduced, the name and "Not introduced" Grey Dim. Steps are words, as the capture has them: the mockup's progress counts ("2 of 5 black markets") are not shown, since `Node.left` is the step's text and nothing in the journal counts them.
- [x] Selecting a node raises it and fills the side panel: the engineer's name as its title, White; the system with its copy icon and the distance from the commander's current system in Label; the status in words; the steps to unlock, numbered, from `Node.left`; WORKS ON, each module type (ship) or modification (suit) they offer with the best grade as hexagons; who they introduce (the nodes naming them in `introduced_by`); for a suit engineer, their referral task (`EngineerLine.referral_task`) and the mods the wiki is unsure of marked "(?)" with its Tip, as the cards show today. Set destination is the panel's primary button, with Add stop beside it.
- [x] The core gives each `Node` what the engineer works on: for a ship engineer, each module group they offer with the best grade, worked out as `engineering_search` does (the max of `engineers::offers` over the group's blueprints, `crates/astromech-engineering/src/engineers.rs:50`) and labelled with the search table's short labels (`data/catalog/engineer-search.json`); for a suit engineer, their mods from `odyssey.json`. Table-driven Rust tests in `crates/astromech-engineering` cover a ship engineer's groups and grades and a suit engineer's mods, and `app/src/preview/fixtures/unlocks.json` is dumped again; the drift test passes. `engineerSpecialties.ts` goes if nothing else uses it.
- [x] Distance is worked out in the window from the commander's position and each engineer's coordinates: ship engineers' from `engineering_engineers` (`EngineerView.coords`, `Engineers.here`), suit engineers' from `odyssey_view`'s `EngineerLine.coords` (astromech-70ts). With no position in the journal the distance is left out.
- [x] While the box holds a search the answer replaces the trees, and clearing it brings them back, as astromech-7did built; the search keeps its own rows and hexagons.
- [x] States: loading one Label Grey line; a failed `engineering_unlocks` read is the shared Failure line with TRY AGAIN, as today; a failed `odyssey_view` leaves the suit nodes without their mods, as today; a commander with no engineer progress in the journal sees every engineer as the common-knowledge ones known and the rest not introduced.
- [x] The brightest things: the selected engineer's distance or step (the panel's main value, Orange Lit), the search box, Set destination and the engineers the commander can work on now in Orange Mid (what can be changed and what needs the commander). Nothing else is orange; no colour outside DESIGN.md's list.
- [x] No printed explanations: what each state means is in the nodes' Tips and the guide.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `unlocks` world: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Where the two trees do not fit side by side beside the panel, the suit tree goes below the ship tree and the panel below both (1100 x 700); text never shrinks. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/engineers/`, with a node selected. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/unlocks/{View,Search}.svelte` in `PAGE_SIZING`, and the `unlocks` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: the cases in `unlocks/View.browser.test.ts` keep their behaviour on the new markup, "lights the introducing engineer on hover" becomes selection lighting the introducers, and the suit mods case reads the panel; a new browser case selects an engineer and finds their works-on grades and steps from the recorded replies. No test of layout or copy.
- [x] `docs/guide/engineering.md`'s The tree section describes both trees, the states and the panel.
- [x] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-engineering` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The tree, its side panel and what each engineer works on. The search and the hexagon component are astromech-7did's; the suit engineers' coordinates are astromech-70ts's.

## Context

Approved capture: `docs/design/nav-spike/tone5-engineers-rest-2560.png` (layout and colour). It shows no side panel and no narrow window; the side panel follows `docs/design/glow-up-engineers.png`'s, in revision 5's colours (no filled selection: the selected node is Hull Raised). `glow-up-engineers.png` is otherwise superseded: its Ship or Suit choice and its filled nodes go.

Replies: `Node.ts` (`name`, `kind`, `system`, `permit`, `depth`, `introduced_by`, `status`, `rank`, `left`, `next`), served cached by `engineering_unlocks` (`crates/astromech-api/src/commands.rs:140`), which is why distance is worked out in the window rather than in each node; `EngineerView.ts` and `Engineers.ts`; `EngineerLine.ts` (`mods`, `referral_task`, `refers`).

## Report

**Calls I made where the item conflicts with itself or with DESIGN.md.** These are raised here for the first time. I didn't put them to the thread that started me, so each is my own default for the reviewer to judge.

1. **Suit mods come with the tree, not from `odyssey_view`.** The core line says `Node` carries each suit engineer's mods from `odyssey.json`. The States line says a failed `odyssey_view` leaves the suit nodes without their mods. Both can't hold, so I followed the core line, which is the tested one. A failed `odyssey_view` now loses only the referral task and the suit engineers' distances. The "(?)" flag also comes with `works_on` (`WorksOn.unsure`).
2. **Distance or step as the panel's main value.** The panel has one Orange Lit value. For an unlocked engineer it is the distance, in Value XL (Felicity: 102 ly). Otherwise it is the first of the numbered steps. In that case the distance sits in Label beside the system, as the panel line asks. It isn't repeated when it is the main value.
3. **Lighting the introducers.** DESIGN.md allows only Seam or orange for lines, so the selected engineer's introducers get an Orange Mid outline, and so do the lines to them. Both are lines, not text or fills.
4. **"Then one line".** Real `Node.left` strings are sentences ("Introduction from Juri Ishmaak: raise your rank with them past grade 3."), so a node's line wraps rather than ending in an ellipsis. That makes some nodes three or four lines tall, unlike the mockup's shortened "Invite: …" lines. Shortening the steps would be a change to the unlock data's wording, so I left it.
5. **Works-on hexagons are orange.** They follow DESIGN.md's grade colours, as the search does: Orange for the grade the commander's rank gets today, Orange Mid for the rest of what the engineer reaches. These are marks, not text. Grey hexagons would mean "reached rank", which is wrong here.
6. **Numbered steps.** Splitting `left` into sentences in the window would be fragile, so `Node` now has `steps`. It holds every step from here to the workshop, in order: the current step (what `left` says), then the introductions, the invitation and the workshop unlock. The Dweller shows "1. Invitation: …" and "2. Unlock at Wyrd: Pay 500,000 CR." I read every node's steps in the dumped fixture. Yi Shen's invitation text from the wiki ("Complete all referral tasks for Baltanos, Eleanor Bresa, & Rosa Dayette") repeats the three referral steps above it. I left the wiki's wording as it is.

**Core** (`e0e35caf`)
- `Node.works_on`: a ship engineer gets each module type with the best grade, from `engineers::best_grade`, which the search now shares, so the two can't drift. Labels come from the search table by `search::type_label`. A type that has its own entry keeps its label ("Pulse", "FSD"). A type that only appears beside others gets its entry's first type added after its own label ("Advanced multi-cannon", "Bi-weave shield"), because Tod and Juri would otherwise both show "Advanced". A type the table lacks keeps its catalog name ("Cargo Rack"). The table itself is unchanged, so the search's grade lines are too. A suit engineer gets their mods from the Odyssey data.
- `tree()` takes the suit offers the way `search()` does. `astromech-api::state::suit_offers` builds them once for both, so astromech-engineering still doesn't depend on astromech-odyssey.
- Tests in `unlocks.rs`:
  - a table of ship engineers' types, grades and order (The Dweller, Selene Jean, Tod, Didi Vatermann);
  - a check that every module type has a label no other type has;
  - a table of suit engineers' mods with the unsure flag;
  - the steps for Hera Tani, Lori Jameson and Kit Fowler;
  - a no-progress commander: the common-knowledge engineers are known and the rest locked.
- `unlocks.json` was dumped again, and it is the only fixture that changed. The drift test passes. Bindings were regenerated (`Node.ts`, a new `WorksOn.ts`).

**Window** (`cd3363d5`)
- `View.svelte` puts the search box (with the summary line beside it) above `ListDetail`. `Tree.svelte` draws one kind's tree. `placeTree` in `lib/unlocks.ts` lays it out in catalog order, so each engineer spans the rows of the engineers they introduce. Yi Shen spans the three Colonia introducers. The lines are an SVG drawn from where the nodes are, redrawn on resize. `Trees.svelte` reads `ListDetail`'s measured width: the trees sit side by side while both fit, and otherwise the suit tree goes below. If a selection stacks the trees, the clicked node is scrolled back into view. `Panel.svelte` is the side panel.
- There is no default selection, so at rest there's no panel. Clicking the selected engineer again closes it.
- New design-system sizes in `tokens.css`: `--node` (12.5rem, so two trees of three columns sit side by side at 2560 x 1440 with the sidebar docked) and `.field.page-search` (27.5rem, the search box's width, as before). They are there because the page may no longer set widths of its own. Both are added to DESIGN.md's Token map.
- The page-sizing entry covers every file of the page: `unlocks/{View,Search,Tree,Trees,Panel}.svelte`.
- The window joins the tree to the engineer locations and the Odyssey lines with `engineerKey`, the core's `engineers::key`. Exact names don't match: Tod is 'Tod "The Blaster" McQuinn' in the unlock catalog but "Tod 'The Blaster' McQuinn" in `engineers.json`, so an exact-name join lost his distance. A browser case covers it.
- `#/unlocks?kind=suit` opens the page (`tabrows.browser.test.ts`).
- `engineerSpecialties.ts` is deleted. Nothing else used it.
- I deleted `STATUS_LABEL` and `columns()` from `lib/unlocks.ts`. Nothing else used them.

**Layout at each size, measured with the route followed.** At rest the trees sit side by side at 1920 and 2560. With an engineer selected at 1920 and 2560, the suit tree goes below the ship tree and the panel sits beside them. At 1100 x 700 (the route strip) the suit tree is below and the panel below both, scrolled into view on selection.

**Captures** are in `docs/design/engineers/`:
- Before: `engineers-before-*` and `engineers-suit-before-*`, from `before-plan.json` and `before-report.json`. The old cards had no selection, so the before captures show the page at rest, ship and suit.
- After: `engineers-{rest,selected,selected-unlocked,selected-suit}-after-*` at 1100 x 700, 1920 x 1080 and 2560 x 1440 at 145 percent, plus `engineers-search-after-*` at 1100 and 2560. They come from `plan.json`, with `report.json` beside them.
- Every capture lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `contrast` or `far`. The exempt Grey Dim text is the not-introduced nodes.

**What the `bright` list shows.** The audit lists the brightest text colours by luminance, cut off after the first few. White (the panel title, the current section's heading) and Options Grey (engineer names) are lighter than any orange, so they always lead. The Orange Mid names of the engineers the commander can work on, and the search box's Orange hint, fall below the cut. What the list does show for the page, the sidebar apart:
- **At rest** (2560): no orange text on the page. The only orange is the frame's "Ctrl+K", "Help" and status line, and the only fill is the active top tab.
- **The Dweller selected** (2560): one orange text, the panel's main value in Orange Lit ("Invitation: Deal in stolen or illicit goods…"). The fills are Set destination, the top tab, and the Orange Mid hexagons for the grades The Dweller reaches.
- **Felicity selected** (1920): one orange text, the main value "102" in Orange Lit. The fills are Set destination, the top tab, and the Orange hexagons for the grades her rank 5 gets today.
Besides those, orange appears only in the marks DESIGN.md allows: the grade hexagons (point 5 above), the introducer outlines and their lines (point 3), and the current section's rule under the selected engineer's tree heading.

**Checks**, all run at the final code:
- `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` are clean.
- `cargo test -p astromech-engineering -p astromech-api` passes: 48 and 41 tests. The bindings check passes.
- `pnpm check` and `pnpm lint` are clean. Lint has both page-sizing parts on for the unlocks page's files, and `unlocks` is off `TOKENS_EXEMPT`.
- `pnpm test` passes: 760 tests in 90 files.
- The journeys pass: 7 of 7.
- `previewstates.browser.test.ts`'s Engineers empty-state pattern now matches the new summary.

**Follow-up to file** (`atelier new` refuses on an item branch, so it needs filing from `main`): the browser preview's `?core=empty` (`app/src/preview/shell.ts`, `engineering_unlocks`) sets every engineer to locked. The core gives the common-knowledge engineers as known (new Rust test `with_no_progress_the_common_knowledge_engineers_are_known_and_the_rest_not_introduced`), so the preview's empty state shows the wrong picture. It should come from a dumped empty-world fixture.

