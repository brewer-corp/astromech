---
schema: atelier.review
schema_version: 2
id: astromech-71hi
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:26.140952489Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author
    commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    recorded_at: 2026-11-07T04:55:53.244783905Z
    resolution_commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    reason: Folded into the plan in 2dc1e59b.
---

medium: astromech-y6d6 Goal vs one-command-list The goal removes 'the endpoint file', and 6i8s lists the token and endpoint file as plumbing that goes, but one-command-list needs every command reachable over the control API, and a second start, onboarding_smoke.py, webview_cost.py and the dev bridge find the core through core.json (control.rs). Fix when planning y6d6: say the control API keeps its endpoint file and token for other clients; only the window's use of it and start-the-core go.
