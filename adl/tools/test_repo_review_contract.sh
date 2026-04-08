#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT"
VERIFY="$ROOT/adl/tools/verify_repo_review_contract.sh"
GOOD="$ROOT/docs/tooling/examples/repo-review/good_repo_review.md"
BAD="$ROOT/docs/tooling/examples/repo-review/bad_repo_review.md"

"$VERIFY" --review "$GOOD" >/dev/null

if "$VERIFY" --review "$BAD" >/dev/null 2>&1; then
  echo "assertion failed: invalid repo review unexpectedly passed" >&2
  exit 1
fi

echo "repo review contract fixtures passed"
