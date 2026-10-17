---
created_at: "2026-10-16T22:33:54.368887727+00:00"
id: "astromech-ub9t"
evidence_type: "test"
commit: "84319549979f0d7d0187ba980a1a9921e30dc1e5"
result: "pass"
captured_at: "2026-10-16T22:33:53.784899648+00:00"
command: "cargo metadata --locked --offline --format-version 1"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-2u1x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-2u1x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "app/src-tauri lockfile resolves offline at 84319549979f0d7d0187ba980a1a9921e30dc1e5"
updated_at: "2026-10-16T22:33:54.369300705+00:00"
---

## Summary

app/src-tauri lockfile resolves offline at 84319549979f0d7d0187ba980a1a9921e30dc1e5

## Command

```console
cargo metadata --locked --offline --format-version 1
```

Exit status: 0

## Stdout

Bytes: 2593555
Truncated: yes

```text
{"packages":[{"name":"adler2","version":"2.0.1","id":"registry+https://github.com/rust-lang/crates.io-index#adler2@2.0.1","license":"0BSD OR MIT OR Apache-2.0","license_file":null,"description":"A simple clean-room implementation of the Adler-32 checksum","source":"registry+https://github.com/rust-lang/crates.io-index","dependencies":[{"name":"rustc-std-workspace-core","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1.0.0","kind":null,"rename":"core","optional":true,"uses_default_features":true,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"adler2","src_path":"/home/supadava/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/adler2-2.0.1/src/lib.rs","edition":"2021","doc":true,"doctest":true,"test":true},{"kind":["bench"],"crate_types":["bin"],"name":"bench","src_path":"/home/supadava/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/adler2-2.0.1/benches/bench.rs","edition":"2021","doc":false,"doctest":false,"test":false}],"features":{"core":["dep:core"],"default":["std"],"rustc-dep-of-std":["core"],"std":[]},"manifest_path":"/home/supadava/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/adler2-2.0.1/Cargo.toml","metadata":{"docs":{"rs":{"rustdoc-args":["--cfg=docsrs"]}},"release":{"no-dev-version":true,"pre-release-commit-message":"Release {{version}}","tag-message":"{{version}}","pre-release-replacements":[{"file":"CHANGELOG.md","replace":"## Unreleased\n\nNo changes.\n\n## [{{version}} - {{date}}](https://github.com/jonas-schievink/adler/releases/tag/v{{version}})\n","search":"## Unreleased\n"},{"file":"README.md","replace":"adler = \"{{version}}\"","search":"adler = \"[a-z0-9\\\\.-]+\""},{"file":"src/lib.rs","replace":"https://docs.rs/adler/{{version}}","search":"https://docs.rs/adler/[a-z0-9\\.-]+"}]}},"publish":null,"authors":["Jonas Schievink <jonasschievink@gmail.com>","oyvindln <oyvindln@users.noreply.github.com>"],"categories":["algorithms"],"keywords":["checksum","integrity","hash","adler32","zlib"],"readme":"README.md","repository":"https://github.com/oyvindln/adler2","homepage":null,"documentation":"https://docs.rs/adler2/","edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"aho-corasick","version":"1.1.5","id":"registry+https://github.com/rust-lang/crates.io-index#aho-corasick@1.1.5","license":"Unlicense OR MIT","license_file":null,"description":"Fast multiple substring searching.","source":"registry+https://github.com/rust-lang/crates.io-index","dependencies":[{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^0.4.17","kind":null,"rename":null,"optional":true,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"memchr","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^2.4.0","kind":null,"rename":null,"optional":true,"uses_default_features":false,"features":[],"target":null,"registry":null},{"name":"doc-comment","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^0.3.3","kind":"dev","rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"aho_corasick","src_path":"/home/supadava/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/aho-corasick-1.1.5/src/lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{"default":["std","perf-literal"],"logging":["dep:log"],"perf-literal":["dep:memchr"],"std":["memchr?/std"]},"manifest_path":"/home/supadava/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/aho-corasick-1.1.5/Cargo.toml","metadata":{"docs":{"rs":{"all-features":true,"rustdoc-args":["--cfg","docsrs","--generate-link-to-definition"]}}},"publish":null,"authors":["Andrew Gallant <jamslam@gmail.com>"],"categories":["text-processing"],"keywords":["string","search","text","pattern","multi"],"readme":"README.md","repository":"https://github.com/BurntSushi/aho-corasick","homepage":"https://github.com/BurntSushi/aho-corasick","documentation":null,"edition":"2021
```

## Stderr

Bytes: 0
Truncated: no

```text
```

