---
created_at: "2026-10-16T22:42:24.932310422+00:00"
id: "astromech-nv75"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "HGE lookup fails when only the population page fails"
updated_at: "2026-10-16T22:42:24.932310422+00:00"
---

## Goal

The HGE lookup fetches a nearest page and a population-filtered page from Spansh. If only the population page fails, the whole lookup errors; it should fall back to the nearest page.

## Done when

A test with the population request failing and the nearest request succeeding returns ranked results from the nearest page, with its fetched age.

## Context

Deferred from the review of the astromech-sfys fixes (crates/astromech-galaxy/src/lib.rs:255).
