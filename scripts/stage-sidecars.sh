#!/usr/bin/env bash
# Copy the core and the voice worker into app/src-tauri/binaries/ under the names Tauri's
# externalBin expects (<name>-<target triple>.exe), so `tauri build` bundles them next to the
# app window in the NSIS installer.
set -euo pipefail
TRIPLE=${TRIPLE:-x86_64-pc-windows-msvc}
SRC=${1:-target/$TRIPLE/release}
[ -d "$SRC" ] || SRC=target/release
DST=app/src-tauri/binaries
mkdir -p "$DST"
for b in astromech-core astromech-voice-worker; do
  if [ ! -f "$SRC/$b.exe" ] && [ "${PLACEHOLDER:-0}" = 1 ]; then
    # CI's plain shell build only needs the files to exist (tauri-build checks externalBin).
    : > "$DST/$b-$TRIPLE.exe"
    echo "placeholder $DST/$b-$TRIPLE.exe"
    continue
  fi
  cp "$SRC/$b.exe" "$DST/$b-$TRIPLE.exe"
  echo "staged $DST/$b-$TRIPLE.exe"
done
