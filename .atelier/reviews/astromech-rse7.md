---
schema: atelier.review
schema_version: 1
id: astromech-rse7
title: 'R2 low: the shell turns errors into strings with e.to_string() (app/src-tauri/src/main.rs:985 and :1020), which keeps only the outermost anyhow context. Several contexts are bare paths (crates/astromech-galaxy/src/neutron.rs:84 ''reading <path>'', :279 ''writing <part>'', :276 ''creating <dir>'', :298 ''replacing <path>''). Scenario: on Windows, antivirus is scanning neutron.bin, the rename fails, and the Route page shows just ''replacing C:\Users\...\neutron.bin'', with no reason and no next step; a truncated neutron.bin shows ''The file there can''t be read: reading <path>''. Fix: format!("{e:#}") in the shell, or contexts that are sentences with the reason.'
target: astromech-1842
identity: review-agent-1842-1
commit: 059d78381235b475652ae78edab486764504ddb4
created_at: 2026-10-22T21:11:03.068315056Z
event:
  event: finding
  recommendation: fix_now
---

R2 low: the shell turns errors into strings with e.to_string() (app/src-tauri/src/main.rs:985 and :1020), which keeps only the outermost anyhow context. Several contexts are bare paths (crates/astromech-galaxy/src/neutron.rs:84 'reading <path>', :279 'writing <part>', :276 'creating <dir>', :298 'replacing <path>'). Scenario: on Windows, antivirus is scanning neutron.bin, the rename fails, and the Route page shows just 'replacing C:\Users\...\neutron.bin', with no reason and no next step; a truncated neutron.bin shows 'The file there can't be read: reading <path>'. Fix: format!("{e:#}") in the shell, or contexts that are sentences with the reason.
