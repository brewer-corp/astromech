---
schema: atelier.review
schema_version: 1
id: astromech-fbn0
title: 'Reviewed fix commit 9f94511 against yum5, davf, nwoo, irtf and merge a609322 (remerge-diff empty: clean). yum5: BoostUsed parsed via opt_f (integer accepted), any value > 0 skips the basis, so the 1-3 (synthesis injection grades) vs 4 (neutron/white-dwarf supercharge) mapping does not matter. davf: Loadout with a different ShipID clears last_jump; the first loadout does not. nwoo: once-per-stretch and no one-jump stretch is sensible (''Next star is scoopable.'' dropped; documented in fuel.rs and V1-DESIGN.md); the shared cooldown has a regression (fix_now finding). irtf: core compares until with the ledger; multiple ships merge to the highest until, consistent with the app planning each ship against the whole inventory; old stored format falls back to empty. Mutation checks: each new Rust test (boosted jump, ship swap, neutron route, route repeat, collected material) fails with its fix reverted. Checks at 6256a28557b84d63e9e7158bc0d6f516b24e2647: cargo fmt --check, clippy -D warnings, cargo test --workspace, cargo metadata --locked --offline (root and app/src-tauri), pnpm lint/check/test (151) all pass.'
target: astromech-77a1
identity: review-agent (77a1 fix review, sonnet)
commit: 6256a28557b84d63e9e7158bc0d6f516b24e2647
created_at: 2026-10-17T02:18:08.739343844Z
event:
  event: review
---

Reviewed fix commit 9f94511 against yum5, davf, nwoo, irtf and merge a609322 (remerge-diff empty: clean). yum5: BoostUsed parsed via opt_f (integer accepted), any value > 0 skips the basis, so the 1-3 (synthesis injection grades) vs 4 (neutron/white-dwarf supercharge) mapping does not matter. davf: Loadout with a different ShipID clears last_jump; the first loadout does not. nwoo: once-per-stretch and no one-jump stretch is sensible ('Next star is scoopable.' dropped; documented in fuel.rs and V1-DESIGN.md); the shared cooldown has a regression (fix_now finding). irtf: core compares until with the ledger; multiple ships merge to the highest until, consistent with the app planning each ship against the whole inventory; old stored format falls back to empty. Mutation checks: each new Rust test (boosted jump, ship swap, neutron route, route repeat, collected material) fails with its fix reverted. Checks at 6256a28557b84d63e9e7158bc0d6f516b24e2647: cargo fmt --check, clippy -D warnings, cargo test --workspace, cargo metadata --locked --offline (root and app/src-tauri), pnpm lint/check/test (151) all pass.
