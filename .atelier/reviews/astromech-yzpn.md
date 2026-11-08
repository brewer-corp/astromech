---
schema: atelier.review
schema_version: 2
id: astromech-yzpn
title: finding on astromech-yis2 @2c03629d
target: astromech-yis2
identity: claude-reviewer-cwtv
commit: 2c03629db80aa723b18791e44f39b941a32fa4be
created_at: 2026-11-07T23:54:55.758725564Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: 594a80acd0b50c79388040e9e59ba45f173e2048
    recorded_at: 2026-11-07T23:56:02.115711362Z
    follow_up: astromech-lltb
    reason: Filed as astromech-lltb
---

cwtv Report, Open items: the edited Spansh fixtures crates/astromech-data/tests/fixtures/spansh-route-*.json and spansh-exact-*.json (all-zero job ids, a trimmed 3-stop Sol to Colonia route), used by worlds/preview/sol and worlds/every-command, stay against astromech-1zhg, and the Report hands them to the plan owner but no follow-up issue is filed. Pre-existing and not made worse by cwtv. File a backlog issue (discovered from this finding) to replace them with recordings.
