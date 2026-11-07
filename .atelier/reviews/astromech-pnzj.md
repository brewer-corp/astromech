---
schema: atelier.review
schema_version: 2
id: astromech-pnzj
title: finding on astromech-l5c6 @9af0c06b
target: astromech-l5c6
identity: claude-reviewer-l5c6
commit: 9af0c06b2de026fe1110a8df9df885c82c22b459
created_at: 2026-11-07T03:07:32.447269687Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author
    commit: 70741a5022d016af9dedcc169d66945f887cbeff
    recorded_at: 2026-11-07T03:10:48.397203900Z
    resolution_commit: 527c5618e564ed847c24642d0622391ef6b2bb01
    reason: Fixed in 527c5618; the reviewer confirmed it in approval astromech-dgn0.
---

low: scripts/preview-publish:25-43 fails quietly or unclearly on edge inputs: --core isn't validated (a typo gives a build where fakeScenario() returns null with no error); a flag missing its value dies with bash 'unbound variable'; the title replace silently does nothing if index.html's <title> changes; the DOCTYPE regex would leave junk on an SVG DOCTYPE with an internal subset. A change in Vite's <head>/<body> shape fails loudly (Python AttributeError under set -e), which is acceptable. Fix: validate --core against empty|new|ready, check $# before shift 2, and assert the title was replaced.
