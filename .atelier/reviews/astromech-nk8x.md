---
schema: atelier.review
schema_version: 2
id: astromech-nk8x
title: finding on astromech-vucf @7045a4d2
target: astromech-vucf
identity: claude-reviewer-jkgv
commit: 7045a4d27585ba1223a8c6459d18bfb3d7274cda
created_at: 2026-11-07T20:55:34.580584028Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/astromech-jkgv.md:69 (Report, line 5) and crates/astromech-core/tests/preview_fixtures.rs:413 The Report says three fixtures are dumped except one part (trade /route/loop, plot /local and exobiology's reported part) and two are hand-written. The table says otherwise. exobiology.json is a whole-file From::Not, unchanged since start, so 2 files are partial (trade, plot) and 3 are wholly hand-written (nearby-hge, exobiology, eddn-log). The test's own 'kept as written' lines show it. Done when 5 asks that the hand-written fixtures the core can produce be replaced by dumps. The record claims that for exobiology's bodies, system and sampling, but the test doesn't do it. Fix one of two ways: split exobiology.json into parts with only /known as From::Not, which is what the Report describes, or correct the Report and say why its other parts can't be dumped.
