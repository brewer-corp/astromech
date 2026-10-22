---
schema: atelier.review
schema_version: 1
id: astromech-bpr1
title: 'R6 low (fix now, trivial): the format spec still says version 1. scripts/gen-neutron.py:15 reads ''8  u32  format version (1)'' under ''File format, version 2''; nk0u''s Done-when makes this docstring the format spec, and the reader''s docs call it ''the same spec''. Also residue in crates/astromech-galaxy/tests/neutron.rs:103-105: the file named v2.bin is first written as version 2 (now the current version) and immediately overwritten as version 3; the dead write reads as if v2 should be refused. Fix: ''(2)'' in the docstring; drop the dead write, and make that case refuse a version-1 file (the format just bumped), keeping the version-3 one. The script docstring''s ''about 60 MB'' loaded vs the reader''s ''about 75 MB'' (cells are 16 MB more) could be aligned in the same edit.'
target: astromech-1842
identity: review-agent-1842-2
commit: 047e839664bce33880db5033c28fedd4ea4b4355
created_at: 2026-10-22T21:21:28.867576538Z
event:
  event: finding
  recommendation: fix_now
---

R6 low (fix now, trivial): the format spec still says version 1. scripts/gen-neutron.py:15 reads '8  u32  format version (1)' under 'File format, version 2'; nk0u's Done-when makes this docstring the format spec, and the reader's docs call it 'the same spec'. Also residue in crates/astromech-galaxy/tests/neutron.rs:103-105: the file named v2.bin is first written as version 2 (now the current version) and immediately overwritten as version 3; the dead write reads as if v2 should be refused. Fix: '(2)' in the docstring; drop the dead write, and make that case refuse a version-1 file (the format just bumped), keeping the version-3 one. The script docstring's 'about 60 MB' loaded vs the reader's 'about 75 MB' (cells are 16 MB more) could be aligned in the same edit.
