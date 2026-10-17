#!/usr/bin/env bash
# Synthetic voice-recognition eval, in one command: builds the voice worker, downloads the
# speech models if they are missing (about 830 MB, SHA-256 checked), renders the 200-phrase set
# (crates/astromech-core/tests/voice-eval-phrases.tsv) with Kokoro, transcribes it with Parakeet
# and scores it by command family. See crates/astromech-core/examples/voice-eval.rs.
#
#   scripts/voice-eval.sh [extra voice-eval args, e.g. --voices bf_emma]
#
# MODELS: models folder (default ~/.cache/astromech/models). OUT: output folder (default
# results/voice-eval, git-ignored). The report is $OUT/report.md. Audio and models are never
# committed (CONTRIBUTING.md).
set -euo pipefail
cd "$(dirname "$0")/.."
MODELS="${MODELS:-$HOME/.cache/astromech/models}"
OUT="${OUT:-results/voice-eval}"
cargo build --release -p astromech-voice --features sherpa --bin astromech-voice-worker
worker="target/release/astromech-voice-worker$( [[ "${OS:-}" == Windows_NT ]] && echo .exe || true )"
cargo run -p astromech-core --example voice-eval -- \
  --worker "$worker" --models "$MODELS" --out "$OUT" --download "$@"
