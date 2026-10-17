---
schema: atelier.review
schema_version: 1
id: astromech-5xdk
title: 'Approved at 2647cb3. Done-when met: one powerplay.<kind> command per Target, the key Targets.svelte''s SearchKit remembers filters under (test asserts lookup(target) == command). Dispatches to Galaxy::powerplay_targets, the same function as the app''s powerplay_targets command. Uses the pledged power from the journal fold (GameState.powerplay, cleared on PowerplayLeave) and the page''s default 40 ly radius; the radius is not a remembered filter. The answer gives system, distance, jumps and Powerplay state. An unpledged commander hears ''Not pledged to a power, so there are no targets.'' and Spansh is not asked. The recorded-fixture test checks the pledged power is excluded from undermining, acquisition is for the pledged power, and the remembered data age reaches Spansh, so it fails if the key or power is wrong. Off the speech thread. Copy test covers each kind, long names and the no-answer lines. Docs updated. Checked: cargo test -p astromech-intent -p astromech-tools -p astromech-core passes; check-deps OK; merges cleanly with main. CI on PR #95: Linux and Windows build green, Windows installer pending at review time. Findings: 2 no-action, 1 defer.'
target: astromech-8joi
identity: review-agent (opus, independent)
commit: 2647cb30c796b6226a8243e764cf1993a1e0c71b
created_at: 2026-10-17T08:17:48.840054453Z
event:
  event: approval
---

Approved at 2647cb3. Done-when met: one powerplay.<kind> command per Target, the key Targets.svelte's SearchKit remembers filters under (test asserts lookup(target) == command). Dispatches to Galaxy::powerplay_targets, the same function as the app's powerplay_targets command. Uses the pledged power from the journal fold (GameState.powerplay, cleared on PowerplayLeave) and the page's default 40 ly radius; the radius is not a remembered filter. The answer gives system, distance, jumps and Powerplay state. An unpledged commander hears 'Not pledged to a power, so there are no targets.' and Spansh is not asked. The recorded-fixture test checks the pledged power is excluded from undermining, acquisition is for the pledged power, and the remembered data age reaches Spansh, so it fails if the key or power is wrong. Off the speech thread. Copy test covers each kind, long names and the no-answer lines. Docs updated. Checked: cargo test -p astromech-intent -p astromech-tools -p astromech-core passes; check-deps OK; merges cleanly with main. CI on PR #95: Linux and Windows build green, Windows installer pending at review time. Findings: 2 no-action, 1 defer.
