#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT_DIR/adl/tools/run_pvf_validation_lane.sh"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

fakebin="$tmpdir/bin"
mkdir -p "$fakebin"
timings_dir="$tmpdir/timings"
mkdir -p "$timings_dir"

cat >"$fakebin/pass_lane.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
python3 - <<'PY' "$timings_dir/pass.start"
import sys, time
from pathlib import Path
Path(sys.argv[1]).write_text(str(time.time()))
time.sleep(1)
PY
echo "lane pass"
EOF

cat >"$fakebin/reuse_lane.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
python3 - <<'PY' "$timings_dir/reuse.start"
import sys, time
from pathlib import Path
Path(sys.argv[1]).write_text(str(time.time()))
time.sleep(1)
PY
echo "PVF_STATUS=reused"
EOF

cat >"$fakebin/fail_lane.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
python3 - <<'PY' "$timings_dir/fail.start"
import sys, time
from pathlib import Path
Path(sys.argv[1]).write_text(str(time.time()))
time.sleep(1)
PY
echo "lane fail" >&2
exit 1
EOF

chmod +x "$fakebin/pass_lane.sh" "$fakebin/reuse_lane.sh" "$fakebin/fail_lane.sh"
export PATH="$fakebin:$PATH"

manifest="$tmpdir/pvf-manifest.json"
changed="$tmpdir/changed.txt"
report="$tmpdir/report.json"

cat >"$manifest" <<'EOF'
{
  "manifest_version": "v0.91.4",
  "lane_classes": [
    "fast_unit",
    "docs",
    "cli_workflow",
    "integration_worktree",
    "release_gate"
  ],
  "lanes": {
    "pass_lane": {
      "lane_class": "fast_unit",
      "owner_surface": "pass lane",
      "command": "pass_lane.sh",
      "resource_profile": "low",
      "determinism": "strict",
      "cache_strategy": "none",
      "release_gate_class": "required_on_pr",
      "default_trigger": "changed_paths",
      "changed_path_hints": ["docs/"],
      "evidence_outputs": ["stdout:pass"]
    },
    "reuse_lane": {
      "lane_class": "docs",
      "owner_surface": "reuse lane",
      "command": "reuse_lane.sh",
      "resource_profile": "low",
      "determinism": "strict",
      "cache_strategy": "artifact_reuse",
      "release_gate_class": "required_on_pr",
      "default_trigger": "changed_paths",
      "changed_path_hints": ["docs/"],
      "evidence_outputs": ["stdout:reuse"]
    },
    "fail_lane": {
      "lane_class": "cli_workflow",
      "owner_surface": "fail lane",
      "command": "fail_lane.sh",
      "resource_profile": "low",
      "determinism": "strict",
      "cache_strategy": "none",
      "release_gate_class": "required_on_pr",
      "default_trigger": "changed_paths",
      "changed_path_hints": ["docs/"],
      "evidence_outputs": ["stdout:fail"]
    },
    "blocked_lane": {
      "lane_class": "integration_worktree",
      "owner_surface": "blocked lane",
      "command": "pass_lane.sh",
      "resource_profile": "medium",
      "determinism": "fixture_bound",
      "cache_strategy": "none",
      "release_gate_class": "required_on_pr",
      "default_trigger": "always",
      "changed_path_hints": [],
      "evidence_outputs": ["stdout:blocked"],
      "requires_worktree": true
    },
    "skipped_lane": {
      "lane_class": "docs",
      "owner_surface": "skipped lane",
      "command": "pass_lane.sh",
      "resource_profile": "low",
      "determinism": "strict",
      "cache_strategy": "none",
      "release_gate_class": "required_on_pr",
      "default_trigger": "changed_paths",
      "changed_path_hints": ["adl/src/"],
      "evidence_outputs": ["stdout:skipped"]
    },
    "deferred_lane": {
      "lane_class": "docs",
      "owner_surface": "deferred lane",
      "command": "pass_lane.sh",
      "resource_profile": "low",
      "determinism": "strict",
      "cache_strategy": "none",
      "release_gate_class": "optional",
      "default_trigger": "manual",
      "changed_path_hints": [],
      "evidence_outputs": ["stdout:deferred"]
    },
    "release_gate_lane": {
      "lane_class": "release_gate",
      "owner_surface": "release-gate lane",
      "command": "pass_lane.sh",
      "resource_profile": "high",
      "determinism": "fixture_bound",
      "cache_strategy": "artifact_reuse",
      "release_gate_class": "manual_release_gate",
      "default_trigger": "release_only",
      "changed_path_hints": ["docs/"],
      "evidence_outputs": ["stdout:release"]
    }
  }
}
EOF

printf 'M\tdocs/guide.md\n' > "$changed"

set +e
"$RUNNER" --manifest "$manifest" --changed-files "$changed" --report-out "$report" >"$tmpdir/stdout.txt" 2>"$tmpdir/stderr.txt"
runner_status=$?
set -e

if [ "$runner_status" -eq 0 ]; then
  echo "expected runner to exit nonzero when at least one lane fails" >&2
  exit 1
fi

python3 - <<'PY' "$report" "$timings_dir"
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
timings_dir = Path(sys.argv[2])

assert report["schema_version"] == "adl.pvf_run.v1"
assert report["mode"] == "pr"
assert report["aggregate_status"] == "failed"

lanes = report["lanes"]
assert lanes["pass_lane"]["status"] == "passed"
assert lanes["reuse_lane"]["status"] == "reused"
assert lanes["fail_lane"]["status"] == "failed"
assert lanes["blocked_lane"]["status"] == "blocked"
assert lanes["skipped_lane"]["status"] == "skipped"
assert lanes["deferred_lane"]["status"] == "deferred"
assert lanes["release_gate_lane"]["status"] == "release_gate_required"

pass_start = float((timings_dir / "pass.start").read_text())
reuse_start = float((timings_dir / "reuse.start").read_text())
fail_start = float((timings_dir / "fail.start").read_text())
assert abs(pass_start - reuse_start) < 0.75
assert abs(pass_start - fail_start) < 0.75
PY

grep -q "aggregate_status=failed" "$tmpdir/stdout.txt"
grep -q "status=release_gate_required" "$tmpdir/stdout.txt"
grep -q "status=deferred" "$tmpdir/stdout.txt"

rm -f "$timings_dir"/pass.start "$timings_dir"/reuse.start "$timings_dir"/fail.start
plan_stdout="$tmpdir/plan.stdout"
"$RUNNER" --manifest "$manifest" --changed-files "$changed" --print-plan >"$plan_stdout"
grep -q "PVF lane plan" "$plan_stdout"
if [ -f "$timings_dir/pass.start" ] || [ -f "$timings_dir/reuse.start" ] || [ -f "$timings_dir/fail.start" ]; then
  echo "expected --print-plan to avoid executing lanes" >&2
  exit 1
fi

cred_manifest="$tmpdir/credential-manifest.json"
cred_report="$tmpdir/credential-report.json"
cat >"$cred_manifest" <<'EOF'
{
  "manifest_version": "v0.91.4",
  "lane_classes": [
    "provider_live"
  ],
  "lanes": {
    "credential_lane": {
      "lane_class": "provider_live",
      "owner_surface": "credential lane",
      "command": "pass_lane.sh",
      "resource_profile": "high",
      "determinism": "live",
      "cache_strategy": "artifact_reuse",
      "release_gate_class": "required_on_pr",
      "default_trigger": "always",
      "changed_path_hints": [],
      "evidence_outputs": ["stdout:credential"],
      "requires_credentials": true
    }
  }
}
EOF

set +e
"$RUNNER" --manifest "$cred_manifest" --changed-files "$changed" --report-out "$cred_report" >"$tmpdir/cred.stdout"
cred_status=$?
set -e
if [ "$cred_status" -eq 0 ]; then
  echo "expected credential-gated run to exit nonzero with blocked aggregate status" >&2
  exit 1
fi
python3 - <<'PY' "$cred_report"
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text())
assert report["aggregate_status"] == "blocked"
assert report["lanes"]["credential_lane"]["status"] == "blocked"
assert report["lanes"]["credential_lane"]["reason"] == "credential_lane_requires_explicit_opt_in"
PY

echo "PASS test_run_pvf_validation_lane"
