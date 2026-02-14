#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  swarm/tools/burst_worktree.sh init <issue_num> --slug <slug> [--root .worktrees/burst]
  swarm/tools/burst_worktree.sh drop <issue_num> --slug <slug> [--root .worktrees/burst]

Notes:
- Branch format is deterministic: codex/<issue_num>-<slug>
- Worktree path format is deterministic: <root>/<issue_num>-<slug>
EOF
}

[[ $# -ge 2 ]] || { usage; exit 2; }
cmd="$1"
issue="$2"
shift 2

slug=""
root=".worktrees/burst"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --slug) slug="$2"; shift 2 ;;
    --root) root="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

[[ "$issue" =~ ^[0-9]+$ ]] || { echo "invalid issue: $issue" >&2; exit 1; }
[[ -n "$slug" ]] || { echo "--slug is required" >&2; exit 1; }

branch="codex/${issue}-${slug}"
path="${root}/${issue}-${slug}"

case "$cmd" in
  init)
    mkdir -p "$root"
    if [[ -d "$path" ]]; then
      echo "EXISTS=$path"
      exit 0
    fi
    git worktree add "$path" "$branch"
    echo "CREATED=$path"
    ;;
  drop)
    if [[ ! -d "$path" ]]; then
      echo "MISSING=$path"
      exit 0
    fi
    git worktree remove "$path"
    echo "REMOVED=$path"
    ;;
  *)
    usage
    exit 2
    ;;
esac
