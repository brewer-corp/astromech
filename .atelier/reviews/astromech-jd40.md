---
schema: atelier.review
schema_version: 1
id: astromech-jd40
title: 'Approved at 5481560. Outbox::init now hands the connection to astromech_store::migrate, which enables WAL with busy retries, runs each migration and its user_version bump in one BEGIN IMMEDIATE transaction and re-reads user_version inside it; busy_timeout is still set first. The v2 rebuild lost its own BEGIN/COMMIT, which is required inside the outer transaction and keeps it atomic. Existing files: v1 runs v2 then v3 (covered by the existing purged-id upgrade test); v2 and v3 skip by user_version (by inspection); a v2 file whose old bump was lost to a crash re-runs v2 harmlessly since outbox_new no longer exists; edsm.sqlite is the same Outbox. Dependency: astromech-eddn -> astromech-store is allowed (ARCHITECTURE: services depend on astromech-store; astromech-history does the same), no cycle. The new two-connection test fails 3/3 with the old migration loop restored (v3 ''table meta already exists'') and passed 15/15 with the fix; all 130 astromech-eddn tests pass and PR #73 CI is green at this head. ARCHITECTURE.md and the MIGRATIONS comment are updated. No findings.'
target: astromech-zc0v
identity: review-agent (opus, independent)
commit: 54815609c71a25359e49dc8534545c854c7f80a3
created_at: 2026-10-17T05:39:07.813821699Z
event:
  event: approval
---

Approved at 5481560. Outbox::init now hands the connection to astromech_store::migrate, which enables WAL with busy retries, runs each migration and its user_version bump in one BEGIN IMMEDIATE transaction and re-reads user_version inside it; busy_timeout is still set first. The v2 rebuild lost its own BEGIN/COMMIT, which is required inside the outer transaction and keeps it atomic. Existing files: v1 runs v2 then v3 (covered by the existing purged-id upgrade test); v2 and v3 skip by user_version (by inspection); a v2 file whose old bump was lost to a crash re-runs v2 harmlessly since outbox_new no longer exists; edsm.sqlite is the same Outbox. Dependency: astromech-eddn -> astromech-store is allowed (ARCHITECTURE: services depend on astromech-store; astromech-history does the same), no cycle. The new two-connection test fails 3/3 with the old migration loop restored (v3 'table meta already exists') and passed 15/15 with the fix; all 130 astromech-eddn tests pass and PR #73 CI is green at this head. ARCHITECTURE.md and the MIGRATIONS comment are updated. No findings.
