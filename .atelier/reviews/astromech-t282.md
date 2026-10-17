---
schema: atelier.review
schema_version: 1
id: astromech-t282
title: 'crates/astromech-core/src/app.rs:965: the startup follow_history(&core.cfg()) takes its config copy without config_lock, after the control API is already serving (app.rs:940). A set_folders saved between that read and follow() would be re-pointed back to the old folder until the next edit. The gap is microseconds and the window opens later (app.rs:998), so this is noted only. Taking config_lock around the startup call would close it.'
target: astromech-5t34
identity: review-agent (5t34 review)
commit: cdaba6007cd054cf1d165f344050e991005f5ea5
created_at: 2026-10-17T00:09:00.774358666Z
event:
  event: finding
  recommendation: no_action
---

crates/astromech-core/src/app.rs:965: the startup follow_history(&core.cfg()) takes its config copy without config_lock, after the control API is already serving (app.rs:940). A set_folders saved between that read and follow() would be re-pointed back to the old folder until the next edit. The gap is microseconds and the window opens later (app.rs:998), so this is noted only. Taking config_lock around the startup call would close it.
