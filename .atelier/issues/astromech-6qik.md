---
created_at: "2026-10-17T00:34:23.648352558+00:00"
id: "astromech-6qik"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Armour lookups match only the current ship's armour"
updated_at: "2026-10-17T00:34:23.648352558+00:00"
---

## Goal

Each armour grade in the outfitting catalogue carries ~47 ships' symbols, so a voice or picker search for 'military grade composite' in a Krait MkII can name a station selling only Adder armour. The app's palette already declines armour for this reason.

## Done when

Armour searches (voice and picker) match only the current ship's armour symbol (GameState.ship or Spansh's Module.ship), with a test where another ship's armour isn't offered.

## Context

Deferred from the astromech-970a review (crates/astromech-galaxy/src/outfitting.rs:360, app/src/lib/outfitting.ts:55).
