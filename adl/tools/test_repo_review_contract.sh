#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERIFY="$ROOT/adl/tools/verify_repo_review_contract.rb"
GOOD="$ROOT/docs/tooling/examples/repo-review/good_repo_review.md"
BAD="$ROOT/docs/tooling/examples/repo-review/bad_repo_review.md"

ruby "$VERIFY" --review "$GOOD" >/dev/null

if ruby "$VERIFY" --review "$BAD" >/dev/null 2>&1; then
  echo "assertion failed: invalid repo review unexpectedly passed" >&2
  exit 1
fi

echo "repo review contract fixtures passed"
