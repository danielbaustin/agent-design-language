#!/usr/bin/env bash
set -euo pipefail

base="${1:?exact base revision required}"
head="${2:?exact substantive repair revision required}"
inventory="${3:?expected changed-path inventory required}"

git cat-file -e "${base}^{commit}"
git cat-file -e "${head}^{commit}"
git merge-base --is-ancestor "$base" "$head"

actual="$(mktemp)"
expected="$(mktemp)"
trap 'rm -f "$actual" "$expected"' EXIT

git diff --name-only "${base}..${head}" | LC_ALL=C sort -u >"$actual"
LC_ALL=C sort -u "$inventory" >"$expected"

path_count="$(wc -l <"$actual" | tr -d ' ')"
if [[ "$path_count" -eq 0 ]]; then
  echo "revision range is empty: ${base}..${head}" >&2
  exit 65
fi

if ! cmp -s "$expected" "$actual"; then
  echo "changed-path inventory does not match ${base}..${head}" >&2
  diff -u "$expected" "$actual" >&2 || true
  exit 66
fi

while IFS= read -r path; do
  case "$path" in
    .csdlc/evidence/5590/*|.csdlc/issues/5590/*|.csdlc/locks/5590.lock|.csdlc/prepared/issues/5590/*) ;;
    *)
      echo "path escapes #5590 preparation scope: $path" >&2
      exit 67
      ;;
  esac
done <"$actual"

git diff --check "${base}..${head}"
echo "revision_scope_proof base=${base} substantive_head=${head} changed_paths=${path_count} inventory=exact diff_check=passed scope=preparation-only"
