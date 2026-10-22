---
created_at: "2026-10-22T23:48:20.205009100+00:00"
id: "astromech-dlrb"
issue_type: "task"
labels: []
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-e8ux"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Send outfitting/2 to EDDN"
updated_at: "2026-10-22T23:48:20.205009100+00:00"
---

## Goal

S6 says outfitting is sent to EDDN from the live game, but the sender never sends it: `companion::SEND_OUTFITTING` is `false` (crates/astromech-eddn/src/companion.rs:26) and every Outfitting visit is logged as skipped ("outfitting/2 is off until PowerPlay and tech-broker modules are filtered", feed.rs:154). It was gated in astromech-xvjs after review R1 and no issue picked it up. Filter the commander-specific modules from journal `Outfitting.json` (it has no sku, so the README sku rule never fires) and turn the schema on, so astromech replaces EDMC for outfitting data.

## Done when

- A fixture `Outfitting.json` that lists PowerPlay and tech-broker modules builds an outfitting/2 message without them that validates against the live schema, and a fixture with only those modules sends nothing.
- `SEND_OUTFITTING` (or its gate) is gone and the feed test that asserts the skip is replaced by one that asserts the message is queued live and not during catch-up.
- The real-journal harness EDDN dry run (`ASTROMECH_REAL_JOURNALS=<copy> cargo test -p astromech-core --release --test real_journals -- --nocapture`) lists outfitting-v2.0.json messages and no "outfitting/2 is off" skips (on 22 Oct it skipped 111).
- The Data sharing page copy ("What gets sent") names outfitting.

## Context

- Strategy S6 and "No data is better than bad data": docs/strategies/replace-the-toolbox.md.
- Review finding R1 and the decision to gate: astromech-xvjs notes (16 Oct); spec notes on astromech-thau.
- Live acceptance after this lands: astromech-uxyq (needs-human).
- Found by validation astromech-e8ux.

## Unknowns

- The complete list of PowerPlay and tech-broker module symbols (wiki-visible data is fine in data/); whether a denylist or a decision record is the better route (R1 offered both).
