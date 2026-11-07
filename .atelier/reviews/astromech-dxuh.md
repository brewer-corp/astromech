---
schema: atelier.review
schema_version: 1
id: astromech-dxuh
title: finding on astromech-ja5a @8fd31c63
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: 8fd31c6366b5b5ca540fcacf3dd0315bcf6a0288
created_at: 2026-11-07T00:07:25.619734562Z
event:
  event: finding
  recommendation: defer
---

low: docs/releases/v1.0.0-alpha.4.md:52, :53 and :115 name a 'Builds' page (Engineering, Builds; 'on Builds'; 'The Builds page cannot find one station...'). There is no Builds page: the build planner is a ship's Engineering view under Engineering, Ships (docs/guide/ships.md:32,49), now a choice on the page after sllo. Pre-existing wording carried from alpha.3, so defer, but cheap to fix with the R1-R3 edits: 'open a ship's Engineering view (the build planner)', 'on a ship's Engineering view and its Info view', 'The build planner cannot find one station...'.
