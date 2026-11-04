---
schema: atelier.review
schema_version: 1
id: astromech-6rig
title: finding on astromech-zftk @87957374
target: astromech-zftk
identity: review-agent-zftk-1
commit: 87957374e2e0aa1cf0537d7dd22d097cdd88ce63
created_at: 2026-11-04T22:37:16.985856580Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/astromech-core/src/pipeline.rs:708-709 Regression of the replay-never-live property. note_startup_lengths now measures the followed journal and, when its handle can't be opened (ek06: a scanner or backup tool holding it), falls back to the directory listing's length, which on Windows lags appends to the journal the game holds open (txzq). open() then replays only up to that short length (read_to, line 421) and the rest, history written before startup, is read after the catch-up by read_tail as Class::Game live: sent to EDDN/EDSM, spoken, companion files loaded live. Before this change the followed journal was replayed by read_new to EOF whatever the listing said (its entry was None, or_insert kept it). The implementer's close reason lists this as open item (2) but nothing tracks it. Fix: for followed, don't use the listing fallback; if the handle can't be opened leave it unmeasured so open() replays all it can read (fail closed, the old behaviour; only lines written during the catch-up are lost to history). Add a unix test like the ek06 one with the followed journal unreadable at measurement and readable at open (e.g. chmod back from the during_catch_up-style hook or measure via a seam).
