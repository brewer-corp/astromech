---
created_at: "2026-10-22T21:18:13.986827176+00:00"
id: "astromech-hhwm"
evidence_type: "test"
commit: "9c69fe72f8f0fb65a25d229f5d7608e5bd449c12"
result: "pass"
captured_at: "2026-10-22T21:17:33.810345278+00:00"
command: "bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test neutron_download && CARGO_BUILD_JOBS=4 cargo test -p astromech-data && CARGO_BUILD_JOBS=4 cargo clippy -p astromech-galaxy -p astromech-data --all-targets -- -D warnings && cargo fmt --all --check && python3 scripts/check-guide.py && python3 scripts/check-deps.py && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "claude-implementer-1842"
target:
  kind: "issue"
  id: "astromech-d4of"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-d4of"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes R2/R4: local-server download tests (6) pass: good download, wrong checksum leaves no file, damaged gzip and 404 keep the old file, a blocked rename gives 'Couldn't replace <path>: <OS reason>. ... Wait a minute, then press Download again.', a stale neutron.bin.part is removed on reading the state and on starting a download, and is kept (with a second download turned down) while one runs. Mutations: no stale removal, and removal during a download, each fail their test. Data crate tests, clippy, fmt, check-deps, check-guide, frontend lint/check/build/test pass. Shell (app/src-tauri) not compiled here: CI-verified only."
updated_at: "2026-10-22T21:18:13.987197506+00:00"
---

## Summary

After review fixes R2/R4: local-server download tests (6) pass: good download, wrong checksum leaves no file, damaged gzip and 404 keep the old file, a blocked rename gives 'Couldn't replace <path>: <OS reason>. ... Wait a minute, then press Download again.', a stale neutron.bin.part is removed on reading the state and on starting a download, and is kept (with a second download turned down) while one runs. Mutations: no stale removal, and removal during a download, each fail their test. Data crate tests, clippy, fmt, check-deps, check-guide, frontend lint/check/build/test pass. Shell (app/src-tauri) not compiled here: CI-verified only.

## Command

```console
bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test neutron_download && CARGO_BUILD_JOBS=4 cargo test -p astromech-data && CARGO_BUILD_JOBS=4 cargo clippy -p astromech-galaxy -p astromech-data --all-targets -- -D warnings && cargo fmt --all --check && python3 scripts/check-guide.py && python3 scripts/check-deps.py && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 13787
Truncated: yes

```text

running 6 tests
test a_partial_file_left_by_a_cut_off_download_is_removed ... ok
test a_running_download_keeps_its_partial_file_and_turns_down_a_second ... ok
test a_failed_rename_says_why_and_what_to_do ... ok
test a_wrong_checksum_leaves_no_file ... ok
test a_failed_download_keeps_the_old_file ... ok
test downloads_checks_and_unpacks_the_file ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test transport::tests::segments_and_cache_keys ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok
test cache::tests::the_shared_file_waits_for_the_other_process ... ok
test cache::tests::a_v015_table_without_expiry_times_is_replaced ... ok
test cache::tests::two_openers_of_a_v015_file_both_get_the_file_backed_cache ... ok
test cache::tests::two_openers_of_a_fresh_file_both_get_the_file_backed_cache ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s


running 21 tests
test a_schema_error_is_one_spoken_sentence_without_the_serde_detail ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test a_broken_cache_still_returns_the_network_answer ... ok
test http_errors_are_plain_words ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test no_network_gives_a_plain_words_error ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test expired_rows_are_pruned_so_the_cache_holds_only_live_answers ... ok
test uncached_calls_always_ask ... ok
test spansh_route_job_round_trip ... ok
test spansh_exact_route_failures_are_plain_words ... ok
test spansh_exact_route_posts_the_drive_and_polls_until_done ... ok
test spansh_trade_route_polls_until_the_job_is_done ... ok
test spansh_system_names_autocomplete ... ok
test spansh_neutron_route_polls_until_done ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok
test spansh_searches_parse_recorded_answers ... ok
test spansh_trade_route_errors_are_plain_words ... ok
test a_cache_locked_by_the_other_process_waits_for_it_then_caches ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.40s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

checked 22 guide pages: 0 problem(s)
dependency rules OK (29 workspace crates)

> astromech-app@1.0.0-alpha.2 lint /home/supadava/projects/astromech-wt/astromech-1842/app
> eslint .


> astromech-app@1.0.0-alpha.2 check /home/supadava/projects/astromech-wt/astromech-1842/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792703869002 START "/home/supadava/projects/astromech-wt/astromech-1842/app"
1792703869011 COMPLETED 431 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@1.0.0-alpha.2 build /home/supadava/projects/astromech-wt/astromech-1842/app
> vite build

vite v7.3.7 building client environment for production...
transforming...
✓ 370 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                                                          0.39 kB │ gzip:  0.26 kB
dist/assets/saira-semi-condensed-vietnamese-600-normal-CtOKQqyp.woff     5.85 kB
dist/assets/saira-semi-condensed-vietnamese-400-normal-DZeVTnup.woff     5.89 kB
dist/assets/saira-semi-condensed-vietnamese-500-normal-Dvrr_M0-.woff     5.90 kB
dist/assets/saira-semi-condensed-vietnamese-600-normal-BleKkWhQ.woff2    7.06 kB
dist/assets/saira-semi-condensed-vietnamese-400-normal-BjlVRgV2.woff2    7.08 kB
dist/assets/saira-semi-condensed-vietnamese-500-normal-DaSOOpQd.woff2    7.14 kB
dist/assets/share-tech-mono-latin-400-normal-15MgdvEM.woff              10.24 kB
dist/assets/share-tech-mono-latin-400-normal-CCqdbj5z.woff2             13.50 kB
dist/assets/saira-semi-condensed-latin-ext-400-normal-FBtzH8lb.woff     13.54 kB
dist/assets/saira-semi-condensed-latin-ext-600-normal-gxH8H1Hk.
```

## Stderr

Bytes: 905
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running tests/neutron_download.rs (target/debug/deps/neutron_download-655309234f20f4c9)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-78c794c8e5ff07b8)
     Running tests/recorded.rs (target/debug/deps/recorded-b2eb9a24a3928e84)
   Doc-tests astromech_data
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
[plugin vite:reporter] 
(!) /home/supadava/projects/astromech-wt/astromech-1842/app/src/modules/route/View.svelte is dynamically imported by /home/supadava/projects/astromech-wt/astromech-1842/app/src/modules/route/manifest.ts but also statically imported by /home/supadava/projects/astromech-wt/astromech-1842/app/src/modules/route/Rail.svelte, dynamic import will not move module into another chunk.
```

