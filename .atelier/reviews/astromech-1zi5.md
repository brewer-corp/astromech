---
schema: atelier.review
schema_version: 1
id: astromech-1zi5
title: 'R9: sending updated_at is tested for the systems lookups (a_strict_age_still_finds_fresh_systems_farther_out and the powerplay test both fail when I remove it, and the first fails when the :age cache-key part is removed), but nothing asserts it for the bodies search (SiteKind::search, sites.rs:98) or colony candidates (candidate_search, colonies.rs:164); those are by inspection only. Add a request assertion to tests/sites.rs and tests/colonies.rs.'
target: astromech-n1ij
identity: review-agent (opus, independent)
commit: df2c4fb438a4f201a3a05fc02770c5536db43657
created_at: 2026-10-17T05:05:44.368675448Z
event:
  event: finding
  recommendation: defer
---

R9: sending updated_at is tested for the systems lookups (a_strict_age_still_finds_fresh_systems_farther_out and the powerplay test both fail when I remove it, and the first fails when the :age cache-key part is removed), but nothing asserts it for the bodies search (SiteKind::search, sites.rs:98) or colony candidates (candidate_search, colonies.rs:164); those are by inspection only. Add a request assertion to tests/sites.rs and tests/colonies.rs.
