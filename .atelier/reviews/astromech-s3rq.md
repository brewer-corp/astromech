---
schema: atelier.review
schema_version: 2
id: astromech-s3rq
title: finding on astromech-y6d6 @b5af3bc1
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: b5af3bc135de96d0aa8250e4b14f524c941710d7
created_at: 2026-11-07T19:47:18.483007797Z
event:
  event: finding
  recommendation: fix_now
---

medium: .github/workflows/release.yml:166-167 with scripts/upgrade_smoke.py:213-237. The job upgrades from the newest v* release, but the script assumes that release is two-process: it starts astromech-shell.exe as the old window and waits 90 s for 'window: page loaded' in astromech-shell.log, then requires the shell to still be running. Once the first one-process release (this build) is published, it is the newest; its astromech-shell.exe is the forwarder, which exits at once and writes no shell log, so old_running fails and, since publish needs upgrade, every release after the next one is blocked. The item's title says 'upgrade from the last two-process release'. Fix: pin the old tag to v1.0.0-alpha.4 (the last two-process release), or make the script start a one-process old release by its own rules; file the one-process-to-newer upgrade test as a follow-up if pinned.
