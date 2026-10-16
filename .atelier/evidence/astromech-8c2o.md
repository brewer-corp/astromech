---
created_at: "2026-10-16T22:14:15.509748697+00:00"
id: "astromech-8c2o"
evidence_type: "validation"
commit: "d802250a228aaa0244ebfb10733b355ce22e53e5"
result: "pass"
captured_at: "2026-10-16T22:14:15.509641888+00:00"
target:
  kind: "issue"
  id: "astromech-j4c9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-j4c9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At d802250 plus the lint fix: cargo fmt --all, cargo clippy --workspace --all-targets -D warnings (clean), cargo test --workspace (exit 0, no failures), scripts/check-deps.py OK (20 crates), app pnpm lint/check/build/test pass (73 tests), app/src-tauri cargo metadata --locked --offline OK. scripts/ui screenshots of #/odyssey (Inventory goods and assets, Blueprints with three sample picks and the shopping list, Engineers) read and correct (local only, ~/.cache, deleted). Not run here: cargo deny (not installed; the crate adds no external dependency) and a build of app/src-tauri (needs Windows: webkit2gtk missing on Linux and the msvc cross-check stops at a C dependency needing lib.exe), so the two new shell commands are checked by Windows CI only."
updated_at: "2026-10-16T22:14:15.510451338+00:00"
---

At d802250 plus the lint fix: cargo fmt --all, cargo clippy --workspace --all-targets -D warnings (clean), cargo test --workspace (exit 0, no failures), scripts/check-deps.py OK (20 crates), app pnpm lint/check/build/test pass (73 tests), app/src-tauri cargo metadata --locked --offline OK. scripts/ui screenshots of #/odyssey (Inventory goods and assets, Blueprints with three sample picks and the shopping list, Engineers) read and correct (local only, ~/.cache, deleted). Not run here: cargo deny (not installed; the crate adds no external dependency) and a build of app/src-tauri (needs Windows: webkit2gtk missing on Linux and the msvc cross-check stops at a C dependency needing lib.exe), so the two new shell commands are checked by Windows CI only.
