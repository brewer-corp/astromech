---
schema: atelier.review
schema_version: 2
id: astromech-cg84
title: finding on astromech-kjfs @fb155297
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: fb155297cf6c350766b8e24aeddc7c7b2ee702c4
created_at: 2026-11-07T04:56:38.922125531Z
event:
  event: finding
  recommendation: defer
---

medium: docs/strategies/one-tested-process.md one-process criterion, astromech-y6d6 Done when (new 50 MB bound) 'the app process's idle memory is at most 50 MB above today's core alone' is a new numeric bound on the shipped app that Sumeet did not set; the strategy's Authority section reserves success criteria to him, so it needs his say. It is checkable (webview_cost.py already reports the core's USS) and plausibly reachable (6i8s expects a windowless Tauri process to cost a few tens of MB), but unmeasured (y6d6's own Unknown), and the former shell state (galaxy.db connections, lookup caches) must really drop when idle for it to hold. 'Today's core' is also a moving baseline, and next to Elite on Sumeet's PC it needs a two-process measurement there too. Fix before y6d6 is planned: pin the baseline (the last two-process release's core, same harness, same machine and profile), and put the 50 MB figure to Sumeet as a needs-human question or decision record; until then treat it as provisional. Doesn't affect the ready increment 7rcn.
