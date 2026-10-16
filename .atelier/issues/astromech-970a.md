---
created_at: "2026-10-16T21:54:25.413632136+00:00"
id: "astromech-970a"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-m30a"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Outfitting lookup follow-ups: mount warning, A-rated, armour and unsold modules, unknown voice system"
updated_at: "2026-10-16T21:59:20.912850340+00:00"
---

## Goal

Close the gaps review found in the station lookups (astromech-m30a):

- R4: when only the mount check empties a page (Spansh can't filter the mount), `stations::find` warns "its search filter may have changed", which is wrong. Say none of the nearest stations sell that mount, or page further.
- R5: "nearest A-rated FSD" drops the rating: "a" is a filler word in `astromech-intent` normalise, so "a rated" loses the letter. Keep a rating letter before "rated"/"grade" (and "class five A").
- R6: ship-specific armour (Lightweight Alloy … Mirrored Surface Composite, Mk II Ablative) and pre-engineered modules (Enduring Feedback Rail Gun, Rapid Phase Multi-Cannon and the other Spansh names not in `data/catalog/outfitting.json`; also Caustic Sink Launcher, Extended Cargo Rack, Long Range Detailed Surface Scanner) aren't searchable; "Basic Discovery Scanner" is pickable but never sold. Bring the catalogue in line with Spansh's `field_values/modules` (fixture `crates/astromech-galaxy/tests/fixtures/spansh-module-names.json`).
- R7: voice lookups search by the journal's system name; a system Spansh doesn't know is HTTP 400, spoken as "Spansh turned the request down: Invalid request." Search from the system's position (journal StarPos) or say plainly that Spansh doesn't know the system.

## Done when

Each item has a test that fails before the fix: a mount-only empty page without the filter warning (R4), "nearest A rated FSD" resolving to rating A (R5), an armour and a pre-engineered module search plus no unsold modules in the picker (R6), and a voice lookup from a system Spansh doesn't know giving a plain answer (R7).

## Context

Found in the independent review of feature/astromech-m30a at 02376ce.

## Notes

- 2026-10-16 [note] Also (review of m30a round 2, N1): when Spansh gives a symbol FDevIDs lacks, fall back to the name so a real seller isn't dropped.
