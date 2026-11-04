---
schema: atelier.review
schema_version: 1
id: astromech-2yh9
title: approval on astromech-fyvh @697ef6aa
target: astromech-fyvh
identity: review-agent-fyvh-1
commit: 697ef6aa4e8e059836cf2a1cc2f9b1e4e7ecca0c
created_at: 2026-11-04T20:42:18.977635111Z
event:
  event: approval
---

Read eb74ee23..697ef6aa (links.rs is_openable_url/climbs, test, evidence, close reason) against Done when. Reran cargo test -p astromech-engineering (all pass), clippy --all-targets -D warnings and fmt --check (clean). Bypass search: replicated the predicate in Node and ran ~5,000 candidates (.., ., %2e mixes, %252e, fullwidth/one-dot-leader dots, ..%2f, ..%5c, ..;, NBSP/ZWSP, overlong UTF-8, //host, @host) on every prefix through WHATWG URL; none passed the predicate and normalised outside its prefix. Tab/newline stripping is covered by the control-char refusal. Checked every link builder (links.ts spansh/inara via encodeURIComponent, help.ts GUIDE_URL + manifest help strings, settings release notes, coriolis_url/edsy_url) still passes. Residual: server-side decoding of %2F (e.g. /search/..%2F..%2Fevil) is not browser normalisation and cannot be refused without breaking legit %2F in Spansh search names; no action.
