---
created_at: "2026-10-16T20:28:48.495792513+00:00"
id: "astromech-yrfv"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ipb5"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "History: one commander for journals from before FID"
updated_at: "2026-10-22T21:11:44.441689991+00:00"
---

## Goal

Journals older than the Frontier id (`FID`) give a commander the key `name:<Commander>`, and later journals give the same commander their `F…` key, so the Logbook lists two commanders and "been here before" misses the old visits. When exactly one FID has used name X, treat `name:X` as that FID in derive and rebuild (and re-key when that FID first appears).

## Done when

A fixture with an old `LoadGame` without `FID` and a later one with it, same name: one commander in `commanders`, and the old visits count in that commander's logbook; two FIDs with the same name stay apart.

## Context

`crates/astromech-history/src/commander.rs`. Found in review of astromech-i4gx/u3f4/3tlm (R2).

## Notes

- 2026-10-16 [note] Also (review of ipb5 round 4, N1): a continuation part inherits the commander from whichever file sorts before it; only inherit when the previous file has the same timestamp stem, else start with no commander.
