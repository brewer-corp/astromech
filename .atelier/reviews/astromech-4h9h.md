---
schema: atelier.review
schema_version: 1
id: astromech-4h9h
title: 'R3 low (for astromech-6bjl): NeutronStars::name reopens the file by path on every call (crates/astromech-galaxy/src/neutron.rs:199). If ''Download the newer file'' replaces neutron.bin while a NeutronStars built from the old file is loaded, names are read from the new file at the old indices and are silently wrong. Fix when the plotter loads it: keep the File opened in open() (Rust opens with share-delete on Windows, so the rename still works), or check the header again before reading names.'
target: astromech-1842
identity: review-agent-1842-1
commit: 059d78381235b475652ae78edab486764504ddb4
created_at: 2026-10-22T21:11:03.222793574Z
event:
  event: finding
  recommendation: defer
---

R3 low (for astromech-6bjl): NeutronStars::name reopens the file by path on every call (crates/astromech-galaxy/src/neutron.rs:199). If 'Download the newer file' replaces neutron.bin while a NeutronStars built from the old file is loaded, names are read from the new file at the old indices and are silently wrong. Fix when the plotter loads it: keep the File opened in open() (Rust opens with share-delete on Windows, so the rename still works), or check the header again before reading names.
