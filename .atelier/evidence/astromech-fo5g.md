---
created_at: "2026-10-17T00:26:46.478645241+00:00"
id: "astromech-fo5g"
evidence_type: "test"
commit: "d4a455a92aa5b2af7697f6ed14ad4332b3c1909b"
result: "pass"
captured_at: "2026-10-17T00:24:57.199224532+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cargo metadata --locked --offline --format-version 1 >/dev/null && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && USERPROFILE=\"$HOME/.cache/astromech-5t34/fakehome\" cargo test -p astromech-core --lib -- a_journal_folder history'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-5t34"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-5t34"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main (cmhv reaction engine): fmt, clippy -D warnings, cargo test --workspace, check-deps, both lockfiles --locked; history and set_folders tests pass with USERPROFILE at a fake home with journals"
updated_at: "2026-10-17T00:26:46.479163028+00:00"
---

## Summary

After merging main (cmhv reaction engine): fmt, clippy -D warnings, cargo test --workspace, check-deps, both lockfiles --locked; history and set_folders tests pass with USERPROFILE at a fake home with journals

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cargo metadata --locked --offline --format-version 1 >/dev/null && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && USERPROFILE="$HOME/.cache/astromech-5t34/fakehome" cargo test -p astromech-core --lib -- a_journal_folder history'
```

Exit status: 0

## Stdout

Bytes: 43606
Truncated: yes

```text

running 3 tests
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_motherlode_always_gets_a_line ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test a_disabled_rule_never_fires ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok
test each_scan_type_gives_its_line ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::claims_and_beacons ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::shopping_tests::cargo_json_overrides_the_replayed_hold ... ok
test tests::save_writes_only_changes ... ok
test tests::survives_restart ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 84 tests
test autostart::tests::state_round_trip ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test calibration::tests::steps_and_errors ... ok
test calibration::tests::level_check_verdicts ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test config::tests::finds_look_alike_config_names ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test control::tests::round_trip_and_token ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::bounce_gives_one_clip ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::replay_fixture_re
```

## Stderr

Bytes: 30542
Truncated: yes

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.26
   Compiling quote v1.0.47
   Compiling serde_core v1.0.229
   Compiling autocfg v1.5.1
    Checking memchr v2.8.3
   Compiling serde v1.0.229
   Compiling num-traits v0.2.19
    Checking cfg-if v1.0.5
   Compiling zmij v1.0.23
   Compiling serde_json v1.0.152
    Checking itoa v1.0.18
   Compiling libc v0.2.190
   Compiling thiserror v2.0.21
    Checking bitflags v2.13.2
    Checking iana-time-zone v0.1.65
    Checking once_cell v1.21.4
    Checking chrono v0.4.45
    Checking smallvec v1.16.3
   Compiling find-msvc-tools v0.1.14
   Compiling syn v3.0.7
   Compiling shlex v2.0.1
    Checking foldhash v0.2.0
    Checking equivalent v1.0.2
    Checking allocator-api2 v0.2.21
   Compiling rustix v1.1.5
   Compiling cc v1.7.0
    Checking linux-raw-sys v0.12.1
    Checking hashbrown v0.17.1
   Compiling pkg-config v0.3.34
   Compiling crc32fast v1.5.2
    Checking simd-adler32 v0.3.10
    Checking adler2 v2.0.1
    Checking stable_deref_trait v1.2.1
   Compiling getrandom v0.4.3
    Checking miniz_oxide v0.9.1
    Checking zlib-rs v0.6.8
    Checking fastrand v2.5.0
   Compiling vcpkg v0.2.15
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling synstructure v0.14.0
   Compiling zerovec-derive v0.11.6
    Checking flate2 v1.1.10
    Checking log v0.4.34
    Checking base64 v0.22.1
   Compiling zerofrom-derive v0.1.8
   Compiling yoke-derive v0.8.4
    Checking tempfile v3.27.0
   Compiling libsqlite3-sys v0.38.2
   Compiling displaydoc v0.2.7
    Checking zerofrom v0.1.8
    Checking hashlink v0.12.2
    Checking yoke v0.8.3
    Checking fallible-iterator v0.3.0
    Checking fallible-streaming-iterator v0.1.9
    Checking zerovec v0.11.8
    Checking writeable v0.6.4
    Checking litemap v0.8.3
    Checking zeroize v1.9.1
    Checking zerotrie v0.2.5
    Checking utf8_iter v1.0.4
   Compiling icu_properties_data v2.3.0
    Checking tinystr v0.8.4
    Checking potential_utf v0.1.6
    Checking icu_locale_core v2.3.0
   Compiling icu_normalizer_data v2.3.0
   Compiling ring v0.17.14
    Checking icu_collections v2.3.0
    Checking icu_provider v2.3.1
    Checking rustls-pki-types v1.15.1
   Compiling anyhow v1.0.104
    Checking percent-encoding v2.3.2
    Checking icu_properties v2.3.0
    Checking icu_normalizer v2.3.0
    Checking getrandom v0.2.17
    Checking subtle v2.6.1
    Checking untrusted v0.9.0
    Checking idna_adapter v1.2.2
   Compiling rustls v0.23.46
    Checking idna v1.1.0
    Checking form_urlencoded v1.2.2
    Checking webpki-roots v1.0.9
   Compiling version_check v0.9.5
    Checking url v2.5.8
    Checking webpki-roots v0.26.11
    Checking astromech-status v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-status)
   Compiling generic-array v0.14.7
    Checking roxmltree v0.21.1
    Checking astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-journal)
    Checking typenum v1.20.1
    Checking astromech-keybinds v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-keybinds)
   Compiling syn v2.0.119
    Checking winnow v1.0.4
    Checking block-buffer v0.10.4
    Checking rusqlite v0.40.2
    Checking astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-materials)
    Checking astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-powerplay)
    Checking toml_parser v1.1.5+spec-1.1.0
    Checking astromech-store v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-store)
    Checking crypto-common v0.1.7
   Compiling heck v0.5.0
    Checking toml_writer v1.1.3+spec-1.1.0
    Checking digest v0.10.7
   Compiling bzip2-sys v0.1.13+1.0.8
    Checking serde_spanned v1.1.2
    Checking toml_datetime v0.7.5+spec-1.1.0
    Checking astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech
```

