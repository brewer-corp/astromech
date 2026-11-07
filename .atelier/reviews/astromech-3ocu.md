---
schema: atelier.review
schema_version: 2
id: astromech-3ocu
title: finding on astromech-7rcn @9be1d69a
target: astromech-7rcn
identity: claude-reviewer-8zdt
commit: 9be1d69a3387eee0fadd751cdaa4dd8d04ed74cd
created_at: 2026-11-07T06:32:27.190202797Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 850ae885d9ee67faa304f05a48a8a238f2ff6287
    recorded_at: 2026-11-07T06:48:28.503023950Z
    resolution_commit: 8a43bf4e1a60c23b1ed2d32eab3aa42bfa112698
    reason: 'Cheap; fixed: argument-less Date rejected too'
---

low: app/eslint-rules/tests.js:26 no-wall-clock catches Date.now() and performance.now() (what Done when asks) but not new Date() or Date() with no arguments, which also read the wall clock; CONTRIBUTING.md (Tests, Determinism) says lint rejects wall-clock reads in test files, so a reader may trust lint more than it deserves (SearchKit.browser.test.ts:168,186 use new Date() safely only because Date is faked there). Suggested: also report a NewExpression/CallExpression of Date with zero arguments, or word the doc as 'Date.now() and performance.now()'.
