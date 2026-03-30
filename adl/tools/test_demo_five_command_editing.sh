#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$(
  cd "$ROOT_DIR" &&
    KEEP_DEMO_TMP=1 bash adl/tools/demo_five_command_editing.sh
)"

MANIFEST_PATH="$(printf '%s\n' "$OUT" | sed -n 's/^five-command editing demo manifest: //p' | tail -n1)"
[[ -n "$MANIFEST_PATH" ]] || {
  echo "assertion failed: expected manifest path in demo output" >&2
  exit 1
}
[[ -f "$MANIFEST_PATH" ]] || {
  echo "assertion failed: manifest path does not exist: $MANIFEST_PATH" >&2
  exit 1
}

python3 - <<PY
import json
from pathlib import Path

manifest = json.loads(Path("$MANIFEST_PATH").read_text())
assert manifest["schema_version"] == "five_command_editing_demo.v1"
assert manifest["demo_entry"] == "bash adl/tools/demo_five_command_editing.sh"

for key in ("pr_init", "editor_adapter", "pr_start", "pr_run", "pr_finish"):
    path = Path(manifest["step_logs"][key])
    assert path.exists(), f"missing step log: {path}"

for key in ("stp", "input_card", "output_card", "run_json", "run_status_json", "run_summary_json", "tracked_finish_note"):
    path = Path(manifest["artifacts"][key])
    assert path.exists(), f"missing artifact: {path}"
PY

grep -Fq "STP      .adl/v0.85/tasks/issue-0042__five-command-editing-demo/stp.md" "$(python3 - <<PY
import json
from pathlib import Path
manifest = json.loads(Path("$MANIFEST_PATH").read_text())
print(manifest["step_logs"]["pr_init"])
PY
)" || {
  echo "assertion failed: pr init log missing STP path" >&2
  exit 1
}

grep -Fq "./adl/tools/pr.sh start 42 --slug five-command-editing-demo" "$(python3 - <<PY
import json
from pathlib import Path
manifest = json.loads(Path("$MANIFEST_PATH").read_text())
print(manifest["step_logs"]["editor_adapter"])
PY
)" || {
  echo "assertion failed: adapter log missing pr start command" >&2
  exit 1
}

grep -Fq "PR RUN ok" "$(python3 - <<PY
import json
from pathlib import Path
manifest = json.loads(Path("$MANIFEST_PATH").read_text())
print(manifest["step_logs"]["pr_run"])
PY
)" || {
  echo "assertion failed: pr run log missing success marker" >&2
  exit 1
}

GH_LOG_PATH="$(python3 - <<PY
import json
from pathlib import Path
manifest = json.loads(Path("$MANIFEST_PATH").read_text())
print(manifest["gh_log"])
PY
)"

grep -Fq "pr list" "$GH_LOG_PATH" || {
  echo "assertion failed: gh log missing pr list call" >&2
  exit 1
}

if grep -Fq "pr create" "$GH_LOG_PATH"; then
  echo "assertion failed: demo should not open a PR when --no-open is set" >&2
  exit 1
fi

echo "editing demo: ok"
