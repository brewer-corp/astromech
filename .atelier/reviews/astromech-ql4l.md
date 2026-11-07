---
schema: atelier.review
schema_version: 2
id: astromech-ql4l
title: approval on astromech-bp23 @c4568f5b
target: astromech-bp23
identity: claude-reviewer-96dv
commit: c4568f5b9790d93cfa0a9ed447b41451ab9baf1e
created_at: 2026-11-07T10:55:05.993409266Z
event:
  event: approval
---

Re-review of the fixes for the 96dv findings in 1ece31b5 and the dispositions (51cc0b95, 57b8bf9d, c4568f5b). ttf7: ran the new example dump-local-route (cargo run -p astromech-galaxy --example dump-local-route) from a git archive of HEAD; its output equals plot.json's local route exactly (Sol, Jackson's Lighthouse 5, HR 3230 4; 9 jumps, 423 ly, fewest). It reads only files already committed (tests/fixtures/neutron-sample.bin, data/catalog/raw-farms.json) and commits no new game data; clippy --examples -D warnings and fmt pass. lfa8: loadShopping is Promise<Shopping>, the pending branch says 'Working out the shopping list…'. dajh: ARCHITECTURE 18.2 now names only missing or wrong-typed fields. hxrb: ArgList is [Args<K>] extends [null]; probe: call('set_ptt') and call('galaxy_systems') without args fail, call('set_ptt', null) and call('status') pass; Settings' run takes BareCoreCmd. qcbo deferred to astromech-8nuc; the deviation decision note recorded. Fresh: pnpm check 0 errors, lint clean, pnpm test 74 files 532 tests, pnpm build then check-dist: 74 files, no browser preview. No fix_now finding remains.
