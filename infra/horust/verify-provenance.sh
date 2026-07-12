#!/bin/sh
set -eu

archive=${1:?usage: verify-provenance.sh /path/to/horust-0.1.13.crate}
expected=$(jq -r '.crate_sha256' "$(dirname "$0")/horust-0.1.13.provenance.json")
actual=$(shasum -a 256 "$archive" | awk '{print $1}')

if [ "$actual" != "$expected" ]; then
  printf '%s\n' "Horust archive checksum mismatch" >&2
  exit 1
fi
printf '%s\n' "Horust archive checksum verified: $actual"
