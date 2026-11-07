---
schema: atelier.review
schema_version: 2
id: astromech-ilwv
title: finding on astromech-y6d6 @b5af3bc1
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: b5af3bc135de96d0aa8250e4b14f524c941710d7
created_at: 2026-11-07T19:47:18.856971611Z
event:
  event: finding
  recommendation: defer
---

low: docs/V1-DESIGN.md §7.5 (Run value paragraph, f6a048be). It now says 'upgrades by either route leave the value as it was (the uninstaller deletes it, except when an upgrade runs it with /UPDATE)'. A player's third route, setup.exe run interactively (or /P), takes PageLeaveReinstall's default and runs the old uninstaller without /UPDATE, which deletes the value for good (astromech-y2m1). A reader can take the sentence as covering every upgrade. Suggest adding the doc line to y2m1's Done when, or naming the interactive case there now.
