#!/usr/bin/env bash
# Render the design mockups (app/src/mockups) to docs/design/*.png with headless Chrome.
# With arguments, render only those mockup ids (scripts/mockup-shots.sh ds-sell ds-places).
set -euo pipefail
cd "$(dirname "$0")/../app"
pnpm -s build >/dev/null
CHROME=${CHROME:-$(command -v google-chrome || command -v google-chrome-stable || command -v chromium || command -v chromium-browser)}
if [[ -z "${CHROME}" ]]; then
  echo "scripts/mockup-shots.sh: Chrome not found (install Google Chrome, or set CHROME)." >&2
  exit 1
fi
pnpm -s exec vite preview --port 4179 --strictPort >/dev/null 2>&1 &
PREV=$!
trap 'kill $PREV' EXIT
sleep 2
# id:size[:file], the file defaulting to mockup-<id>. The ds-* ones are DESIGN.md's after-mockups.
SHOTS=(activities:1920x1080 ship:1440x900 keybinds:1440x900 controller:1440x900 settings:1440x900 tray:640x520
  ds-materials:1600x1000:design-system-materials-after ds-sell:1600x1000:design-system-places-sell-after
  ds-mining:1600x1000:design-system-places-mining-after ds-places:1600x1000:design-system-places-home-after
  ds-settings:1600x1000:design-system-settings-after
  ds-settings:1100x700:design-system-header-1100
  glow-icons:1400x640:glow-up-icons)
# The last is DESIGN.md's one-row header at its 1100 px minimum window width (astromech-pey5).
for p in "${SHOTS[@]}"; do
  IFS=: read -r id size file <<<"$p"
  if [[ $# -gt 0 && " $* " != *" $id "* ]]; then continue; fi
  "$CHROME" --headless=new --user-data-dir="$(mktemp -d)" --disk-cache-size=1 --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
    --window-size=${size/x/,} --virtual-time-budget=3000 \
    --screenshot="../docs/design/${file:-mockup-$id}.png" "http://localhost:4179/#/mockup/$id" 2>/dev/null
done
ls -la ../docs/design
