#!/usr/bin/env bash
set -euo pipefail

REMOTE_REF="${1:-}"
REQUESTED_REF="${2:-}"

if [[ -z "$REMOTE_REF" || -z "$REQUESTED_REF" ]]; then
  echo "verify_spot_advertised_ref: usage: <remote-branch> <requested-commit>" >&2
  exit 2
fi

git check-ref-format --branch "$REMOTE_REF" >/dev/null
ADVERTISED_COMMIT="$(git rev-parse --verify "refs/remotes/origin/${REMOTE_REF}^{commit}")"
REQUESTED_COMMIT="$(git rev-parse --verify "${REQUESTED_REF}^{commit}")"

if [[ "$ADVERTISED_COMMIT" != "$REQUESTED_COMMIT" ]]; then
  echo "verify_spot_advertised_ref: requested commit is not the advertised branch tip" >&2
  exit 1
fi

printf 'PASS advertised_ref_commit_bound branch=%s\n' "$REMOTE_REF"
