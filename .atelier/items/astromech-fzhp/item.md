---
schema: atelier.item
schema_version: 3
id: astromech-fzhp
title: Session and Logbook as time
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
- astromech-coq6
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:44.022030945Z
---

## Goal

A session reads as what it earned over its length, by activity, and the logbook reads as travel by day. Today Session is four stat boxes, a table of credits by activity and a bullet list (`app/src/modules/session/View.svelte`), and the Logbook is a places table and a timeline table of timestamps (`app/src/modules/logbook/View.svelte`).

## Done when

- [ ] The core gives the session's earnings over time: `Session` (`crates/astromech-history/src/session.rs:29`) gains `timeline`, the session's payments in time order, each with its time, kind and amount, read from the same `earnings` table the totals come from (`session.rs`, the `SELECT kind, sum(amount)` query; every row there has its `ts`). A table-driven test in `crates/astromech-history` covers a session with payments of three kinds and one that spans a later `LoadGame` (left out). `app/src/preview/fixtures/session.json` is dumped again; the drift test passes.
- [ ] Session, under NOW since astromech-coq6, is a Board page on the type roles. The head: "Latest session" or "Session" as the Title, with the commander, start and end and length in Label. The page's main value: credits per hour (`per_hour`) in Value XL Orange Lit; beside it what was earned (`total`, its rule in the `EARNED` Tip), jumps and distance in Value XL Options Grey.
- [ ] **Over time**: one horizontal band across the session's length, start to now, with a mark at each payment's time, each activity its own row of marks labelled at its left (as many rows as activities, in the totals' order); hovering a mark gives its time, kind and amount in a Tip. The marks are Prose Grey. No axis labels beyond the start and end times in Label.
- [ ] **By activity**: each kind (`earnings`) one row: its name (`EARNING_NAMES`) in Name, the amount as the ranked value in Value L Orange, the payments count in Label, and for trade its profit in Label. Data sold, bounties awarded and carrier jumps (today's "Also" list) join the head's Label line or this list as rows, each with its Tip as today; none is dropped.
- [ ] Logbook stays under LOGBOOK, one page, on the Board layout and the type roles, its two lists side by side where they fit and the places above the days where they do not. Its search box and the totals line stay at the top. The timeline (`timeline`, newest first, at most 200 entries as the command asks, `crates/astromech-api/src/commands.rs:195`) is grouped by day: a day's date as a Section heading with the day's jumps and dockings counted in Label, then each arrival or docking as one row: the time in Figures, the system with its copy icon (`SystemChip`) and the station in Name when docked, the jump distance in Label. The places matching the search (`places`) are the list's other half, each place one two-line row: the name, then its visits and first and last visit in Label. Rows are read-only and not selectable: there is nothing more to show about a place.
- [ ] States: no session yet, reading the history, a failed poll (the Stale line, or the words with no action) and nothing earned keep today's words; the logbook's import progress line and its error line stay; no match says so as today.
- [ ] The brightest things: Session's credits per hour (main value) and no control (nothing changes on Session, "none"); the Logbook's search box (what can be changed) with no main value beyond the totals (Value L Options Grey) and nothing that needs the commander. Orange for the ranked amounts, the main value and the search only; no colour outside DESIGN.md's list.
- [ ] No printed explanations: each figure's rule in its Tip.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `session` and `logbook` fixtures: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Before-and-after captures of both pages at 1100 x 700 and 2560 x 1440 in `docs/design/session-logbook/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/{session,logbook}/View.svelte` in `PAGE_SIZING` (font size is on for Session today), and the `{session,logbook}` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: `session/View.browser.test.ts` and `logbook/View.browser.test.ts` keep their cases on the new markup; one new case finds the session's payments on the band from the recorded reply. Grouping the timeline by day is a pure function with a table test, its dates in UTC as the page shows them. No test of layout or copy.
- [ ] `docs/guide/now.md`'s Session section (moved there by astromech-coq6) and `logbook.md`'s Logbook section describe the pages.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-history` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The Session and Logbook pages and the session's payment times in the core. The Now page's session block (astromech-coq6) reads the same reply's totals.

## Context

There is no mockup. Replies: `session_summary` (`Session.ts`: `earnings: EarningTotal[]` with `kind`, `amount`, `count`; `total`, `trade_profit`, `per_hour`, `jumps`, `distance_ly`, `carrier_jumps`, `bounties_awarded`, `bounty_value`, `data_sold`), `logbook` (`Logbook.ts`: `totals`, `places: Place[]`, `timeline: Visit[]` with `ts`, `kind`, `star_system`, `station_name`, `jump_dist`, `via`). The session fixture's world records eight payments of eight kinds in one 30-minute session (`app/src/preview/fixtures/session.json`); if the band needs more to show its shape, the `history` world gains payments from recorded journal lines and the fixture is dumped again.
