---
created_at: "2026-10-17T06:33:31.615337226+00:00"
id: "astromech-sd0k"
issue_type: "decision"
labels: []
priority: "P3"
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
title: "Which exobiology value wins where sources disagree?"
updated_at: "2026-10-17T06:33:31.615337226+00:00"
---

## Context

The values feed the unsold organic total and the biology summary; a wrong value misleads which body to sample.

## Question

data/catalog/exobiology.json lists value disagreements under `uncertain`. Fonticulua Fluctus was settled at 20,000,000 (the wiki's 16,777,215 is 2^24-1). Still open: Concha Biconcavis (wiki 16,777,215, also 2^24-1; EDR 19,010,800) and Bacterium Nebulus / Scopulum (wiki 9.1 M / 8.6 M; EDR and Spansh about 5.3 M / 4.9 M).

## Options

- Wiki values throughout, except obvious placeholders such as 2^24-1.
- Check in game: sell a sample of each and read the journal's SellOrganicData value.
