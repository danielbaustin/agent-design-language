#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  swarm/tools/burst_continue.sh <burst_dir> --issues "193,194,195" [--write continue.sh]

Notes:
- Determines the next pending issue by checking run_log markers:
  "- ... #<issue> executed via adl_pr_cycle ..."
- Safe to rerun; output is deterministic for the same inputs.
EOF
}

[[ $# -ge 1 ]] || { usage; exit 2; }
burst_dir="$1"
shift

issues=""
write_file=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --issues) issues="$2"; shift 2 ;;
    --write) write_file="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

[[ -d "$burst_dir" ]] || { echo "missing burst dir: $burst_dir" >&2; exit 1; }
[[ -n "$issues" ]] || { echo "--issues is required" >&2; exit 1; }

run_log="${burst_dir}/run_log.md"
[[ -f "$run_log" ]] || { echo "missing run log: $run_log" >&2; exit 1; }

IFS=',' read -r -a arr <<<"$issues"
next_issue=""
for raw in "${arr[@]}"; do
  i="$(echo "$raw" | tr -d '[:space:]')"
  [[ -n "$i" ]] || continue
  if ! rg -n "#${i} executed via adl_pr_cycle" "$run_log" >/dev/null 2>&1; then
    next_issue="$i"
    break
  fi
done

if [[ -z "$next_issue" ]]; then
  cmd='echo "Burst already complete. No pending issues."'
else
  cmd="echo \"Next pending issue: ${next_issue}\""
fi

if [[ -n "$write_file" ]]; then
  out="${burst_dir}/${write_file}"
  cat >"$out" <<EOF
#!/usr/bin/env bash
set -euo pipefail
$cmd
EOF
  chmod +x "$out"
  echo "WROTE=$out"
else
  echo "$cmd"
fi
