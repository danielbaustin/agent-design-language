#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WITH_PR_HYGIENE="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-pr-hygiene) WITH_PR_HYGIENE="1"; shift ;;
    -h|--help)
      cat <<'EOF'
Usage:
  swarm/tools/preflight_review.sh [--with-pr-hygiene]

Runs one-command preflight checks:
- tooling sanity + fmt + clippy + tests (via batched_checks.sh)
- schema-focused tests
- demo/example-focused tests

Optional:
- PR hygiene checks for branch naming and issue linkage
EOF
      exit 0
      ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

echo "• Running batched checks…"
bash "$ROOT/swarm/tools/batched_checks.sh"

echo "• Running schema/example focused checks…"
(
  cd "$ROOT/swarm"
  cargo test --test schema_tests
  cargo test --test demo_tests
)

if [[ "$WITH_PR_HYGIENE" == "1" ]]; then
  echo "• Running optional PR hygiene checks…"
  (
    cd "$ROOT"
    branch="$(git rev-parse --abbrev-ref HEAD)"
    if [[ ! "$branch" =~ ^codex/[0-9]+- ]]; then
      echo "Branch naming check failed: expected codex/<issue>-<slug>, got '$branch'" >&2
      exit 1
    fi
    if ! git log -1 --pretty=%B | rg -n "Closes #[0-9]+" >/dev/null 2>&1; then
      echo "Issue linkage check failed: latest commit message missing 'Closes #<n>'" >&2
      exit 1
    fi
  )
fi

echo "preflight review: OK"
