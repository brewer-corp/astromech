---
schema: atelier.review
schema_version: 1
id: astromech-4t93
title: approval on astromech-8e8m @f98697e3
target: astromech-8e8m
identity: review-agent-8e8m-1
commit: f98697e34ec33696a67d9b4585c31dfa61e3a3ba
created_at: 2026-11-04T23:05:10.792946159Z
event:
  event: approval
---

Reviewed 1d2219d7..f98697e3: push-only atelier land step (github.event.before semantics, checkout fetch-depth 0 + SHA ref leaves origin remote/refspec so main@{upstream} resolves, fetch of before fails closed, !cancelled()), land at pin 1f5fd1c2 makes no commits/fetches/pushes without --push (session.rs land_checked, git.rs check_base_commits), PR path unchanged, CONTRIBUTING accurate. R1 (astromech-54wb) fixed in 5c8c2e19: push runs grouped per sha so no push's land check is cancelled or replaced; PR runs still cancel superseded ones. Re-ran emulation (a)-(g) in a fresh clone against the current workflow: same results (a,d refuse; b,c,e,f pass; g exits 128). Added: GitHub merge-button merge of reviewed az6g passes, ff passes, squash fails (already refused by doctor). Not run on GitHub; YAML parsed with PyYAML, actionlint not run.
