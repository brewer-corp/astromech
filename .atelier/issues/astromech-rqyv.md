---
created_at: "2026-10-15T20:36:13.657456145+00:00"
id: "astromech-rqyv"
issue_type: "decision"
labels:
- "needs-human"
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-rh44"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Sign v1.0, or ship it unsigned?"
updated_at: "2026-10-15T20:36:13.657456145+00:00"
---

## Question

Should v1.0 ship as a signed Windows build?

## Options

- **Sign through the SignPath Foundation** (ARCHITECTURE §16): free for OSS. SignPath needs an OSI licence for every component (GPL-3.0 binaries qualify) and may hesitate over software that injects input. Removes the SmartScreen warning once reputation builds.
- **Ship unsigned**, as releases do today (V1-DESIGN §7.4 and §9 #7, the README, `release.yml`). Users click through SmartScreen; the install docs explain it.

## Context

When decided, update V1-DESIGN or ARCHITECTURE §16 to match. The v1.0 release item (astromech-rh44) waits on this.
