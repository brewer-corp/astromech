---
created_at: "2026-10-23T00:51:33.087358280+00:00"
id: "astromech-naqw"
evidence_type: "validation"
commit: "0f794799493b307a4e72cda7b9e8625817abea6a"
result: "pass"
captured_at: "2026-10-23T00:51:33.087249176+00:00"
agent_identity: "validator-e8ux-2"
target:
  kind: "issue"
  id: "astromech-e8ux"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e8ux"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S8 (re-check 23 Oct, at 0f794799 = main be64422f): pass. Claims: a live system view with body values, a system biology summary and the exobiology sampling helper, shown by journal replay; and the §18.7 M7 voice questions ('Is this system worth mapping?', 'which organics haven't I scanned here?') resolve and answer from replayed data (the 22 Oct fail astromech-ciqa: all five returned NotUnderstood). Ran fresh: cargo test -p astromech-exploration -p astromech-exobiology -p astromech-intent -p astromech-announce (with eddn: 351 tests, 0 fail), astromech-core --test exploration_voice 10/10 and --test copy 9/9 (synthetic journal: system value, worth mapping, biology left, all-sampled, unsold, return visit from history, tester dry run; no network, no keys); real_journals harness on a copy of ~/edjournal folds the current system (39 bodies with earlier visits) and unsold data, 0 problems. Independent scratch harness outside the repo driving the core Assistant with Origin::Voice over the real journal copy with the history imported: all five phrases from 22 Oct plus 'what's left to sample' and 'unsold data' route to exploration.worth_mapping/biology/system_value/unsold and answer, matching the System page fold (Shinrarta Dezhra: '1 body worth mapping, 875 thousand credits more. Founders World: Earth-like world, 1.2 million mapped.'; 'Worth 375 thousand credits now. 1.3 million credits with the best mapped.'; 'Unsold data worth 1.9 million credits.'; no biology there). Replaying a copy truncated at WISE 0713-2917 5's SAASignalsFound (real bio signal): 'Which organics haven't I scanned here?' -> '1 genus left to sample on 1 body. 5: Bacterium.' The non-voice parts (System page values, Biology page summary, sampling helper, callouts) are unchanged since the 22 Oct pass (no app/src changes to those pages since 327c059). Limits: the real journals have no ScanOrganic, so sampling progress in a voice answer is shown only on the synthetic journal; app pages not re-previewed this time; spoken output checked as text, not through TTS or speech recognition."
updated_at: "2026-10-23T00:51:33.088208134+00:00"
---

S8 (re-check 23 Oct, at 0f794799 = main be64422f): pass. Claims: a live system view with body values, a system biology summary and the exobiology sampling helper, shown by journal replay; and the §18.7 M7 voice questions ('Is this system worth mapping?', 'which organics haven't I scanned here?') resolve and answer from replayed data (the 22 Oct fail astromech-ciqa: all five returned NotUnderstood). Ran fresh: cargo test -p astromech-exploration -p astromech-exobiology -p astromech-intent -p astromech-announce (with eddn: 351 tests, 0 fail), astromech-core --test exploration_voice 10/10 and --test copy 9/9 (synthetic journal: system value, worth mapping, biology left, all-sampled, unsold, return visit from history, tester dry run; no network, no keys); real_journals harness on a copy of ~/edjournal folds the current system (39 bodies with earlier visits) and unsold data, 0 problems. Independent scratch harness outside the repo driving the core Assistant with Origin::Voice over the real journal copy with the history imported: all five phrases from 22 Oct plus 'what's left to sample' and 'unsold data' route to exploration.worth_mapping/biology/system_value/unsold and answer, matching the System page fold (Shinrarta Dezhra: '1 body worth mapping, 875 thousand credits more. Founders World: Earth-like world, 1.2 million mapped.'; 'Worth 375 thousand credits now. 1.3 million credits with the best mapped.'; 'Unsold data worth 1.9 million credits.'; no biology there). Replaying a copy truncated at WISE 0713-2917 5's SAASignalsFound (real bio signal): 'Which organics haven't I scanned here?' -> '1 genus left to sample on 1 body. 5: Bacterium.' The non-voice parts (System page values, Biology page summary, sampling helper, callouts) are unchanged since the 22 Oct pass (no app/src changes to those pages since 327c059). Limits: the real journals have no ScanOrganic, so sampling progress in a voice answer is shown only on the synthetic journal; app pages not re-previewed this time; spoken output checked as text, not through TTS or speech recognition.
