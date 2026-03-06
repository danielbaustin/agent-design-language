#!/usr/bin/env bash
set -euo pipefail

# Deterministic coverage gate enforcement for CI/local usage.
# Expected cwd: swarm/

SUMMARY_JSON="${1:-coverage-summary.json}"
WORKSPACE_THRESHOLD="${WORKSPACE_LINE_THRESHOLD:-90}"
FILE_THRESHOLD="${PER_FILE_LINE_THRESHOLD:-80}"
EXCLUDE_REGEX="${EXCLUDE_FROM_FILE_FLOOR_REGEX:-/swarm/src/bin/swarm.rs$|/swarm/src/bin/swarm_remote.rs$}"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for coverage gate enforcement."
  exit 1
fi

if [ ! -s "$SUMMARY_JSON" ]; then
  echo "Coverage summary json not found or empty: $SUMMARY_JSON"
  exit 1
fi

total_row="$(jq -r '
  [.data[].files[]
    | {covered:(.summary.lines.covered // 0), count:(.summary.lines.count // 0)}
  ]
  | reduce .[] as $x ({covered:0, count:0}; .covered += $x.covered | .count += $x.count)
  | .percent = (if .count == 0 then 0 else (.covered * 100.0 / .count) end)
  | [.covered, .count, .percent]
  | @tsv
' "$SUMMARY_JSON")"

workspace_pct="$(echo "$total_row" | awk -F '\t' '{printf "%.2f", $3 + 0}')"
printf "Workspace line coverage: %s%% (threshold: %s%%)\n" "$workspace_pct" "$WORKSPACE_THRESHOLD"

awk -v pct="$workspace_pct" -v threshold="$WORKSPACE_THRESHOLD" '
  BEGIN {
    if ((pct + 0) < threshold) {
      printf "Coverage below workspace threshold: %.2f%% < %s%%\n", pct, threshold
      exit 1
    }
  }'

per_file_rows="$(jq -r '
  [.data[].files[]
    | select(.filename | contains("/swarm/src/"))
    | {
        f:(
          .filename
          | gsub("\\\\"; "/")
          | sub("^/home/runner/work/[^/]+/[^/]+/"; "")
          | sub("^/__w/[^/]+/[^/]+/"; "")
          | sub("^[A-Za-z]:/a/[^/]+/[^/]+/"; "")
        ),
        covered:(.summary.lines.covered // 0),
        count:(.summary.lines.count // 0)
      }
  ]
  | sort_by(.f)
  | group_by(.f)[]
  | {
      f: .[0].f,
      covered: (map(.covered) | add),
      count: (map(.count) | add),
      percent: (if (map(.count) | add) == 0 then 0 else ((map(.covered) | add) * 100.0 / (map(.count) | add)) end)
    }
  | [.f, .covered, .count, .percent]
  | @tsv
' "$SUMMARY_JSON")"

if [ -z "$per_file_rows" ]; then
  echo "No per-file rows found for /swarm/src in $SUMMARY_JSON"
  exit 1
fi

below="$(echo "$per_file_rows" | awk -F '\t' -v threshold="$FILE_THRESHOLD" -v exclude_re="$EXCLUDE_REGEX" '
  ($1 ~ exclude_re) { next }
  ($4 + 0) < threshold {
    printf "%s\t%d/%d\t%.2f%%\n", $1, $2, $3, $4
  }
')"

if [ -n "$below" ]; then
  echo "Per-file line coverage below ${FILE_THRESHOLD}%:"
  echo "$below" | awk -F '\t' '{printf "  - %s (%s)\n", $1, $3}'
  echo "Exclusions regex: $EXCLUDE_REGEX"
  exit 1
fi

echo "Per-file line coverage gate passed (>= ${FILE_THRESHOLD}% after documented exclusions)."
