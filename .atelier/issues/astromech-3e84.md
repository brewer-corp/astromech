---
created_at: "2026-10-15T20:21:44.853201090+00:00"
id: "astromech-3e84"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-lstz"
  - kind: "issue"
    id: "astromech-xur0"
  children:
  - kind: "issue"
    id: "astromech-1tmv"
  - kind: "issue"
    id: "astromech-6y47"
  - kind: "issue"
    id: "astromech-dp15"
  - kind: "issue"
    id: "astromech-zmy2"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Reaction engine with prospector and scan callouts"
updated_at: "2026-10-15T20:21:45.022377166+00:00"
---

## Scope

Only what the HUD doesn't show, or shows somewhere easy to miss mid-action (strategy Shape). V1-DESIGN §1.5 #5's default callouts (low fuel, interdiction, heat) are replaced by this list.

## Outcome

Journal and Status events drive rules that speak, toast or notify, and the two callouts Sumeet values most work: prospector results for high-value ores, and being scanned. This replaces EDCoPilot's mining and scan announcements.

## Proof

Journal replay tests per rule (spoken text, cooldown, quiet while replaying), a word-budget test on all callout copy, and a mining session in the real game. Independent review.

## Delivers

S7

## Design

EDDI and EDCoPilot are the models ([REPORT §8.3](docs/REPORT.md)). Copy register: [V1-DESIGN §6.4](docs/V1-DESIGN.md).
