---
schema: atelier.review
schema_version: 2
id: astromech-nx9p
title: approval on astromech-vucf @18b6c603
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 18b6c6030f3b5d477a2ab3ff9d7a0df6b1305674
created_at: 2026-11-07T15:21:04.453292957Z
event:
  event: approval
---

Review of 04cf0853 (d2nq teardown) and 18b6c603. after_reply: handle() runs the deferred effects after send() whether or not the send failed, so a client that disconnects before reading still quits (send errors, no SIGPIPE in Rust) and nothing waits on the client. The only user is quit, whose effect is an unbounded mpsc send, so it can't block. A panicking handler's effects still come back. Call::run's nested dispatch runs its effects at nested return, as documented; no handler nests quit. The installed --quit and second-start quit change only in order (reply before Ui::Quit) and the tray's Quit is untouched: that is quitting, not in-play behaviour, so it is within the strategy's Authority. Following::stop takes both handles under the lock and joins outside it, so no deadlock; only world mode calls it. Its 50x test is deterministic for a correct stop (the thread's channel is gone once joined), with no sleep, wall-clock read or retry, so it fits decision 1zhg; only a mutant's detection is probabilistic. Ran: cargo test -p astromech-core -p astromech-api, 379 passed, 0 failed; the world_mode binary 100 times, 100 passed; afterwards no astromech-world-* or astromech-test-home-* dirs, no world cores running, and the real ~/.local/share/astromech absent. One low finding deferred (history stop while the control API still serves).
