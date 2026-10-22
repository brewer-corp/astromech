---
schema: atelier.review
schema_version: 1
id: astromech-ek06
title: 'R2 [low] crates/astromech-core/src/pipeline.rs:651: note_startup_lengths measures the latest-by-name journal only through File::open, and drops it from opened when the open fails (if let Ok). A journal that can''t be opened at that moment (an AV scanner or another tool briefly holding it on Windows) is then treated as new this run, so follow_newest opens it live from its first line: the astromech-nrhv bug (startup history sent to EDDN/EDSM and spoken) comes back for that run. Fix: .or_else(|_| e.metadata()) as 2h0q did for the history import (a listed length that lags is still far better than none). Consider measuring every journal through a handle with the listing as the fallback: the history import already opens every journal on each pass, and on Windows the listing can also lag for a game journal that isn''t latest by name (repeated DST hour, foreign later-stamped file).'
target: astromech-g3qt
identity: review-agent-g3qt-1
commit: 98e6827717c46e9692d70b475f2300e4eb9f4b51
created_at: 2026-10-22T21:06:26.138439450Z
event:
  event: finding
  recommendation: fix_now
---

R2 [low] crates/astromech-core/src/pipeline.rs:651: note_startup_lengths measures the latest-by-name journal only through File::open, and drops it from opened when the open fails (if let Ok). A journal that can't be opened at that moment (an AV scanner or another tool briefly holding it on Windows) is then treated as new this run, so follow_newest opens it live from its first line: the astromech-nrhv bug (startup history sent to EDDN/EDSM and spoken) comes back for that run. Fix: .or_else(|_| e.metadata()) as 2h0q did for the history import (a listed length that lags is still far better than none). Consider measuring every journal through a handle with the listing as the fallback: the history import already opens every journal on each pass, and on Windows the listing can also lag for a game journal that isn't latest by name (repeated DST hour, foreign later-stamped file).
