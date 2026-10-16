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
closed_at: "2026-10-16T18:20:01.755996922+00:00"
status: "done"
title: "Sign v1.0, or ship it unsigned?"
updated_at: "2026-10-16T18:20:01.755996922+00:00"
---

## Question

Should v1.0 ship as a signed Windows build, and through which route?

## Options

- **SignPath Foundation** (ARCHITECTURE §16.3): free for open source. Every release needs manual approval, and the publisher shown is "SignPath Foundation". It needs an OSI licence for every component: GPL-3.0 binaries qualify, but how it treats the runtime-downloaded `DirectML.dll` is unverified. Approval isn't guaranteed for software that injects input.
- **Azure Artifact Signing**: about $10 a month (third-party figure, unverified against Microsoft's page). Individuals in the US and Canada are eligible after identity checks. The publisher shown is Sumeet's legal name. No per-release approval, and it signs from GitHub Actions.
- **Ship unsigned**, as releases do today (V1-DESIGN §7.4, `release.yml`). Users click through SmartScreen; the install docs explain it.

Any signed route still builds SmartScreen reputation over time; no certificate type skips that now. The CI work (signing the installer, the core and the voice worker) is about the same for both signed routes.

## Decision

Ship v1.0 unsigned (Sumeet, 2026-10-16). Revisit signing once there is evidence people use astromech; Azure Artifact Signing is the likely route then. V1-DESIGN §7.4 and ARCHITECTURE §16.3 already describe unsigned releases, so no doc change is needed.

## Close Reason

Ship v1.0 unsigned; revisit signing once astromech has users.
