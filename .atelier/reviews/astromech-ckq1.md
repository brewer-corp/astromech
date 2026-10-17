---
schema: atelier.review
schema_version: 1
id: astromech-ckq1
title: UnsoldCache (crates/astromech-history/src/exploration.rs stamp) keys on length+mtime of history.sqlite and -wal. After a WAL reset the length can repeat, so freshness rests on mtime alone; a filesystem with coarse mtimes (FAT, 2 s) could serve one stale answer until the next commit. history.sqlite lives on NTFS/ext4 under the data dir, so no action; noted for anyone moving the cache to other files.
target: astromech-dt5h
identity: review-agent (opus, independent)
commit: 797432034fbbec3d2aa0eaa7a812437b9455afe2
created_at: 2026-10-17T08:00:13.536034672Z
event:
  event: finding
  recommendation: no_action
---

UnsoldCache (crates/astromech-history/src/exploration.rs stamp) keys on length+mtime of history.sqlite and -wal. After a WAL reset the length can repeat, so freshness rests on mtime alone; a filesystem with coarse mtimes (FAT, 2 s) could serve one stale answer until the next commit. history.sqlite lives on NTFS/ext4 under the data dir, so no action; noted for anyone moving the cache to other files.
