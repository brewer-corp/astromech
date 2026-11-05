#!/bin/sh
# Download only the first MB megabytes of a public Spansh dump (HTTP Range) into dumps/,
# for sampling record shapes and sizes. Usage: dump-sample.sh <file> <MB>
UA="astromech-spike/q1 (+https://github.com/brewer-corp/astromech)"
dir="$(dirname "$0")/dumps"
mkdir -p "$dir"
end=$(( $2 * 1024 * 1024 - 1 ))
curl -sS -A "$UA" -r "0-$end" -o "$dir/$1.head" -w "%{http_code} %{size_download}\n" "https://downloads.spansh.co.uk/$1"
