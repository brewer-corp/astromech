---
schema: atelier.review
schema_version: 1
id: astromech-mubt
title: 'R4 low: if the app is killed mid-download, the Part guard''s Drop never runs and neutron.bin.part (up to 184 MB) stays in the data folder until the next download truncates it (crates/astromech-galaxy/src/neutron.rs:243-250, 277-279). neutron.bin itself is never left half-written, so this costs disk space only. Fix: remove a stale .part in installed() or when the shell reports status.'
target: astromech-1842
identity: review-agent-1842-1
commit: 059d78381235b475652ae78edab486764504ddb4
created_at: 2026-10-22T21:11:03.340956455Z
event:
  event: finding
  recommendation: defer
---

R4 low: if the app is killed mid-download, the Part guard's Drop never runs and neutron.bin.part (up to 184 MB) stays in the data folder until the next download truncates it (crates/astromech-galaxy/src/neutron.rs:243-250, 277-279). neutron.bin itself is never left half-written, so this costs disk space only. Fix: remove a stale .part in installed() or when the shell reports status.
