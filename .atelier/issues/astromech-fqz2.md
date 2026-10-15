---
created_at: "2026-10-15T20:30:24.606880852+00:00"
id: "astromech-fqz2"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "v1.0 release"
updated_at: "2026-10-15T20:30:24.606880852+00:00"
---

## Goal

Polish from testing, README, code signing (SignPath) and a GitHub Release.

## Done when

The v1.0 installer on GitHub Releases passes `signtool verify /pa` with the release certificate, and installs on a clean Windows account.

## Context

Open decision for Sumeet: releases are unsigned today (V1-DESIGN §9 #7, the README, `release.yml`), while ARCHITECTURE and V1-DESIGN §7.4 plan SignPath.
