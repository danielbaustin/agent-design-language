#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

REPO="$TMPDIR_ROOT/repo"
mkdir -p "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback"
mkdir -p "$REPO/.worktrees/adl-wp-1479/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback"
mkdir -p "$REPO/adl/tools"
cp "$ROOT_DIR/adl/tools/normalize_adl_cards.sh" "$REPO/adl/tools/normalize_adl_cards.sh"
chmod +x "$REPO/adl/tools/normalize_adl_cards.sh"

cat > "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/stp.md" <<'EOF'
---
issue_card_schema: adl.issue.v1
slug: duplicate-fallback
title: '[v0.87.1][tools] Duplicate fallback issue'
labels:
- track:roadmap
- type:task
- area:tools
- version:v0.87.1
issue_number: 1479
status: draft
action: edit
depends_on: []
milestone_sprint: Sprint 2
required_outcome_type:
- docs
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes: []
pr_start:
  enabled: false
  slug: duplicate-fallback
---

## Summary

Duplicate fallback issue.
EOF

cat > "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sip.md" <<'EOF'
# ADL Input Card

Task ID: issue-1479
Run ID: issue-1479
Version: v0.87.1
Title: [v0.87.1][tools] Duplicate fallback issue
Branch: not bound yet
EOF

cp "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/stp.md" \
  "$REPO/.worktrees/adl-wp-1479/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/stp.md"
cp "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sip.md" \
  "$REPO/.worktrees/adl-wp-1479/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sip.md"

mkdir -p "$REPO/.adl/cards/1479"
ln -s ../v0.87.1/tasks/issue-1479__duplicate-fallback/sip.md "$REPO/.adl/cards/1479/input_1479.md"

(
  cd "$REPO"
  bash adl/tools/normalize_adl_cards.sh --root "$REPO" >/dev/null
)

[[ -f "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sor.md" ]] || {
  echo "assertion failed: root sor.md was not materialized" >&2
  exit 1
}
[[ -f "$REPO/.adl/cards/1479/input_1479.md" ]] || {
  echo "assertion failed: root input link does not resolve" >&2
  exit 1
}
[[ -f "$REPO/.adl/cards/1479/stp_1479.md" ]] || {
  echo "assertion failed: root stp link does not resolve" >&2
  exit 1
}
[[ -f "$REPO/.adl/cards/1479/output_1479.md" ]] || {
  echo "assertion failed: root output link does not resolve" >&2
  exit 1
}
[[ -f "$REPO/.worktrees/adl-wp-1479/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sor.md" ]] || {
  echo "assertion failed: worktree sor.md was not materialized" >&2
  exit 1
}
[[ -f "$REPO/.worktrees/adl-wp-1479/.adl/cards/1479/input_1479.md" ]] || {
  echo "assertion failed: worktree input link does not resolve" >&2
  exit 1
}
[[ -f "$REPO/.worktrees/adl-wp-1479/.adl/cards/1479/output_1479.md" ]] || {
  echo "assertion failed: worktree output link does not resolve" >&2
  exit 1
}

grep -Fq "Materialized the canonical bootstrap output card" "$REPO/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sor.md" || {
  echo "assertion failed: materialized root sor did not contain bootstrap summary" >&2
  exit 1
}
grep -Fq "Branch: not bound yet" "$REPO/.worktrees/adl-wp-1479/.adl/v0.87.1/tasks/issue-1479__duplicate-fallback/sor.md" || {
  echo "assertion failed: worktree sor did not preserve branch state" >&2
  exit 1
}

echo "normalize_adl_cards: ok"
