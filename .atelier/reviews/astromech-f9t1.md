---
schema: atelier.review
schema_version: 2
id: astromech-f9t1
title: finding on astromech-zk7z @71bf8e6e
target: astromech-zk7z
identity: independent reviewer subagent (claude-opus-5-5), atelier-mmpv rollout
commit: 71bf8e6efe219ea91fe03b9a89259e23ce8c5f84
created_at: 2026-11-07T14:55:57.881123850Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/astromech-zk7z.md:29 and the Report. Once this merges, the in-flight branches feature/astromech-v6r3, astromech-y6d6 and astromech-vucf cannot use atelier merge until they sync main in. On a scratch clone, after atelier merge of this branch into main, `atelier merge <id> --check` on each unsynced branch stops with workflow_issue_status_invalid on astromech-278z ('backlog' not valid for on_base), although the merged tree (git merge-tree) has 278z in todo: Atelier judges the branch's own records by main's new policy. `atelier land`, the Stop hook, prints the same error on those branches at every handoff. `atelier merge <id> --sync` clears it: clean on all three, a stand-in approval on v6r3 was kept, and merge --check then shows only each branch's own gates. The Report says v6r3 'still merges cleanly' and names no sync, and Agent Factory's reference gives no such reason to sync, so their agents meet an unexplained error. Fix: say in the Report that each in-flight branch runs `atelier merge <id> --sync` after this lands, tell their owners, and file an Atelier bug (merge and land should judge the merged records, or a branch's records by the branch's policy).
