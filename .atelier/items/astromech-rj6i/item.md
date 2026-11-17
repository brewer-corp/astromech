---
schema: atelier.item
schema_version: 3
id: astromech-rj6i
title: Powerplay on one screen
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:42.585058701Z
---

## Goal

Powerplay answers three questions at a glance: how far to the next rank, how long the cycle has left, and what this system wants. Today the standing is a label and value list with the merits to go as one figure among many, the system's reinforcement and undermining are two numbers in a sentence, and the four panels are laid out by a grid of fixed column widths (`app/src/modules/powerplay/View.svelte`).

## Done when

- [ ] astromech-lf3a is fixed (taken over from the checkpoint): with the route sidebar docked, no system name or state on Powerplay is cut or ends in an ellipsis. `node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only powerplay` (the checkpoint's tool, from astromech-8e22) lists nothing under `clipped` at 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) or 3840 x 2160 (180 percent) (it listed 10 at 1920 and 3 at 3840 at the checkpoint), and the 1920 and 3840 captures show This system's name and every Where to work row's system and state whole.
- [ ] The core gives the start of the current rank: `NextRank` (`crates/astromech-powerplay/src/standing.rs:63`) gains `from`, the lifetime merits the current rank needed, from the same rank table as `merits` (`rules().merits_for(rank)`, `standing.rs:197-206`, `data/catalog/powerplay.json`). A row in the crate's existing standing tests covers it, and `app/src/preview/fixtures/powerplay.json` is dumped again; the drift test passes.
- [ ] Powerplay is a Board page on the type roles, laid out as `docs/design/glow-up-powerplay.png`. The head: the power's name as the Title, White, with "Pledged 21 weeks · rank 12" in Label (`time_pledged_secs`, `rank`).
- [ ] **Next rank**: the merits to go (`next.to_go`) as the page's main value in Value XL Orange Lit with "merits to rank 13" in Label; under it a meter from `from` to `merits` filled to the lifetime merits, Prose Grey on Mark Off; "Rank 13 needs 79,000 · reward" in Label. Rank 1's note (`next.note`) replaces the figure while it applies.
- [ ] **This cycle**: the time left until `week_ends` in Value XL Options Grey ("2 d 14 h left"), the end in Label in the commander's words ("Ends Thursday 07:00 UTC").
- [ ] **Your merits**: this cycle's merits (`week_merits`) in Value XL Options Grey; this session's (`session_merits`) and the lifetime total (`merits`) in Label. The "weekly assignment rewards aren't logged" note stays a Tip on the figure.
- [ ] **This system**: the system's name with its copy icon and ExtLinks, its state and controlling power in Label; for a controlled system, reinforcement against undermining as one bar, the reinforcement part Prose Grey and the undermining part Label Grey, split by the two figures (`reinforcement`, `undermining`), with both figures under it and one Label line saying which is ahead and by how much; the control progress through the state's band (`control_progress`) as a thin meter with its percentage and its Tip. Unoccupied: each power's acquisition progress as a meter with its percentage. Other powers active (`contesting`) in Label. "As of your last jump" in a Tip on the system's name.
- [ ] **What earns merits here**: the suggestions as a compact list, each name in Name with Ethos, Weak spot and On foot as badges and its rule (`how`) in its Tip, as `Suggestions.svelte` does; a rule not checked in game keeps its "?" with the reason in the Tip (astromech-c1sq and astromech-66pq check them). The rules' source and date stay one Label line with its Tip.
- [ ] **Where to work**: `Targets.svelte` as today (its Choice of Acquire, Reinforce and Undermine, and its rows), moved into the Board's third column; its rows are the grouped two-line rows astromech-3ca6 built where they are not already.
- [ ] States: not pledged shows "Not pledged" as the head with today's one line on how to pledge, This system and Where to work as today; reading the journal one Label Grey line per block; a failed poll keeps the last answer with the Stale line, or says so with no action (it re-reads every 2 s); no Powerplay in this system and nothing here earning merits keep today's words.
- [ ] The brightest things: the merits to the next rank (main value), the Where to work choice and its row actions (what can be changed), and nothing that needs the commander ("none" on the glance sheet). No colour outside DESIGN.md's list: the bar's two greys carry no meaning beyond which side is which, which the figures under it say.
- [ ] No printed explanations: every rule and note is in a Tip.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, pledged in a controlled system (the recorded `powerplay` fixture) and not pledged: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the blocks fall to one or two columns. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/powerplay/`; the before is `docs/design/calmer-powerplay-after.png` or astromech-8e22's. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/powerplay/{View,Suggestions,Targets}.svelte` in `PAGE_SIZING`, and the `powerplay` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: the cases in `powerplay/View.browser.test.ts` and `Targets.browser.test.ts` keep their behaviour on the new markup. The time-left words ("2 d 14 h") are a pure function with a table test using fake timers' system time. No test of layout or copy.
- [ ] `docs/guide/powerplay.md` describes the blocks.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test -p astromech-powerplay` pass, and `pnpm check`, `pnpm lint` and `pnpm test`, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Powerplay's page and the rank's start in the core. The Now page's Powerplay block (astromech-coq6) reads the same reply.

## Context

Mockup: `docs/design/glow-up-powerplay.png`. Its layout stands; under revision 5 the figures and names are grey with only the main value orange, the bar is grey, not orange, the Where to work choice is a Choice (ember cells, the chosen one underlined, not filled) and the rows are grouped. The mockup prints a rule open under "Power commodities"; the rules stay in Tips.

Replies: `Overview.ts` (`standing`, `here`, `system`), `StandingView.ts` (`power`, `rank`, `merits`, `time_pledged_secs`, `session_merits`, `week_merits`, `week_ends`, `next`), `NextRank.ts`, `PowerplaySystemView.ts` (`control_progress`, `reinforcement`, `undermining`, `contesting`, `acquisition`, `suggestions`), `Suggestion.ts` (`uncertain`). The cycle clock needs no core change: `week_ends` is the cycle's end from the rules' cycle start (`standing.rs`, `merits_this_week`).
