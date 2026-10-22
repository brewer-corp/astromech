---
schema: atelier.review
schema_version: 1
id: astromech-ugp2
title: 'R8 low: installed() checks Downloading::running and removes neutron.bin.part in two steps without holding DOWNLOADS (crates/astromech-galaxy/src/neutron.rs:220-222). If a status read stalls between the check and the remove while a download starts, fetches the checksum and creates the .part, the running download''s file is deleted; the download then fails at read_header(&part) with the misleading ''isn''t one this version of astromech reads ... Update astromech'' (neutron.rs:~331). Needs a stall of a network round trip between two adjacent statements, and fails safe (old file kept). The same ''Update astromech'' advice is given for any I/O error reading the unpacked .part back (e.g. a sharing violation), not only a format mismatch. Fix if touched: remove the stale .part while holding the lock, and give ''Update astromech'' only for a Header::parse failure.'
target: astromech-1842
identity: review-agent-1842-2
commit: 047e839664bce33880db5033c28fedd4ea4b4355
created_at: 2026-10-22T21:21:29.220891922Z
event:
  event: finding
  recommendation: no_action
---

R8 low: installed() checks Downloading::running and removes neutron.bin.part in two steps without holding DOWNLOADS (crates/astromech-galaxy/src/neutron.rs:220-222). If a status read stalls between the check and the remove while a download starts, fetches the checksum and creates the .part, the running download's file is deleted; the download then fails at read_header(&part) with the misleading 'isn't one this version of astromech reads ... Update astromech' (neutron.rs:~331). Needs a stall of a network round trip between two adjacent statements, and fails safe (old file kept). The same 'Update astromech' advice is given for any I/O error reading the unpacked .part back (e.g. a sharing violation), not only a format mismatch. Fix if touched: remove the stale .part while holding the lock, and give 'Update astromech' only for a Header::parse failure.
