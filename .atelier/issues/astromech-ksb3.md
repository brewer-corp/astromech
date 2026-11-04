---
created_at: "2026-10-22T21:22:09.269392549+00:00"
id: "astromech-ksb3"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T21:05:29.210846464+00:00"
status: "done"
title: "Neutron download: log why the download could not be reached"
updated_at: "2026-11-04T21:05:29.210846464+00:00"
---

## Goal

When the neutron star file can't be downloaded (TLS or proxy error, DNS failure, a transfer cut short), the shell log says why, and the page keeps its plain sentence.

## Done when

`DownloadError::Offline` carries the underlying error, the shell writes it to astromech-shell.log, and a test shows a refused connection's reason reaching the error value.

## Context

Review finding R7 (astromech-h7cb) on feature/astromech-1842: crates/astromech-data/src/download.rs:16. A user behind a corporate proxy sees 'Cannot reach the download' with nothing in the log.

## Close Reason

DownloadError::Offline already held the transport error's text; it was never logged because the shell logs the error's Display (the sentence). Added DownloadError::detail() and a shell helper download_detail() that downcasts the anyhow error and appends ' (cause)' to the astromech-shell.log line for neutron_download and neutron_check failures. The page still gets only the sentence. Test: a_refused_connection_keeps_its_reason. Limit: the shell change is verified by Windows CI's build only, not run; the voice-model download (astromech-voice models.rs) has the same gap and was left alone (out of this issue's scope).
