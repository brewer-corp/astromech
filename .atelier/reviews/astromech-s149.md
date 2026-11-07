---
schema: atelier.review
schema_version: 2
id: astromech-s149
title: approval on astromech-y6d6 @8fa5f13a
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: 8fa5f13aae0ac0a37ff4468c0ed2b2e35d22a6c6
created_at: 2026-11-07T20:16:58.778644208Z
event:
  event: approval
---

Re-reviewed 9lh3's fixes in 74e8546b (findings kfub, s3rq, ilwv, 81t0). kfub: call() returns r['value'] when present, else the bare reply; a stand-in server sending alpha.4's {ok, version, pid} now passes old_running's status wait, and wrapped and null values still come back as before; the only other status wait (one_app) treats a bare old reply as a version mismatch and keeps waiting. s3rq: the upgrade job pins v1.0.0-alpha.4, with the workflow comment, CONTRIBUTING, V1-DESIGN §7.8 and the script's docstring saying why; follow-up astromech-7pl5 covers one-process-to-newer upgrades. ilwv: V1-DESIGN §7.5 names the interactive setup.exe upgrade and y2m1. 81t0: LOCKED_EXES comment corrected; backlog astromech-mqqa filed. 8fa5f13a only records v3u4's CI evidence. Approval covers the code at HEAD; PR #218's checks, including the upgrade job, were still pending when this was recorded, so Done when lines 1, 2, 4, 6 and 7 still need that CI run.
