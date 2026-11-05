---
schema: atelier.review
schema_version: 1
id: astromech-3uyj
title: finding on astromech-51ue @249bf97a
target: astromech-51ue
identity: reviewer-fable-51ue-final
commit: 249bf97a81ab3640206b4a73b02f7eeaf70a95de
created_at: 2026-11-05T16:09:28.819644957Z
event:
  event: finding
  recommendation: no_action
---

info: proof and provenance notes, no change needed. (1) git range-diff 326fd18e..origin/archive/feature-51ue-pre-rewrite 326fd18e..HEAD: every code commit replays '=' and git diff archive..HEAD touches only .atelier records and docs/design/validation PNGs, so the dropped approvals (kp4n, gqpm, kzye) and findings (abu3, tc9s, 7r5k, 6q7m, m881, gasg, n1xc, hdj3, vm1x, zp4e, mbng, et0g) were read from the archive as context and the code they cover was re-reviewed here. (2) PR #208's last CI run is on 53330152, whose app/, crates/, docs and DESIGN/PRODUCT content equal HEAD's (git diff --stat: 56 PNGs only): Windows MSVC build + tests and app shell, WebView2, installer smoke test and Linux all SUCCESS; its one failure, 'atelier doctor', is 'findings.disposed: 4 fix_now findings resolved after the current approval; they need a fresh approval', the gate this review closes, not a code failure. (3) cargo clippy --workspace --all-targets -D warnings was satisfied from the shared target dir's cache (Finished in 0.43 s, no source changed since the validator's run); fmt, tests, lint, check, build, check-guide and check-deps ran fresh. (4) Committed content checked: the two Spansh bodies fixtures and app/src/lib/fixtures/hotspots.json are public Spansh answers (no commander, path or window title); the 56 validation PNGs render the fake core's sample journal, 'Example DualVKB' preset and fixture routes; the controller diagram in strip-controller-*.png is the community VKB image already on main. No Frontier assets or personal logs. (5) lib/hotspots.ts ORES and astromech_journal::ore::VALUE_ORDER hold the same 15 symbols, and lib/galaxy.ts FACTION_STATES the same 7 ids as remote.rs QUERIES, today (drift tests stay deferred: z9rd, vaz9).
