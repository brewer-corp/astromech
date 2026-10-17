---
created_at: "2026-10-17T21:41:11.975557061+00:00"
id: "astromech-xi2i"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice: T0, lookup and unavailable phrases below 95% on the synthetic eval"
updated_at: "2026-10-17T21:44:37.073380679+00:00"
---

## Goal

Look at the commands below 95% on the synthetic eval (astromech-yil0, docs/evidence/astromech-yil0/report.md); every family is at or above 95%, so these are per-command shortfalls. The T1 and ship-control ones are filed separately as alpha (throttle.zero, confirm, galaxy_map.paste).

## Done when

Each is fixed (alias, MISHEARD entry or a better phrase) or judged acceptable against real clips (astromech-cgrw), and `scripts/voice-eval.sh` is rerun.

## Context

- status.legal 87.5%: "notoriety" heard "No toriety." (bm_george), nearest is cancel, NoMatch.
- status.fuel, status.credits, status.shields 93.8%: "fuel status" heard "Fuel state Ursa.", "rebuy" heard "Raban" / "Goodbye.", "shields" heard "She ultimate." (noise).
- status.ship 93.8%: "current ship" heard "Current Scheber", asks status.location.
- hyperspace_jump 90.6%: "engage frameshift drive" heard "Engage frames of drive", asks find.module twice. Not alpha: the command is in UNAVAILABLE and only answers "I can't do that yet", so nothing is pressed.
- Family / condition: status / noise 94.6%, unavailable / cockpit+noise 93.8%.
