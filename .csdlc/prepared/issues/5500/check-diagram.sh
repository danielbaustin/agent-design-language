#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
source_file="$root/.csdlc/prepared/issues/5500/diagram.mmd"
scratch="$(mktemp -d "${TMPDIR:-/tmp}/adl-5500-diagram.XXXXXX")"
trap 'rm -rf "$scratch"' EXIT

command -v mmdc >/dev/null
mmdc --quiet --input "$source_file" --output "$scratch/diagram.svg"
test -s "$scratch/diagram.svg"
printf '{"status":"pass","parser":"mermaid-cli","diagram":".csdlc/prepared/issues/5500/diagram.mmd"}\n'
