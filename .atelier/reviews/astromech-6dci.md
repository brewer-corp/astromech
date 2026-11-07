---
schema: atelier.review
schema_version: 1
id: astromech-6dci
title: finding on astromech-ja5a @8fd31c63
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: 8fd31c6366b5b5ca540fcacf3dd0315bcf6a0288
created_at: 2026-11-07T00:07:25.846712422Z
event:
  event: finding
  recommendation: defer
---

low: docs/releases/v1.0.0-alpha.4.md:16 announces following routes plotted in the galaxy map (a66e0008, k1fw), but no in-game step checks the detour and clear cases: rk4s only exercises leg plots of a neutron route. Suggest adding to rk4s or cxgq: 'Once, plot a detour in the galaxy map: astromech follows it quietly, and the Route page shows the neutron route Paused with Resume. Clear the galaxy-map route: following stops.' Defer: no open needs-human issue covers it, and check-release-notes passes.
