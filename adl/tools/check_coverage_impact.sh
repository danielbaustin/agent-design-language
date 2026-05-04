#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
BASE="origin/main"
HEAD="HEAD"
INCLUDE_WORKTREE=false
SUMMARY=""
CHANGED_FILES_FILE=""
THRESHOLD="${PER_FILE_LINE_THRESHOLD:-80}"
LARGE_FILE_LINES="${COVERAGE_IMPACT_LARGE_FILE_LINES:-200}"
LARGE_FILE_DELTA="${COVERAGE_IMPACT_LARGE_FILE_DELTA:-80}"
REQUIRE_SUMMARY_FOR_RISK=false
PRINT_RISK_FILTERS=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/check_coverage_impact.sh [options]

Options:
  --base <rev>                    Base revision for changed-file detection.
  --head <rev>                    Head revision for changed-file detection.
  --include-working-tree          Compare base against the current working tree.
  --summary <coverage-summary>    cargo llvm-cov JSON summary to check.
  --changed-files <file>          Explicit changed-file list for tests. Lines may be
                                  "path" or "STATUS<TAB>path".
  --threshold <percent>           Per-file line threshold. Defaults to PER_FILE_LINE_THRESHOLD or 80.
  --require-summary-for-risk      Fail when risky changed Rust files lack summary evidence.
  --print-risk-filters            Print one candidate test filter per risky changed Rust file and exit.
  -h, --help                      Show this help.

This is a fast authoring-time guard. It does not replace the full GitHub
adl-coverage job; it catches likely per-file coverage failures before PR
publication when Rust source files are added or heavily changed.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --base)
      BASE="${2:-}"
      shift 2
      ;;
    --head)
      HEAD="${2:-}"
      shift 2
      ;;
    --include-working-tree)
      INCLUDE_WORKTREE=true
      shift
      ;;
    --summary)
      SUMMARY="${2:-}"
      shift 2
      ;;
    --changed-files)
      CHANGED_FILES_FILE="${2:-}"
      shift 2
      ;;
    --threshold)
      THRESHOLD="${2:-}"
      shift 2
      ;;
    --require-summary-for-risk)
      REQUIRE_SUMMARY_FOR_RISK=true
      shift
      ;;
    --print-risk-filters)
      PRINT_RISK_FILTERS=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$BASE" ]; then
  echo "coverage-impact: --base cannot be empty" >&2
  exit 2
fi

changed_rows() {
  if [ -n "$CHANGED_FILES_FILE" ]; then
    cat "$CHANGED_FILES_FILE"
    return
  fi
  if [ "$INCLUDE_WORKTREE" = true ]; then
    git -C "$ROOT" diff --name-status --diff-filter=ACMR "$BASE" -- 2>/dev/null || true
    return
  fi
  git -C "$ROOT" diff --name-status --diff-filter=ACMR "$BASE...$HEAD" -- 2>/dev/null || \
    git -C "$ROOT" diff --name-status --diff-filter=ACMR "$BASE" "$HEAD" -- 2>/dev/null || true
}

normalize_changed_path() {
  awk -F '\t' '
    NF == 1 { print "M\t" $1; next }
    $1 ~ /^R/ && NF >= 3 { print $1 "\t" $3; next }
    NF >= 2 { print $1 "\t" $2; next }
  '
}

changed_source_rows="$(
  changed_rows \
    | normalize_changed_path \
    | awk -F '\t' '
        $2 ~ /^adl\/src\/.*\.rs$/ &&
        $2 !~ /^adl\/src\/.*_tests\.rs$/ &&
        $2 !~ /^adl\/src\/(.+\/)?tests\.rs$/ &&
        $2 !~ /^adl\/src\/.*\/tests\/.*\.rs$/ {
          print $1 "\t" $2
        }
      '
)"

if [ -z "$changed_source_rows" ]; then
  if [ "$PRINT_RISK_FILTERS" = true ]; then
    exit 0
  fi
  echo "Coverage-impact preflight: no changed production adl/src Rust files."
  exit 0
fi

line_count_for_path() {
  local path="$1"
  if [ -f "$ROOT/$path" ]; then
    wc -l <"$ROOT/$path" | tr -d ' '
  else
    echo 0
  fi
}

changed_line_delta_for_path() {
  local path="$1"
  if [ -n "$CHANGED_FILES_FILE" ]; then
    echo 0
    return
  fi
  local out
  if [ "$INCLUDE_WORKTREE" = true ]; then
    out="$(git -C "$ROOT" diff --numstat "$BASE" -- "$path" 2>/dev/null || true)"
  else
    out="$(git -C "$ROOT" diff --numstat "$BASE...$HEAD" -- "$path" 2>/dev/null || true)"
  fi
  if [ -z "$out" ]; then
    echo 0
    return
  fi
  echo "$out" | awk '($1 ~ /^[0-9]+$/ && $2 ~ /^[0-9]+$/) { total += $1 + $2 } END { print total + 0 }'
}

candidate_filter_for_path() {
  local path="$1"
  basename "$path" .rs
}

file_is_structural_module_barrel() {
  local path="$1"
  [ -f "$ROOT/$path" ] || return 1
  case "$(basename "$path")" in
    mod.rs|lib.rs) ;;
    *) return 1 ;;
  esac

  awk '
    /^[[:space:]]*$/ { next }
    /^[[:space:]]*\/\// { next }
    /^[[:space:]]*#\[/ { next }
    /^[[:space:]]*(pub[[:space:]]+)?mod[[:space:]]+[A-Za-z_][A-Za-z0-9_]*;[[:space:]]*$/ { next }
    /^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?use[[:space:]].*;[[:space:]]*$/ { next }
    { exit 1 }
  ' "$ROOT/$path"
}

risk_rows=""
while IFS=$'\t' read -r status path; do
  [ -n "$path" ] || continue
  lines="$(line_count_for_path "$path")"
  delta="$(changed_line_delta_for_path "$path")"
  reason=""
  case "$status" in
    A*) reason="new Rust source file" ;;
  esac
  if [ -z "$reason" ] && [ "$lines" -ge "$LARGE_FILE_LINES" ] && [ "$delta" -ge "$LARGE_FILE_DELTA" ]; then
    reason="large Rust source file with ${delta} changed lines"
  fi
  if [ -n "$reason" ]; then
    risk_rows="${risk_rows}${status}"$'\t'"${path}"$'\t'"${lines}"$'\t'"${delta}"$'\t'"${reason}"$'\n'
  fi
done <<EOF
$changed_source_rows
EOF

if [ -n "$SUMMARY" ] && [ -s "$SUMMARY" ]; then
  if ! command -v jq >/dev/null 2>&1; then
    echo "jq is required when --summary is supplied." >&2
    exit 1
  fi
  failures=""
  missing=""
  while IFS=$'\t' read -r _status path; do
    [ -n "$path" ] || continue
    row="$(jq -r --arg path "$path" '
      [
        .data[].files[]
        | {
            filename: (
              .filename
              | gsub("\\\\"; "/")
              | sub("^/home/runner/work/[^/]+/[^/]+/"; "")
              | sub("^/__w/[^/]+/[^/]+/"; "")
              | sub("^[A-Za-z]:/a/[^/]+/[^/]+/"; "")
            ),
            covered: (.summary.lines.covered // 0),
            count: (.summary.lines.count // 0)
          }
        | select(.filename == $path or .filename == ("/" + $path) or (.filename | endswith("/" + $path)))
      ]
      | if length == 0 then empty else
          {
            covered: (map(.covered) | add),
            count: (map(.count) | add)
          }
          | .percent = (if .count == 0 then 0 else (.covered * 100.0 / .count) end)
          | [.covered, .count, .percent]
          | @tsv
        end
    ' "$SUMMARY")"
    if [ -z "$row" ]; then
      if file_is_structural_module_barrel "$path"; then
        continue
      fi
      missing="${missing}  - ${path} (no coverage row in ${SUMMARY})"$'\n'
      continue
    fi
    pct="$(printf '%s\n' "$row" | awk -F '\t' '{ printf "%.2f", $3 + 0 }')"
    covered_count="$(printf '%s\n' "$row" | awk -F '\t' '{ printf "%s/%s", $1, $2 }')"
    if ! awk -v pct="$pct" -v threshold="$THRESHOLD" 'BEGIN { exit ((pct + 0) < (threshold + 0)) ? 0 : 1 }'; then
      continue
    fi
    failures="${failures}  - ${path} (${covered_count}, ${pct}% < ${THRESHOLD}%)"$'\n'
  done <<EOF
$changed_source_rows
EOF

  if [ -n "$missing" ] || [ -n "$failures" ]; then
    echo "Coverage-impact preflight failed for changed Rust source files:"
    [ -z "$missing" ] || printf '%s' "$missing"
    [ -z "$failures" ] || printf '%s' "$failures"
    echo "Full adl-coverage remains authoritative; fix or add focused tests before publication."
    exit 1
  fi
  echo "Coverage-impact preflight passed for changed Rust source files using ${SUMMARY}."
  exit 0
fi

if [ "$PRINT_RISK_FILTERS" = true ]; then
  if [ -z "$risk_rows" ]; then
    exit 0
  fi
  printf '%s' "$risk_rows" \
    | while IFS=$'\t' read -r _status path _lines _delta _reason; do
        [ -n "$path" ] || continue
        candidate_filter_for_path "$path"
      done \
    | awk '!seen[$0]++'
  exit 0
fi

if [ "$REQUIRE_SUMMARY_FOR_RISK" = true ] && [ -n "$risk_rows" ]; then
  echo "Coverage-impact preflight needs coverage evidence for risky changed Rust source files:"
  while IFS=$'\t' read -r _status path lines delta reason; do
    [ -n "$path" ] || continue
    filter="$(candidate_filter_for_path "$path")"
    echo "  - ${path} (${reason}; ${lines} lines, ${delta} changed)"
    echo "    next action: cd adl && CARGO_INCREMENTAL=0 cargo llvm-cov --workspace --all-features --json --summary-only --output-path target/coverage-impact-summary.json -- ${filter}"
  done <<EOF
$risk_rows
EOF
  echo "Then rerun: bash adl/tools/check_coverage_impact.sh --base ${BASE} --include-working-tree --summary adl/target/coverage-impact-summary.json --require-summary-for-risk"
  exit 1
fi

echo "Coverage-impact preflight passed: no risky changed Rust source files require local summary evidence."
