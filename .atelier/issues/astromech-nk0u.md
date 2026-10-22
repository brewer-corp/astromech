---
created_at: "2026-10-22T19:30:24.608507844+00:00"
id: "astromech-nk0u"
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
title: "Build and publish the neutron star file"
updated_at: "2026-10-22T20:47:00.369844275+00:00"
---

## Goal

A hand-run CI job turns Spansh's neutron dump into a compact file the app can download and search, and publishes it on a `neutron` pre-release, the same way `starmap.db.gz` is published.

## Done when

- `scripts/gen-neutron.py --dump systems_neutron.json.gz --out neutron.bin` writes the file from the real dump, keeping only systems whose `mainStar` is "Neutron Star", and from a small sample dump in the tests.
- `.github/workflows/neutron.yml` (workflow_dispatch only) downloads the dump, runs the script, gzips the result and uploads it with its SHA-256 to the `neutron` pre-release (`--latest=false`, so the updater never reads it).
- A Rust reader in `astromech-galaxy` opens a fixture built by the script from the sample and returns each star's position and name; the test fails if the format or the order changes on one side only.
- The format is documented in the script's docstring and the reader's module docs: a header with a magic, a format version, the star count and the build date; positions as three i32 in 1/32 ly, sorted by grid cell so a cell is one contiguous range; names in a separate block read only for the stops of a route. The gzipped file is under 100 MB.

## Scope

New script, workflow and reader module. No app or plotter change.

## Context

`scripts/gen-starmap.py` and `.github/workflows/starmap.yml` are the pattern to follow (user agent string, pre-release handling). Measured on 22 Oct 2026: 5,045,154 neutron stars, 96 MB of positions with id64s (60 MB gzipped), 102 MB of names (23 MB gzipped). The id64 is not needed to plot; leave it out unless a consumer needs it.
