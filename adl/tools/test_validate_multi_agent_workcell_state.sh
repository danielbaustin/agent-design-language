#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
validator="$repo_root/adl/tools/validate_multi_agent_workcell_state.py"
good_fixture="$repo_root/docs/milestones/v0.91.4/review/multi_agent_workcell/fixtures/workcell_state_packet_example.json"
bad_fixture="$repo_root/docs/milestones/v0.91.4/review/multi_agent_workcell/fixtures/workcell_state_packet_invalid.json"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

python3 "$validator" "$good_fixture" > "$tmpdir/good.txt"
grep -Fq 'PASS: multi-agent workcell state valid' "$tmpdir/good.txt"

if python3 "$validator" "$bad_fixture" >"$tmpdir/bad.txt" 2>"$tmpdir/bad.err"; then
  echo "assertion failed: invalid workcell state fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'github_issue_state must be OPEN for parallel-admitted worker shards' "$tmpdir/bad.err"

python3 - "$good_fixture" "$tmpdir/path_escape.json" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
data["planner_manifest_path"] = "../tmp/outside.json"
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/path_escape.json" >"$tmpdir/path_escape.out" 2>"$tmpdir/path_escape.err"; then
  echo "assertion failed: path-escape fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'must stay within the repo-relative tree' "$tmpdir/path_escape.err"

python3 - "$good_fixture" "$tmpdir/missing_hook_truth.json" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
data["conductor_hooks"]["closeout_reconciliation"]["canonical_truth"] = [
    "github_issue_state",
    "sor",
    "sprint_state"
]
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/missing_hook_truth.json" >"$tmpdir/missing_hook_truth.out" 2>"$tmpdir/missing_hook_truth.err"; then
  echo "assertion failed: hook-truth fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'canonical_truth missing required surfaces: closeout_artifacts' "$tmpdir/missing_hook_truth.err"

python3 - "$good_fixture" "$tmpdir/missing_dependency.json" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
data["shard_assignments"][2]["dependencies"] = ["missing-shard"]
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/missing_dependency.json" >"$tmpdir/missing_dependency.out" 2>"$tmpdir/missing_dependency.err"; then
  echo "assertion failed: missing-dependency fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'references unknown shard ids: missing-shard' "$tmpdir/missing_dependency.err"

python3 - "$good_fixture" "$tmpdir/self_dependency.json" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
data["shard_assignments"][0]["dependencies"] = ["ollama-worker-a"]
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/self_dependency.json" >"$tmpdir/self_dependency.out" 2>"$tmpdir/self_dependency.err"; then
  echo "assertion failed: self-dependency fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'must not contain itself' "$tmpdir/self_dependency.err"

python3 - "$good_fixture" "$tmpdir/nonworker_backend.json" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
data["shard_assignments"][3]["execution_backend"] = "local_ollama"
data["shard_assignments"][3]["model_hint"] = "qwen2.5-coder:32b"
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/nonworker_backend.json" >"$tmpdir/nonworker_backend.out" 2>"$tmpdir/nonworker_backend.err"; then
  echo "assertion failed: non-worker backend fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'must not declare execution_backend/model_hint for non-worker roles' "$tmpdir/nonworker_backend.err"

python3 - "$good_fixture" "$tmpdir/open_done_sor.json" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
data["shard_assignments"][5]["github_issue_state"] = "OPEN"
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/open_done_sor.json" >"$tmpdir/open_done_sor.out" 2>"$tmpdir/open_done_sor.err"; then
  echo "assertion failed: open-done-sor fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'github_issue_state must be CLOSED when closeout_status is closed_out' "$tmpdir/open_done_sor.err"

echo "PASS test_validate_multi_agent_workcell_state"
