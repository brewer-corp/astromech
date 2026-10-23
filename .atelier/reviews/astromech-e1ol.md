---
schema: atelier.review
schema_version: 1
id: astromech-e1ol
title: 'Reviewed git diff main...HEAD at 70f1895d6b71686794c8ef3f4fd2a6885d716b46 (sdg7 speakable rules, 550b speaker close and R1/R2, joma copy rule and copy.rs test, xi2i intent folds and fuel state, V1-DESIGN 6.5, PLOTTING.md, voice-commands guide, eval report, atelier records). Reran: cargo test --workspace (pass), fmt --check, clippy -D warnings on voice/core/intent, check-deps, check-guide (all clean). Mutation checks: each xi2i fold and the fuel state phrase removed makes eval_mishearings fail; removing the queue_reply closed check, the DrainOnExit closed flag, or restoring the old closed-last order each fails its speech test; speech tests 40/40 stable. Probed ~25 everyday phrases with the new folds: none executes a key-press command (engage/frameshift fold only into UNAVAILABLE commands; fuel state is a read-only answer). Not rerun: scripts/voice-eval.sh (report read, 0 wrong executes claimed). Contract: fail on two small points (B scale, NGC/IC sector names in the answer); fitness pass.'
target: astromech-uuqt
identity: review-agent-uuqt-1
commit: 70f1895d6b71686794c8ef3f4fd2a6885d716b46
created_at: 2026-10-22T22:59:21.022817145Z
event:
  event: review
---

Reviewed git diff main...HEAD at 70f1895d6b71686794c8ef3f4fd2a6885d716b46 (sdg7 speakable rules, 550b speaker close and R1/R2, joma copy rule and copy.rs test, xi2i intent folds and fuel state, V1-DESIGN 6.5, PLOTTING.md, voice-commands guide, eval report, atelier records). Reran: cargo test --workspace (pass), fmt --check, clippy -D warnings on voice/core/intent, check-deps, check-guide (all clean). Mutation checks: each xi2i fold and the fuel state phrase removed makes eval_mishearings fail; removing the queue_reply closed check, the DrainOnExit closed flag, or restoring the old closed-last order each fails its speech test; speech tests 40/40 stable. Probed ~25 everyday phrases with the new folds: none executes a key-press command (engage/frameshift fold only into UNAVAILABLE commands; fuel state is a read-only answer). Not rerun: scripts/voice-eval.sh (report read, 0 wrong executes claimed). Contract: fail on two small points (B scale, NGC/IC sector names in the answer); fitness pass.
