---
schema: atelier.review
schema_version: 1
id: astromech-2374
title: finding on astromech-kwoi @271f1ce9
target: astromech-kwoi
identity: reviewer-sonnet-f7i0
commit: 271f1ce9c3894a42d20e7b00b44612c520001742
created_at: 2026-11-05T09:13:16.130797037Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/astromech-kwoi.md Context still says 'astromech-sllo folds Trade's mode tabs, so this starts after it', which 271f1ce9 reverses (the blocked_by edge is gone and a Note says so). An implementer reading Context top-down is told to wait for sllo; the Note sits below it. Fix: replace the Context line with 'astromech-sllo now waits for this one (see Notes)' so the issue has one sequence.
