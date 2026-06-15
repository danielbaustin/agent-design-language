#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
validator="$repo_root/adl/tools/validate_v0915_multi_agent_quality_comparison.py"
packet_dir="$repo_root/docs/milestones/v0.91.5/review/multi_agent_quality_comparison"
state_name="v0915_multi_agent_quality_comparison_state_2026-06-15.json"
packet_name="MULTI_AGENT_QUALITY_COMPARISON_2026-06-15.md"
tmpdir="$(mktemp -d "$repo_root/.tmp_multi_agent_quality_comparison.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

python3 "$validator" "$packet_dir" >"$tmpdir/root.out"
grep -Fq 'PASS: multi-agent quality comparison bundle valid' "$tmpdir/root.out"

(
  cd /private/tmp
  python3 "$validator" "$packet_dir" >"$tmpdir/cwd.out"
)
grep -Fq 'PASS: multi-agent quality comparison bundle valid' "$tmpdir/cwd.out"

cp -R "$packet_dir" "$tmpdir/bad_remote"
python3 - "$tmpdir/bad_remote" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
repo_root = root.parents[1]
lane_path = root / "lane_outputs" / "multi_agent_watcher_gemini_2_5_flash_lite.md"
lane_path.write_text(lane_path.read_text().replace("route probe completed", "route probe missing"))
state_path = root / "v0915_multi_agent_quality_comparison_state_2026-06-15.json"
state = json.loads(state_path.read_text())
for lane in state["multi_agent"]["lanes"]:
    if lane["lane_id"] == "multi_agent_watcher_gemini_2_5_flash_lite":
        lane["output_path"] = lane_path.relative_to(repo_root).as_posix()
state_path.write_text(json.dumps(state, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/bad_remote" >"$tmpdir/bad_remote.out" 2>"$tmpdir/bad_remote.err"; then
  echo "assertion failed: bad_remote fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'multi_agent_watcher_gemini_2_5_flash_lite missing snippet: route probe completed' "$tmpdir/bad_remote.err"

cp -R "$packet_dir" "$tmpdir/bad_summary"
python3 - "$tmpdir/bad_summary/$state_name" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["summary"]["multi_agent_parallel_duration_seconds"] = data["summary"]["single_agent_duration_seconds"] + 1
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/bad_summary" >"$tmpdir/bad_summary.out" 2>"$tmpdir/bad_summary.err"; then
  echo "assertion failed: bad_summary fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'better comparison requires multi-agent parallel duration to be lower than single-agent duration' "$tmpdir/bad_summary.err"

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
path.write_text(path.read_text().replace("Status: `better`", "Status: `mixed`"))
PY
if python3 "$validator" "$tmpdir/bad_packet" >"$tmpdir/bad_packet.out" 2>"$tmpdir/bad_packet.err"; then
  echo "assertion failed: bad_packet fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'packet missing required text: Status: `better`' "$tmpdir/bad_packet.err"

cp -R "$packet_dir" "$tmpdir/local_artifact_ref"
python3 - "$tmpdir/local_artifact_ref/$state_name" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["single_agent"]["artifact_paths"] = [
    "docs/milestones/v0.91.5/review/multi_agent_quality_comparison/artifacts/single_agent_response.json"
]
data["multi_agent"]["lanes"][0]["artifact_paths"] = [
    "docs/milestones/v0.91.5/review/multi_agent_quality_comparison/artifacts/lane_response.json"
]
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 "$validator" "$tmpdir/local_artifact_ref" >"$tmpdir/local_artifact_ref.out" 2>"$tmpdir/local_artifact_ref.err"; then
  echo "assertion failed: local artifact reference fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'single_agent must not require local-only artifact_paths in tracked state' "$tmpdir/local_artifact_ref.err"

cp -R "$packet_dir" "$tmpdir/extra_lane_output"
printf '# stale output\n' >"$tmpdir/extra_lane_output/lane_outputs/unreferenced_model_lane.md"
if python3 "$validator" "$tmpdir/extra_lane_output" >"$tmpdir/extra_lane_output.out" 2>"$tmpdir/extra_lane_output.err"; then
  echo "assertion failed: extra lane output fixture unexpectedly validated" >&2
  exit 1
fi
grep -Fq 'unreferenced lane output files present:' "$tmpdir/extra_lane_output.err"

echo "PASS test_v0915_multi_agent_quality_comparison"
