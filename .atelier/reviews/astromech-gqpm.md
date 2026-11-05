---
schema: atelier.review
schema_version: 1
id: astromech-gqpm
title: approval on astromech-kwoi @b59bfe06
target: astromech-kwoi
identity: reviewer-fable-kwoi
commit: b59bfe06cbc77259f52c02a6a4adae596a989658
created_at: 2026-11-05T15:10:17.842010004Z
event:
  event: approval
---

Reviewed 047b1ff7..b59bfe06 (kwoi) against its Done when, bi94, g8d5 item 5, DESIGN.md (Places home, Lookup result row, States, Two Tab Rows, Buttons/Links), AGENTS, CONTRIBUTING, ARCHITECTURE, V1-DESIGN. Read Market/Cargo/RawSites/FactionStates, places.ts routes, galaxy.ts nearbyResults, the three source pages, View keying, the tests and guide diffs. Reran in app/: pnpm lint, pnpm check (460 files, 0/0), pnpm build, pnpm test (70 files, 455 pass); python3 scripts/check-guide.py (22 pages, 0 problems). Verified lookup ids unchanged (trade.sell[.anywhere], trade.buy[.anywhere], trade.cargo, sites.<kind>, faction.<id>) so remembered filters and the core's voice lookups (astromech-core trade.rs/lookups.rs, no Rust changed) are unaffected; FACTION_STATES ids equal remote.rs QUERIES and cover every material source query; main's trade View kept its mode in local state with no palette entry and Sites/Nearby embeds had no address, so 'old URLs still work' reduces to the source pages, which App.test covers along with all nine links; grep finds no reference to Sites.svelte, CommodityInput or the mode tabs outside release notes and the deferred V1-DESIGN rows. Viewed all 14 screenshots: Trade's third tab row is gone, finders show the result row. Five findings, four defer, one info; no fix_now.
