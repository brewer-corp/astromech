---
created_at: "2026-10-16T18:12:14.655360777+00:00"
id: "astromech-ojwr"
issue_type: "task"
labels:
- "size-l"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-3njr"
  - kind: "issue"
    id: "astromech-4fe3"
  - kind: "issue"
    id: "astromech-dqmv"
  - kind: "issue"
    id: "astromech-rk4s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:54:32.489832937+00:00"
status: "done"
title: "Port Coriolis's ship calculations to Rust"
updated_at: "2026-10-16T18:54:32.489832937+00:00"
---

## Goal

A pure Rust crate that takes a ship, its fitted modules and their engineering (blueprint, grade, roll and experimental) and returns its stats. It should return mass (hull, unladen and laden), jump range (unladen, laden and max, including the Guardian FSD booster), fuel and cargo, power draw against capacity, shield strength and resistances including boosters, armour and hull resistances, raw and sustained DPS, distributor capacity and recharge per WEP/SYS/ENG, weapon drain against recharge (time to empty), speed and boost. The Ships page needs this.

## Done when

Fixture `Loadout` events for at least five reference builds give stats that match Coriolis to its displayed rounding, as unit tests. The builds should cover combat, exploration and mining ships, and at least one with charge-enhanced and one with weapon-focused distributors, mass manager and deep charge FSDs, and thermal-resistant shield boosters.

## Scope

Calculations and their tables only; no UI. Module and blueprint tables go in `data/`, and coriolis-data is credited in NOTICE.md.

## Context

Coriolis's code is MIT, so porting its maths is fine. Its data is player-visible game data, which may live in the repo ([ARCHITECTURE §18.4](docs/ARCHITECTURE.md)). EDSY is CC BY-NC, so use it for reference only and never copy its code. Reuse the existing blueprint tables in `crates/astromech-engineering` rather than duplicating them. This supersedes astromech-4f6x.

## Close Reason

astromech-ship returns every stat in the goal; five synthetic reference builds (combat, exploration, mining; charge-enhanced and weapon-focused PDs, Mass Manager and Deep Charge, thermal-resistant boosters) match Coriolis to its displayed rounding in cargo test -p astromech-ship. Independent review outstanding (arranged by the orchestrator).
