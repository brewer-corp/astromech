#!/usr/bin/env bash
# Render the design mockups (app/src/mockups) to docs/design/*.png with headless Chrome.
set -euo pipefail
cd "$(dirname "$0")/../app"
pnpm -s build >/dev/null
CHROME=${CHROME:-$(command -v google-chrome || command -v chromium || command -v chromium-browser)}
pnpm -s exec vite preview --port 4179 --strictPort >/dev/null 2>&1 &
PREV=$!
trap 'kill $PREV' EXIT
sleep 2
for p in tray:420x720 keybinds:1440x900 search:1440x900 voice:1440x900; do
  id=${p%%:*}; size=${p#*:}
  "$CHROME" --headless=new --user-data-dir="$(mktemp -d)" --disk-cache-size=1 --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
    --window-size=${size/x/,} --virtual-time-budget=3000 \
    --screenshot="../docs/design/mockup-$id.png" "http://localhost:4179/#/mockup/$id" 2>/dev/null
done
ls -la ../docs/design
