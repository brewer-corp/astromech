---
created_at: "2026-10-22T19:30:24.767724623+00:00"
id: "astromech-d4of"
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
status: "in_progress"
title: "Download the neutron star file in the app"
updated_at: "2026-10-22T20:53:32.011815197+00:00"
---

## Goal

The Route page offers to download the neutron star file once (about 80 MB) into the data folder, shows progress, and uses it from then on. A user who never downloads it still plots neutron routes through Spansh.

## Done when

- The shell downloads the latest `neutron.bin.gz` from the `neutron` pre-release, checks its SHA-256, unpacks it to `neutron.bin` in the data folder (`astromech_store::Paths`) through a temporary file and a rename, so an interrupted download never leaves a half file in place.
- The Route page's neutron plotter shows whether the file is there, its build date, a **Download** button with progress, and a **Check for a newer file** action; nothing downloads without the user pressing it.
- Tests: a recorded or local-server download that succeeds, one with a wrong checksum that leaves no `neutron.bin`, and a component test for the three states (missing, downloading, ready).
- docs/guide/routes.md says what the download is, how big it is and where it goes.

## Scope

Shell (`app/src-tauri`), the Route page (`app/src/modules/route`), the guide. The plotter itself is a separate child.

## Context

Data folder rules: docs/ARCHITECTURE.md, **Paths**. Remote calls go through `astromech_data::Client`; this one is a large file, so it needs streaming rather than the cached-body path. The updater feed work (astromech-rcui) also downloads release assets; reuse what fits.

## Unknowns

Whether GitHub release asset downloads need the same user agent and redirect handling the updater uses.
