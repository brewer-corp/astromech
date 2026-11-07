---
schema: atelier.review
schema_version: 2
id: astromech-u6ak
title: finding on astromech-bp23 @ad9ad743
target: astromech-bp23
identity: claude-reviewer-batch1
commit: ad9ad743534d4cc42681c9bdcec10cbf34aad7d1
created_at: 2026-11-07T08:05:02.096251743Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: af09b190cc19733c6b3d74a63a09039912020fec
    recorded_at: 2026-11-07T08:14:20.084747943Z
    resolution_commit: 1bbaa850eaaa6ebf776a219516cc18d8c8a904f2
    reason: Old speech restored for Open and Failed; only a lookup during the first open that then fails differs
---

medium: crates/astromech-core/src/actions.rs:73-85, assistant.rs:261-273, :285-289, :303-307 (k6da, zmo7 fix) Spoken behaviour while playing changed: with no star map a voice lookup (nearest site, powerplay targets, faction, hotspot) now says 'Looking that up.' then NO_STAR_MAP (was the line alone), and with neither position nor star map the NO_POSITION line now wins (targets: NOT_PLEDGED > NO_STAR_MAP > NO_POSITION became NOT_PLEDGED > NO_POSITION > [Looking that up.] NO_STAR_MAP). Only a failure path (galaxy.db won't open), but the strategy's Authority section (docs/strategies/one-tested-process.md:89-91) says any change to what the shipped app does while playing comes back to Sumeet, and no record shows he accepted it. Cheap way to keep the old speech and still keep the open off the speech thread: warm the holder off-thread at Session::start (zmo7's second option) and give GalaxyHolder a non-opening peek (Open(Arc) | Failed | NotTried; remember the last open's failure, still retrying on get). On the speech thread: Open -> old path unchanged; Failed -> say NO_STAR_MAP alone, in the old check order, and retry the open on a spawned thread; NotTried (only during the warm-up) -> today's new path. About 20 lines plus a site_lookup test. Otherwise put the deviation to Sumeet (needs-human) before merge.
