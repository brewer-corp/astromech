---
schema: atelier.review
schema_version: 3
round: 4
commit: 801d93860f518cd7628fe6c1e5c0a8357477fa3a
reviewers:
- thread:delegated-task:command%3Amcp%3Aa2ff8ecd-9bd5-430d-a2ff-147ae09eea93%3Adelegate-task%3A631d54e4-0245-4c47-9d3e-e1bf96c610b7
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T11:43:49.008026310Z
---

## Summary

Approve commit `801d93860f518cd7628fe6c1e5c0a8357477fa3a`. There are no open fix-now findings.

- **Complexity: high.** The default page coordinates several independent polls, retains answers across failures and manages a keyed asynchronous cargo recommendation, alongside shared frame state and navigation changes.
- **Value: high.** Opening on the commander's current activity is the item's intended outcome; navigation alone would not provide the requested live answers.
- **Risk of merging: medium.** This changes the default view and actionable cargo advice, but refreshes and failures are visible, regression cases cover them, and the change can be reverted.

The value warrants the scope. Existing commands, saved finder filters, shared error components and layout tokens keep it proportionate. I found no materially smaller change that delivers the same outcome.

Reviewed the full diff against main and the revision since round 3. Checked the Done when lines against navigation and unchanged addresses, the station head and ordered services board, conditional blocks and main-value selection, cargo invalidation and stale-response suppression, shared takes wording, failure retention and recovery, preview events and fixture generation, lint enrollment and guide changes. Traced the cargo lookup through its current origin, ship and Cargo.json inputs. The Now-specific revision since round 3 changes the imported Powerplay formatter from `until` to main's `timeLeft`; its arguments and output usage agree with the renamed helper.

Independently passed, with full test suites run sequentially:

- `pnpm check` (zero errors or warnings) and `pnpm lint`.
- `pnpm test` (94 files, 800 tests), including Now's retained-answer, initial-failure, recovery, jump and unchanged-key cases, plus the navigation and pure-function tables.
- `pnpm exec tsc -p journeys` and all seven journey tests.
- `cargo test -p astromech-app --test preview_fixtures` (three tests, without fixture updates).
- `python3 scripts/check-guide.py` (23 pages, zero problems).
- `pnpm build` and `python3 scripts/check-dist.py app/dist` (no preview or dev bridge in the production bundle).
- `pnpm bridge-check` (assertions pass and exit zero; its existing shutdown diagnostic is recorded below).

Inspected the committed docked 1100 x 700 and in-space 2560 x 1440 captures, capture plans and all nine audit entries. The listed geometry and contrast checks are empty, and sidebar modes match the specified sizes. I did not recapture the audit or repeat the six header-width measurements. I did not test Windows/WebView2, a live Elite session or live Ardent, or rerun the full Rust workspace checks. The risky exploration-data warning remains checked from source rather than a positive recorded fixture. Only this review file was edited manually.

## Finding 1: Bridge check still logs dependency-scan errors after passing
Recommendation: defer

This is the existing deferred finding `3-8r87.2`, also recorded as `2-z8l9.2`, rather than a regression in this revision. `app/dev/bridge-check.ts:19` starts Vite with normal dependency discovery; its `finally` closes the server at line 54 as soon as the HTTP-only assertions finish. In this review, `pnpm bridge-check` again printed its successful real-core comparison and exited zero, then logged "Failed to run dependency scan" and "The server is being restarted or closed. Request is outdated" from `vite:dep-scan`.

Give this bridge-only check dependency-discovery settings appropriate to its use, or coordinate discovery with shutdown. The diagnostic makes a passing check look broken, but this script is unchanged by the item, its assertions pass, and the journeys and production build pass independently. It remains later tooling work and does not block Now.
