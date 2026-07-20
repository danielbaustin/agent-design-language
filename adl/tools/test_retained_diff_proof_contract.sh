#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 || -z "$1" || -z "$2" ]]; then
  echo "usage: $0 <base-revision> <head-revision>" >&2
  exit 64
fi

base_revision=$1
head_revision=$2

git rev-parse --verify "${base_revision}^{commit}" >/dev/null
git rev-parse --verify "${head_revision}^{commit}" >/dev/null
git diff --check "${base_revision}..${head_revision}"
