---
schema: atelier.review
schema_version: 1
id: astromech-lus6
title: approval on astromech-51ue @249bf97a
target: astromech-51ue
identity: reviewer-fable-51ue-final
commit: 249bf97a81ab3640206b4a73b02f7eeaf70a95de
created_at: 2026-11-05T16:09:47.269273500Z
event:
  event: approval
---

Reviewed 326fd18e..249bf97a (f7i0 with a3a26297, kwoi, 2hbw with d16846ab replayed as 95a052b9, kyqw's receipts and 56 screenshots, the 2374 Context fix in 6d87aa43) against 51ue's Goal and Done when, the four children's Done when, decisions bi94, g8d5 (items 3, 5, 6, 7, 10) and ghyo (item 9), DESIGN.md (Frame, Lookup result row, Route strip, Find or plot, Places home, States), PRODUCT.md, AGENTS.md, CONTRIBUTING.md (committed content), ARCHITECTURE.md. Confirmed the history rewrite replayed every code commit unchanged (range-diff all '=', archive diff is records and validation PNGs only) and read the dropped review records from the archive as context. Read in full: lib/hotspots.ts, places.ts, find.ts, palette.ts, search.ts changes, OriginInput, SearchKit changes, Hotspots/Market/Cargo/RawSites/FactionStates/FinderPage/View.svelte, PlotBar.svelte, App.svelte, the Trade/Materials/Exploration page changes, astromech-market mining.rs (most_hotspots banding, rank, floor, next_band), spansh.rs filters, astromech-tools mining.rs, core lookups.rs/assistant.rs/tester.rs, the intent grammar, the Rust and vitest tests, the docs and guide diffs. Every Done when item of 51ue is met on the Linux build: strip on every page but onboarding, Add stop and Set destination with the parked route and Resume on every result and in the header box, Places home with ten finders at one URL each and old links and entries still opening them, Powerplay/Exploration/Colonisation kept with Exploration's embed now a Services link, the DESIGN.md row and states on every finder, one Find or plot box on Ctrl+K that plots, adds and opens finders from phrases, the mining voice query with its grammar and guide. Proof rerun on 249bf97a: in app/ pnpm lint, pnpm check (460 files, 0/0), pnpm build, pnpm test (70 files, 464 passed, 0 skipped); check-guide (22 pages, 0 problems); check-deps OK; cargo test -p astromech-core -p astromech-galaxy -p astromech-market -p astromech-tools -p astromech-data (536 passed, 0 failed, 2 ignored: voice model and real neutron file); cargo fmt --all --check clean; cargo clippy --workspace --all-targets -D warnings clean (cache-satisfied, see astromech-3uyj). Windows shell: PR #208 CI on 53330152, code-identical to HEAD. Three findings, none fix_now: s55s (Enter before Spansh answers on a system that starts an entry label, defer), d3vj (stale V1-DESIGN Exploration row, defer), 3uyj (proof and provenance, no action).
