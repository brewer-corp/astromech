---
schema: atelier.review
schema_version: 1
id: astromech-djv8
title: 'R1 medium: the name index''s group size (256) is never exercised by the tests. The sample has 35 stars, so the file has one name group; changing NAME_GROUP on one side only (scripts/gen-neutron.py:47 or crates/astromech-galaxy/src/neutron.rs:43) still rebuilds the identical fixture and passes every test, yet every name past star 255 in the real file would be wrong. nk0u''s Done-when requires the test to fail when the format changes on one side only. Fix: a sample with more than 256 neutron stars (a second synthetic dump is fine), with the Rust test reading names on both sides of a group boundary, or store the group size in the header and check it.'
target: astromech-1842
identity: review-agent-1842-1
commit: 059d78381235b475652ae78edab486764504ddb4
created_at: 2026-10-22T21:11:02.918983371Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: the name index's group size (256) is never exercised by the tests. The sample has 35 stars, so the file has one name group; changing NAME_GROUP on one side only (scripts/gen-neutron.py:47 or crates/astromech-galaxy/src/neutron.rs:43) still rebuilds the identical fixture and passes every test, yet every name past star 255 in the real file would be wrong. nk0u's Done-when requires the test to fail when the format changes on one side only. Fix: a sample with more than 256 neutron stars (a second synthetic dump is fine), with the Rust test reading names on both sides of a group boundary, or store the group size in the header and check it.
