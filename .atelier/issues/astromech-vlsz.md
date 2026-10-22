---
created_at: "2026-10-17T19:08:10.959886648+00:00"
id: "astromech-vlsz"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-vlsz
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:51:24.565422903+00:00"
status: "done"
title: "Spansh plotters: send the drive's own supercharge multiplier"
updated_at: "2026-10-22T20:51:24.565422903+00:00"
---

## Goal

Neutron and exact routes plotted with Spansh use the neutron boost the ship's drive really has, so a Caspian Explorer with the Mk II drive gets ×6 routes instead of ×4 ones. This is the quick fix; the local neutron plotter (astromech-1842) reuses the same multiplier.

## Done when

- `astromech_ship::Fsd` carries `supercharge_multiplier`, 6 for `int_hyperdrive_overcharge_size8_class5_overchargebooster_mkii` and 4 for every other drive, matching Spansh's site.
- `NeutronRoute` and `ExactRoute` both send it as `supercharge_multiplier`, and `astromech_galaxy::plot` sets it from the loadout.
- Tests: a Mk II loadout gives 6 and an ordinary SCO size 8 gives 4 (`astromech-ship`), and both request forms carry the value (`astromech-data`).
- The guide's limit line about the usual ×4 boost (docs/guide/routes.md, "Limits" under Plot a route with Spansh) and the matching paragraph in docs/PLOTTING.md are gone.

## Scope

`crates/astromech-ship`, `crates/astromech-data/src/spansh.rs`, `crates/astromech-galaxy/src/plot.rs`, the two docs. No UI change: the range line already names the ship.

## Context

Spansh's site bundle (read 22 Oct 2026) sets `superchargeMultiplier: 6` on that one drive and posts `supercharge_multiplier` (4 or 6) from both the neutron plotter form (`/api/route`, radio buttons "regular" and "overcharge") and the exact plotter. `ExactRoute::form` hard-codes 4 (`spansh.rs`); `NeutronRoute` sends no multiplier, so Spansh assumes 4. Review finding R2 on PR #122 (astromech-94j4).

## Close Reason

Both Spansh plotters send the drive's own supercharge multiplier (6 for the Mk II drive); reviewed and approved (astromech-6jt9).
