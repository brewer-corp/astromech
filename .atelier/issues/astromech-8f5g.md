---
created_at: "2026-10-18T03:21:28.466964762+00:00"
id: "astromech-8f5g"
issue_type: "task"
labels:
- "voice"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T04:16:06.965264750+00:00"
status: "superseded"
title: "Voice: which words may back up a sound-alike key press ('car down')"
updated_at: "2026-10-18T04:16:06.965264750+00:00"
---

## Goal

Decide which words may back up a sound-alike for a key press. Today any phrase word longer than two letters heard as spelled counts, so particles like "down", "off", "out" do and "on", "in", "up" do not: "car down" executes gear down (car only sounds like gear), while "car up" asks.

## Done when

A product decision on whether particles count as backing words, implemented with tests, and the voice eval shows no lost correct executes beyond what the decision accepts.

## Context

Review finding R2 (round 2) of astromech-vkzj. Asking on every sound-alike would turn 18 correctly executing eval transcripts into questions, so "a second word backs it up" stays; the question is only which words count.

## Close Reason

Subsumed by the astromech-vkzj change of direction: a phonetic-key match now caps the phrase below execute, and nothing backs a sound-alike up, so 'car down' asks (no particle question left).
