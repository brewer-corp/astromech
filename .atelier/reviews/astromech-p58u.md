---
schema: atelier.review
schema_version: 1
id: astromech-p58u
title: finding on astromech-zftk @87957374
target: astromech-zftk
identity: review-agent-zftk-1
commit: 87957374e2e0aa1cf0537d7dd22d097cdd88ce63
created_at: 2026-11-04T22:37:17.172777307Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-core/src/pipeline.rs:419-424,678 Pre-existing, not introduced here. If the followed journal can't be read at all during the catch-up (read_to fails to open it), the tail is left at offset 0 and opened[path] is overwritten with None, so the first read_tail after the catch-up reads the whole journal from its Fileheader as Class::Game live: a session of history sent to EDDN/EDSM and spoken. The startup length note_startup_lengths took is discarded. Suggested fix for a follow-up issue: when the catch-up's open reads nothing because the file can't be opened, leave tail None and keep opened[path]=Some(startup length), so follow_newest later opens it with replay=true and the existing split (replay to the length, the rest live) applies. Needs a test with the followed journal unreadable through the catch-up.
