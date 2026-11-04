---
created_at: "2026-10-22T21:50:43.634445902+00:00"
id: "astromech-fyvh"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-fyvh
priority: "P2"
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
closed_at: "2026-11-04T20:39:01.511252594+00:00"
status: "done"
title: "open_external: refuse paths that climb out of an allowed prefix"
updated_at: "2026-11-04T20:39:01.511252594+00:00"
---

## Goal

`is_openable_url` refuses URLs whose path leaves the allowed prefix after normalisation, so `https://github.com/brewer-corp/astromech/../../evil/x` (and its `%2e%2e` and backslash forms) can't open another page on an allowed host.

## Done when

The predicate refuses `..` and percent-encoded dot segments and backslashes in the path (or compares the normalised path against the prefix), with tests for each form on every allowed host, and every link the app builds still passes.

## Context

Review finding R1 (astromech-av5v) on feature/astromech-0k88: crates/astromech-engineering/src/links.rs:62. Browsers normalise these to e.g. https://github.com/evil/x. A compromised webview could open any page on github.com, inara.cz or spansh.co.uk (not another host; no explorer switch injection). Existed for the other hosts before.

## Close Reason

is_openable_url refuses a dot segment in the path (before ? or #; '.', '..' and their %2e forms, any case) and a backslash anywhere in the URL. Comparing the raw path against the prefix is then sound, because a path with no dot segments and no backslashes is already normalised as far as the prefix is concerned. New test paths_that_climb_out_of_a_prefix_are_refused covers each form on Spansh, Inara and GitHub (the path-ended prefixes) and backslashes on Coriolis and EDSY, whose prefixes end in the query or fragment so their path cannot climb. only_our_links_are_openable (every link shape the app builds) still passes. Dot segments in a query or fragment are left alone: a browser does not resolve them.
