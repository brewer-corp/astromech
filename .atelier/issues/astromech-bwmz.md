---
created_at: "2026-10-16T18:12:15.141981342+00:00"
id: "astromech-bwmz"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T21:59:40.484855028+00:00"
status: "done"
title: "Rank HGE suggestions by distance and population"
updated_at: "2026-10-16T21:59:40.484855028+00:00"
---

## Goal

High Grade Emissions only spawn reliably in systems with a population over 1 billion, but the search ignores population. Rank suggestions by a score that combines distance and population: systems under 1 billion rank well below those over it, and bigger and closer is better.

## Done when

Lookup tests from a fixed position show a 2-billion system 30 ly away ranked above a 200-million system 5 ly away, and the score is shown next to each result.

## Context

`crates/astromech-materials/src/hge.rs` and `farm.rs`. Population comes from the galaxy data.

## Close Reason

HGE lookups rank by population and distance (hge_score); a 2B system at 30 ly ranks above a 200M system at 5 ly in the fixed-position test, and the score shows on each row. Evidence msxr, ckbk, 0nml.
