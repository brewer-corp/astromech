---
schema: atelier.review
schema_version: 2
id: astromech-4k6h
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:04.813026306Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: f0227722610a3bdd39f5c52f649443dd1ce03e32
    recorded_at: 2026-11-07T13:50:45.915425041Z
    follow_up: astromech-zgoo
    reason: zgoo's lint sweep; noted there
---

low: crates/astromech-core/src/world.rs:437-458 world_wait's 30 s guard reads Instant::now and uses Condvar::wait_timeout on real time. Under decision astromech-1zhg this is acceptable: it only fails a stuck call and decides no outcome, and moving it to the manual clock would hang a wait nobody advances. But astromech-zgoo will disallow both methods and its enumerated list of reads that must stay real omits this guard, so zgoo's implementer has to guess. Fix: add 'world_wait hang guard' to zgoo's list, with #[allow(clippy::disallowed_methods)] and this reason. Defer to astromech-zgoo.
