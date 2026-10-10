# Contributing

Thanks for your interest! astromech is pre-alpha, so expect big changes.

- Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) first, and open an issue before large changes.
- Before a PR: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `python3 scripts/check-deps.py`, `cargo deny check`, and in `app/`:
  `pnpm lint && pnpm check && pnpm build`.
- Respect the dependency rules: actions go through `astromech-policy`; voice and tools never use
  the input executor directly.
- Never commit game data, Frontier assets, copies of default `.binds` files, models, or personal
  logs and results (journals, device paths, window titles). Use synthetic fixtures.
- New dependencies must pass `cargo deny` (permissive licences only).
- Contributions are accepted under the MIT licence.
