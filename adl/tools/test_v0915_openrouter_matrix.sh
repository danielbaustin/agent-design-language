#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

packet_dir="docs/milestones/v0.91.5/review/openrouter_matrix"
key_file="${ADL_OPENROUTER_KEY_FILE:-$HOME/keys/openrouter.key}"
live_tmp=""
tmpdir=""

if [[ -z "${OPENROUTER_API_KEY:-}" && ! -s "$key_file" ]]; then
  echo "SKIP live runner check: missing OPENROUTER_API_KEY and $key_file" >&2
else
  live_tmp="$(mktemp -d "$repo_root/.tmp-openrouter-live.XXXXXX")"
  trap 'rm -rf "$tmpdir" "$live_tmp"' EXIT
  python3 adl/tools/run_v0915_openrouter_matrix.py --out "$live_tmp/packet"
  python3 - <<'PY' "$live_tmp/packet/openrouter_matrix_state_2026-06-14.json"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
lanes = data.get("lanes", [])
if len(lanes) != 6:
    raise SystemExit("expected six lanes in live runner state output")
statuses = {lane.get("contract_status") for lane in lanes}
allowed = {"supported", "non_proving", "blocked_missing_credential"}
if not statuses.issubset(allowed):
    raise SystemExit(f"unexpected live runner statuses: {sorted(statuses)}")
negative = [lane for lane in lanes if lane.get("role") == "negative_control"]
if len(negative) != 1 or negative[0].get("failure_kind") != "provider_auth_missing":
    raise SystemExit("live runner missing fail-closed negative control")
supported_or_non_proving = [
    lane for lane in lanes if lane.get("role") != "negative_control"
]
if len(supported_or_non_proving) != 5:
    raise SystemExit("expected five routed live lanes")
PY
fi

python3 adl/tools/validate_v0915_openrouter_matrix.py "$packet_dir"

(
  cd /private/tmp
  python3 "$repo_root/adl/tools/validate_v0915_openrouter_matrix.py" \
    "$repo_root/$packet_dir"
)

tmpdir="$(mktemp -d "$repo_root/.tmp-openrouter-matrix.XXXXXX")"
trap 'rm -rf "$tmpdir" "$live_tmp"' EXIT

cp -R "$packet_dir" "$tmpdir/packet"
python3 - <<'PY' "$tmpdir/packet/openrouter_matrix_state_2026-06-14.json"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
for lane in data["lanes"]:
    if lane.get("role") == "negative_control":
        lane["failure_kind"] = "provider_timeout"
        break
path.write_text(json.dumps(data, indent=2) + "\n")
PY
if python3 adl/tools/validate_v0915_openrouter_matrix.py "$tmpdir/packet"; then
  echo "expected negative-control drift to fail" >&2
  exit 1
fi

rm -rf "$tmpdir/packet"
cp -R "$packet_dir" "$tmpdir/packet"
python3 - <<'PY' "$tmpdir/packet/OPENROUTER_MATRIX_PROOF_2026-06-14.md"
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text()
path.write_text(text.replace("## Non-Proving Paths", "## Prior Paths"))
PY
if python3 adl/tools/validate_v0915_openrouter_matrix.py "$tmpdir/packet"; then
  echo "expected packet heading drift to fail" >&2
  exit 1
fi

cargo test --manifest-path adl/Cargo.toml provider_setup_supports_all_declared_families -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_setup_supports_model_override_for_openrouter -- --nocapture
cargo test --manifest-path adl/Cargo.toml provider_substrate_accepts_native_openai_anthropic_deepseek_and_openrouter_kinds -- --nocapture

echo "PASS test_v0915_openrouter_matrix"
