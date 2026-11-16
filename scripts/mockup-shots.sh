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
  glow-icons:1400x640:glow-up-icons
  nav-route:2560x1440:nav-spike/1-sidebar-materials-2560 nav-route-over:1100x700:nav-spike/1-route-over-page-1100
  nav-panel:2560x1440:nav-spike/2-sidebar-and-detail-panel-2560 nav-collapsed:2560x1440:nav-spike/3-collapsed-strip-2560
  nav-noroute:2560x1440:nav-spike/4-no-route-2560 nav-ships:2560x1440:nav-spike/5-ships-2560
  nav-suits:2560x1440:nav-spike/6-suits-2560 nav-materials:2560x1440:nav-spike/7-materials-on-foot-2560
  nav-engineers:2560x1440:nav-spike/8-engineers-2560 nav-engineers:1100x700:nav-spike/8-engineers-1100
  nav-engineers-laser:2560x1440:nav-spike/9-engineers-laser-2560 nav-engineers-laser:1100x700:nav-spike/9-engineers-laser-1100
  nav-panel:1590x900:nav-spike/header-stacked-1590
  hier-A-sell:2560x1440:nav-spike/hier-A-sell-2560 hier-A-engineers:2560x1440:nav-spike/hier-A-engineers-2560
  hier-A-over:1100x700:nav-spike/hier-A-over-page-1100 hier-B-sell:2560x1440:nav-spike/hier-B-sell-2560
  hier-B-engineers:2560x1440:nav-spike/hier-B-engineers-2560 hier-B-collapsed:2560x1440:nav-spike/hier-B-collapsed-2560
  tone-0-sell:2560x1440:nav-spike/tone-0-current-sell-2560 tone-1-sell:2560x1440:nav-spike/tone-1-sell-2560
  tone-2-sell:2560x1440:nav-spike/tone-2-sell-2560 tone-3-sell:2560x1440:nav-spike/tone-3-sell-2560
  tone-4-sell:2560x1440:nav-spike/tone-4-sell-2560 tone-2-engineers:2560x1440:nav-spike/tone-2-engineers-2560
  tone-4-engineers:2560x1440:nav-spike/tone-4-engineers-2560
  tone5-sell:2560x1440:nav-spike/tone5-sell-2560 tone5-sell:1100x700:nav-spike/tone5-sell-1100
  tone5-engineers-laser:2560x1440:nav-spike/tone5-engineers-laser-2560 tone5-engineers-rest:2560x1440:nav-spike/tone5-engineers-rest-2560
  tone5-suits:2560x1440:nav-spike/tone5-suits-2560)
# tone-compare-sell.png is the five tone-*-sell shots in a labelled grid, put together by hand.
# The nav-* ones are the nav spike (branch spike/nav-rework): the route as a left sidebar, SHIPS split from ENGINEERING.
# The last is DESIGN.md's one-row header at its 1100 px minimum window width (astromech-pey5).
for p in "${SHOTS[@]}"; do
  IFS=: read -r id size file <<<"$p"
  if [[ $# -gt 0 && " $* " != *" $id "* ]]; then continue; fi
  "$CHROME" --headless=new --user-data-dir="$(mktemp -d)" --disk-cache-size=1 --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
    --window-size=${size/x/,} --virtual-time-budget=3000 \
    --screenshot="../docs/design/${file:-mockup-$id}.png" "http://localhost:4179/#/mockup/$id" 2>/dev/null
done
ls -la ../docs/design
