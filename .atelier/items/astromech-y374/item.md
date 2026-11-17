---
schema: atelier.item
schema_version: 3
id: astromech-y374
title: 'Replace chrono and chrono-tz: RustSec lists both as unmaintained, and deny.toml ignores them until then'
status: draft
mission: null
labels:
- size-l
- chore
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T13:27:00.986405153Z
---

## Goal

cargo-deny passes without ignoring an advisory. RustSec published RUSTSEC-2026-0543 (chrono is unmaintained) and RUSTSEC-2026-0555 (chrono-tz is unmaintained) on 14 Sep 2026, with no patched version and jiff or time as the alternatives. astromech-9uj3 found them once the SQLite advisory stopped hiding them, and added both to `deny.toml`'s `[advisories] ignore`, naming this item, so CI could go green.

chrono is a direct dependency of 11 workspace crates (about 90 `chrono::` uses: journal timestamps, the session clock, Powerplay cycle times, announcements). Unmaintained is informational: no known vulnerability, but no one will fix the next one.

## Done when

- [ ] No workspace crate depends on chrono or chrono-tz directly; the workspace uses jiff (or time, if a reason is written down here).
- [ ] `deny.toml` no longer ignores RUSTSEC-2026-0543 or RUSTSEC-2026-0555, and `cargo deny check` passes. If a third-party crate still pulls chrono in, the ignore stays for that crate alone, with the crate named.
- [ ] Journal timestamps, Powerplay cycle times and time-zone reads give the same values as before on the recorded fixtures: the existing tests pass unchanged.

## Scope

The time library only. No change to what any timestamp means or how it is shown.
