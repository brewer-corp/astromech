---
schema: atelier.item
schema_version: 3
id: astromech-m6gw
title: Engineering > Engineers as the unlock tree
status: draft
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-8e22
- astromech-70ts
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:41.864039859Z
---

## Goal

The engineers read as the tree they are: who introduces whom, where the commander stands with each, and what to do next, for ship and suit engineers side by side. Today the page shows one kind at a time behind a Ship or Suit choice, as cards in columns that state their prerequisites in words (`app/src/modules/unlocks/View.svelte`), and what an engineer works on is two module types from `app/src/lib/engineerSpecialties.ts`, which is incomplete.

## Done when

- [ ] Engineering > Engineers is a List with detail page on the type roles, laid out as `docs/design/nav-spike/tone5-engineers-rest-2560.png`: the search box astromech-7did built on top with the summary line beside it ("10 of 25 ship and 4 of 13 suit engineers unlocked · 15 to work on", counted from the tree), then SHIP ENGINEERS and SUIT ENGINEERS as two trees side by side. The Ship and Suit choice goes; `#/unlocks?kind=suit` (the old On foot link, `App.svelte`'s `ON_FOOT`) still opens the page. Colonia engineers sit in the same trees, where their introducers put them, as the capture shows.
- [ ] Each tree's columns follow `Node.depth`, with a Seam line from each engineer to each one in `Node.introduced_by` (`engineering_unlocks`). A node is a Seam outline on Void, never a cell: its name in Name, then one line. By DESIGN.md's Engineer state entry: unlocked, the name Options Grey with the rank as White hexagons (`Hexes.svelte`, `tone="rank"`; a suit engineer, who has no ranks, shows one) and the system in Label; one the commander can work on now (`Node.next`, not unlocked), the name Orange Mid and the step left (`Node.left`) in Prose Grey; barred, the name and reason Red Low; not introduced, the name and "Not introduced" Grey Dim. Steps are words, as the capture has them: the mockup's progress counts ("2 of 5 black markets") are not shown, since `Node.left` is the step's text and nothing in the journal counts them.
- [ ] Selecting a node raises it and fills the side panel: the engineer's name as its title, White; the system with its copy icon and the distance from the commander's current system in Label; the status in words; the steps to unlock, numbered, from `Node.left`; WORKS ON, each module type (ship) or modification (suit) they offer with the best grade as hexagons; who they introduce (the nodes naming them in `introduced_by`); for a suit engineer, their referral task (`EngineerLine.referral_task`) and the mods the wiki is unsure of marked "(?)" with its Tip, as the cards show today. Set destination is the panel's primary button, with Add stop beside it.
- [ ] The core gives each `Node` what the engineer works on: for a ship engineer, each module group they offer with the best grade, worked out as `engineering_search` does (the max of `engineers::offers` over the group's blueprints, `crates/astromech-engineering/src/engineers.rs:50`) and labelled with the search table's short labels (`data/catalog/engineer-search.json`); for a suit engineer, their mods from `odyssey.json`. Table-driven Rust tests in `crates/astromech-engineering` cover a ship engineer's groups and grades and a suit engineer's mods, and `app/src/preview/fixtures/unlocks.json` is dumped again; the drift test passes. `engineerSpecialties.ts` goes if nothing else uses it.
- [ ] Distance is worked out in the window from the commander's position and each engineer's coordinates: ship engineers' from `engineering_engineers` (`EngineerView.coords`, `Engineers.here`), suit engineers' from `odyssey_view`'s `EngineerLine.coords` (astromech-70ts). With no position in the journal the distance is left out.
- [ ] While the box holds a search the answer replaces the trees, and clearing it brings them back, as astromech-7did built; the search keeps its own rows and hexagons.
- [ ] States: loading one Label Grey line; a failed `engineering_unlocks` read is the shared Failure line with TRY AGAIN, as today; a failed `odyssey_view` leaves the suit nodes without their mods, as today; a commander with no engineer progress in the journal sees every engineer as the common-knowledge ones known and the rest not introduced.
- [ ] The brightest things: the selected engineer's distance or step (the panel's main value, Orange Lit), the search box, Set destination and the engineers the commander can work on now in Orange Mid (what can be changed and what needs the commander). Nothing else is orange; no colour outside DESIGN.md's list.
- [ ] No printed explanations: what each state means is in the nodes' Tips and the guide.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `unlocks` world: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Where the two trees do not fit side by side beside the panel, the suit tree goes below the ship tree and the panel below both (1100 x 700); text never shrinks. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/engineers/`, with a node selected. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small` or `far`, or the Report says why a listing is not a fault.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/unlocks/{View,Search}.svelte` in `PAGE_SIZING`, and the `unlocks` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: the cases in `unlocks/View.browser.test.ts` keep their behaviour on the new markup, "lights the introducing engineer on hover" becomes selection lighting the introducers, and the suit mods case reads the panel; a new browser case selects an engineer and finds their works-on grades and steps from the recorded replies. No test of layout or copy.
- [ ] `docs/guide/engineering.md`'s The tree section describes both trees, the states and the panel.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-engineering` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The tree, its side panel and what each engineer works on. The search and the hexagon component are astromech-7did's; the suit engineers' coordinates are astromech-70ts's.

## Context

Approved capture: `docs/design/nav-spike/tone5-engineers-rest-2560.png` (layout and colour). It shows no side panel and no narrow window; the side panel follows `docs/design/glow-up-engineers.png`'s, in revision 5's colours (no filled selection: the selected node is Hull Raised). `glow-up-engineers.png` is otherwise superseded: its Ship or Suit choice and its filled nodes go.

Replies: `Node.ts` (`name`, `kind`, `system`, `permit`, `depth`, `introduced_by`, `status`, `rank`, `left`, `next`), served cached by `engineering_unlocks` (`crates/astromech-api/src/commands.rs:140`), which is why distance is worked out in the window rather than in each node; `EngineerView.ts` and `Engineers.ts`; `EngineerLine.ts` (`mods`, `referral_task`, `refers`).
