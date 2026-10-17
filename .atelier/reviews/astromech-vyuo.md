---
schema: atelier.review
schema_version: 1
id: astromech-vyuo
title: '[medium] crates/astromech-galaxy/src/outfitting.rs:360 (fits) with data/catalog/outfitting.json armour entries: an armour search matches any ship''s armour, because each armour grade carries all ~47 ships'' symbols and fits() accepts any of them. Scenario: in a Krait MkII, ''nearest military grade composite'' answers with the nearest station selling Adder MGC (Spansh module ship=''Adder''), which may not sell Krait armour. The app already declines armour in wantFor for this reason (app/src/lib/outfitting.ts:55), but voice and the picker don''t. Suggested follow-up: narrow armour symbols to the current ship (GameState.ship / Spansh Module.ship), or say the answer is for any ship.'
target: astromech-970a
identity: review-agent (970a review)
commit: c672283ee1d91440319cda080c675a3610873109
created_at: 2026-10-17T00:33:50.444242625Z
event:
  event: finding
  recommendation: defer
---

[medium] crates/astromech-galaxy/src/outfitting.rs:360 (fits) with data/catalog/outfitting.json armour entries: an armour search matches any ship's armour, because each armour grade carries all ~47 ships' symbols and fits() accepts any of them. Scenario: in a Krait MkII, 'nearest military grade composite' answers with the nearest station selling Adder MGC (Spansh module ship='Adder'), which may not sell Krait armour. The app already declines armour in wantFor for this reason (app/src/lib/outfitting.ts:55), but voice and the picker don't. Suggested follow-up: narrow armour symbols to the current ship (GameState.ship / Spansh Module.ship), or say the answer is for any ship.
