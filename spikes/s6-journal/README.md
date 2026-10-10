# S6: journal parser evaluation (8 Oct 2026)

`harness/` is a Rust harness on `ed-journals = "=0.13.0"`. Build it with `cargo build --release`
(strict) or `--features unknown` (allow-unknown), then run `s6-harness <dir-or-json>...`
(`S6_VERBOSE=1` lists failures).

The corpora it ran on are not in this repo: the `ed-journals` test files (MIT, from
github.com/rster2002/ed-journals), kayahr/ed-journal as an event-list reference, and samples
harvested from other tools. Verdict and numbers: docs/ARCHITECTURE.md §17.2.
