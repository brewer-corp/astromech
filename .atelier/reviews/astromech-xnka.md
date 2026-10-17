---
schema: atelier.review
schema_version: 1
id: astromech-xnka
title: 'Reviewed git diff main...HEAD: SessionSummary::replayed now takes the pending entry and rebuilds Ended with it, armed from state.logged_in, spoken reset as before; module doc and ARCHITECTURE line updated accurately. Engine::fire calls replayed only for non-live inputs, and the pipeline re-reads journals as replays after going live, which is the path the issue describes. Kept pending cannot speak a wrong or stale session (query pinned to the captured commander and timestamp, end >= ts filter, wait clock preserved), and pending is only ever set by a live ending line, so the fresh-launch, crashed-session and mid-session-start guarantees are untouched; their tests pass. cargo test -p astromech-announce -p astromech-gamestate all green. Mutation: replacing the kept pending with None fails a_replayed_line_during_the_wait_keeps_the_pending_summary. Residual: replays still reset ''spoken'', allowing a double summary in a contrived re-read case (pre-existing, recorded as DEFER).'
target: astromech-hk7e
identity: review-agent (opus, independent)
commit: 1baa007d76cef6bfbad14b771352d4bf94480782
created_at: 2026-10-17T06:25:25.841457725Z
event:
  event: approval
---

Reviewed git diff main...HEAD: SessionSummary::replayed now takes the pending entry and rebuilds Ended with it, armed from state.logged_in, spoken reset as before; module doc and ARCHITECTURE line updated accurately. Engine::fire calls replayed only for non-live inputs, and the pipeline re-reads journals as replays after going live, which is the path the issue describes. Kept pending cannot speak a wrong or stale session (query pinned to the captured commander and timestamp, end >= ts filter, wait clock preserved), and pending is only ever set by a live ending line, so the fresh-launch, crashed-session and mid-session-start guarantees are untouched; their tests pass. cargo test -p astromech-announce -p astromech-gamestate all green. Mutation: replacing the kept pending with None fails a_replayed_line_during_the_wait_keeps_the_pending_summary. Residual: replays still reset 'spoken', allowing a double summary in a contrived re-read case (pre-existing, recorded as DEFER).
