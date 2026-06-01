#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT_DIR/adl/tools/run_pvf_validation_lane.sh"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
manifest="$ROOT_DIR/docs/milestones/v0.91.4/features/PVF_CI_RELEASE_POLICY_MANIFEST_v0.91.4.json"
docs_changed="$tmpdir/docs.changed"
runtime_changed="$tmpdir/runtime.changed"
release_changed="$tmpdir/release.changed"
docs_report="$tmpdir/docs.report.json"
runtime_report="$tmpdir/runtime.report.json"
release_report="$tmpdir/release.report.json"

printf 'M\tadl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md\n' > "$docs_changed"
printf 'M\tadl/src/lib.rs\n' > "$runtime_changed"
printf 'M\tdocs/milestones/v0.91.4/README.md\n' > "$release_changed"

set +e
"$RUNNER" --manifest "$manifest" --changed-files "$docs_changed" --report-out "$docs_report" >"$tmpdir/docs.stdout"
docs_status=$?
"$RUNNER" --manifest "$manifest" --changed-files "$runtime_changed" --report-out "$runtime_report" >"$tmpdir/runtime.stdout"
runtime_status=$?
set -e

if [ "$docs_status" -ne 0 ]; then
  echo "expected docs-only PR mode to exit 0 for passed aggregate, got $docs_status" >&2
  cat "$tmpdir/docs.stdout" >&2
  exit 1
fi

if [ "$runtime_status" -ne 1 ]; then
  echo "expected runtime PR mode to exit 1 for release_gate_required, got $runtime_status" >&2
  cat "$tmpdir/runtime.stdout" >&2
  exit 1
fi

set +e
"$RUNNER" --manifest "$manifest" --changed-files "$release_changed" --mode release --report-out "$release_report" >"$tmpdir/release.stdout"
release_status=$?
set -e

if [ "$release_status" -ne 0 ]; then
  echo "expected release mode to exit 0 for passed aggregate, got $release_status" >&2
  cat "$tmpdir/release.stdout" >&2
  exit 1
fi

python3 - <<'PY' "$docs_report" "$runtime_report" "$release_report"
import json
import sys
from pathlib import Path

docs = json.loads(Path(sys.argv[1]).read_text())
runtime = json.loads(Path(sys.argv[2]).read_text())
release = json.loads(Path(sys.argv[3]).read_text())

assert docs["aggregate_status"] == "passed"
assert docs["lanes"]["docs_only_pr"]["status"] == "passed"
assert docs["lanes"]["docs_only_reuse_candidate"]["status"] == "reused"
assert docs["lanes"]["runtime_pr_fast"]["status"] == "skipped"
assert docs["lanes"]["authoritative_release_gate"]["status"] == "skipped"

assert runtime["aggregate_status"] == "release_gate_required"
assert runtime["lanes"]["docs_only_pr"]["status"] == "skipped"
assert runtime["lanes"]["docs_only_reuse_candidate"]["status"] == "skipped"
assert runtime["lanes"]["runtime_pr_fast"]["status"] == "passed"
assert runtime["lanes"]["authoritative_release_gate"]["status"] == "release_gate_required"

assert release["aggregate_status"] == "passed"
assert release["lanes"]["docs_only_pr"]["status"] == "passed"
assert release["lanes"]["docs_only_reuse_candidate"]["status"] == "reused"
assert release["lanes"]["runtime_pr_fast"]["status"] == "skipped"
assert release["lanes"]["authoritative_release_gate"]["status"] == "passed"
PY

grep -q "aggregate_status=passed" "$tmpdir/docs.stdout"
grep -q "aggregate_status=release_gate_required" "$tmpdir/runtime.stdout"
grep -q "aggregate_status=passed" "$tmpdir/release.stdout"
grep -q "docs_only_reuse_candidate status=reused" "$tmpdir/release.stdout"

echo "PASS test_pvf_ci_release_policy"
