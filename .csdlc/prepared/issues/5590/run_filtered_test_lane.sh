#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: run_filtered_test_lane.sh <manifest> <filter>" >&2
  exit 64
fi

manifest=$1
filter=$2
inventory=$(cargo test --manifest-path "$manifest" "$filter" -- --list)
count=$(printf '%s\n' "$inventory" | awk '/: test$/ { count += 1 } END { print count + 0 }')

if [ "$count" -eq 0 ]; then
  echo "filtered test lane matched zero tests: manifest=$manifest filter=$filter" >&2
  exit 65
fi

printf 'filtered_test_inventory manifest=%s filter=%s count=%s\n' "$manifest" "$filter" "$count"
cargo test --manifest-path "$manifest" "$filter"
