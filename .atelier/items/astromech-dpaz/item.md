---
schema: atelier.item
schema_version: 3
id: astromech-dpaz
title: Exploration > System as a system tree
status: ready
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
implementers: []
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

- [ ] The core gives each body its parents: `BodyView` (`crates/astromech-exploration/src/system.rs:30`, built at line 340) gains `parents`, copied from the journal's `Scan` `Parents` that `ScanBody` already parses (`crates/astromech-journal/src/detail.rs:120-121`: nearest first, `Star`, `Planet`, `Ring` or `Null` for a barycentre, with its BodyID). A table-driven test in `crates/astromech-exploration` covers a moon of a planet of a star, a planet of a barycentre and a body scanned before its parent.
- [ ] Exploration > System is a List with detail page on the type roles. Its head: the system as the Title, White, with its copy icon; "14 bodies, all scanned · 3 worth mapping" in Label (`body_count`, `all_found`, the bodies' `worth_mapping`); the value now in Value XL Options Grey and, as the page's main value, the value with every body worth mapping mapped (`value_if_mapped`) in Value XL Orange Lit, each with its unit in Label and its rule in a Tip.
- [ ] The tree: each body under its nearest parent that is a star or a planet (barycentres are not drawn; a body whose parent is not scanned yet hangs from its nearest scanned ancestor, else the root), stars at the root, children by distance from the arrival star. A Seam line joins each body to its parent, as `docs/design/glow-up-system.png`. Each row: the mapped value (`value_mapped`) right-aligned in Figures with its unit, the distance from arrival in Figures, the body's type icon (the Glyph star and body sets, astromech-dsyu, in Label Grey, its accessible name the class), the short name (`short_name`) in Name Options Grey, the class in Prose Grey, then the tags it has today as badges: Landable, Terraformable, Bio N and Geo N from the signals, First discovery, Worth mapping, Mapped, First footfall likely, Landed N times. A body worth mapping shows its mapped value in Value L, Options Grey; the rest in Figures, Prose Grey. The body-class-to-icon mapping is one function with a table test over the classes in the preview fixture.
- [ ] Selecting a body raises its row and fills the side panel: the short name as title, White, the system under it; the mapped value as the panel's main value in Value XL Orange Lit with the value now and "not mapped yet" or "mapped" in Label; then a fact list: class, distance, gravity, signals (biological and geological), surface (landable, terraformable), discovery (discovered before or not, mapped before or not), landings. A body with biological signals has OPEN IN BIOLOGY, a text action to Exploration > Biology. The mockup's "What to do" paragraphs are not drawn: the badges and the panel's facts say the same without prose.
- [ ] Under the tree, Unsold data and Where to sell stay as they are today in content (the cartographic, organic and total figures from `exploration_unsold`, the commander and since when, and the links to the nearest Universal Cartographics and Vista Genomics), as Board-style blocks with one value each. The risky reminder ("! 48 M cr aboard. Dying loses all of it") stays as the orange "!" line, at the top of the page under the head, since it is what needs the commander.
- [ ] States: no system yet, loading, a failed poll with the Stale line, and `?core=empty` (`Empty.browser.test.ts`) keep today's words; a system with nothing scanned says so in one line and shows no tree.
- [ ] The brightest things: the value if mapped (main value), the selected row and OPEN IN BIOLOGY (what can be changed) and the unsold-data warning (what needs the commander). The tree is not ranked by value, so no orange column; no colour outside DESIGN.md's list (a star icon in the tree is Label Grey; the route's star colours belong to the route).
- [ ] No printed explanations: the value rules and what a badge means go in Tips.
- [ ] The preview's explorer world (`worlds/preview/explorer/world.jsonl`, the exploration crate's synthetic lines) gains `Parents` on every `Scan` and enough bodies to show the tree's shape: a second star, two planets with moons and a barycentre. `app/src/preview/fixtures/exploration.json` is dumped again (`ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-app --test preview_fixtures`); the drift test passes.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with a body selected: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the panel goes below the tree. At 3840 x 2160 the rows keep their own width and the panel sits after them. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/system-tree/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/exploration/View.svelte` in `PAGE_SIZING`, and the `exploration` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: `exploration/View.browser.test.ts`'s cases keep their behaviour on the new markup; one new browser case selects a moon and finds it under its planet with its facts in the panel. The tree-building rule (nearest drawn parent) is a pure function in `app/src/lib/exploration.ts` with a table test beside it.
- [ ] `docs/guide/exploration.md`'s System section describes the tree and the panel.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-exploration` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Exploration > System and the parents in the core. Biology is astromech-evj3.

## Context

Mockup: `docs/design/glow-up-system.png`. Its layout stands; under revision 5 the selected row is Hull Raised with lit text, not filled orange, names and classes are grey, and only the main value is orange. Replies: `SystemView.ts` (`body_count`, `all_found`, `bodies`, `value`, `value_if_mapped`), `BodyView.ts`, `UnsoldData.ts`. The exploration fixture's world is `explorer` (`app/src-tauri/tests/preview_fixtures.rs:545-550`).
