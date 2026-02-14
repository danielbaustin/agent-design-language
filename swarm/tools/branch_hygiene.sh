#!/usr/bin/env bash
set -eo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APPLY="0"
INCLUDE_STALE="0"
REMOTE_MERGED="0"

usage() {
  cat <<'EOF'
Usage:
  swarm/tools/branch_hygiene.sh [--apply] [--include-stale] [--remote-merged]

Safe defaults:
- Dry-run only (no deletes) unless --apply is set.
- Local merged branches are candidates by default.
- Stale local branches ([gone] upstream) are only deleted with --include-stale.
- Remote branch deletion is opt-in via --remote-merged.

Examples:
  swarm/tools/branch_hygiene.sh
  swarm/tools/branch_hygiene.sh --apply
  swarm/tools/branch_hygiene.sh --apply --include-stale
  swarm/tools/branch_hygiene.sh --apply --remote-merged
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --apply) APPLY="1"; shift ;;
    --include-stale) INCLUDE_STALE="1"; shift ;;
    --remote-merged) REMOTE_MERGED="1"; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

cd "$ROOT"
git fetch --prune origin main >/dev/null

current_branch="$(git rev-parse --abbrev-ref HEAD)"

all_local=()
while IFS= read -r b; do
  all_local+=("$b")
done < <(git for-each-ref refs/heads --format='%(refname:short)')
local_merged=()
local_stale=()

for b in "${all_local[@]}"; do
  [[ "$b" == "main" || "$b" == "master" || "$b" == "$current_branch" ]] && continue
  if git merge-base --is-ancestor "$b" origin/main; then
    local_merged+=("$b")
  fi
done

while IFS= read -r line; do
  b="${line%% *}"
  track="${line#* }"
  [[ "$b" == "main" || "$b" == "master" || "$b" == "$current_branch" ]] && continue
  if [[ "$track" == "[gone]" ]]; then
    local_stale+=("$b")
  fi
done < <(git for-each-ref refs/heads --format='%(refname:short) %(upstream:trackshort)')

remote_merged=()
if [[ "$REMOTE_MERGED" == "1" ]]; then
  while IFS= read -r rb; do
    [[ "$rb" == "origin/HEAD" || "$rb" == "origin/main" || "$rb" == "origin/master" ]] && continue
    short="${rb#origin/}"
    [[ "$short" == codex/* ]] || continue
    remote_merged+=("$short")
  done < <(git branch -r --merged origin/main --format='%(refname:short)')
fi

echo "Branch hygiene candidates (base: origin/main)"
echo "Current branch preserved: $current_branch"
echo
echo "Local merged candidates: ${#local_merged[@]}"
for b in "${local_merged[@]}"; do echo "  - $b"; done
echo
echo "Local stale candidates ([gone] upstream): ${#local_stale[@]}"
for b in "${local_stale[@]}"; do echo "  - $b"; done
echo
echo "Remote merged candidates (codex/* only): ${#remote_merged[@]}"
for b in "${remote_merged[@]}"; do echo "  - origin/$b"; done

if [[ "$APPLY" != "1" ]]; then
  echo
  echo "Dry-run only. Re-run with --apply to delete local merged branches."
  echo "Use --include-stale to also delete local stale branches."
  echo "Use --remote-merged to consider origin/codex/* merged branches for deletion."
  exit 0
fi

for b in "${local_merged[@]}"; do
  git branch -d "$b"
done

if [[ "$INCLUDE_STALE" == "1" ]]; then
  for b in "${local_stale[@]}"; do
    git branch -D "$b"
  done
fi

if [[ "$REMOTE_MERGED" == "1" && "${#remote_merged[@]}" -gt 0 ]]; then
  git push origin --delete "${remote_merged[@]}"
fi

echo "Branch hygiene apply complete."
