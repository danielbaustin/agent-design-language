#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
candidate="${ADL_CARGO_BUILD_ROOT:-}"

if [[ -z "$candidate" ]]; then
  candidate="${ADL_FASTWORK_ROOT:-/Volumes/FastWork}"
fi

if [[ ! -d "$candidate" || ! -w "$candidate" ]]; then
  echo "Cargo validation requires a writable external build root: $candidate" >&2
  exit 2
fi

build_root="$(cd "$candidate" && pwd -P)"
case "$build_root/" in
  "$ROOT_DIR/"|"$ROOT_DIR/"*)
    echo "Cargo validation build root must be outside the repository: $build_root" >&2
    exit 2
    ;;
esac

export CARGO_HOME="$build_root/cargo-home"
export CARGO_TARGET_DIR="$build_root/cargo-target"
export GIT_AUTHOR_NAME="${GIT_AUTHOR_NAME:-ADL Validation}"
export GIT_AUTHOR_EMAIL="${GIT_AUTHOR_EMAIL:-validation@agent-logic.invalid}"
export GIT_COMMITTER_NAME="${GIT_COMMITTER_NAME:-$GIT_AUTHOR_NAME}"
export GIT_COMMITTER_EMAIL="${GIT_COMMITTER_EMAIL:-$GIT_AUTHOR_EMAIL}"
for cargo_path in "$CARGO_HOME" "$CARGO_TARGET_DIR"; do
  if [[ -L "$cargo_path" ]]; then
    echo "Cargo validation child path must not be a symlink: $cargo_path" >&2
    exit 2
  fi
done
mkdir -p "$CARGO_HOME" "$CARGO_TARGET_DIR"
for cargo_path in "$CARGO_HOME" "$CARGO_TARGET_DIR"; do
  canonical_cargo_path="$(cd "$cargo_path" && pwd -P)"
  if [[ "$canonical_cargo_path" != "$cargo_path" ]]; then
    echo "Cargo validation child path escaped the selected build root: $cargo_path" >&2
    exit 2
  fi
done

if [[ $# -eq 0 ]]; then
  echo "usage: run_cargo_validation.sh <command> [args...]" >&2
  exit 2
fi

exec "$@"
