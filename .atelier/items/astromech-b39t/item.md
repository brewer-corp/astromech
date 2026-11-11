---
schema: atelier.item
schema_version: 3
id: astromech-b39t
title: 'cargo deny: check the window''s dependencies too'
status: draft
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T00:42:38.899692147Z
---

## Goal

cargo deny has never checked the shipped window's dependency tree: it lived in its own workspace, and since astromech-jdks the root check runs without the `window` feature. Check it, since those crates ship in the installer.

## Done when

- [ ] deny.toml's [graph] enables `window` (features = ["window"]) and `cargo deny check` passes, with each new exception explained in deny.toml.

## Context

Trying it on 2026-11-06 found: target-lexicon's licence Apache-2.0 WITH LLVM-exception not allowed (permissive; add it); unmaintained advisories for zbus_names, zvariant, zvariant_derive (Linux-only Tauri deps) and proc-macro-error; async-executor 1.14.0 yanked.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-jdks (Faster release builds: one workspace, a mock game crate, the version from the tag)
