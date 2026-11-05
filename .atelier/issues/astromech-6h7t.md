---
created_at: "2026-10-22T23:06:19.774601582+00:00"
id: "astromech-6h7t"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-6h7t
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
closed_at: "2026-11-04T20:37:14.548315054+00:00"
status: "done"
title: "Spansh neutron routes report legs as jumps"
updated_at: "2026-11-04T20:37:14.548315054+00:00"
---

## Goal

A neutron route plotted by Spansh shows its real number of jumps, not its number of legs.

## Done when

`astromech_galaxy::plot` sets `Plotted.jumps` for Spansh neutron routes from the sum of each leg's jumps (or Spansh's jump total, whichever the API documents), and the recorded Sol to Colonia test asserts 166 jumps, not 128.

## Context

Found by the astromech-6bjl implementer on 22 Oct 2026: Spansh's `total_jumps` in the neutron result counts legs; plot.rs reports it as jumps, so the Route page under-reports. Recorded fixtures: crates/astromech-galaxy/tests/fixtures/spansh-neutron-*.json on feature/astromech-1842.

## Close Reason

Plotted.jumps for Spansh neutron routes is now the sum of each waypoint's jumps (Spansh's total_jumps counts legs; documented on NeutronPlan). New test a_neutron_route_counts_jumps_not_legs plots the full recorded Sol to Colonia route (spansh-neutron-sol-colonia-50-4.json) and asserts 166 jumps over 129 stops. The older tests on the trimmed spansh-route-result.json fixture (two legs, total_jumps left at 128) now assert 11. No guide change: the Route panel already says 'how many jumps'.
