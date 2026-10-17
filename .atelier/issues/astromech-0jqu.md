---
created_at: "2026-10-15T20:36:13.654720384+00:00"
id: "astromech-0jqu"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-7vpa"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T04:48:21.150467206+00:00"
status: "done"
title: "Surface guidance"
updated_at: "2026-10-17T04:48:21.150467206+00:00"
---

## Goal

Bearing and distance to a lat/long from Status (great-circle).

## Done when

Table tests against known bearings and distances on bodies of different radii.

## Context

Pure maths. The exobiology sampling helper uses it.

## Close Reason

Bearing and distance maths in astromech_exobiology::surface with table tests on bodies of several radii; Status.json position and PlanetRadius parsed.
