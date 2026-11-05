#!/bin/sh
# HEAD each public Spansh dump named on the command line: size and date only, nothing downloaded.
UA="astromech-spike/q1 (+https://github.com/brewer-corp/astromech)"
for f in "$@"; do
  printf "%s " "$f"
  curl -sSI -A "$UA" "https://downloads.spansh.co.uk/$f" | grep -iE "^HTTP|content-length|last-modified" | tr '\r\n' '  '
  echo
  sleep 1
done
