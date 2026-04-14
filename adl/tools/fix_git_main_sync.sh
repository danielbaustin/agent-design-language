#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "fix-git: not inside a git checkout" >&2
  exit 1
}

git -C "$repo_root" switch main
git -C "$repo_root" fetch origin main
git -C "$repo_root" merge --ff-only origin/main
