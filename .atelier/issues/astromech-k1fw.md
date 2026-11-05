---
created_at: "2026-10-23T01:17:57.996021169+00:00"
id: "astromech-k1fw"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-k1fw
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-g0hm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T01:09:03.548034761+00:00"
status: "done"
title: "Galaxy-map route: a NavRouteClear read late can drop a newer plot"
updated_at: "2026-11-05T01:09:03.548034761+00:00"
---

## Goal

Galaxy-map plots and clears reach the route follower in the order the game made them.

## Done when

- A live `NavRoute.json` is handed to the follower when the journal's `NavRoute` line is processed (or a clear is ignored while the route file still has hops), and a test covers clear-then-plot with the journal line read late.

## Context

Review of astromech-g0hm: `NavRoute.json` is read on Status.json changes but `NavRouteClear` when its journal line is read, so a quick clear then re-plot can arrive as plotted(B) then cleared() if journal appends lag (astromech-txzq). Effect: the Route page shows no route while the game has one plotted; galaxy-map routes are silent, so nothing is spoken or copied wrongly.

## Notes

- 2026-11-04 [decision] The route file orders galaxy-map clears; a NavRouteClear read while it still has hops is ignored
  Reason: The game rewrites NavRoute.json empty on a clear (docs/REPORT.md), so the file alone carries plots and clears in the game's order. Engine::route now hands an empty route that follows one with hops to the rules as Input::Route, and route_follow turns it into Follower::cleared(); the live NavRouteClear line still clears, but only while the last NavRoute.json read (GameState::route_hops) is empty. Either read order ends right: file first (a re-plot B after the clear is already in the state, so the late clear line is ignored) or journal first (the line is ignored while the file still has hops, and the empty file read next clears). Revised after review finding astromech-yi77: the first version only had the guard, so a clear line read before its empty file was dropped for good. The route briefing and fuel rules also get Input::Route; both return nothing for an empty route (fewer than 2 hops; fuel_line's hops.first()?), and no other rule matches Input::Route.
  Rejected: Hand NavRoute.json to the follower when the journal's NavRoute line is processed: needs a file read in the journal path, a timestamp check against the event (the file can lag the event, as the EDDN feed's wait shows), and re-sends a plot the follower already has, which rebuilds the galaxy-map route and resets its progress. Remembering an ignored clear and applying it when the file goes empty: the empty file still has to reach the rule, which needs the same Engine::route change, plus extra state in the rule.
- 2026-11-04 [note] Implementer report: galaxy-map plots and clears now reach the follower in NavRoute.json's order. Engine::route (crates/astromech-announce/src/lib.rs) passes an empty route that follows one with hops to the rules; route_follow (src/route_follow.rs) turns it into cleared(), and ignores a live NavRouteClear line while GameState::route_hops is non-empty. docs/PLOTTING.md updated. Tests: a_clear_read_after_a_newer_plot_is_ignored (clear then re-plot, journal read late) and a_clear_read_before_its_empty_file_clears_when_the_file_is_read (journal first, from review finding astromech-yi77); without the fixes the first gave an extra 'cleared' and the second never cleared. Route briefing and fuel with an empty Input::Route: read their code (route.rs needs 2 hops, fuel_line returns None on no hops; no other rule matches Input::Route) and the existing test the_same_route_written_again_is_not_said_again_and_a_cleared_route_is_quiet now sends the empty route through every built-in rule with fuel set and still asserts silence. Checks: cargo fmt --all --check, clippy -p astromech-announce --all-targets -D warnings, cargo test -p astromech-announce and -p astromech-core all pass. Still unverified: that the game always rewrites NavRoute.json empty on a clear (docs/REPORT.md says so); if not, a clear read while the file has hops would be dropped. Review of the revision outstanding.

## Close Reason

Implementer report. Changed:
- route_follow ignores a live NavRouteClear while the last NavRoute.json read still has hops.
- Engine::route passes an empty route that follows a non-empty one to the rules, and route_follow turns it into cleared().
- docs/PLOTTING.md is updated.
Decision note: ignore the late clear and pass the empty file through, rather than reading the file in the journal path or remembering ignored clears.
Proof:
- a_clear_read_after_a_newer_plot_is_ignored (the journal line read late) and a_clear_read_before_its_empty_file_clears_when_the_file_is_read (the journal arriving first). Both fail without the engine change.
- The route briefing and fuel rules stay quiet on an empty route (tests/route.rs).
- Announce and core tests, clippy and fmt pass (evidence after merging main). PR #205.
Review: finding astromech-yi77 (a clear lost when the journal arrives first) was fixed in 65ae8a16; approval astromech-3wy9.
Deviations: the first revision missed the case where the journal arrives first; review caught it.
Open items: that the game always rewrites NavRoute.json empty on a clear is per docs/REPORT.md and not proven by a fixture.
