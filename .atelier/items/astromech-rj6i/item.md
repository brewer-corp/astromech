---
schema: atelier.item
schema_version: 3
id: astromech-rj6i
title: Powerplay on one screen
status: changes_requested
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A1aa6ab70-cdbb-4f08-8e4d-7c8220f7a4d4
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:42.585058701Z
---

## Goal

Powerplay answers three questions at a glance: how far to the next rank, how long the cycle has left, and what this system wants. Today the standing is a label and value list with the merits to go as one figure among many, the system's reinforcement and undermining are two numbers in a sentence, and the four panels are laid out by a grid of fixed column widths (`app/src/modules/powerplay/View.svelte`).

## Done when

- [x] astromech-lf3a is fixed (taken over from the checkpoint): with the route sidebar docked, no system name or state on Powerplay is cut or ends in an ellipsis. `node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only powerplay` (the checkpoint's tool, from astromech-8e22) lists nothing under `clipped` at 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) or 3840 x 2160 (180 percent) (it listed 10 at 1920 and 3 at 3840 at the checkpoint), and the 1920 and 3840 captures show This system's name and every Where to work row's system and state whole.
- [x] The core gives the start of the current rank: `NextRank` (`crates/astromech-powerplay/src/standing.rs:63`) gains `from`, the lifetime merits the current rank needed, from the same rank table as `merits` (`rules().merits_for(rank)`, `standing.rs:197-206`, `data/catalog/powerplay.json`). A row in the crate's existing standing tests covers it, and `app/src/preview/fixtures/powerplay.json` is dumped again; the drift test passes.
- [x] Powerplay is a Board page on the type roles, laid out as `docs/design/glow-up-powerplay.png`. The head: the power's name as the Title, White, with "Pledged 21 weeks · rank 12" in Label (`time_pledged_secs`, `rank`).
- [x] **Next rank**: the merits to go (`next.to_go`) as the page's main value in Value XL Orange Lit with "merits to rank 13" in Label; under it a meter from `from` to `merits` filled to the lifetime merits, Prose Grey on Mark Off; "Rank 13 needs 79,000 · reward" in Label. Rank 1's note (`next.note`) replaces the figure while it applies.
- [x] **This cycle**: the time left until `week_ends` in Value XL Options Grey ("2 d 14 h left"), the end in Label in the commander's words ("Ends Thursday 07:00 UTC").
- [x] **Your merits**: this cycle's merits (`week_merits`) in Value XL Options Grey; this session's (`session_merits`) and the lifetime total (`merits`) in Label. The "weekly assignment rewards aren't logged" note stays a Tip on the figure.
- [x] **This system**: the system's name with its copy icon and ExtLinks, its state and controlling power in Label; for a controlled system, reinforcement against undermining as one bar, the reinforcement part Prose Grey and the undermining part Label Grey, split by the two figures (`reinforcement`, `undermining`), with both figures under it and one Label line saying which is ahead and by how much; the control progress through the state's band (`control_progress`) as a thin meter with its percentage and its Tip. Unoccupied: each power's acquisition progress as a meter with its percentage. Other powers active (`contesting`) in Label. "As of your last jump" in a Tip on the system's name.
- [x] **What earns merits here**: the suggestions as a compact list, each name in Name with Ethos, Weak spot and On foot as badges and its rule (`how`) in its Tip, as `Suggestions.svelte` does; a rule not checked in game keeps its "?" with the reason in the Tip (astromech-c1sq and astromech-66pq check them). The rules' source and date stay one Label line with its Tip.
- [x] **Where to work**: `Targets.svelte` as today (its Choice of Acquire, Reinforce and Undermine, and its rows), moved into the Board's third column; its rows are the grouped two-line rows astromech-3ca6 built where they are not already.
- [x] States: not pledged shows "Not pledged" as the head with today's one line on how to pledge, This system and Where to work as today; reading the journal one Label Grey line per block; a failed poll keeps the last answer with the Stale line, or says so with no action (it re-reads every 2 s); no Powerplay in this system and nothing here earning merits keep today's words.
- [x] The brightest things: the merits to the next rank (main value), the Where to work choice and its row actions (what can be changed), and nothing that needs the commander ("none" on the glance sheet). No colour outside DESIGN.md's list: the bar's two greys carry no meaning beyond which side is which, which the figures under it say.
- [x] No printed explanations: every rule and note is in a Tip.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, pledged in a controlled system (the recorded `powerplay` fixture) and not pledged: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the blocks fall to one or two columns. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/powerplay/`; the before is `docs/design/calmer-powerplay-after.png` or astromech-8e22's. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/powerplay/{View,Suggestions,Targets}.svelte` in `PAGE_SIZING`, and the `powerplay` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: the cases in `powerplay/View.browser.test.ts` and `Targets.browser.test.ts` keep their behaviour on the new markup. The time-left words ("2 d 14 h") are a pure function with a table test using fake timers' system time. No test of layout or copy.
- [x] `docs/guide/powerplay.md` describes the blocks.
- [x] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-powerplay` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Powerplay's page and the rank's start in the core. The Now page's Powerplay block (astromech-coq6) reads the same reply.

## Context

Mockup: `docs/design/glow-up-powerplay.png`. Its layout stands; under revision 5 the figures and names are grey with only the main value orange, the bar is grey, not orange, the Where to work choice is a Choice (ember cells, the chosen one underlined, not filled) and the rows are grouped. The mockup prints a rule open under "Power commodities"; the rules stay in Tips.

Replies: `Overview.ts` (`standing`, `here`, `system`), `StandingView.ts` (`power`, `rank`, `merits`, `time_pledged_secs`, `session_merits`, `week_merits`, `week_ends`, `next`), `NextRank.ts`, `PowerplaySystemView.ts` (`control_progress`, `reinforcement`, `undermining`, `contesting`, `acquisition`, `suggestions`), `Suggestion.ts` (`uncertain`). The cycle clock needs no core change: `week_ends` is the cycle's end from the rules' cycle start (`standing.rs`, `merits_this_week`).

## Report

Powerplay is now one Board. The merits to the next rank (Value XL, Orange Lit) lead three answers across the top: Next rank, This cycle and Your merits. This system, What earns merits here and Where to work sit under them. Nothing on the page is cut at any size I measured, astromech-lf3a included.

**The audit.** `node scripts/browser/audit.mjs --plan docs/design/powerplay/plan.json` made 22 captures, and all of them list nothing under `sideways`, `clipped`, `overlaps`, `small`, `contrast` or `far` (`docs/design/powerplay/report.json`):
- Pledged on Acquire at the item's sizes (1100 x 700 and 1920 x 1080 at 100 percent, 2560 x 1440 at 145, 3840 x 2160 at 180), plus 1400 x 900 at 100 and 1200 x 800 at 115, which sit just past the column breakpoints.
- Reinforce at all six sizes, and Undermine at the three largest.
- Not pledged at 1100, 1920 and 2560.
- Loading (`slow=never`) and a failed first read (`fail=`) at 1100 and 1920. The captures are the after; the befores are astromech-8e22's `docs/design/increment2/powerplay-1100x700.png` and `powerplay-2560x1440.png`.

**astromech-lf3a.** The checkpoint's own command, run from a copy of `docs/design/increment2/plan.json`, now lists 0 under `clipped` at all four sizes (it listed 10 at 1920 and 3 at 3840). The copy wrote to `target/`, because `--only` writes the PNGs into the plan's `out` and would have overwritten the befores. There were two causes:
- Where to work had a share of an auto-fit grid, so its rows went compact and cut their place track.
- On Reinforce and Undermine, the state line under the system name ("Li Yong-Rui stronghold 25% · reinforcement 0 / undermining 166") was wider than the place track's 25rem cap.

**The fixes.**
- **Where to work's track.** It has a fixed 45.5rem track. That is SearchKit's `fits` for these rows (720 px at 100 percent) plus 8 px to spare. The track is in `View.svelte`'s `grid-template-columns`, which the page-sizing lint allows. A reviewer may read it as the page setting a width; the alternative was a block-grid component in `lib/ui/layout` for one page, which I didn't think worth it yet.
- **Reinforce and Undermine rows.** Reinforcement and undermining are now the row's own figures group, one over the other, under "Reinforcement · undermining". The line under the name keeps the state and progress. `targetDetail` lost that part, and the `Targets.browser.test.ts` case now checks the two parts in the row.

**Columns.** The breakpoints are container queries on the page's own box (`.room`), not on `win`, which includes the side margins.
- From 85.5rem: three columns. That covers 1920 at 100 and 2560 at 145, both docked, and 3840 at 180.
- From 64rem: two columns, Where to work beside a single column of the other blocks. That covers the 1100 x 700 strip (its page is 64.6rem), so the Choice and its rows are in the first screen.
- Below 64rem: one column.

**Core.** `NextRank.from` comes from `rules().merits_for(rank)`, and is 0 below rank 1. Two rows in `tests/standing.rs` cover it: rank 9 starts from 47,000 and rank 0 from 0. The TS binding and `powerplay.json` were dumped again; the fixture's only change is `"from": 47000`.

**New and changed pieces.**
- `lib/ui/Meter.svelte`: DESIGN.md's meter, drawn as a gradient, so no page sets a fill width. `rest` draws the unfilled part Label Grey for the reinforcement-against-undermining bar. When both figures are 0, the bar sits at half and the line says "Level this cycle".
- `lib/powerplay.ts` gains `timeLeft` ("2 d 14 h", "5 h 12 min", "12 min", "0 min" once past) and `endsAt` ("Thursday 07:00 UTC", read from `week_ends`). `timeLeft` has a table test under fake timers' system time (`powerplay.test.ts`).
- `pledged` now says weeks from two weeks on, for the head's "Pledged 37 weeks · rank 9".

**The preview.** It moves the recorded `week_ends` (22 Oct) forward by whole weeks to the cycle the clock is in (`shell.ts`). Without that, every capture read "0 min left". The drift test keeps the fixture's own value, which is unchanged.

**States.**
- Loading: one Label line per block.
- A failed first read: the alert at the head, and "Not shown until the journal can be read." in each block.
- A failed later poll: the Stale line over the last answer.
- Not pledged: "Not pledged" as the Title with today's pledge line. The three standing blocks are gone. This system, What earns merits here and Where to work keep today's words.
- Pledged before the game logs a rank: "Rank unknown" in Next rank.
- Unoccupied: each acquiring power on its own line of one grid, so their meters line up. The browser test covers it; there is no capture, because the preview's recorded system is a controlled one.

The not-pledged captures use `core=empty`, which has no route, so they show the rail rather than the docked sidebar. Reaching the page under `core=empty` needs setup skipped, so `audit.mjs` gained a `clickText` action, in its own commit.

**Brightness (2560 `bright`, the sidebar apart).** By luminance, the White Title (the power) and the wordmark lead, then Options Grey names and figures. Among the orange, Orange Lit holds only 7,871 (the main value) and the chosen Acquire. Orange holds the other choices, the row figures and actions, the Spansh and Inara links and CHANGE. Nothing needs the commander.

**Tests changed for the new markup or wording.**
- "262 days" became "Pledged 37 weeks".
- The also-active check is scoped to the This system region instead of a `dd`.
- Not pledged is found by its heading.
- `App.browser.test.ts` and `help.browser.test.ts` wait for "Next rank" instead of "My standing".
- `previewstates`' empty text is "Not pledged".

**Known limits.**
- On Reinforce and Undermine the place track has about 8 px above its 17.5rem floor. A state line longer than any in the fixture, such as one naming two other active powers, will end in an ellipsis. The fix then is the row's grouping, not a wider track.
- `until` in `lib/powerplay.ts` is now `timeLeft`, with "min" and no "h" under an hour. Nothing else imports it on `main`, but astromech-coq6, which reads the same reply for Now, may.

**Checks run.** These pass: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-powerplay`, `cargo test -p astromech-core --lib`, `pnpm check`, `pnpm lint`, `pnpm test` (766 tests), `cargo test -p astromech-app --test preview_fixtures` (the drift test, run plainly), `python3 scripts/check-guide.py` and the 7 journeys.
