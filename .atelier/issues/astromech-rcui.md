---
created_at: "2026-10-20T18:48:41.225650431+00:00"
id: "astromech-rcui"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ship an alpha with the brewer-corp updater feed"
updated_at: "2026-10-20T18:48:41.225650431+00:00"
---

## Goal

alpha.1 and alpha.2 reach updates only through GitHub's redirect from spadaval/astromech (see astromech-u6ky). Ship the next alpha with the brewer-corp feed and download URLs, so installs that update to it stop depending on the redirect.

## Done when

- A tagged alpha built from a main that includes astromech-u6ky is published on brewer-corp/astromech.
- Its `latest.json` `url` and the feed it compiles in point at brewer-corp.
- Before tagging: `ASTROMECH_UPDATE_KEY` is present on brewer-corp/astromech (it moved with the transfer on 2026-10-20). Without it, release.yml fails rather than publishing unsigned.
