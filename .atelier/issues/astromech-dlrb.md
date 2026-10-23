---
created_at: "2026-10-22T23:48:20.205009100+00:00"
id: "astromech-dlrb"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-dlrb
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
status: "review"
title: "Send outfitting/2 to EDDN"
updated_at: "2026-10-23T00:01:15.685422480+00:00"
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

## Notes

- 2026-10-22 [note] Filter (7dcf8372): the README's sku rule, with each journal module's sku taken from EDCD/FDevIDs outfitting.csv entitlement (c356129) via data/catalog/eddn-outfitting.json (scripts/gen-eddn-outfitting.py): 1033 station symbols, 178 commander (PowerPlay ELITE_SPECIFIC_V_POWER_*, Guardian/human tech-broker unlocks, paid-ship armour such as Cobra Mk V/Type-8/Mandalay, FDevIDs '?' and 'removed'; a symbol any row marks commander-only is, e.g. the Enzyme Missile Rack whose pre-engineered row is blank). Cross-checked against the wiki's Powerplay modules table (all 12) and Technology Broker unlock tables (tests POWERPLAY/TECH_BROKER). A module in neither list sends nothing for that visit (Unusable::UnknownModule, logged): a list without it would also be wrong; a game update adding a module stops outfitting sends until the data file is regenerated. EDMC's journal path (plugins/eddn.py export_journal_outfitting, dc90353) drops only int_planetapproachsuite and applies no sku filter, so astromech is stricter than EDMC here; EDMC's CAPI path uses the same sku rule. The real Jameson Memorial file (1084 items) has 0 unknown symbols and 51 commander-only ones.
