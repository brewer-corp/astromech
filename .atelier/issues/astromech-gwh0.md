---
created_at: "2026-10-17T04:56:08.861471011+00:00"
id: "astromech-gwh0"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:53:39.254213189+00:00"
status: "done"
title: "Mining run: start only on prospector or collector limpets; test the rate to the latest line"
updated_at: "2026-10-22T21:53:39.254213189+00:00"
---

## Goal

A repair, fuel or hatchbreaker limpet launch starts a mining run, so the Mining page shows "0 t" instead of "No mining yet" (finding astromech-29ip). Start the run only on prospector or collection limpets. Add a test where a non-mining journal line after the last refine lowers tons per hour (finding astromech-aqrl).

## Done when

A repair limpet alone leaves no run; the rate test with a later non-mining line passes.

## Close Reason

Done and independently reviewed on feature/astromech-fdh3 (approval astromech-o2y7).
