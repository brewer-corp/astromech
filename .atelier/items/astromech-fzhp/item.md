---
schema: atelier.item
schema_version: 3
id: astromech-fzhp
title: Session and Logbook as time
status: changes_requested
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
- astromech-coq6
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A58efe55c-574f-4b31-90f6-99f31f7acc75
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:44.022030945Z
---

## Goal

A session reads as what it earned over its length, by activity, and the logbook reads as travel by day. Today Session is four stat boxes, a table of credits by activity and a bullet list (`app/src/modules/session/View.svelte`), and the Logbook is a places table and a timeline table of timestamps (`app/src/modules/logbook/View.svelte`).

## Done when

- [x] The core gives the session's earnings over time: `Session` (`crates/astromech-history/src/session.rs:29`) gains `timeline`, the session's payments in time order, each with its time, kind and amount, read from the same `earnings` table the totals come from (`session.rs`, the `SELECT kind, sum(amount)` query; every row there has its `ts`). A table-driven test in `crates/astromech-history` covers a session with payments of three kinds and one that spans a later `LoadGame` (left out). `app/src/preview/fixtures/session.json` is dumped again; the drift test passes.
- [x] Session, under NOW since astromech-coq6, is a Board page on the type roles. The head: "Latest session" or "Session" as the Title, with the commander, start and end and length in Label. The page's main value: credits per hour (`per_hour`) in Value XL Orange Lit; beside it what was earned (`total`, its rule in the `EARNED` Tip), jumps and distance in Value XL Options Grey.
- [x] **Over time**: one horizontal band across the session's length, start to now, with a mark at each payment's time, each activity its own row of marks labelled at its left (as many rows as activities, in the totals' order); hovering a mark gives its time, kind and amount in a Tip. The marks are Prose Grey. No axis labels beyond the start and end times in Label.
- [x] **By activity**: each kind (`earnings`) one row: its name (`EARNING_NAMES`) in Name, the amount as the ranked value in Value L Orange, the payments count in Label, and for trade its profit in Label. Data sold, bounties awarded and carrier jumps (today's "Also" list) join the head's Label line or this list as rows, each with its Tip as today; none is dropped.
- [x] Logbook stays under LOGBOOK, one page, on the Board layout and the type roles, its two lists side by side where they fit and the places above the days where they do not. Its search box and the totals line stay at the top. The timeline (`timeline`, newest first, at most 200 entries as the command asks, `crates/astromech-api/src/commands.rs:195`) is grouped by day: a day's date as a Section heading with the day's jumps and dockings counted in Label, then each arrival or docking as one row: the time in Figures, the system with its copy icon (`SystemChip`) and the station in Name when docked, the jump distance in Label. The places matching the search (`places`) are the list's other half, each place one two-line row: the name, then its visits and first and last visit in Label. Rows are read-only and not selectable: there is nothing more to show about a place.
- [x] States: no session yet, reading the history, a failed poll (the Stale line, or the words with no action) and nothing earned keep today's words; the logbook's import progress line and its error line stay; no match says so as today.
- [x] The brightest things: Session's credits per hour (main value) and no control (nothing changes on Session, "none"); the Logbook's search box (what can be changed) with no main value beyond the totals (Value L Options Grey) and nothing that needs the commander. Orange for the ranked amounts, the main value and the search only; no colour outside DESIGN.md's list.
- [x] No printed explanations: each figure's rule in its Tip.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `session` and `logbook` fixtures: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Before-and-after captures of both pages at 1100 x 700 and 2560 x 1440 in `docs/design/session-logbook/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/{session,logbook}/View.svelte` in `PAGE_SIZING` (font size is on for Session today), and the `{session,logbook}` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: `session/View.browser.test.ts` and `logbook/View.browser.test.ts` keep their cases on the new markup; one new case finds the session's payments on the band from the recorded reply. Grouping the timeline by day is a pure function with a table test, its dates in UTC as the page shows them. No test of layout or copy.
- [x] `docs/guide/now.md`'s Session section (moved there by astromech-coq6) and `logbook.md`'s Logbook section describe the pages.
- [x] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-history` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The Session and Logbook pages and the session's payment times in the core. The Now page's session block (astromech-coq6) reads the same reply's totals.

## Context

There is no mockup. Replies: `session_summary` (`Session.ts`: `earnings: EarningTotal[]` with `kind`, `amount`, `count`; `total`, `trade_profit`, `per_hour`, `jumps`, `distance_ly`, `carrier_jumps`, `bounties_awarded`, `bounty_value`, `data_sold`), `logbook` (`Logbook.ts`: `totals`, `places: Place[]`, `timeline: Visit[]` with `ts`, `kind`, `star_system`, `station_name`, `jump_dist`, `via`). The session fixture's world records eight payments of eight kinds in one 30-minute session (`app/src/preview/fixtures/session.json`); if the band needs more to show its shape, the `history` world gains payments from recorded journal lines and the fixture is dumped again.

## Report

**Session now leads with credits per hour and shows when each payment came in, and the Logbook reads as travel by day.** Both pages are on the Board layout and the type roles, with page sizing on and no tokens exemption.

### Core (own commit, 5165eee4)

- **`Session.timeline`:** `Vec<Payment>` (`ts`, `kind`, `amount`), read from the `earnings` table over the same `[start, next LoadGame)` window as the totals, ordered by `ts, event_id`. `Payment` is exported from `astromech-history` and to `app/src/lib/api/types/Payment.ts`.
- **Test:** `session_timeline_lists_its_payments_in_time_order` is table-driven: three kinds in one session (trade, mission, bounty), and a session followed by a later `LoadGame` whose payment is left out. Each case also checks that the timeline sums to `total`.
- **Fixture:** `session.json` was dumped again and gains `timeline`. The drift test passes.

### Session

- **Head:** "Latest session" or "Session" as the Title, then the commander, start to end UTC, length and source in Label.
- **Values:** credits per hour in Value XL Orange Lit, then earned (the `EARNED` Tip), jumps and distance in Value XL Options Grey. Each figure's rule is in its Tip.
- **The old "Also" list:** data sold, bounties awarded with their value, and carrier jumps (when there are any) are now a Label line under the values. Each keeps its Tip, and none is dropped.
- **Layout:** By activity and Over time sit side by side where the page is at least 62rem wide. Below that, By activity comes first, so the ranked list shows at 1100 x 700. The item names Over time first; I put the ranked list ahead because it carries the orange.
- **By activity:** each kind is a row, aligned in a subgrid: the name in Name, the amount in Value L Orange, then the payments count and, for trade, the profit (with its Tip) in Label.
- **Over time:** one row per activity in the totals' order, labelled at its left. Each payment is a 2 px Prose Grey tick on a Seam baseline, and the start and end clock times sit under the band in Label.
- **Mark Tips:** each mark's Tip holds the amount, kind and time. Its accessible name says the same (`role="img"`), so the tip uses `stop: false` and a long trading session doesn't add hundreds of tab stops. The hover target is wider than the tick.
- **Where the band ends:** it runs from `start` to the reply's `end` (the session's last event), not the wall clock. For a live session the two are nearly the same, but the fixture is from September, and a wall-clock end would crush every mark against the left edge.
- **Fixture (own commit, 1508251c):** the recorded world had one payment of each of eight kinds, so no row of the band held more than one mark. `worlds/preview/history` (the preview's copy of the history crate's journals; nothing ties the two) gains four payments between 10:26 and 10:29: two trade sales, an exploration sale and a combat bond. The fixture was dumped again. Trade, exploration and combat bonds now show two or three marks each. Now's session block reads 14.6 in place of 14.5, and the tests follow the new recorded figures.
- **Mark Tip:** hovering the last mark in the preview shows "5,000 cr / Other vouchers / 10:17 UTC".
- **Payments in one minute share a mark** (round 1, 7dbbb57c): `bandRows()` in `lib/logbook.ts` makes one mark per kind per minute, the precision the Tip shows, with a table test. A shared mark's Tip gives the total, the count and each amount ("2,500 cr / Trade, 2 payments: 1,500 + 1,000 cr / 10:18 UTC"), and its accessible name lists them. The preview world records a second sale at 10:18:00 (hydrogen fuel, 1,000 cr) to cover it from journal input, so trade is 11,500 cr from 4 payments and per hour 14,643,677.
- **Errors:** a failed first read keeps today's words, now in Alarm Red with "!" as Now shows it. Before, it was orange, which this item no longer allows. A later failed read keeps the Stale line.

### Logbook

- **Head:** the commander, "(Legacy)" and source in Label; the totals in Value L Options Grey with their units; the import progress line, or its error line in Alarm Red with "!"; then the Search box.
- **Two lists:** Places and the days sit side by side from 58rem of page width; below that, Places comes first.
- **Places:** each place is a two-line row. Line 1 is the name in Name, with the system's copy icon; a station adds its system in Label before the icon. Line 2 is the visits, first and last in Label. Rows are not selectable and have no hover. "Showing the N most recent of M", "No visits yet." and the no-match line are kept.
- **Days:** each day is a Section heading with the date and the day's jumps and dockings in Label (with a Tip saying what they count). Under it, one row per arrival or docking: the time in Figures, the system (or the station, then the system in Label) with its copy icon, and the jump distance right-aligned in Label, or "carrier jump". "Nothing yet." is kept.
- **`byDay()`:** in `lib/logbook.ts`, a pure function with a table test in `lib/logbook.test.ts`. It groups by the UTC date of the journal timestamp. A day counts FSD jumps only, as Session does, so carrier jumps and `Location` logins are left out. The counts cover only the rows shown (at most 200, narrowed by the search), so the oldest day can be partial; the Tip says so.

### Checks

- **Captures:** before captures at 1100 x 700 and 2560 x 1440 are `before-*.png`, from `plan-before.json` into `before-report.json`. After captures come from `plan.json` into `report.json`. Session, logbook, a search match and no match are taken at 1100 x 700, 1920 x 1080 and 2560 x 1440 (step 145). No session, no commander (`core=empty`) and a failed read of each page are taken at 1100 x 700.
- **Route frame:** the route is the strip at 1100 and docked at 1920 and 2560.
- **Audit:** every capture lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast`.
- **Long names:** the day rows' place column is `minmax(0, max-content)`, so a long station and system name wraps (own commit, 8eeab708). I checked this with a 39-character settlement and a procedural system name set in `logbook.json` locally, then reverted: at 1100 x 700 the audit listed nothing.
- **`bright`, Session:** with the sidebar apart, it leads with White, which is the page Title and the wordmark. Then come the Options Grey figures and names, and among the colours the main value (14,643,677, Orange Lit) is first. There is no control on Session.
- **`bright`, Logbook:** the search box is the only control. Its orange placeholder and ember cell are the only orange on the page besides the frame's. There is no main value beyond the totals, in Options Grey.
- **Found, not fixed (frame):** at 145 percent Text size in a 1100 px window, the header's top-tab row runs 292 px past the window on every page I tried (Session, Logbook, Powerplay, Settings), so the document scrolls sideways. A first run never picks 145 for a 1080p display, and no Done when names that size. It needs an item for the header; I could not file one from this branch, since planning happens on `main`.
- **Tool change (own commit, fff59fa3):** `scripts/browser/audit.mjs` gains a `{ "type": [css, text] }` action, because the plan could only click and wait and so could not capture a search.

### Tests and gates

- **Browser tests:** `session/View.browser.test.ts` keeps its totals case on the new markup and adds one case that finds each recorded payment on the band, at its time and in totals order. `logbook/View.browser.test.ts` keeps both cases. Now's `#/session` test waits on "Latest session" in place of "Earned".
- **Gates, all passing:** `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test` for `-p astromech-history`, `-p astromech-core --lib bindings`, `-p astromech-api` and `-p astromech-app --test preview_fixtures`. Then `pnpm lint`, `pnpm check`, `pnpm test` (96 files, 850 tests) and the journeys (7 of 7), run again after the last commit. `report.json` is from the final pages.

### After approval

- **Merged main (6e502110):** `app/eslint.config.js` conflicted. Main had enrolled Station (astromech-7vxi), Engineers (astromech-m6gw) and System (astromech-dpaz) in `PAGE_SIZING` and dropped their `TOKENS_EXEMPT` entries, and this branch had done the same for Session and Logbook. The resolution keeps every enrolment and drops all four finished exemptions.
- **Test fix that main needed (78186117):** `eslint-rules/pageSizing.test.js` named `station/View.svelte` as its example of a page not yet enrolled, so it failed on main once astromech-7vxi enrolled that page. Its example is now `stored/View.svelte`, which is still waiting on astromech-gvy0.
- **Gates on the merged head:** fmt, clippy, the history, bindings and fixture drift tests, lint, check, `pnpm test` (97 files, 890 tests) and the 7 journeys all pass.

### Responses

- 1-ohh1.1: fixed in 7dbbb57c
