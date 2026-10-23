---
schema: atelier.review
schema_version: 1
id: astromech-e34o
title: 'Low, crates/astromech-intent/src/lib.rs:385: the template ''what should i map'' turns everyday PTT speech that was NoMatch on main (''what should I do'', ''what should I say'', ''what should I buy'') into Ask ''Did you mean what should I map?'' (0.75-0.83); likewise ''what is in this system'' / ''what is here'' now Ask biology and ''how much is my ship/cargo worth'' Ask unsold. No wrong executes (all ask-only, nothing pressed). Consider dropping ''what should i map'' or adding these as near-miss assertions.'
target: astromech-m3xd
identity: review-agent-m3xd-1
commit: a83e6a6a42217c36b9fb3348031207138ce1f7f9
created_at: 2026-10-23T00:28:21.817481406Z
event:
  event: finding
  recommendation: defer
---

Low, crates/astromech-intent/src/lib.rs:385: the template 'what should i map' turns everyday PTT speech that was NoMatch on main ('what should I do', 'what should I say', 'what should I buy') into Ask 'Did you mean what should I map?' (0.75-0.83); likewise 'what is in this system' / 'what is here' now Ask biology and 'how much is my ship/cargo worth' Ask unsold. No wrong executes (all ask-only, nothing pressed). Consider dropping 'what should i map' or adding these as near-miss assertions.
