---
schema: atelier.review
schema_version: 1
id: astromech-yxnu
title: 'Approved at HEAD. carry_over copies only when the app dir has the file and the data folder doesn''t, uses create_new so it never overwrites, never deletes or moves the original; failure is logged and only means the old overrides aren''t seen. Old shell app_dir (LOCALAPPDATA\astromech\data) matches Paths::app_dir on Windows. Read main.rs call site for compile errors: none seen (data borrowed for log before move into discover). cargo test -p astromech-store 14 passed; check-deps OK; mutations caught. One DEFER (non-atomic copy). Windows CI was pending at review time.'
target: astromech-7w5d
identity: review-agent (opus, independent)
commit: 8a46cb6d7e122d233226efd204a31f8aee0f1735
created_at: 2026-10-17T07:46:22.782801316Z
event:
  event: approval
---

Approved at HEAD. carry_over copies only when the app dir has the file and the data folder doesn't, uses create_new so it never overwrites, never deletes or moves the original; failure is logged and only means the old overrides aren't seen. Old shell app_dir (LOCALAPPDATA\astromech\data) matches Paths::app_dir on Windows. Read main.rs call site for compile errors: none seen (data borrowed for log before move into discover). cargo test -p astromech-store 14 passed; check-deps OK; mutations caught. One DEFER (non-atomic copy). Windows CI was pending at review time.
