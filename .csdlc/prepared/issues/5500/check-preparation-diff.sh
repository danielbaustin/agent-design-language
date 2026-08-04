#!/usr/bin/env bash
set -euo pipefail

base_file=.csdlc/prepared/issues/5500/preparation-base.txt
test -s "$base_file"
base="$(tr -d '[:space:]' < "$base_file")"
git cat-file -e "$base^{commit}"
git diff --check "$base"..HEAD

changed="$({ git diff --name-only "$base"..HEAD; git diff --name-only; git diff --cached --name-only; git status --porcelain=v1 --untracked-files=all | cut -c4-; } | sort -u)"
bad="$(printf '%s\n' "$changed" | grep -Ev '^(\.csdlc/issues/5500/|\.csdlc/prepared/issues/5500/|\.csdlc/locks/5500\.lock$|$)' || true)"
if [[ -n "$bad" ]]; then
  printf 'out-of-scope preparation paths:\n%s\n' "$bad" >&2
  exit 2
fi

git diff --check
git diff --cached --check
