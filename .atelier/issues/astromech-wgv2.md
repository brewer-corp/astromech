---
created_at: "2026-10-15T20:21:44.852844475+00:00"
id: "astromech-wgv2"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-lstz"
  children:
  - kind: "issue"
    id: "astromech-axzc"
  - kind: "issue"
    id: "astromech-i5za"
  - kind: "issue"
    id: "astromech-w1dy"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Nearby outfitting, ships and services"
updated_at: "2026-10-15T20:21:45.002298785+00:00"
---

## Outcome

From the current or a typed system, the user finds the nearest station selling a module (by group, class, rating and mount), a ship, or a service, in the app and by voice. This replaces Inara's nearest-module search for daily use.

## Proof

Lookups against recorded Spansh/Ardent responses, voice phrase tests, and a check in the real game that the top answer sells the item. Independent review.

## Delivers

S1

## Unknowns

The Spansh stations-search request shape for module and ship filters.
