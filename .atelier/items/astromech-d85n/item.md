---
schema: atelier.item
schema_version: 3
id: astromech-d85n
title: 'Colonisation candidates: report truncation and test the caps'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T02:49:37.081149012Z
---

## Goal

Make the colonisation candidates lookup say when it has truncated results, and test its caps.

## Done when

- [ ] - When more than 100 unpopulated systems match, the page says farther systems were left out (`CANDIDATES` in crates/astromech-galaxy/src/colonies.rs; Spansh's page `count` is currently ignored). - Tests exercise the 40-system body detail cap and the 3-page bodies limit, and hitting the limit is reported instead of silently dropping bodies. - The unchecked count is taken after the age filter. - From the review of astromech-20dy (R1–R3, low).
