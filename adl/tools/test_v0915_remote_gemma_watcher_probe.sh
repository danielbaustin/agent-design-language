#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
validator="$repo_root/adl/tools/validate_v0915_remote_gemma_watcher_probe.py"
packet_dir="$repo_root/docs/milestones/v0.91.5/review/remote_gemma_watcher"
state_name="v0915_remote_gemma_watcher_state_2026-06-15.json"
packet_name="REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md"
tmpdir="$(mktemp -d "$repo_root/.tmp_remote_gemma_probe_test.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

python3 "$validator" "$packet_dir" >"$tmpdir/root.out"
grep -Fq 'PASS: remote gemma watcher proof bundle valid' "$tmpdir/root.out"

(
  cd /private/tmp
  python3 "$validator" "$packet_dir" >"$tmpdir/cwd.out"
)
grep -Fq 'PASS: remote gemma watcher proof bundle valid' "$tmpdir/cwd.out"

cp -R "$packet_dir" "$tmpdir/invalid_status"
python3 - "$tmpdir/invalid_status/$state_name" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
for lane in data["lanes"]:
    if lane["lane_id"] == "adapter_gemma4_31b":
        lane["status"] = "empty_output"
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/invalid_status" >"$tmpdir/invalid_status.out" 2>"$tmpdir/invalid_status.err"; then
  echo "assertion failed: invalid status fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'adapter_gemma4_31b status must be useful_output' "$tmpdir/invalid_status.err"

cp -R "$packet_dir" "$tmpdir/missing_phrase"
python3 - "$tmpdir/missing_phrase" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
repo_root = root.parents[1]
lane_path = root / "lane_outputs" / "adapter_gemma4_31b.md"
lane_text = lane_path.read_text()
lane_path.write_text(lane_text.replace("route probe completed", "route probe missing"))

state_path = root / "v0915_remote_gemma_watcher_state_2026-06-15.json"
state = json.loads(state_path.read_text())
for lane in state["lanes"]:
    if lane["lane_id"] == "adapter_gemma4_31b":
        lane["output_path"] = lane_path.relative_to(repo_root).as_posix()
state_path.write_text(json.dumps(state, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/missing_phrase" >"$tmpdir/missing_phrase.out" 2>"$tmpdir/missing_phrase.err"; then
  echo "assertion failed: missing-phrase fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'adapter_gemma4_31b output missing phrase: route probe completed' "$tmpdir/missing_phrase.err"

cp -R "$packet_dir" "$tmpdir/local_artifact_ref"
python3 - "$tmpdir/local_artifact_ref/$state_name" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["inventory"]["tags_snapshot_path"] = "docs/milestones/v0.91.5/review/remote_gemma_watcher/artifacts/ollama_tags_snapshot.json"
data["lanes"][0]["artifact_paths"] = [
    "docs/milestones/v0.91.5/review/remote_gemma_watcher/artifacts/adapter_gemma4_31b_result.json"
]
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/local_artifact_ref" >"$tmpdir/local_artifact_ref.out" 2>"$tmpdir/local_artifact_ref.err"; then
  echo "assertion failed: local artifact reference fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'inventory must not require local-only tags_snapshot_path in tracked state' "$tmpdir/local_artifact_ref.err"

cp -R "$packet_dir" "$tmpdir/missing_reliability_gate"
python3 - "$tmpdir/missing_reliability_gate/$state_name" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["summary"].pop("reliability_gate", None)
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/missing_reliability_gate" >"$tmpdir/missing_reliability_gate.out" 2>"$tmpdir/missing_reliability_gate.err"; then
  echo "assertion failed: missing reliability gate fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'summary.reliability_gate must be passed' "$tmpdir/missing_reliability_gate.err"

cp -R "$packet_dir" "$tmpdir/bad_packet"
python3 - "$tmpdir/bad_packet/$packet_name" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text()
path.write_text(text.replace("historical empty output", "historical watcher note"))
PY
if python3 "$validator" "$tmpdir/bad_packet" >"$tmpdir/bad_packet.out" 2>"$tmpdir/bad_packet.err"; then
  echo "assertion failed: bad-packet fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'packet missing required text: historical empty output' "$tmpdir/bad_packet.err"

echo "PASS test_v0915_remote_gemma_watcher_probe"
