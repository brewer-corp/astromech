---
schema: atelier.review
schema_version: 3
round: 4
commit: a552a939d4a4b8e46168079a5a936c0e7dcd836b
reviewers:
- thread:delegated-task:command%3Amcp%3A059c3bbf-b3b2-4306-83e5-79d4c716a004%3Adelegate-task%3Ad098c551-8b99-4755-be62-4b9ae2c507f2
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T10:26:18.740154947Z
---

## Summary

Complexity: high. Persisted route metadata, source precedence, a details cache and replot settings form a contract across the plotters, follower, API and preview.

Value: high. The Route timeline needs these fields to distinguish listed jumps from count-only legs and known star data from unknowns.

Risk of merging: medium. Incorrect navigation metadata reaches the commander, but the change is reversible and does not modify game files.

The value warrants the scope. I found no substantially smaller change that supplies the required source contracts, and no problem requiring changes.

Reviewed `main...a552a939d4a4b8e46168079a5a936c0e7dcd836b` against every Done when line and the Report. Read the route persistence and timeline code, plotter and CSV mappings, NavRoute handoff, stop-details lookup and cache, replot command path, generated bindings, fixture provenance and documentation. Checked the earlier findings against the current implementation and regression cases: startup NavRoute is known without following or copying, failed detail batches preserve successful batches and allow later requests, and matching plotter flags survive a missing NavRoute star class.

Also inspected the merge from main since round 3, including its combined diff and the conflict resolution in `app/src/preview/fakecore.ts`. The route implementation, plotters, replot handler, shell preview handler and timeline fixture are unchanged since that approval. The resolved preview imports and samples preserve both the route timeline and main's ship-status sample; the combined command registry and preview fixtures pass their checks.

Validation passed:

- `cargo test -p astromech-core --lib route -- --nocapture` (35 tests).
- `cargo test -p astromech-core --lib bindings` (2 tests).
- `cargo test -p astromech-announce route_follow` (5 tests).
- `cargo test -p astromech-galaxy --test stops --test plot --test plot_route` (14 tests).
- `cargo test -p astromech-app --test preview_fixtures --test every_window_command` (4 tests, including fixture drift and both new commands).
- `python3 scripts/check-deps.py` and `python3 scripts/check-guide.py`.
- In `app/`, `pnpm check`, `pnpm lint`, and `pnpm exec vitest run src/modules/route src/preview/coverage.test.ts src/preview/fakecore.contract.browser.test.ts src/preview/fakecore.preview.browser.test.ts` (55 tests).

`git diff --check` reports trailing whitespace in ts-rs-generated bindings; the binding drift test confirms those files match the generator. I changed no source file. I did not rerun the full workspace suite, clippy, cargo-deny, frontend build or journeys, or validate live Spansh and Elite on Windows. The Report's broader validation claims were not independently rerun in this review.
