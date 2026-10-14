# Contributing

Thanks for your interest! astromech is pre-alpha, so expect big changes.

- Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) first, and open an issue before large changes.
- Before a PR: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `python3 scripts/check-deps.py`, `cargo deny check`, and in `app/`:
  `pnpm lint && pnpm check && pnpm build`.
- UI changes: run the Vite app (`cd app && pnpm dev`) and check it with headless Chrome:
  `scripts/ui --url http://127.0.0.1:5173/#/materials --wait Iron --screenshot /tmp/materials.png --text`.
  Needs Google Chrome (or `CHROME` pointing at Chromium). `scripts/app-shots.sh` and
  `scripts/mockup-shots.sh` use the same binary.
- Respect the dependency rules: actions go through `astromech-policy`; voice and tools never use
  the input executor directly.
- Player-visible game data (anything on the Elite Dangerous wiki) is fine in `data/`; credit its
  source in `NOTICE.md`. Never commit Frontier assets, copies of game files (default `.binds`),
  datamined data, models, or personal logs and results (journals, device paths, window titles).
  Use synthetic fixtures.
- The star map preload is not committed and not rebuilt per release. The manual **Star map
  preload** workflow (`.github/workflows/starmap.yml`) builds it from Spansh's systems dump and
  publishes `starmap.db.gz` on the `starmap` pre-release; tagged release builds download that.
  Refresh it only when it's worth it. Local shell builds bundle an empty file (the app fills in
  from EDSM as you travel); for the real one, `gh release download starmap` and gunzip it into
  `app/src-tauri/resources/starmap.db`.
- New dependencies must pass `cargo deny` (permissive licences only).
- Contributions are accepted under the MIT licence.
