#!/usr/bin/env bash
set -euo pipefail

die() {
  echo "fix-git: $*" >&2
  exit 1
}

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" ||
  die "not inside a git checkout"

if [[ -n "$(git -C "$repo_root" status --porcelain)" ]]; then
  die "refusing to switch with local changes in $repo_root"
fi

git -C "$repo_root" checkout main
git -C "$repo_root" fetch origin main
git -C "$repo_root" merge --ff-only origin/main
