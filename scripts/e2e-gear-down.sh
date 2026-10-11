#!/usr/bin/env bash
# Gear-down end-to-end under Wine (or on Windows with Git Bash: set RUN="" ).
# Runs the real astromech-core.exe (typed commands, no speech models) against the mock game
# window, which receives the SendInput key and rewrites Status.json after 0-1000 ms.
set -euo pipefail
RUN=${RUN-wine}
T=${TARGET_DIR:-target/x86_64-pc-windows-msvc/release}
W=$(mktemp -d)
mkdir -p "$W/journal" "$W/Bindings"
cat > "$W/Bindings/DualVKB.4.2.binds" <<'X'
<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="DualVKB" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="231D3201" Key="Joy_4" /><Secondary Device="Keyboard" Key="Key_L" /></LandingGearToggle>
</Root>
X
printf 'DualVKB\nDualVKB\nDualVKB\nDualVKB\n' > "$W/Bindings/StartPreset.4.start"
winpath() { if [ -n "$RUN" ]; then winepath -w "$1"; else cygpath -w "$1"; fi; }
cat > "$W/astromech.toml" <<X
data_dir = '$(winpath "$W/data")'
[game]
journal_dir = '$(winpath "$W/journal")'
bindings_dir = '$(winpath "$W/Bindings")'
game_dir = '$(winpath "$W/nogame")'
[controls]
ptt = { device = "231D3201", key = "Joy_15" }
X
# A mis-named config must stop the app with a message naming the look-alike.
mkdir -p "$W/misnamed" && cp "$T/astromech-core.exe" "$W/misnamed/" && cp "$W/astromech.toml" "$W/misnamed/astromech.toml.txt"
if echo quit | $RUN "$W/misnamed/astromech-core.exe" --console --no-voice > "$W/misnamed.txt" 2>&1; then
  echo "E2E FAIL: started without a config"; exit 1
fi
grep -q 'astromech.toml.txt' "$W/misnamed.txt" || { cat "$W/misnamed.txt"; echo "E2E FAIL: look-alike not named"; exit 1; }
# Key_L = scan 0x26; gear = bit 2 (4); delay 0-1000 ms; drop the first press (miss: logged, never re-pressed).
$RUN "$T/examples/mockgame.exe" "$(winpath "$W/journal")" 26 0 4 0-1000 25 1 &
MOCK=$!
sleep 3
{
  sleep 4; echo "gear down"      # first press dropped by the mock: logged "not confirmed", no retry
  sleep 4; echo "gear down"      # asked again: lands, "confirmed in N ms"
  sleep 4; echo "gear down"      # already down: presses nothing
  sleep 1; echo "gear up"
  echo "gear up"                 # duplicate while in flight
  sleep 4; echo "!kill"; echo "gear down"; echo "!rearm"
  sleep 1; echo "quit"
} | $RUN "$T/astromech-core.exe" --console --no-voice --config "$(winpath "$W/astromech.toml")" | tee "$W/out.txt"
wait $MOCK || true
echo "---- mock keys.log"; cat "$W/journal/keys.log"
fail() { echo "E2E FAIL: $1"; exit 1; }
grep -q "Landing gear down." "$W/out.txt" || fail "no confirmation"
grep -q "Landing gear is already down." "$W/out.txt" || fail "no already-down"
grep -q "Already on it." "$W/out.txt" || fail "no duplicate block"
grep -q "Kill switch is on" "$W/out.txt" || fail "kill switch didn't block"
[ "$(grep -c 'not confirmed within' "$W/out.txt")" = 1 ] || fail "expected exactly one unconfirmed press"
grep -q "didn't respond" "$W/out.txt" && fail "failure was spoken"
grep -q "confirmed in" "$W/out.txt" || fail "no confirmation logged"
grep -q "Shinrarta Dezhra" "$W/out.txt" || true
[ "$(grep -c 'down scan=26' "$W/journal/keys.log")" = 3 ] || fail "expected 3 Key_L presses (miss, asked again, gear up)"
grep -q "dropped this press" "$W/journal/keys.log" || fail "mock didn't drop"
echo "E2E OK ($W)"
