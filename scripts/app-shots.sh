#!/usr/bin/env bash
# Screenshot every app screen in the browser preview (example fixture, no Tauri) with headless
# Chrome. Usage: scripts/app-shots.sh [out dir]   (default /workspace/m1-screens)
set -euo pipefail
OUT=${1:-/workspace/m1-screens}
mkdir -p "$OUT"
cd "$(dirname "$0")/../app"
pnpm -s build >/dev/null
CHROME=${CHROME:-$(command -v google-chrome || command -v google-chrome-stable || command -v chromium || command -v chromium-browser)}
if [[ -z "${CHROME}" ]]; then
  echo "scripts/app-shots.sh: Chrome not found (install Google Chrome, or set CHROME). For clicks and waits use scripts/ui." >&2
  exit 1
fi
pnpm -s exec vite preview --port 4180 --strictPort >/dev/null 2>&1 &
PREV=$!
trap 'kill $PREV' EXIT
sleep 2
shot() { # name size url
  "$CHROME" --headless=new --user-data-dir="$(mktemp -d)" --disk-cache-size=1 --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
    --window-size=${2/x/,} --virtual-time-budget=4000 --screenshot="$OUT/$1.png" "http://localhost:4180/$3" 2>/dev/null
}
shot 01-activities-flight 1920x1080 "#/activities"
shot 02-activities-exploration 1920x1080 "?activity=exploration-fss-dss#/activities"
shot 03-activities-pressed 1920x1080 "?activity=trading&press=231D3201:Joy_4#/activities"
shot 04-keybinds 1600x1000 "#/keybinds"
shot 05-keybinds-pressed 1600x1000 "?press=231D3201:Joy_4#/keybinds"
shot 06-keybinds-search 1600x1000 "?q=night#/keybinds"
shot 07-bind-dialog 1600x1000 "?bind=NightVisionToggle:Secondary#/keybinds"
shot 08-bind-dialog-pressed 1600x1000 "?bind=NightVisionToggle:Secondary&press=Keyboard:Key_N#/keybinds"
shot 09-controller-left 1600x1000 "#/controller"
shot 10-controller-pressed 1600x1000 "?press=231D3201:Joy_10#/controller"
shot 11-controller-unknown 1600x1000 "?press=231D0200:Joy_31#/controller"
shot 12-presets 1600x1000 "#/presets"
shot 13-diagnostics-keytester 1600x1000 "?press=231D0200:Joy_1#/diagnostics"
ls -la "$OUT"
