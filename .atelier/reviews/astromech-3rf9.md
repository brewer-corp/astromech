---
schema: atelier.review
schema_version: 2
id: astromech-3rf9
title: finding on astromech-y6d6 @916c97c3
target: astromech-y6d6
identity: claude-reviewer-y6d6-merge
commit: 916c97c3d378b97645c35f5e0085e265eb6510ee
created_at: 2026-11-08T15:37:16.781998495Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: ed935fa6804b1f1e96f8849861861319e30ed8ce
    recorded_at: 2026-11-08T15:37:58.465592540Z
    follow_up: astromech-ttq7
    reason: Low, no behaviour change; fixing it now would rerun Windows CI on the approved merge. Follow-up astromech-ttq7.
---

low: app/src-tauri/src/host.rs:368 (916c97c3) #[allow(clippy::disallowed_methods)] covers the whole of host::run, including the tauri_app.run event-loop closure, though only STARTED.get_or_init(Instant::now) on :370 needs it. A later Instant::now, elapsed or thread::sleep added to run's RunEvent handler (Exit, ExitRequested) or its startup would pass the zgoo clock lint unflagged. The other new allows are narrow (the on_page_load let at :121, source.rs's epoch let, read_status_whole's sleep) or on small functions (rebuild_window, press_listener). Fix: move the allow onto the one statement, e.g. #[allow(clippy::disallowed_methods)] let _ = STARTED.get_or_init(Instant::now);
