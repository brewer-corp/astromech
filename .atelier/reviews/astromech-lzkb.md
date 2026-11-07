---
schema: atelier.review
schema_version: 2
id: astromech-lzkb
title: review on astromech-bp23 @10994559
target: astromech-bp23
identity: claude-reviewer-96dv
commit: 109945598112dc967a6781b2fcb4cf70ae23aae2
created_at: 2026-11-07T10:45:34.306898759Z
event:
  event: review
---

Review of astromech-96dv (bb8a2493..10994559: 05ae48d8 start, 0d1c7c84 code, 10994559 close) against its Done when, its two notes (se44 fixture gaps, finding 34i3) and its decision note, bp23, 1zhg, CONTRIBUTING Tests and ARCHITECTURE 18.2. Not approved: three fix_now findings (ttf7 invented local-route jumps vs 1zhg; lfa8 odyssey null branch and type left; dajh ARCHITECTURE overstates the fixture check), one defer (qcbo neutron 128 vs 11, unfiled), one no_action (hxrb ArgList optional for Button|null and union K). Ran fresh: pnpm check 0 errors, lint clean, pnpm test 74 files 532 tests; pnpm build then check-dist: 74 files, no browser preview. Deliberate failures in a scratch copy: call('galaxy_systems') without args, call('quit'), args to status, an extra arg field, a fixture missing NearbyHit.exact, a wrong field type, a failure-shaped Done<Biology> all fail; a non-fresh extra fixture field and a union-typed cmd pass. Preview builds of bb8a2493 and HEAD, 24 pages x (?core=ready, no core): 48 PNGs byte-identical, page text identical except Settings' Updates line. Re-ran dump-ships-fixture and engineering dump-fixture: equal to ships.json and engineering.json. All se44 fixture gaps are filled; 34i3's lines are corrected and the remaining 'shell' mentions mean the proxy or its events. Re-exports: no consumer meaning change found (undefined checks, literal lookups with fallbacks). No preview handler keys overlap between shell, binds and core. Not run: galaxy dump-nearby (network), the 10 Places finders and ?route=exact shots, cargo bindings test (no Rust or generated file changed).
