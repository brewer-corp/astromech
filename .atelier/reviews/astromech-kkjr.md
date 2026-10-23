---
schema: atelier.review
schema_version: 1
id: astromech-kkjr
title: 'Independent review (review-agent-1842-2) of the whole branch at 047e8396, focused on fix commits 56c342fb (djv8, format v2) and 9c69fe72 (rse7, mubt). Read script, reader, download client, tests, fixtures, docs, workflow and app/src-tauri/src/main.rs (read only; not compiled here, no glib). Re-ran: test_gen_neutron.py (2 ok), cargo test neutron (5 ok) and neutron_download (6 ok), vitest NeutronFile.test.ts (4 ok), clippy on astromech-galaxy/astromech-data (clean). Checked: script struct <8sIIIIqI is 36 bytes at the offsets the reader parses; the reader takes the group size from the header only, so a one-sided change to NAME_GROUP now fails the byte-identical fixture test, and the 600-star fixture reads names across three groups. Downloading guard: started before Part, so a refused second download never deletes the running .part; Part drops before the guard (reverse declaration order); unwind runs both Drops (no panic=abort), poisoned lock recovered; the app is single-instance, so a per-process guard suffices. Shell sends and logs {e:#}. The neutron pre-release isn''t published yet (gh release view: not found), so Download 404s until the workflow runs after merge.'
target: astromech-1842
identity: review-agent-1842-2
commit: 047e839664bce33880db5033c28fedd4ea4b4355
created_at: 2026-10-22T21:21:28.705281528Z
event:
  event: review
---

Independent review (review-agent-1842-2) of the whole branch at 047e8396, focused on fix commits 56c342fb (djv8, format v2) and 9c69fe72 (rse7, mubt). Read script, reader, download client, tests, fixtures, docs, workflow and app/src-tauri/src/main.rs (read only; not compiled here, no glib). Re-ran: test_gen_neutron.py (2 ok), cargo test neutron (5 ok) and neutron_download (6 ok), vitest NeutronFile.test.ts (4 ok), clippy on astromech-galaxy/astromech-data (clean). Checked: script struct <8sIIIIqI is 36 bytes at the offsets the reader parses; the reader takes the group size from the header only, so a one-sided change to NAME_GROUP now fails the byte-identical fixture test, and the 600-star fixture reads names across three groups. Downloading guard: started before Part, so a refused second download never deletes the running .part; Part drops before the guard (reverse declaration order); unwind runs both Drops (no panic=abort), poisoned lock recovered; the app is single-instance, so a per-process guard suffices. Shell sends and logs {e:#}. The neutron pre-release isn't published yet (gh release view: not found), so Download 404s until the workflow runs after merge.
