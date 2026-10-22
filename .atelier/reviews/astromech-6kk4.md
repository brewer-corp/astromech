---
schema: atelier.review
schema_version: 1
id: astromech-6kk4
title: 'Verified fix_now resolutions for h9kb and zdy2 at main (fa3b668e). h9kb (a17364cf): normalise joins ''i''+''ll'' to ''iwill'' (lib.rs:476), not ''ill''; MISHEARD maps iwill->all with score min(MISHEARD_CAP=0.80) (lib.rs:843/847); all_stop_mishearings_ask_and_never_execute asserts score <= MISHEARD_CAP for I''ll/I’ll stop and I''ll stop here/now/please, and a later test (astromech-rcbi) keeps literal ''ill stop''/''al stop'' < EXECUTE. CARGO_BUILD_JOBS=4 cargo test -p astromech-intent: 29 passed, 0 failed. zdy2 (c0cdbf39): docs/evidence/astromech-y296/{report,beam-search-report,hotwords-report}.md exist; ARCHITECTURE.md:668 links the directory; quoted 3101/3136 matches beam/hotwords reports and 3105 greedy baseline matches docs/evidence/astromech-yil0/report.md (referenced from beam-search-report). git merge-base --is-ancestor: a17364cf and c0cdbf39 are ancestors of 3764f8a6, and 3764f8a6 is an ancestor of main.'
target: astromech-y296
identity: review-agent-y296-3
commit: 3764f8a6c188867b6d7e7f7e060a69acbfddd2b7
created_at: 2026-10-22T23:11:51.767699025Z
event:
  event: approval
---

Verified fix_now resolutions for h9kb and zdy2 at main (fa3b668e). h9kb (a17364cf): normalise joins 'i'+'ll' to 'iwill' (lib.rs:476), not 'ill'; MISHEARD maps iwill->all with score min(MISHEARD_CAP=0.80) (lib.rs:843/847); all_stop_mishearings_ask_and_never_execute asserts score <= MISHEARD_CAP for I'll/I’ll stop and I'll stop here/now/please, and a later test (astromech-rcbi) keeps literal 'ill stop'/'al stop' < EXECUTE. CARGO_BUILD_JOBS=4 cargo test -p astromech-intent: 29 passed, 0 failed. zdy2 (c0cdbf39): docs/evidence/astromech-y296/{report,beam-search-report,hotwords-report}.md exist; ARCHITECTURE.md:668 links the directory; quoted 3101/3136 matches beam/hotwords reports and 3105 greedy baseline matches docs/evidence/astromech-yil0/report.md (referenced from beam-search-report). git merge-base --is-ancestor: a17364cf and c0cdbf39 are ancestors of 3764f8a6, and 3764f8a6 is an ancestor of main.
