---
created_at: "2026-10-15T20:36:13.647520981+00:00"
id: "astromech-0474"
issue_type: "feature"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-huab"
  children:
  - kind: "issue"
    id: "astromech-cgrw"
  - kind: "issue"
    id: "astromech-gl7q"
  - kind: "issue"
    id: "astromech-i0th"
  - kind: "issue"
    id: "astromech-pvhn"
  - kind: "issue"
    id: "astromech-sor5"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-a50f"
    type: "related"
  - kind: "issue"
    id: "astromech-orel"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "M3 exit: voice in the COVAS register"
updated_at: "2026-10-16T18:26:25.247824052+00:00"
---

## Goal

Voice replies use the speaker the user picks in Settings (astromech-orel), through the Cockpit chain tuned in the headset to sit in the same register as in-game COVAS, and recognition passes the 95% execute-or-ask bar on the VR clip set.

## Done when

The eval run's report, and Sumeet's verdict from the headset, recorded as evidence. Delivers v1.0 S1.

## Scope

Already built: Kokoro TTS with the Cockpit colour chain (band-limit, presence, compression, saturation, comb, pip), Natural/Cockpit/Harsh presets, the coloured confirmation cache and a cuttable mixer (`astromech_voice::colour`, `mixer`; V1-DESIGN §6.4, §7.12). The speaker is already a per-request `sid` and part of the TTS cache key, so switching voices needs no restart. Which speaker ships as the default is not a v1.0 question.

## Design

[V1-DESIGN §6.4](docs/V1-DESIGN.md).
