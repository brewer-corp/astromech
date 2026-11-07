---
schema: atelier.review
schema_version: 2
id: astromech-sj9e
title: finding on astromech-y6d6 @e9dea942
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: e9dea94204159f14c934bcc2c0de5b6d7240d3d4
created_at: 2026-11-07T14:05:56.897933815Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/astromech-dl53.md (after-merge line) says 'a TerminateProcess (taskkill /F) stays off for 90 s' as a fixed expectation. The text it replaced said 'per the decision'. What End task does is option 2 of decision astromech-lkwj, which is still Sumeet's and open (2b = it comes back). As written, the check assumes 2a and would test the wrong behaviour if he picks 2b. Fix: 'stays off or comes back as decision astromech-lkwj says', as the CI restored-state bullet already does.
