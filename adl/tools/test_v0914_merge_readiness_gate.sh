#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PACKET_ROOT="$ROOT_DIR/docs/milestones/v0.91.4/review/merge_readiness"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

python3 "$ROOT_DIR/adl/tools/validate_v0914_merge_readiness_gate.py" "$PACKET_ROOT" >/dev/null

cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" \
  finish_validation_plan_supports_focused_local_ci_gated_mode -- --nocapture
cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" \
  finish_validation_profile_uses_actual_changed_paths_not_broad_stage_request -- --nocapture
cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" \
  finish_helper_paths_run_focused_local_ci_gated_validation -- --nocapture
cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" \
  card_lifecycle_allows_explicit_srp_policy_exception -- --nocapture
cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" \
  card_lifecycle_blocks_completed_sor_before_terminal_closeout -- --nocapture

BROKEN_ROOT="$TMPDIR_ROOT/broken"
mkdir -p "$BROKEN_ROOT"
cp "$PACKET_ROOT"/README.md "$BROKEN_ROOT"/README.md
cp "$PACKET_ROOT"/MERGE_READINESS_GATE_PACKET_v0.91.4.md \
  "$BROKEN_ROOT"/MERGE_READINESS_GATE_PACKET_v0.91.4.md
cp "$PACKET_ROOT"/ct_demo_001_merge_gate_snapshot.json \
  "$BROKEN_ROOT"/ct_demo_001_merge_gate_snapshot.json
python3 - "$PACKET_ROOT/ct_demo_001_merge_gate_profile_report.md" "$BROKEN_ROOT/ct_demo_001_merge_gate_profile_report.md" <<'PY'
from pathlib import Path
import sys

src = Path(sys.argv[1]).read_text(encoding="utf-8")
Path(sys.argv[2]).write_text(
    src.replace("## Lifecycle Blockers", "## Lifecycle Status", 1),
    encoding="utf-8",
)
PY

if python3 "$ROOT_DIR/adl/tools/validate_v0914_merge_readiness_gate.py" "$BROKEN_ROOT" >/dev/null 2>"$TMPDIR_ROOT/fail.stderr"; then
  echo "assertion failed: validator accepted packet missing lifecycle blockers section" >&2
  exit 1
fi

grep -Fq "report missing required snippets" "$TMPDIR_ROOT/fail.stderr" || {
  echo "assertion failed: missing fail-closed report validator message" >&2
  exit 1
}

BROKEN_SNAPSHOT_ROOT="$TMPDIR_ROOT/broken-snapshot"
mkdir -p "$BROKEN_SNAPSHOT_ROOT"
cp "$PACKET_ROOT"/README.md "$BROKEN_SNAPSHOT_ROOT"/README.md
cp "$PACKET_ROOT"/MERGE_READINESS_GATE_PACKET_v0.91.4.md \
  "$BROKEN_SNAPSHOT_ROOT"/MERGE_READINESS_GATE_PACKET_v0.91.4.md
cp "$PACKET_ROOT"/ct_demo_001_merge_gate_profile_report.md \
  "$BROKEN_SNAPSHOT_ROOT"/ct_demo_001_merge_gate_profile_report.md
python3 - "$PACKET_ROOT/ct_demo_001_merge_gate_snapshot.json" "$BROKEN_SNAPSHOT_ROOT/ct_demo_001_merge_gate_snapshot.json" <<'PY'
from pathlib import Path
import json
import sys

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
data["merge_truth_inferred_from_local_validation"] = True
Path(sys.argv[2]).write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
PY

if python3 "$ROOT_DIR/adl/tools/validate_v0914_merge_readiness_gate.py" "$BROKEN_SNAPSHOT_ROOT" >/dev/null 2>"$TMPDIR_ROOT/snapshot.stderr"; then
  echo "assertion failed: validator accepted snapshot that infers merge truth from local validation" >&2
  exit 1
fi

grep -Fq "snapshot field 'merge_truth_inferred_from_local_validation' expected False, found True" "$TMPDIR_ROOT/snapshot.stderr" || {
  echo "assertion failed: missing snapshot fail-closed validator message" >&2
  exit 1
}

echo "PASS test_v0914_merge_readiness_gate"
