---
schema: atelier.review
schema_version: 1
id: astromech-l7d0
title: 'Checked, no action: open_external allow-list is now app dir, log dir and data_dir(), directories only, canonicalised and starts_with-checked; data_dir comes from the user''s own config, so it widens only to a folder the user named. Core''s update-apply CLI log now follows Paths::log_dir (portable-aware) where it used app_dir/logs before, and Core info/diagnostics log_dir now reports the real sink folder; both are sensible and the endpoint file core.json stays in app_dir in every layout. check-deps OK; cargo test -p astromech-store -p astromech-core all pass (98 + others, 0 failed).'
target: astromech-4yds
identity: review-agent (sonnet, independent)
commit: 971e5efaa926c58970fd440f72b15847d1d6a6cb
created_at: 2026-10-17T06:35:09.522714903Z
event:
  event: finding
  recommendation: no_action
---

Checked, no action: open_external allow-list is now app dir, log dir and data_dir(), directories only, canonicalised and starts_with-checked; data_dir comes from the user's own config, so it widens only to a folder the user named. Core's update-apply CLI log now follows Paths::log_dir (portable-aware) where it used app_dir/logs before, and Core info/diagnostics log_dir now reports the real sink folder; both are sensible and the endpoint file core.json stays in app_dir in every layout. check-deps OK; cargo test -p astromech-store -p astromech-core all pass (98 + others, 0 failed).
