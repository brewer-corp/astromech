---
schema: atelier.review
schema_version: 3
round: 3
commit: a3eb5f763348099d038cb272f617e8c2812d2575
reviewers:
- thread:delegated-task:command%3Amcp%3A059c3bbf-b3b2-4306-83e5-79d4c716a004%3Adelegate-task%3A00d4e145-200e-4a80-823f-888b49f9a207
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T10:21:15.275457085Z
---

## Summary

Complexity: high. The change adds route metadata, saved plot settings, NavRoute source selection, a stop-details cache, commands, and a reply contract shared by Rust and the preview.

Value: high. The Route timeline needs these fields to distinguish listed jumps from count-only legs and known star data from unknowns.

Risk of merging: medium. Incorrect navigation metadata reaches the player, but the change is reversible and does not modify game files.

The value warrants the scope. I found no substantially smaller change that supplies the required source contracts. The carried finding is fixed, and I found no further problems requiring changes.

I reviewed the change at a3eb5f763348099d038cb272f617e8c2812d2575 against all six Done when lines and the Report, including the diff since round 2. I checked the neutron, exact, local and CSV mappings; the documented per-source contract; NavRoute precedence and class-derived scoopability; journal position; optional persisted plot settings and the replot command path; per-stop caching and partial lookup failures; generated bindings; and the preview world's recorded inputs.

For Finding 1, NavRoute still supplies the jump list and classes first. Matching plotter flags now fill missing flags before `JumpView::of` resolves the class-first result. I independently exercised the real headless core in a temporary copy of the preview route world: its startup NavRoute listed Sol and Tascheter Sector MN-T b3-6 with the latter's optional star class absent, and `route_set` received the checked-in recorded exact route. The first leg returned `hops_from: nav_route`, position 0, and a jump with `star_class: null`, `scoopable: yes`, and `neutron: false`, agreeing with the stop. The regression test and existing neither-source-knows case also pass.

Validation passed:

- `cargo test -p astromech-core --lib route -- --nocapture` (35 tests).
- `cargo test -p astromech-core --lib bindings` (2 tests).
- `cargo test -p astromech-announce route_follow` (5 tests).
- `cargo test -p astromech-galaxy --test stops --test plot --test plot_route` (14 tests).
- `cargo test -p astromech-app --test preview_fixtures --test every_window_command` (4 tests, including fixture drift and both new commands).
- `python3 scripts/check-deps.py` and `python3 scripts/check-guide.py`.
- In `app/`, `pnpm check` and `pnpm exec vitest run src/modules/route src/preview/coverage.test.ts src/preview/fakecore.contract.browser.test.ts src/preview/fakecore.preview.browser.test.ts` (55 tests).

I changed no source file. I did not rerun the full workspace, clippy, cargo-deny, frontend lint/build, or journeys, and did not validate live Spansh or Elite on Windows. The Report's broader check claims are not independently verified here.

## Finding 1: Preserve plotter flags when NavRoute omits a star class
Recommendation: fix now
Was: 2-x15q.3
Response: fixed in 32558e9b: a NavRoute jump with no star class takes the plotter's scoopable and neutron flags for the same system; test `a_galaxy_map_jump_with_no_class_keeps_the_plotters_flags`.
Now: fixed
