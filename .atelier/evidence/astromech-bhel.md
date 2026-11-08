---
created_at: "2026-11-08T16:29:28.513468684+00:00"
id: "astromech-bhel"
evidence_type: "validation"
commit: "9402d4b94bb62f3016357090b1e881513ac5d2af"
result: "pass"
captured_at: "2026-11-08T16:29:28.513422699+00:00"
uri: "https://github.com/brewer-corp/astromech/actions/runs/42496995856"
agent_identity: "claude-orchestrator-kjfs"
target:
  kind: "issue"
  id: "astromech-kjfs"
  role: "validates"
criteria:
- slug: "no-fake-in-release"
  result: "pass"
criteria_mission: "astromech-kjfs"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kjfs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "ci.yml on main at 64ee930a: the Linux job builds the release frontend and runs scripts/check-dist.py app/dist, which fails on any preview transport or fixture string; it reported 'check-dist: 76 files in app/dist, no browser preview or dev bridge'."
updated_at: "2026-11-08T16:29:28.693499049+00:00"
---

ci.yml on main at 64ee930a: the Linux job builds the release frontend and runs scripts/check-dist.py app/dist, which fails on any preview transport or fixture string; it reported 'check-dist: 76 files in app/dist, no browser preview or dev bridge'.
