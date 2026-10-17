---
created_at: "2026-10-17T19:08:10.959886648+00:00"
id: "astromech-vlsz"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Exact plotter: send the drive's own supercharge multiplier"
updated_at: "2026-10-17T19:08:10.959886648+00:00"
---

## Goal

Exact routes plotted by astromech use the neutron boost the ship's drive really has, as Spansh's site does.

## Done when

`ExactRoute` carries `supercharge_multiplier` worked out from the FSD symbol (Spansh's site maps `int_hyperdrive_overcharge_size8_class5_overchargebooster_mkii` to 6, everything else 4; check the current site bundle), `astromech_galaxy::plot` sets it from the loadout, a test covers the Mk II drive, and the guide's limit line about it goes.

## Context

Review finding R2 on PR #122 (astromech-94j4): the form hard-codes 4. The guide states the limit.
