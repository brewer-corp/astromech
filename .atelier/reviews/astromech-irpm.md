---
schema: atelier.review
schema_version: 2
id: astromech-irpm
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:25.752174551Z
event:
  event: finding
  recommendation: fix_now
---

high: docs/strategies/one-tested-process.md one-process criterion (measurement) S11's 250 MB budget is the window's WebView group with the core and voice worker excluded (scripts/webview_cost.py lines 14 and 108-118; ARCHITECTURE 18.1). In one process the core's heap is in the same process, so 'idle memory stays inside the S11 budget' either silently becomes core+window under 250 MB (a new and possibly unreachable budget, and the Authority section says any rise above S11 goes back to Sumeet) or needs a harness that can't separate them. Nothing owns updating webview_cost.py/test_webview_cost.py and .github/workflows/webview-cost.yml (they stage the core and time cold starts with the core not running/after a core crash). Fix: say which processes are summed and what budget applies after the merge, make y6d6 own the harness and workflow change, and use the Windows webview-cost CI job as the before-merge stand-in, with Sumeet's PC run (with Elite, as y6d6 says but the criterion doesn't) as the named human measurement.
