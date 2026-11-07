---
schema: atelier.review
schema_version: 2
id: astromech-t9w7
title: finding on astromech-bp23 @791ff421
target: astromech-bp23
identity: claude-reviewer-se44
commit: 791ff421536f6dbcc4eac9a98c4a0ad985c55af0
created_at: 2026-11-07T09:14:41.328848757Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/astromech-core/tests/fixtures/core-replies.json:19 and crates/astromech-core/src/app.rs:2569-2650 (core_replies_keep_their_json, se44) The recorded-replies fixture is host-specific, so cargo test --workspace fails on CI (ci.yml runs it on ubuntu-24.04 and windows-latest). status.models_dir is recorded as /home/supadava/.local/share/astromech/data/models (Config::models_dir falls back to Paths::current(), not the test's data_dir); rerun with HOME=/tmp/... it fails 'a reply's JSON changed' on that field. On Windows it also breaks: the <dir> mask is a string replace of d.path().display() over serialised JSON, where backslashes are escaped, so app_dir/config/journal_dir stay unmasked; and mic_check/mic_list take their cfg(windows) branches (real recording and device list), so 'recording needs Windows' and names [] are not what Windows answers. It also commits a personal home path. Fix: set [voice] models_dir in the test config (or mask models_dir), mask paths on the parsed Value (or replace the JSON-escaped form) and normalise separators, and drop or cfg-gate the mic_* entries (or mask their host-dependent fields); then re-record from the json! version (960792f2) so the before/after comparison stays genuine.
