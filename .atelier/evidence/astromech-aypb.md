---
created_at: "2026-11-07T16:24:29.964117082+00:00"
id: "astromech-aypb"
evidence_type: "validation"
commit: "b418f6047820c4f1e6e6335cb5035f00001315c7"
result: "pass"
captured_at: "2026-11-07T16:24:29.963998381+00:00"
uri: "https://github.com/brewer-corp/astromech/actions/runs/42384570600"
agent_identity: "astromech-7i2k implementer (Claude)"
target:
  kind: "issue"
  id: "astromech-7i2k"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-7i2k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "PR #218 at b418f6047820c4f1e6e6335cb5035f00001315c7, all checks pass. CI run 42384570629, job 'Windows (MSVC build + tests, app shell)' (128439136232): steps 'clippy (Windows-only code paths)', 'build', 'tests', 'app shell (Tauri) build' and 'app shell clippy' succeed; the best-effort step 'one input source, two subscribers, before and after restart' ran cargo test -p astromech-input --test source_sendinput -- --ignored: 'test two_subscribers_see_a_sent_key_before_and_after_restart ... ok', '1 passed; 0 failed ... finished in 1.32s' (both subscribers see a SendInput F24, a second keyboard registration in the process takes it away, both see it again after restart()). Release build run 42384570600, job 'Windows installer (build, install, smoke test)' (128439136245): onboarding smoke 'Result: **PASS**. Install to verified gear-down: 9.75 s', with steps 'push-to-talk and kill switch bound by pressing keys (1.25 s)', 'kill switch: a tap of F8 killed, a 1.2 s hold re-armed (session started after the app) (1.32 s)' and 'kill switch: a tap of F8 killed, a 1.2 s hold re-armed (game running before the app) (1.32 s)'. Also green on the same head: Linux job (run 42384570629), MVP criteria (42384570626), App window cost (42384570647), One process (Windows) probe and restart (42384570621), atelier doctor (42384570672)."
updated_at: "2026-11-07T16:24:30.235065086+00:00"
---

PR #218 at b418f6047820c4f1e6e6335cb5035f00001315c7, all checks pass. CI run 42384570629, job 'Windows (MSVC build + tests, app shell)' (128439136232): steps 'clippy (Windows-only code paths)', 'build', 'tests', 'app shell (Tauri) build' and 'app shell clippy' succeed; the best-effort step 'one input source, two subscribers, before and after restart' ran cargo test -p astromech-input --test source_sendinput -- --ignored: 'test two_subscribers_see_a_sent_key_before_and_after_restart ... ok', '1 passed; 0 failed ... finished in 1.32s' (both subscribers see a SendInput F24, a second keyboard registration in the process takes it away, both see it again after restart()). Release build run 42384570600, job 'Windows installer (build, install, smoke test)' (128439136245): onboarding smoke 'Result: **PASS**. Install to verified gear-down: 9.75 s', with steps 'push-to-talk and kill switch bound by pressing keys (1.25 s)', 'kill switch: a tap of F8 killed, a 1.2 s hold re-armed (session started after the app) (1.32 s)' and 'kill switch: a tap of F8 killed, a 1.2 s hold re-armed (game running before the app) (1.32 s)'. Also green on the same head: Linux job (run 42384570629), MVP criteria (42384570626), App window cost (42384570647), One process (Windows) probe and restart (42384570621), atelier doctor (42384570672).
