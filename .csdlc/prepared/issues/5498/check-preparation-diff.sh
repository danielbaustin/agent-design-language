#!/usr/bin/env bash
set -euo pipefail

base_file=".csdlc/prepared/issues/5498/preparation-base.txt"
base="$(tr -d '[:space:]' < "$base_file")"
head="$(git rev-parse HEAD)"

git cat-file -e "${base}^{commit}"
printf 'preparation_base=%s\npreparation_head=%s\n' "$base" "$head"
git diff --check "${base}...${head}"
