---
schema: atelier.review
schema_version: 2
id: astromech-1vi3
title: finding on astromech-y6d6 @e9dea942
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: e9dea94204159f14c934bcc2c0de5b6d7240d3d4
created_at: 2026-11-07T14:05:57.814268238Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: 579c3a8ebda8dbeb856438298aa50b1fb3b7ac3d
    recorded_at: 2026-11-07T14:07:21.320075611Z
    follow_up: astromech-dl53
    reason: Noted on dl53
---

low: .atelier/issues/astromech-dl53.md (CI job bullet and Notes) and wgtr's Report say 'WER never engages on the runner'. But fqw9's own data shows WerSvc going from stopped to running during the 'runner WER' configuration, before the workflow step that starts it, and the Report lists that under Surprises. Something triggered WER's service. Neither the probe nor the record captures AeDebug (HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\AeDebug, which Visual Studio on the image can set) or WER LocalDumps, either of which can divert a crash from WER's restart path. The inconclusive verdict stands. The fix is wording: 'no restart, report or WerFault was observed', not 'never engaged'. Also, dl53's 'Records, not asserted' step should log AeDebug and the WerSvc state before and after each crash, so the next run can tell the causes apart.
