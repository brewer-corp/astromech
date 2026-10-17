---
schema: atelier.review
schema_version: 1
id: astromech-bnic
title: 'R1 medium session_summary.rs:158-163: arming needs a LoadGame the rule sees live, but no rule runs during the silent startup catch-up (lib.rs:7), so starting astromech after the game is already logged in leaves the rule unarmed and that session''s real logout is silent. Not a regression of the d2e4 requirement and the safe direction, but a user-visible gap; V1-DESIGN says ''only arms after a LoadGame seen live'' without naming it. Suggested: arm from game state (a login seen in the catch-up) or state the limit in the doc.'
target: astromech-oo8y
identity: review-agent (sonnet, independent)
commit: 9b1966aabd613a587ba08e605b06a50a2f416294
created_at: 2026-10-17T05:18:02.327751284Z
event:
  event: finding
  recommendation: defer
---

R1 medium session_summary.rs:158-163: arming needs a LoadGame the rule sees live, but no rule runs during the silent startup catch-up (lib.rs:7), so starting astromech after the game is already logged in leaves the rule unarmed and that session's real logout is silent. Not a regression of the d2e4 requirement and the safe direction, but a user-visible gap; V1-DESIGN says 'only arms after a LoadGame seen live' without naming it. Suggested: arm from game state (a login seen in the catch-up) or state the limit in the doc.
