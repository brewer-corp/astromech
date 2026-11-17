---
schema: atelier.item
schema_version: 3
id: astromech-nheq
title: CONTRIBUTING.md tells you to run atelier land and atelier doctor, which Atelier 0.7.0 does not have
status: ready
mission: null
labels:
- docs
- size-s
priority: medium
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T07:08:24.737495894Z
trivial: null
caused_by: null
created_at: 2026-11-17T07:08:24.654931846Z
---

## Goal

The checks CONTRIBUTING.md tells an agent to run before pushing exist. Lines 110 to 118 say to run `atelier land` on `main` and `atelier doctor`; the installed Atelier 0.7.0 has neither (`land` is an unrecognized subcommand), so implementers skip the step and say so in every report (astromech-woqv, astromech-5lh2, 12 and 17 Nov 2026). The pre-push hook is the only check on a push.

## Done when

- [ ] CONTRIBUTING.md, AGENTS.md and the CI workflows name only commands the pinned Atelier has (`atelier check` is today's record check), and say what checks a push to `main`.
- [ ] astromech-sx84 ("atelier land fails 'review event history missing'") is dropped or rewritten against today's commands, with the reason.

## Scope

Docs, workflows and that one record. Whether merges should run the code checks is astromech-r198.
