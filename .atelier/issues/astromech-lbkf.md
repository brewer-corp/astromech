---
created_at: "2026-10-15T20:30:24.596973270+00:00"
id: "astromech-lbkf"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-egtp"
  children:
  - kind: "issue"
    id: "astromech-4os7"
  - kind: "issue"
    id: "astromech-kvmy"
  - kind: "issue"
    id: "astromech-vot9"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ws34"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "M3 exit: voice in the COVAS register"
updated_at: "2026-10-15T20:30:24.727443927+00:00"
---

## Scope

Already built: Kokoro TTS with the Cockpit colour chain (band-limit, presence, compression, saturation, comb, pip), Natural/Cockpit/Harsh presets, the coloured confirmation cache and a cuttable mixer (`astromech_voice::colour`, `mixer`; V1-DESIGN §6.4, §7.12).

## Outcome

Voice replies use a flat British voice through the Cockpit chain that Sumeet judges to sit in the same register as in-game COVAS, and recognition passes the 95% execute-or-ask bar on the VR clip set.

## Proof

The eval run's report, and Sumeet's verdict from the headset, recorded as evidence.

## Delivers

v1.0 S1

## Design

[V1-DESIGN §6.4](docs/V1-DESIGN.md).
