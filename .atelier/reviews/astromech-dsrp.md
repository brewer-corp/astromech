---
schema: atelier.review
schema_version: 1
id: astromech-dsrp
title: 'R3 [low] crates/astromech-core/src/pipeline.rs:631: catch_up measures startup lengths only after replaying the followed journal. Anything the game appends to its current journal during that replay (when it isn''t the followed one, which is the nrhv case), and any journal it creates in that window, counts as history: replayed, not sent or spoken. That breaks ARCHITECTURE''s rule against missing live lines, though only for the length of the catch-up. Measuring before open() narrows it, but then the followed journal gets Some(len) and splits during catch-up, when EDDN uses self.live without caught_up. That needs its own look, so not for this branch.'
target: astromech-g3qt
identity: review-agent-g3qt-1
commit: 98e6827717c46e9692d70b475f2300e4eb9f4b51
created_at: 2026-10-22T21:06:26.444752409Z
event:
  event: finding
  recommendation: defer
---

R3 [low] crates/astromech-core/src/pipeline.rs:631: catch_up measures startup lengths only after replaying the followed journal. Anything the game appends to its current journal during that replay (when it isn't the followed one, which is the nrhv case), and any journal it creates in that window, counts as history: replayed, not sent or spoken. That breaks ARCHITECTURE's rule against missing live lines, though only for the length of the catch-up. Measuring before open() narrows it, but then the followed journal gets Some(len) and splits during catch-up, when EDDN uses self.live without caught_up. That needs its own look, so not for this branch.
