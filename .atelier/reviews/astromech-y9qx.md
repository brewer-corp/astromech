---
schema: atelier.review
schema_version: 1
id: astromech-y9qx
title: finding on astromech-t34c @75b3bfe8
target: astromech-t34c
identity: review-agent-t34c-1
commit: 75b3bfe8704fc0dc1fb74864bd7c7f73a4f93ba9
created_at: 2026-11-04T23:55:08.347422235Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/pipeline.rs:578-600,606-614,765-770 (lpsk being_written, made sticky by 5b1bfc68) A journal the game has just left (rollover or relaunch) keeps a fresh last line for up to 120 s, and its header still agrees with its name, so being_written(old) is true. If a tool touches it in that window (and sync tools act right after a file closes), old is newest by mtime and takes the session; note_writing then retires the game's real session (latest by name) and holds old. Reproduced with a probe test in a scratch clone at 75b3bfe8704fc0dc1fb74864bd7c7f73a4f93ba9: followed=old, writing=old, the new journal's stamp retired, classify(new)=Gated. It fails closed. Recovery comes when the game next writes and its journal is newest by mtime again: being_written(new) switches back and set_game_session un-retires it. That is immediate on Linux (probe: the next line was sent). On Windows it waits for NTFS to refresh the directory entry, 30-60 s per the astromech-txzq diagnosis. Meanwhile the game's lines are not read; when reopened they are Gated, so any already older than 120 s when read are treated as history, not live. A plain !retired guard in being_written does not fix it: old was never game_session in this run, so it is not retired, and the guard stops the real session coming back (probe: stuck, 1 sent instead of 2). Suggested fix: in follow_newest, let being_written(n) win only if n's last line is stamped after the followed journal's start_time (the game left old before it began new). After a clock correction J2's fresh line is still after J1's header once real time has passed it, and start_time is None while J1's header is still in the future.
