---
schema: atelier.item
schema_version: 3
id: astromech-r198
title: 'Merges run no checks: lint, tests and journeys can break on main unnoticed'
status: draft
mission: null
labels:
- tooling
- size-s
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T07:00:30.972119936Z
---

## Goal

A change cannot merge to main with the lint, the tests or the journeys failing. Today `.atelier/config.toml` names no merge check and no forge, so `atelier merge` runs nothing: three journeys broke on 12 Nov 2026 and nobody saw until 17 Nov (astromech-5lh2), and each implementer runs whichever checks it remembers.

## Done when

- [ ] (provisional) `.atelier/config.toml` lists the merge checks, or names the CI workflow that runs them: `pnpm check`, `pnpm lint`, `pnpm test`, the journeys, and the Rust fmt, clippy and tests for a change that touches a crate.
- [ ] (provisional) The checks finish inside the 9 minutes `atelier merge` waits, on this machine's 4 GB per-agent memory limit; if the journeys cannot, the item says where they run instead.

## Context

Agent Factory's readiness line 7 ("every merge runs the checks"). Sumeet's call: it adds minutes to every merge, and this repository chose no forge on purpose.
