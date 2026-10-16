---
created_at: "2026-10-15T20:36:13.648255681+00:00"
id: "astromech-cmhv"
issue_type: "feature"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-6920"
  - kind: "issue"
    id: "astromech-77a1"
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-gtza"
  - kind: "issue"
    id: "astromech-huab"
  - kind: "issue"
    id: "astromech-oo8y"
  children:
  - kind: "issue"
    id: "astromech-0zcu"
  - kind: "issue"
    id: "astromech-ngyt"
  - kind: "issue"
    id: "astromech-omi1"
  - kind: "issue"
    id: "astromech-w8ai"
  - kind: "issue"
    id: "astromech-wq3f"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Reaction engine with prospector and scan callouts"
updated_at: "2026-10-16T18:26:24.161862282+00:00"
---

## Goal

Journal and Status events drive rules that speak, toast or notify, and the two callouts Sumeet values most work: prospector results for high-value ores, and being scanned. This replaces EDCoPilot's mining and scan announcements.

## Done when

Journal replay tests per rule (spoken text, cooldown, quiet while replaying), a word-budget test on all callout copy. Independent review. The game check (astromech-6920) runs after this merges. Delivers S7.

## Scope

Only what the HUD doesn't show, or shows somewhere easy to miss mid-action (strategy Shape). V1-DESIGN §1.5 #5's default callouts (low fuel, interdiction, heat) are replaced by this list.

## Design

EDDI and EDCoPilot are the models ([REPORT §8.3](docs/REPORT.md)). Copy register: [V1-DESIGN §6.4](docs/V1-DESIGN.md).
