---
created_at: "2026-10-17T19:37:35.948104079+00:00"
id: "astromech-st8o"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDSM uploader has no timestamp-age guard on live events"
updated_at: "2026-10-17T19:37:35.948104079+00:00"
---

## Goal

EDDN drops a live journal line whose timestamp is more than 5 minutes old (feed.rs MAX_EVENT_AGE_SECS) in case a journal is re-read as live (backup tool, another client). The EDSM uploader (edsm/mod.rs Uploader::journal) trusts the live flag alone, so the same re-read would upload old lines. Decide whether EDSM should apply the same guard.

## Done when

Either the EDSM uploader applies the same age guard with a test, or the guide and a code comment record why it does not. Found while fixing astromech-koa6, whose guide wording now says EDSM has no timestamp check.
