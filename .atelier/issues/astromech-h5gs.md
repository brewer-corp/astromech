---
created_at: "2026-10-17T06:33:31.541953355+00:00"
id: "astromech-h5gs"
issue_type: "task"
labels: []
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
title: "Exobiology follow-ups: failed lookups, carrier arrivals, re-logged samples, localised names, Horizons symbols"
updated_at: "2026-10-17T06:33:31.541953355+00:00"
---

## Goal

Deferred findings from the astromech-znhx review:
- astromech-pp0v: a failed Spansh lookup is cached for the session as "no biology"; don't cache errors, show "unknown".
- astromech-tymm: the known-biology lookup starts only on FSDJump; also on CarrierJump and a login into a system.
- astromech-ulyt: the once-per-sample cue key can repeat when the same species is re-logged on the same body, silencing the second cue; after a restart mid-sample the cue repeats once.
- astromech-unpa: a sampled species is stored under its localised name and looked up by English name only; use the symbol.
- astromech-7wx9: Horizons-era species have no journal symbols in the catalogue (valued at their genus minimum), and the genus lookup lacks $Codex_Ent_Seed_Name; (brain trees).
- astromech-n7yc: docs say the Biology page uses the same fold as the System page (same type, different instance); unused GUI_SAA constant.

## Done when

A test per item.
