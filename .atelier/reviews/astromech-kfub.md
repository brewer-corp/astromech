---
schema: atelier.review
schema_version: 2
id: astromech-kfub
title: finding on astromech-y6d6 @b5af3bc1
target: astromech-y6d6
identity: claude-reviewer-9lh3
commit: b5af3bc135de96d0aa8250e4b14f524c941710d7
created_at: 2026-11-07T19:47:18.105423795Z
event:
  event: finding
  recommendation: fix_now
---

high: scripts/upgrade_smoke.py:98-111 and 225-226. call() returns r.get('value'), but v1.0.0-alpha.4's control server (crates/astromech-core/src/control.rs handle) writes the command's JSON as is: status is {ok, version, pid, ...} with no value envelope. So call('status') against the old core gives (True, None) and old_running's wait lambda raises AttributeError ('NoneType' has no attribute 'get') on the first answer, which wait doesn't catch: both routes fail before any upgrade, the upgrade job can't pass, and with publish needing upgrade no release publishes. Reproduced with a stand-in server sending alpha.4's reply shape. Fix: accept the old shape, e.g. return True, (r['value'] if 'value' in r else r).
