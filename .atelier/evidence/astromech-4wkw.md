---
created_at: "2026-11-04T22:09:08.213792157+00:00"
id: "astromech-4wkw"
evidence_type: "validation"
commit: "c1c1f0cd7d702f6d52cd902e8dc84f0806097e0b"
result: "pass"
captured_at: "2026-11-04T22:09:08.213754787+00:00"
path: "docs/evidence/astromech-8e8m/report.md"
agent_identity: "implement-agent-8e8m"
target:
  kind: "issue"
  id: "astromech-8e8m"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-8e8m"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Local emulation of the push-event job (workflow run: scripts executed in a scratch clone, Atelier built at ATELIER_REV 1f5fd1c2, commit c1c1f0cd): (a) one commit marking leaf astromech-1b43 done on top of before=1d2219d7: doctor step passes, land step fails land_base_done_claim, exit 1; (b) real reviewed range eb74ee23..1d2219d7 (atelier merge integrations, 47 commits): both steps pass; (c) records-only planning commit (atelier issue create): both pass; (d) README commit on main: land fails land_base_unreviewed; (e) before all zeros: land step skipped, exit 0; (f) force push with before not an ancestor: passes, checks only the new commit; (g) unfetchable before: step fails (exit 128). Not run on GitHub."
updated_at: "2026-11-04T22:09:08.214066199+00:00"
---

Local emulation of the push-event job (workflow run: scripts executed in a scratch clone, Atelier built at ATELIER_REV 1f5fd1c2, commit c1c1f0cd): (a) one commit marking leaf astromech-1b43 done on top of before=1d2219d7: doctor step passes, land step fails land_base_done_claim, exit 1; (b) real reviewed range eb74ee23..1d2219d7 (atelier merge integrations, 47 commits): both steps pass; (c) records-only planning commit (atelier issue create): both pass; (d) README commit on main: land fails land_base_unreviewed; (e) before all zeros: land step skipped, exit 0; (f) force push with before not an ancestor: passes, checks only the new commit; (g) unfetchable before: step fails (exit 128). Not run on GitHub.
