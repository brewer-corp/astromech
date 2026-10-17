---
schema: atelier.review
schema_version: 1
id: astromech-glkh
title: 'Non-atomic copy (low), crates/astromech-store/src/paths.rs carry_over: create_new then io::copy into the final name, so a crash or I/O error mid-copy leaves a truncated device-overrides.json in the data folder; later starts see to.exists() and never retry, and Api::overrides() reads the bad file. The user''s original in the app dir is untouched, so nothing is lost, only hidden. Fix: copy to a temp file in data_dir and hard_link/rename-noclobber it into place, or remove the partial file on copy error.'
target: astromech-7w5d
identity: review-agent (opus, independent)
commit: 8a46cb6d7e122d233226efd204a31f8aee0f1735
created_at: 2026-10-17T07:46:21.929941358Z
event:
  event: finding
  recommendation: defer
---

Non-atomic copy (low), crates/astromech-store/src/paths.rs carry_over: create_new then io::copy into the final name, so a crash or I/O error mid-copy leaves a truncated device-overrides.json in the data folder; later starts see to.exists() and never retry, and Api::overrides() reads the bad file. The user's original in the app dir is untouched, so nothing is lost, only hidden. Fix: copy to a temp file in data_dir and hard_link/rename-noclobber it into place, or remove the partial file on copy error.
