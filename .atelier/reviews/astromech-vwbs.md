---
schema: atelier.review
schema_version: 2
id: astromech-vwbs
title: finding on astromech-vucf @4744ece0
target: astromech-vucf
identity: claude-reviewer-1hh9
commit: 4744ece07c9a33b4183c5178bf79bbd8e9b5c504
created_at: 2026-11-07T18:48:11.189788066Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/app.rs:1185, :1200 and :418 (1hh9, the tester clock) arm and read TESTER with Instant::now, while start_voice (session.rs:629) now reads TESTER.armed(clk.now()). Identical under SystemClock, so the installed app is unchanged. Under the ManualClock, which only advances on steps, clk.now() falls behind real time, so a world that arms test mode and then advances past the lease would still see it armed (and status() reads it on real time). No world tests test mode yet. Suggested follow-up: a note on astromech-zgoo, whose Instant::now lint will flag these three sites, saying they move to the core's clock (not an allow) so the arm, the status line and the loop share one clock.
