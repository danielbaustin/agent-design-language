#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/adl-archive-runs-test.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/.adl/runs/v0-3-demo/logs"
printf '{}\n' >"$TMP/.adl/runs/v0-3-demo/run_summary.json"
printf '{}\n' >"$TMP/.adl/runs/v0-3-demo/logs/trace_v1.json"

mkdir -p "$TMP/.adl/runs/retry-success"
printf '{}\n' >"$TMP/.adl/runs/retry-success/run.json"

mkdir -p "$TMP/.adl/reports/demo-example/runs/review-godel-demo"
printf '{}\n' >"$TMP/.adl/reports/demo-example/runs/review-godel-demo/run_status.json"

mkdir -p "$TMP/artifacts/v0871/provider_mock/runtime/runs/v0-87-1-provider-mock-demo/logs"
printf '{}\n' >"$TMP/artifacts/v0871/provider_mock/runtime/runs/v0-87-1-provider-mock-demo/run_summary.json"
printf '{"schema_version":"trace_run_manifest.v1","milestone":"v0.87.1"}\n' >"$TMP/artifacts/v0871/provider_mock/runtime/runs/v0-87-1-provider-mock-demo/run_manifest.json"
printf '{}\n' >"$TMP/artifacts/v0871/provider_mock/runtime/runs/v0-87-1-provider-mock-demo/logs/trace_v1.json"

"$ROOT/adl/tools/archive_run_artifacts.sh" --repo-root "$TMP" >/tmp/adl-archive-dry-run.out

[[ -f "$TMP/.adl/trace-archive/MANIFEST.tsv" ]]
[[ ! -d "$TMP/.adl/trace-archive/milestones/v0.3/runs/v0-3-demo" ]]
grep -q $'v0-3-demo\tv0.3' "$TMP/.adl/trace-archive/MANIFEST.tsv"
grep -q $'retry-success\tunclassified' "$TMP/.adl/trace-archive/MANIFEST.tsv"
grep -q $'v0-87-1-provider-mock-demo\tv0.87.1' "$TMP/.adl/trace-archive/MANIFEST.tsv"
grep -q $'v0-87-1-provider-mock-demo\tv0.87.1\t.adl/trace-archive/milestones/v0.87.1/runs/v0-87-1-provider-mock-demo\twould-copy\t3\tyes\tyes\tyes' "$TMP/.adl/trace-archive/MANIFEST.tsv"

"$ROOT/adl/tools/archive_run_artifacts.sh" --repo-root "$TMP" --apply --prune-active-runs >/tmp/adl-archive-apply.out

[[ -f "$TMP/.adl/trace-archive/milestones/v0.3/runs/v0-3-demo/run_summary.json" ]]
[[ -f "$TMP/.adl/trace-archive/milestones/unclassified/runs/retry-success/run.json" ]]
[[ -f "$TMP/.adl/trace-archive/milestones/unclassified/runs/review-godel-demo/run_status.json" ]]
[[ -f "$TMP/.adl/trace-archive/milestones/v0.87.1/runs/v0-87-1-provider-mock-demo/run_summary.json" ]]
[[ -f "$TMP/.adl/trace-archive/milestones/v0.87.1/runs/v0-87-1-provider-mock-demo/ARCHIVE_SOURCE.txt" ]]
[[ -f "$TMP/.adl/runs/README.md" ]]
[[ ! -d "$TMP/.adl/runs/v0-3-demo" ]]
[[ ! -d "$TMP/.adl/runs/retry-success" ]]
find "$TMP/.adl/trace-archive/source-roots" -path '*/repo-adl-runs-flat/v0-3-demo/run_summary.json' -type f | grep -q .
find "$TMP/.adl/trace-archive/source-roots" -path '*/repo-adl-runs-flat/retry-success/run.json' -type f | grep -q .
grep -q 'Active .adl/runs entries moved to source-roots: `2`' "$TMP/.adl/trace-archive/README.md"
"$ROOT/adl/tools/archive_run_artifacts.sh" --repo-root "$TMP" --prune-active-runs >/tmp/adl-archive-prune-without-apply.out 2>&1 && {
  echo "expected --prune-active-runs without --apply to fail" >&2
  exit 1
}
grep -q -- '--prune-active-runs requires --apply' /tmp/adl-archive-prune-without-apply.out

echo "PASS archive_run_artifacts"
