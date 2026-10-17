---
schema: atelier.review
schema_version: 1
id: astromech-s5ji
title: 'R3 [low] crates/astromech-announce/src/session_summary.rs replayed(): it resets the whole Ended (pending, spoken) on every replayed journal line, not only in the startup catch-up. The pipeline also replays after going live (a journal already opened this run, or an older one a sync tool touches, open(n, true)); a replay during the 30 s wait drops a pending summary, and armed then follows whatever file was replayed. Rare and the hub state is clobbered by such replays anyway (pre-existing), so defer; a later fix could arm only (leave pending/spoken) or have the engine tell rules once when the catch-up ends.'
target: astromech-627q
identity: review-agent (opus, independent)
commit: 4910e731b0539fd52abe0edc195f7fe40f8d6ebe
created_at: 2026-10-17T05:38:06.643732709Z
event:
  event: finding
  recommendation: defer
---

R3 [low] crates/astromech-announce/src/session_summary.rs replayed(): it resets the whole Ended (pending, spoken) on every replayed journal line, not only in the startup catch-up. The pipeline also replays after going live (a journal already opened this run, or an older one a sync tool touches, open(n, true)); a replay during the 30 s wait drops a pending summary, and armed then follows whatever file was replayed. Rare and the hub state is clobbered by such replays anyway (pre-existing), so defer; a later fix could arm only (leave pending/spoken) or have the engine tell rules once when the catch-up ends.
