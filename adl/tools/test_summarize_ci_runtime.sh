#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/summarize_ci_runtime.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_has() {
  local haystack="$1"
  local needle="$2"
  if ! grep -Fq "$needle" <<<"$haystack"; then
    echo "expected output to contain: $needle" >&2
    echo "actual output:" >&2
    echo "$haystack" >&2
    exit 1
  fi
}

fixture="$TMP/jobs.json"
cat >"$fixture" <<'EOF'
{
  "jobs": [
    {
      "name": "adl-ci",
      "startedAt": "2026-05-20T16:00:00Z",
      "completedAt": "2026-05-20T16:08:00Z",
      "conclusion": "success",
      "steps": [
        {
          "name": "Classify changed paths",
          "startedAt": "2026-05-20T16:00:05Z",
          "completedAt": "2026-05-20T16:00:15Z",
          "conclusion": "success"
        },
        {
          "name": "Install Rust toolchain",
          "startedAt": "2026-05-20T16:00:15Z",
          "completedAt": "2026-05-20T16:02:15Z",
          "conclusion": "success"
        },
        {
          "name": "test",
          "startedAt": "2026-05-20T16:02:15Z",
          "completedAt": "2026-05-20T16:07:45Z",
          "conclusion": "success"
        },
        {
          "name": "Rust validation skipped by path policy",
          "startedAt": "2026-05-20T16:07:45Z",
          "completedAt": "2026-05-20T16:07:50Z",
          "conclusion": "skipped"
        }
      ]
    },
    {
      "name": "adl-coverage",
      "startedAt": "2026-05-20T16:00:00Z",
      "completedAt": "2026-05-20T16:16:45Z",
      "conclusion": "success",
      "steps": [
        {
          "name": "Determine PR fast coverage filters",
          "startedAt": "2026-05-20T16:00:05Z",
          "completedAt": "2026-05-20T16:00:25Z",
          "conclusion": "success"
        },
        {
          "name": "Coverage run and summary (json)",
          "startedAt": "2026-05-20T16:00:25Z",
          "completedAt": "2026-05-20T16:16:25Z",
          "conclusion": "success"
        },
        {
          "name": "Upload coverage artifact",
          "startedAt": "2026-05-20T16:16:25Z",
          "completedAt": "2026-05-20T16:16:40Z",
          "conclusion": "success"
        },
        {
          "name": "Coverage skipped by path policy",
          "startedAt": "2026-05-20T16:16:40Z",
          "completedAt": "2026-05-20T16:16:45Z",
          "conclusion": "success"
        }
      ]
    }
  ]
}
EOF

markdown_output="$(python3 "$SCRIPT" "$fixture" --job-budget adl-coverage=900 --category-budget coverage-execution=900)"
assert_has "$markdown_output" "# CI Runtime Budget Report"
assert_has "$markdown_output" '| `adl-coverage` | 1005.0 | 900.0 | over_budget | `coverage-execution` | `success` |'
assert_has "$markdown_output" '| `rust-test-execution` | 330.0 |'
assert_has "$markdown_output" '| `skipped-policy` | 5.0 |'
assert_has "$markdown_output" '| `Coverage run and summary (json)` | `coverage-execution` | 960.0 | 900.0 | over_budget |'
assert_has "$markdown_output" "inspect coverage lane policy"

json_output="$(python3 "$SCRIPT" "$fixture" --format json --job-budget adl-coverage=900 --category-budget coverage-execution=900)"
assert_has "$json_output" '"name": "adl-coverage"'
assert_has "$json_output" '"over_budget": true'
assert_has "$json_output" '"category": "coverage-execution"'
assert_has "$json_output" '"category": "skipped-policy"'

echo "PASS test_summarize_ci_runtime"
