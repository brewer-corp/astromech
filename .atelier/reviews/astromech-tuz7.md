---
schema: atelier.review
schema_version: 1
id: astromech-tuz7
title: 'R1 [medium] crates/astromech-store/src/paths.rs:117 carry_over_with: regression on filesystems without hard links. CreateHardLinkW is NTFS-only, so with the data folder on FAT32/exFAT (a portable install on a USB stick: data_dir = exe_dir/astromech-data, or a data_dir override on such a drive) hard_link always errors; the temp is removed and Err returned, main.rs:145 only logs it, and Api::discover opens the data folder without the overrides. Fails safe (nothing overwritten or deleted, original stays in the app dir) but the user''s saved binding overrides are silently not seen on every launch, where the old create_new+copy worked there. Released versions (v0.1.x) exist, so carried-over users are real. Fix: when hard_link fails with anything but AlreadyExists, fall back to the previous no-overwrite path for the final name (OpenOptions::create_new on the target and copy from the staged temp, removing the target if that copy fails), and add a test driving the fallback (e.g. make the link step injectable).'
target: astromech-pb3o
identity: review-agent (opus, independent)
commit: 4298bfb2a7d7baf0bcab20f3ab9534e470434f1c
created_at: 2026-10-17T08:42:08.863283485Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] crates/astromech-store/src/paths.rs:117 carry_over_with: regression on filesystems without hard links. CreateHardLinkW is NTFS-only, so with the data folder on FAT32/exFAT (a portable install on a USB stick: data_dir = exe_dir/astromech-data, or a data_dir override on such a drive) hard_link always errors; the temp is removed and Err returned, main.rs:145 only logs it, and Api::discover opens the data folder without the overrides. Fails safe (nothing overwritten or deleted, original stays in the app dir) but the user's saved binding overrides are silently not seen on every launch, where the old create_new+copy worked there. Released versions (v0.1.x) exist, so carried-over users are real. Fix: when hard_link fails with anything but AlreadyExists, fall back to the previous no-overwrite path for the final name (OpenOptions::create_new on the target and copy from the staged temp, removing the target if that copy fails), and add a test driving the fallback (e.g. make the link step injectable).
