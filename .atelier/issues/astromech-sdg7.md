---
created_at: "2026-10-16T19:40:10.082487639+00:00"
id: "astromech-sdg7"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-cmhv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T23:08:28.125622560+00:00"
status: "done"
title: "Harden the TTS name reading rules"
updated_at: "2026-10-22T23:08:28.125622560+00:00"
---

## Goal

Reduce false positives and widen coverage in `crates/astromech-voice/src/speakable.rs`: single-letter units and scales after a bare number are rewritten without context ("in 5 m" becomes metres, "Deck 2 K" thousand); short catalogue prefixes match anywhere ("Col. Smith" becomes Collinder); only five ore names are respelled and token matching misses possessives and quotes.

## Done when

Spaced units limited to unambiguous ones (ly, ls, %, Cr, MJ, km); short prefixes require a following number; more ore and commodity names (Alexandrite, Monazite, Tritium, ...); the table gains negative rows (2:30, x2, Krait Mk II, T-10, Type-9, FSD, OK, bare Cr, 5 m, Col. Smith, Prua Phoe XX-Y e1-2345, a date, non-ASCII input).

## Context

Found in review of astromech-w8ai (R1, R3, R6, R10).

## Close Reason

Done and independently reviewed on feature/astromech-uuqt (approval astromech-5asu).
