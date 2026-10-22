---
created_at: "2026-10-17T19:45:12.253105436+00:00"
id: "astromech-6a7c"
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
status: "review"
title: "The window can follow a stale config.path after a --config run"
updated_at: "2026-10-22T21:44:39.370036213+00:00"
---

## Goal

Paths::publish_config (astromech-c9em) tells the window which config file the tray core uses, in config.path. It is only written when the tray core starts, so a deleted --config target, a console-mode run (run_console does not publish) or two cores with different --config values can leave the window reading a file the core is not using.

## Done when

The window falls back to the default file when the published path does not exist, with a test (astromech-store paths::tests::the_window_reads_the_config_the_core_published). Console runs leave config.path alone, as Paths::publish_config documents: they serve no window, so the window's own tray core is the one to publish.

## Context

Revised 22 Oct 2026 while implementing (claude-implementer-0k88). The first Done when asked console runs to publish or clear config.path. From the code, neither is right: run_console serves no core.json and takes no single-instance lock, so the window never talks to a console core. If no tray core runs, the window starts one (--background, no --config), which publishes. Publishing from a console run would point the window away from the tray core it does use, and clearing would do the same when that tray core runs with --config. Two tray cores can't both publish: app::run takes the instance lock before choose_config, so a second start exits first. The window's startup reads (the remote cache's data_dir, before its own tray core has published) can still see an earlier --config run's file while it exists; that's the remaining gap if one matters.
