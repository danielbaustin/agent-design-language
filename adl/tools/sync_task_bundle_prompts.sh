#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: adl/tools/sync_task_bundle_prompts.sh [--root <repo-root>] [--scope <scope>]

Synchronize local prompt artifacts from the legacy draft layout into the canonical
task-bundle layout under:

  .adl/<scope>/tasks/<task-id>__<slug>/

For GitHub issue-backed tasks, the current task-id format is:

  issue-<zero-padded-issue-number>

The sync is additive and non-destructive:
- stp.md is copied from .adl/issues/<scope>/bodies/issue-<n>-<slug>.md
- stp.stub.md is copied from the matching -stub file when present
- sip.md and sor.md are copied from .adl/cards/<n>/ when present

Legacy paths remain in place as a compatibility layer until workflow tooling is updated.
EOF
}

repo_root() {
  if [[ -n "${ROOT:-}" ]]; then
    printf '%s\n' "$ROOT"
    return 0
  fi
  git rev-parse --show-toplevel
}

issue_pad() {
  local raw="$1"
  if [[ ! "$raw" =~ ^[0-9]+$ ]]; then
    echo "invalid issue number: $raw" >&2
    return 1
  fi
  printf '%04d\n' "$((10#$raw))"
}

copy_if_exists() {
  local src="$1"
  local dest="$2"
  if [[ -f "$src" ]]; then
    cp -f "$src" "$dest"
  fi
}

write_readme() {
  local readme="$1"
  cat >"$readme" <<'EOF'
# Local Task Bundles

This directory is the canonical local draft home for prompt task bundles.

Each GitHub issue-backed task bundle currently lives at:

- .adl/<scope>/tasks/issue-<zero-padded-issue-number>__<slug>/

With canonical files:

- stp.stub.md
- stp.md
- sip.md
- sor.md

The older draft paths remain as a compatibility layer for current workflow tooling:

- .adl/issues/<scope>/bodies/
- .adl/cards/<issue>/

Use adl/tools/sync_task_bundle_prompts.sh to refresh this canonical local view
from the compatibility layout until the workflow tooling writes task bundles directly.
EOF
}

ROOT=""
SCOPE="v0.85"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      ROOT="$2"
      shift 2
      ;;
    --scope)
      SCOPE="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

ROOT="$(repo_root)"
BODIES_DIR="$ROOT/.adl/issues/$SCOPE/bodies"
CARDS_DIR="$ROOT/.adl/cards"
TASKS_DIR="$ROOT/.adl/$SCOPE/tasks"

mkdir -p "$TASKS_DIR"
write_readme "$TASKS_DIR/README.md"

shopt -s nullglob
for stp in "$BODIES_DIR"/issue-*.md; do
  base="$(basename "$stp")"
  if [[ "$base" == *-stub.md ]]; then
    continue
  fi

  if [[ ! "$base" =~ ^issue-([0-9]+)-(.+)\.md$ ]]; then
    continue
  fi

  issue="${BASH_REMATCH[1]}"
  slug="${BASH_REMATCH[2]}"
  issue_padded="$(issue_pad "$issue")"
  task_id="issue-$issue_padded"
  bundle_dir="$TASKS_DIR/${task_id}__${slug}"

  mkdir -p "$bundle_dir"
  cp -f "$stp" "$bundle_dir/stp.md"

  stub="${stp%.md}-stub.md"
  copy_if_exists "$stub" "$bundle_dir/stp.stub.md"
  copy_if_exists "$CARDS_DIR/$issue/input_$issue.md" "$bundle_dir/sip.md"
  copy_if_exists "$CARDS_DIR/$issue/output_$issue.md" "$bundle_dir/sor.md"
done
shopt -u nullglob

echo "Synchronized local task bundles under: $TASKS_DIR"
