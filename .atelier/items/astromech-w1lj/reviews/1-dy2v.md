---
schema: atelier.review
schema_version: 3
round: 1
commit: 864c124d637c47d41fb77c8c50827f68ca5b5678
reviewers:
- thread:delegated-task:command%3Amcp%3Ab752400b-8fa8-44ff-b37d-ad8958e994a0%3Adelegate-task%3Ad521ccae-a1c9-4f5d-92b8-329aa9311765
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T11:03:41.366592449Z
---

## Summary

- **Complexity: high.** Adds replay state for loadouts and the worn id, plus a contract shared by Rust, generated TypeScript, and preview fixtures.
- **Value: high.** The planned Ships > Suits page needs loadout membership and stable owned-item ids to attach plans to gear shared between loadouts.
- **Risk of merging: medium.** Changes the data returned to the window and pick identity, but is reversible and does not execute game actions or migrate saved picks.

The value justifies the change. The implementation reuses the existing journal reader and owned-gear maps; a substantially smaller change would omit a required part of the contract. No findings.

Reviewed `git diff main...864c124d637c47d41fb77c8c50827f68ca5b5678` against every Done when line and the Report. Checked that all three loadout events replace membership by LoadoutID, only SuitLoadout and SwitchSuitLoadout change the worn id, Flight Suit events leave no kept loadout worn, slots have deterministic order, and loadouts resolve suits and weapons through the current owned records. Existing purchase, upgrade, sale, grade and mod handling remains intact. Gear seen only in loadout events is still added. Reviewed the table cases for switching, slot movement, previously unseen gear, shared suits, and Flight Suit exclusion.

Checked both Pick variants for backwards-compatible deserialization and omitted absent item fields. Shopping still costs each input pick once and returns its item id; the new table covers separate items and mixed old/new picks. The item-aware pickKey is consistent with duplicate detection, removal, reply matching, and the preview's sample matcher. No command-map edit is needed because its existing types reference the regenerated OdysseyView and Pick types.

Inspected the preview journal and regenerated fixture: four ordered loadouts, exactly one worn, the first G1 Maverick shared by two loadouts, a distinct G1 Maverick in another, and the G1 P-15. The sample picks target the intended journal ids. The Report's fixture adjustments are reflected in the inputs and output.

Independently passed:

- `cargo fmt --all -- --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test -p astromech-odyssey` (18 tests).
- `cargo test -p astromech-app --test preview_fixtures` (3 tests, including fixture drift).
- `cargo test -p astromech-core --lib bindings` (2 tests, including checked-in generated types).
- In `app/`, `pnpm check` (0 errors and warnings), then `pnpm test` (758 tests across 90 files).

Did not run the whole workspace test suite, a release build, or journeys. Did not verify events in the running game. The item's documented gaps around rename/delete/equip/remove events and startup SuitLoadout emission remain outside this change and are already tracked by astromech-d0zh and astromech-t8hg.
