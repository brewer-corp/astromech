---
created_at: "2026-10-17T17:24:21.254152268+00:00"
id: "astromech-9u0l"
evidence_type: "validation"
commit: "bab9e8a53ff8f18fbc4b2bb90ad741f1c84e868c"
result: "pass"
captured_at: "2026-10-17T17:24:21.254082017+00:00"
agent_identity: "docs-agent"
target:
  kind: "issue"
  id: "astromech-md1k"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-md1k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "data-sharing.md lists the EDDN schemas and triggers from crates/astromech-eddn (companion.rs, journal.rs, events.rs, feed.rs: commodity/3, shipyard/2, fcmaterials_journal/1, journal/1 for six events, and the eleven event schemas; outfitting/2 off), what message.rs removes, the 5 minute live limit (MAX_EVENT_AGE_SECS), and what the EDSM uploader sends (every event as written plus transient fields, minus EDSM's discard list). Other pages from app/src/modules/{powerplay,colonisation,exploration,exobiology,logbook,session,station} and the crates behind them. routes.md is a placeholder as asked. Checked by inspection, not by running the app."
updated_at: "2026-10-17T17:24:21.254549499+00:00"
---

data-sharing.md lists the EDDN schemas and triggers from crates/astromech-eddn (companion.rs, journal.rs, events.rs, feed.rs: commodity/3, shipyard/2, fcmaterials_journal/1, journal/1 for six events, and the eleven event schemas; outfitting/2 off), what message.rs removes, the 5 minute live limit (MAX_EVENT_AGE_SECS), and what the EDSM uploader sends (every event as written plus transient fields, minus EDSM's discard list). Other pages from app/src/modules/{powerplay,colonisation,exploration,exobiology,logbook,session,station} and the crates behind them. routes.md is a placeholder as asked. Checked by inspection, not by running the app.
