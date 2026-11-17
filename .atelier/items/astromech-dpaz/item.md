---
schema: atelier.item
schema_version: 3
id: astromech-dpaz
title: Exploration > System as a system tree
status: approved
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Ac25b2190-40b8-48d3-ad05-1e0cae8078c5
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:39.699712044Z
---

## Goal

The bodies of the current system read as the system is built: stars, planets and moons in a tree, each with its body icon and its mapped value, the ones worth mapping standing out, and the selected body's facts beside them. Today it is a flat table of every scanned body in scan order (`app/src/modules/exploration/View.svelte`), which at 3840 x 2160 spreads across the whole width with about 640 px between a body's tags and its value (increment 1 evidence, mission notes, 12 Nov 2026).

## Done when

- [x] The core gives each body its parents: `BodyView` (`crates/astromech-exploration/src/system.rs:30`, built at line 340) gains `parents`, copied from the journal's `Scan` `Parents` that `ScanBody` already parses (`crates/astromech-journal/src/detail.rs:120-121`: nearest first, `Star`, `Planet`, `Ring` or `Null` for a barycentre, with its BodyID). A table-driven test in `crates/astromech-exploration` covers a moon of a planet of a star, a planet of a barycentre and a body scanned before its parent.
- [x] Exploration > System is a List with detail page on the type roles. Its head: the system as the Title, White, with its copy icon; "14 bodies, all scanned · 3 worth mapping" in Label (`body_count`, `all_found`, the bodies' `worth_mapping`); the value now in Value XL Options Grey and, as the page's main value, the value with every body worth mapping mapped (`value_if_mapped`) in Value XL Orange Lit, each with its unit in Label and its rule in a Tip.
- [x] The tree: each body under its nearest parent that is a star or a planet (barycentres are not drawn; a body whose parent is not scanned yet hangs from its nearest scanned ancestor, else the root), stars at the root, children by distance from the arrival star. A Seam line joins each body to its parent, as `docs/design/glow-up-system.png`. Each row: the mapped value (`value_mapped`) right-aligned in Figures with its unit, the distance from arrival in Figures, the body's type icon (the Glyph star and body sets, astromech-dsyu, in Label Grey, its accessible name the class), the short name (`short_name`) in Name Options Grey, the class in Prose Grey, then the tags it has today as badges: Landable, Terraformable, Bio N and Geo N from the signals, First discovery, Worth mapping, Mapped, First footfall likely, Landed N times. A body worth mapping shows its mapped value in Value L, Options Grey; the rest in Figures, Prose Grey. The body-class-to-icon mapping is one function with a table test over the classes in the preview fixture.
- [x] Selecting a body raises its row and fills the side panel: the short name as title, White, the system under it; the mapped value as the panel's main value in Value XL Orange Lit with the value now and "not mapped yet" or "mapped" in Label; then a fact list: class, distance, gravity, signals (biological and geological), surface (landable, terraformable), discovery (discovered before or not, mapped before or not), landings. A body with biological signals has OPEN IN BIOLOGY, a text action to Exploration > Biology. The mockup's "What to do" paragraphs are not drawn: the badges and the panel's facts say the same without prose.
- [x] Under the tree, Unsold data and Where to sell stay as they are today in content (the cartographic, organic and total figures from `exploration_unsold`, the commander and since when, and the links to the nearest Universal Cartographics and Vista Genomics), as Board-style blocks with one value each. The risky reminder ("! 48 M cr aboard. Dying loses all of it") stays as the orange "!" line, at the top of the page under the head, since it is what needs the commander.
- [x] States: no system yet, loading, a failed poll with the Stale line, and `?core=empty` (`Empty.browser.test.ts`) keep today's words; a system with nothing scanned says so in one line and shows no tree.
- [x] The brightest things: the value if mapped (main value), the selected row and OPEN IN BIOLOGY (what can be changed) and the unsold-data warning (what needs the commander). The tree is not ranked by value, so no orange column; no colour outside DESIGN.md's list (a star icon in the tree is Label Grey; the route's star colours belong to the route).
- [x] No printed explanations: the value rules and what a badge means go in Tips.
- [x] The preview's explorer world (`worlds/preview/explorer/world.jsonl`, the exploration crate's synthetic lines) gains `Parents` on every `Scan` and enough bodies to show the tree's shape: a second star, two planets with moons and a barycentre. `app/src/preview/fixtures/exploration.json` is dumped again (`ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-app --test preview_fixtures`); the drift test passes.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with a body selected: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the panel goes below the tree. At 3840 x 2160 the rows keep their own width and the panel sits after them. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/system-tree/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/exploration/View.svelte` in `PAGE_SIZING`, and the `exploration` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: `exploration/View.browser.test.ts`'s cases keep their behaviour on the new markup; one new browser case selects a moon and finds it under its planet with its facts in the panel. The tree-building rule (nearest drawn parent) is a pure function in `app/src/lib/exploration.ts` with a table test beside it.
- [x] `docs/guide/exploration.md`'s System section describes the tree and the panel.
- [x] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-exploration` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Exploration > System and the parents in the core. Biology is astromech-evj3.

## Context

Mockup: `docs/design/glow-up-system.png`. Its layout stands; under revision 5 the selected row is Hull Raised with lit text, not filled orange, names and classes are grey, and only the main value is orange. Replies: `SystemView.ts` (`body_count`, `all_found`, `bodies`, `value`, `value_if_mapped`), `BodyView.ts`, `UnsoldData.ts`. The exploration fixture's world is `explorer` (`app/src-tauri/tests/preview_fixtures.rs:545-550`).

## Report

Exploration > System is now a tree on List with detail. Commits f70045d3 (core) and 4c8824b0 (page).

**Core.** `BodyView.parents` is a list of `BodyParent { kind, body_id }` copied from the scan.
`each_body_carries_its_parents` in `crates/astromech-exploration/tests/system.rs` covers:

- a moon of a planet of a star, scanned before its planet;
- a planet of a barycentre;
- a body in a ring.

**Tree and icons.** Both are pure functions in `app/src/lib/exploration.ts`, with table tests in
`exploration.test.ts`.

- `bodyTree`: stars sit at the root. Every other body hangs from the first `Star` or `Planet` in
  its parents that has been scanned, and goes to the root if there is none. Children are ordered
  by distance, then by name. A loop in bad parent data still draws every body once.
- `bodyGlyph`: stars go through the route's `starKind` table, scoopable for the KGBFOAM classes
  and giants. Planets map by class, and an unknown class draws as rocky.

**World.** The explorer world keeps its five bodies and their IDs, since the landings and samples
point at ID 5. It adds six: a second star B (A and B orbit barycentre 0), A 1, the moon A 3 a,
B 1 with its moon B 1 a, and AB 1 circling the barycentre. Every scan now has `Parents`. The new
fixture has 11 bodies, 3 of them worth mapping. Unsold is 38.7 M cr, still under the 50 M cr
risky mark, so the existing "no reminder" case keeps its behaviour. The world is now a superset of
the crate's `unexplored()` lines, and the fixture test's comment says so.

**Kept, though the item's badge list doesn't name them.** Dropping any of these would remove
something the page shows today:

- The Rings and Belt badges. The type icon follows the class, not the rings, because its
  accessible name is the class.
- "Efficiently": the Mapped badge's Tip says whether mapping was within the efficiency target, and
  the panel says "mapped, over the efficiency target" when it was not.
- The `source` label ("Sample system") at the end of the head's Label line.

**Badge Tips.** In the tree, each badge's Tip uses `stop: false`, so the treegrid keeps one tab
stop (the sample has 25 badges) and screen readers get the meaning through `aria-describedby`.
The keyboard reaches the meanings in the panel: it shows the selected body's tags again, each
with a Tip that keeps its tab stop, so Tab from the selected row lands on the first tag. The
column head's Tip and the head values' Tips keep their tab stops too.

**Selection.** The selection is held by body name, so it survives the 3 s poll. A change of
system (address or name) clears it, so returning to a system starts with nothing selected.
Nothing is selected at first. The tree is a `treegrid`: rows carry `aria-level`, and the
arrow keys, Home and End move the selection.

**Layout.** Unsold data and Where to sell are rendered after the ListDetail, not inside it. At
1100 the stacked panel then sits right under the tree instead of after the unsold tiles.
The panel's threshold is the tree's measured natural width: its value, distance
and body tracks plus its widest row of tags on one line. The panel goes below the tree before
any tag would wrap. The body name keeps
a common start for classes within a level with `min-width: var(--sp-12)`.

**Captures and audit.** The plan and the captures are in `docs/design/system-tree/`:

- `plan.json` covers the tree with A 4 a selected at 1100, 1659 (the dock width), 1920, 2560 at
  145 percent and 3840 at 180 percent. It also covers the unselected tree, loading, failed and
  `?core=empty`.
- `before-*.png` is today's page on the new world, taken before the page changed. `after-*.png`
  is the new page. Both are at 1100 x 700 and 2560 x 1440.
- Every capture lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `contrast` or
  `far`.
- At 1100 the frame is the strip and the panel goes below the tree. At 1920 and 2560 the sidebar
  is docked and the panel sits beside the tree. At 3840 the rows keep their own width and the
  panel follows them.
- The page's `bright` list, the sidebar apart, ranks by luminance. It leads with White: the
  wordmark, the system title and the selected body's name, which DESIGN.md makes White. Then come
  Options Grey names and figures (the value now, the names, the worth-mapping values) and Prose
  Grey data. Among the orange, Orange Lit holds only the main value (10,510,589 cr if mapped) and
  the panel's value. Orange holds OPEN IN BIOLOGY and the two sell links.
- By luminance White and grey always outrank orange, so this ordering is not a fault. The selected
  row is the only Hull Raised row. The tree has no orange column because it is not ranked. This is
  the reading astromech-rj6i's Report gave for Powerplay.
- `report.json` is from 9258322e, the page with round 1's fixes.
- The sample is under the risky mark, so nothing on the page needs the commander. The orange
  reminder line is checked by its browser test, not in a capture.

**Tags never wrap beside the panel.** Measured in the sample, in px at 100 percent Text size,
with A 4 a selected and the sidebar docked:

| Window | Tree's natural width | Room beside the panel | Result |
|---|---|---|---|
| 1100 x 700 | 947 | 641 | panel below the tree |
| 1659 x 900 (the dock width) | 947 | 856 | panel below the tree, no tag wraps |
| 1920 x 1080 | 947 | 1,117 | panel beside the tree |
| 2560 x 1440 at 145 percent | 943 | 966 | panel beside the tree, 23 px to spare |

A system with a longer class name or more tags on its busiest row moves the panel below the tree
at a wider window, since the threshold is measured, not fixed. Tags wrap only on a stacked page
narrower than the tree itself (a large Text size in a small window), rather than scrolling
sideways. So this page keeps its panel beside the rows at 1920 and 2560 as the item asks. At the
dock width, unlike the finders, its panel goes below.

**Other changes.**

- `eslint-rules/pageSizing.test.js` used Exploration as its example of a page not yet enrolled.
  It now uses Station, which astromech-7vxi has yet to rebuild.
- In `stale.browser.test.ts`, the Exploration entry matches the new head line.

**Checks.** All of these pass:

- `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test -p astromech-exploration`, the `preview_fixtures` drift test, the core's
  `bindings` test and `copy`;
- `cargo test -p astromech-api`;
- `pnpm check`, `pnpm lint` and `pnpm test` (802 tests, at 9258322e). At load average 52, an
  earlier full run failed three timing-sensitive cases in `App.browser.test.ts` and
  `tabrows.browser.test.ts`. This change does not touch either file, and both passed alone and in
  the later full runs;
- `pnpm exec tsc -p journeys` and `node --test 'journeys/*.journey.ts'` (7 of 7).

### Responses

- 1-flbt.1: fixed in 9258322e. The panel repeats the selected body's tags with Tips that keep their tab stops, and Tab from the selected row reaches them (tested in the moon case).
- 1-flbt.2: fixed in 9258322e. The panel's threshold is the tree's measured natural width, so at 1659 x 900 it stacks and no tag wraps (captures and audit in 8c3f8b47).
- 1-flbt.3: fixed in 9258322e. A change of system clears the selection, with a browser test that failed before the fix.
