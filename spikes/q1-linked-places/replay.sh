#!/bin/sh
# Rerun every recipe in FINDINGS.md. With --offline (the default here) nothing goes to the
# network: answers come from cache/, which a live run fills (`LIVE=1 sh replay.sh`).
set -e
cd "$(dirname "$0")"
cargo build --release --quiet
Q1=./target/release/q1-linked-places
OFF=--offline
[ -n "$LIVE" ] && OFF=
cargo test --release --quiet
$Q1 $OFF recipe1 origin=Sol radius=80 link=20 allegiance=Federation trader=Manufactured
$Q1 $OFF recipe1 origin=Sol radius=80 link=20 allegiance=Federation trader=Manufactured spansh_state=off
$Q1 $OFF recipe1 "origin=Shinrarta Dezhra" radius=80 link=20 allegiance=Empire trader=Manufactured fanout=on fanout_max=40
$Q1 $OFF recipe1 origin=Achenar radius=80 link=20 allegiance=Empire trader=Manufactured
$Q1 $OFF recipe1 origin=Maia radius=80 link=20 allegiance=Empire trader=Manufactured
$Q1 $OFF recipe1 origin=Sol radius=30 link=15 allegiance=Federation trader=Manufactured fanout=on fanout_max=45
$Q1 $OFF recipe2 origin=Sol radius=60 link=30 mineral=Platinum min_price=250000
$Q1 $OFF recipe2 "origin=Shinrarta Dezhra" radius=60 link=30 mineral=Platinum min_price=250000
$Q1 $OFF recipe2 origin=Achenar radius=60 link=30 mineral=Platinum min_price=250000
$Q1 $OFF recipe2 origin=Maia radius=60 link=30 mineral=Platinum min_price=250000
$Q1 $OFF recipe2 origin=Sol radius=60 link=30 mineral=Painite min_price=0
$Q1 $OFF recipe2 origin=Sol radius=60 link=30 "mineral=Low Temperature Diamonds" min_price=0
$Q1 $OFF recipe2 origin=Sol radius=25 link=15 mineral=Platinum min_price=250000 fanout=on fanout_max=45
OFFLINE=$OFF sh ./coverage.sh Sol "Shinrarta Dezhra" Achenar Maia Colonia "Beagle Point"
