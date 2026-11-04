---
schema: atelier.review
schema_version: 1
id: astromech-nkvf
title: approval on astromech-ki3j @3057c961
target: astromech-ki3j
identity: review-agent-ki3j-1
commit: 3057c9616ce7d77b96589d0aee330e194166aa5d
created_at: 2026-11-04T21:17:15.262886190Z
event:
  event: approval
---

Re-reviewed bae3148e..3057c9616ce7d77b96589d0aee330e194166aa5d (74fbb4ba fix for astromech-xy2g, dispositions c9d5fcb2/0c87176e, evidence 91baf1ee, note 3057c961). 120 s slack: only a journal begun within 120 s of the followed one can now be switched to when touched (a session under two minutes), so the original bug comes back only for that shape, as the decision note says; real touched older journals are hours older. Startup-history safety is unchanged: which journal is followed doesn't affect note_startup_lengths or the opened/classify liveness rule, so a switch to a startup-held journal is still a replay up to its startup length. Future-header rule checked at runtime and at startup (catch_up falls back to newest when either header is untrusted; a followed untrusted one gives way). The five tests whose older headers moved from ts(30)/ts(60) to ts(600) keep their purpose: each still asserts that an older journal is not followed and that nothing from it is live. The move only takes them out of the new slack window; the within-slack follow-as-replay path is the same one the conflict-copy tests exercise. The four new xy2g tests cover a foreign journal ahead (runtime and startup) and a clock corrected back by more and by less than the slack. cargo test -p astromech-core -p astromech-journal: all pass (185 lib). clippy -D warnings and fmt --check: clean. One new medium defer finding (startup variant of the residual pin); no fix_now.
