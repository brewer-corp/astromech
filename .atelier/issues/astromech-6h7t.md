---
created_at: "2026-10-22T23:06:19.774601582+00:00"
id: "astromech-6h7t"
issue_type: "bug"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Spansh neutron routes report legs as jumps"
updated_at: "2026-10-22T23:06:19.774601582+00:00"
---

## Goal

A neutron route plotted by Spansh shows its real number of jumps, not its number of legs.

## Done when

`astromech_galaxy::plot` sets `Plotted.jumps` for Spansh neutron routes from the sum of each leg's jumps (or Spansh's jump total, whichever the API documents), and the recorded Sol to Colonia test asserts 166 jumps, not 128.

## Context

Found by the astromech-6bjl implementer on 22 Oct 2026: Spansh's `total_jumps` in the neutron result counts legs; plot.rs reports it as jumps, so the Route page under-reports. Recorded fixtures: crates/astromech-galaxy/tests/fixtures/spansh-neutron-*.json on feature/astromech-1842.
