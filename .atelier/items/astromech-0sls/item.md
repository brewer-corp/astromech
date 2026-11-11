---
schema: atelier.item
schema_version: 3
id: astromech-0sls
title: 'Onboarding smoke test: check the health step and cover the mic steps where possible'
status: draft
mission: null
labels:
- alpha
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T20:10:24.727472731Z
---

## Goal

The onboarding smoke test passes the mic and calibration steps with Next (the runner has no mic) and doesn't check what the Health step shows. Assert the Health step's expected state (everything but the speech models and calibration ready), and cover the mic steps if a virtual audio device can be installed on the runner.

## Done when

- [ ] scripts/onboarding_smoke.py fails when the Health step shows an unexpected problem.

## Context

Review of astromech-flru (R6).
