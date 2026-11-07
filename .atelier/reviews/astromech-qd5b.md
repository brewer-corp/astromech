---
schema: atelier.review
schema_version: 2
id: astromech-qd5b
title: approval on astromech-bp23 @8197eae3
target: astromech-bp23
identity: claude-reviewer-8s6k
commit: 8197eae3aaea51f8468babfeca01b516675004b2
created_at: 2026-11-07T06:52:38.114232523Z
event:
  event: approval
---

Review of astromech-8s6k (a4a183c7..8197eae3) against its Done when, bp23, decisions unh9/6i8s/1zhg. Read astromech-api (registry, control), Core::commands vs the deleted handle match (54 entries; window set equals the deleted ALLOWED list; quit/open_window local), window.rs call_with/status/core_alive/core_status/watchdog/route_set caller and its imports by inspection (window feature not built here), instance.rs, main.rs --quit, app.rs second start, onboarding_smoke.py, release.yml smoke, installer --quit, check-deps, ARCHITECTURE 4. Reran fresh: cargo test -p astromech-api 7/7; cargo test --workspace 1263 passed 0 failed; clippy -D warnings, fmt --check, check-deps, cargo deny clean; headless core under temp XDG/LOCALAPPDATA: status envelope, quit/open_window with origin app_window 'not allowed', unknown command, value-level route error, bogus origin bad request, second start, --quit exit 0 and core.json removed. Three defer findings (emsi, xrwd, srpi). Outstanding before merge: Windows CI shell build/clippy with --features window and the Release install and onboarding smokes on the PR.
