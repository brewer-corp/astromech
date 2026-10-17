---
schema: atelier.review
schema_version: 1
id: astromech-anht
title: 'R3 [medium] crates/astromech-core/tests/trade_voice.rs a_voice_lookup_then_the_trade_page_make_one_ardent_request: the test builds both clients by hand with Cache::open on CACHE_FILE, so it proves two clients on one file share answers but not the production wiring. Mutation: pointing Client::shared at another directory and removing .with_shared_remote(remote) in session.rs leaves cargo test -p astromech-core -p astromech-data fully green. Add a test that goes through Client::shared (e.g. a shared-with-transport constructor) and the Session/Assistant wiring; fits astromech-zs2q''s ''two consumers share the limit and the cache'' test.'
target: astromech-proz
identity: review-agent (opus, independent)
commit: 72df8be9943607bf04534ac08f1161785b65f852
created_at: 2026-10-17T05:23:23.978593457Z
event:
  event: finding
  recommendation: defer
---

R3 [medium] crates/astromech-core/tests/trade_voice.rs a_voice_lookup_then_the_trade_page_make_one_ardent_request: the test builds both clients by hand with Cache::open on CACHE_FILE, so it proves two clients on one file share answers but not the production wiring. Mutation: pointing Client::shared at another directory and removing .with_shared_remote(remote) in session.rs leaves cargo test -p astromech-core -p astromech-data fully green. Add a test that goes through Client::shared (e.g. a shared-with-transport constructor) and the Session/Assistant wiring; fits astromech-zs2q's 'two consumers share the limit and the cache' test.
