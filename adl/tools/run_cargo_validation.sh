#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
candidate="${ADL_CARGO_BUILD_ROOT:-}"
lock_guard_dir=""
lock_guard_active=0
lock_guard_drift=0
lock_paths=()
lock_existed=()

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

if [[ "$(basename "$1")" == "cargo" && "${ADL_ALLOW_CARGO_LOCK_UPDATE:-0}" != "1" ]]; then
  locked=0
  for arg in "$@"; do
    [[ "$arg" == "--locked" ]] && locked=1
  done
  if [[ "$locked" != "1" ]]; then
    echo "Cargo validation requires --locked; set ADL_ALLOW_CARGO_LOCK_UPDATE=1 only for an explicit dependency update" >&2
    exit 2
  fi
fi

capture_lockfiles() {
  local path index=0
  lock_guard_dir="$(mktemp -d "$build_root/.adl-cargo-lock-guard.XXXXXX")"
  while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    lock_paths+=("$path")
    if [[ -f "$ROOT_DIR/$path" ]]; then
      lock_existed+=(1)
      cp -p "$ROOT_DIR/$path" "$lock_guard_dir/$index"
    else
      lock_existed+=(0)
    fi
    index=$((index + 1))
  done < <(git -C "$ROOT_DIR" ls-files -- '*Cargo.lock')
  lock_guard_active=1
}

restore_lockfile_drift() {
  local index path
  [[ "$lock_guard_active" == "1" ]] || return 0
  lock_guard_drift=0
  for index in "${!lock_paths[@]}"; do
    path="${lock_paths[$index]}"
    if [[ "${lock_existed[$index]}" == "1" ]]; then
      if [[ ! -f "$ROOT_DIR/$path" ]] || ! cmp -s "$lock_guard_dir/$index" "$ROOT_DIR/$path"; then
        mkdir -p "$(dirname "$ROOT_DIR/$path")"
        cp -p "$lock_guard_dir/$index" "$ROOT_DIR/$path"
        echo "Cargo validation restored invocation-created lockfile drift: $path" >&2
        lock_guard_drift=1
      fi
    elif [[ -e "$ROOT_DIR/$path" ]]; then
      rm -f "$ROOT_DIR/$path"
      echo "Cargo validation removed invocation-created lockfile: $path" >&2
      lock_guard_drift=1
    fi
  done
}

cleanup_lock_guard() {
  restore_lockfile_drift || true
  [[ -z "$lock_guard_dir" ]] || rm -rf "$lock_guard_dir"
  lock_guard_active=0
}

trap cleanup_lock_guard EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

capture_lockfiles
set +e
"$@"
command_status=$?
set -e
restore_lockfile_drift
if [[ "$lock_guard_drift" == "1" ]]; then
  exit 1
fi
exit "$command_status"
