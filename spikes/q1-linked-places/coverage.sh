#!/bin/sh
# Hotspot coverage: ringed bodies (any reserve level) against bodies with any ore hotspot
# recorded in a ring, within 50 ly of each origin given. Two Spansh searches per origin.
Q1="$(dirname "$0")/target/release/q1-linked-places"
ORES='"Void Opal","Low Temperature Diamonds","Painite","Platinum","Musgravite","Grandidierite","Alexandrite","Benitoite","Monazite","Rhodplumsite","Serendibite","Osmium","Palladium","Bromellite","Tritium"'
for o in "$@"; do
  ringed=$("$Q1" $OFFLINE probe bodies "$o" '{"distance":{"min":"0","max":"50"},"reserve_level":{"value":["Pristine","Major","Common","Low","Depleted"]}}' 1 2>/dev/null | sed -n 's/^count //p')
  hot=$("$Q1" $OFFLINE probe bodies "$o" "{\"distance\":{\"min\":\"0\",\"max\":\"50\"},\"ring_signals\":[{\"name\":[$ORES]}]}" 1 2>/dev/null | sed -n 's/^count //p')
  echo "$o: ringed bodies $ringed, with hotspots recorded $hot"
done
