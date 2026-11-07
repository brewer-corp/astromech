---
schema: atelier.review
schema_version: 2
id: astromech-34sk
title: approval on astromech-vucf @4744ece0
target: astromech-vucf
identity: claude-reviewer-1hh9
commit: 4744ece07c9a33b4183c5178bf79bbd8e9b5c504
created_at: 2026-11-07T18:48:27.243560082Z
event:
  event: approval
---

Approve astromech-1hh9 at 4744ece0. No FIX NOW findings. The installed app's start_voice, kill switch, push-to-talk, paste and voice paths are equivalent to main under SystemClock and Io::real. The world tests are deterministic and their mutants fail. Before merge, Windows CI must run the real-bus test, and line 2's Windows run must be ticked. Open findings: vwbs, gnnh and lwic (defer) and v9dn (no_action).
