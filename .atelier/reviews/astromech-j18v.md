---
schema: atelier.review
schema_version: 2
id: astromech-j18v
title: finding on astromech-y6d6 @ab14505e
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: ab14505e796bee36cf0670bb126f90c2319b831c
created_at: 2026-11-07T14:07:55.333358461Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/astromech-y6d6.md:53 now says Windows CI shows a faked --restarted start 'with the kill switch and Pause voice and the End task behaviour as decision astromech-lkwj says'. CI can't show the End task behaviour: dl53's CI job covers only the restored state, and TerminateProcess stays off or comes back only on Sumeet's PC (dl53's after-merge line), which y6d6:58 now covers. Fix when dl53 lands: move 'and the End task behaviour' out of the CI sentence.
