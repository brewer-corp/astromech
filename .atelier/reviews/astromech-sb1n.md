---
schema: atelier.review
schema_version: 1
id: astromech-sb1n
title: finding on astromech-f2k1 @47967160
target: astromech-f2k1
identity: review-agent-f2k1-1
commit: 47967160ec0492c800103d37c0319669e6210fb2
created_at: 2026-11-04T22:40:35.119910548Z
event:
  event: finding
  recommendation: defer
---

low: docs/ARCHITECTURE.md:1774 still says the core's --config overrides the config and 'the window can't see that'. That has been false since astromech-6a7c, and this change makes config.path a note that lasts only while the core runs. Suggested fix: one sentence there saying the tray core publishes its config path in config.path in the app dir while it runs, the window reads that, and the default applies when no note exists. Pre-existing, so it can be deferred, but it is cheap to fix now.
