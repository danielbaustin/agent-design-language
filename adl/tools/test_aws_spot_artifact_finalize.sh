#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FINALIZER="$ROOT/adl/tools/aws_spot_artifact_finalize.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

source_commit="$(git -C "$ROOT" rev-parse HEAD)"
image_digest="sha256:$(printf 'a%.0s' {1..64})"
image="123456789012.dkr.ecr.us-west-2.amazonaws.com/adl-builder@$image_digest"
digest_hash="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$image_digest")"
cache_volume_hash="$(python3 -c 'import hashlib; print(hashlib.sha256(b"vol-0123456789abcdef0").hexdigest())')"

make_fixture() {
  local root="$1"
  mkdir -p "$root/artifacts"
  python3 - "$root/summary.json" "$source_commit" "$digest_hash" <<'PY'
import json
import sys
path, commit, digest_hash = sys.argv[1:]
payload = {
  "status": "passed",
  "account_identity": {"account_id": "123456789012", "arn": "arn:aws:iam::123456789012:role/test"},
  "launch": {"purchase_option": "spot", "instance_id": "i-0123456789abcdef0"},
  "cache_volume": {"created": False, "attachment_state": "attached", "mount_path": "/mnt/adl-cache", "volume_id": "vol-0123456789abcdef0"},
  "cleanup": {"termination_attempted": True, "final_instance_state": "terminated", "termination_error": None},
  "launch_surface": {"ssh_debug_enabled": True, "vpc_id": "vpc-0123456789abcdef0", "subnet_id": "subnet-0123456789abcdef0", "security_group_id": "sg-0123456789abcdef0"},
  "timings": {"total_seconds": 120, "launch_seconds": 20, "ssm_ready_seconds": 10, "remote_command_seconds": 80, "teardown_seconds": 10},
  "remote_summary": {"builder_proof": {
    "builder_image_immutable": True,
    "builder_image_digest_sha256": digest_hash,
    "toolchain_verified": True,
    "source_commit_verified": True,
    "source_commit": commit,
    "cache_mount_verified": True,
    "cache_writable": True,
    "host_validation_tools_installed": False,
    "builder_image_architecture": "amd64",
    "cache_target_preexisting_entries": 42,
    "cache_target_preexisting_bytes": 4096,
    "cache_free_bytes": 90000000000,
    "validation_seconds": 73,
  }},
}
open(path, "w", encoding="utf-8").write(json.dumps(payload) + "\n")
PY
  cat >"$root/artifacts/command-status.log" <<'LOG'
status=ssh_debug_ready instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
status=ssh_tail_started instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
LOG
  echo 'account=123456789012 arn:aws:iam::123456789012:role/test i-0123456789abcdef0' >"$root/artifacts/events.jsonl"
}

run_finalizer() {
  local root="$1"
  python3 "$FINALIZER" \
    --summary "$root/summary.json" \
    --artifact-dir "$root/artifacts" \
    --wrapper-summary "$root/artifacts/wrapper-final-summary.json" \
    --expected-source-commit "$source_commit" \
    --expected-image "$image" \
    --expected-cache-volume-id-sha256 "$cache_volume_hash" \
    --estimated-hourly-cost-usd 0.15 \
    --runner-exit-code 0
}

pass="$TMP/pass"
make_fixture "$pass"
run_finalizer "$pass" >"$pass/finalize.out"
python3 - "$pass/summary.json" "$pass/artifacts/wrapper-final-summary.json" "$pass/artifacts/.private/control-summary.json" <<'PY'
import json
import sys
public = open(sys.argv[1], encoding="utf-8").read()
wrapper = json.load(open(sys.argv[2], encoding="utf-8"))
private = open(sys.argv[3], encoding="utf-8").read()
assert "123456789012" not in public
assert "arn:aws:" not in public
assert "i-0123456789abcdef0" not in public
assert "123456789012" in private
assert wrapper["self_verification"]["passed"] is True
assert wrapper["cache_target_preexisting_entries"] == 42
assert wrapper["cost"]["estimated_compute_cost_usd"] == 0.005
PY
if rg -n '123456789012|arn:aws:|i-0123456789abcdef0|192\.0\.2\.10' "$pass/artifacts" -g '!control-summary.json' >/dev/null; then
  echo "public artifact retained an AWS identity" >&2
  exit 1
fi

teardown="$TMP/teardown"
make_fixture "$teardown"
python3 - "$teardown/summary.json" <<'PY'
import json, sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["cleanup"]["final_instance_state"] = "running"
open(path, "w", encoding="utf-8").write(json.dumps(data) + "\n")
PY
if run_finalizer "$teardown" >"$teardown/out" 2>"$teardown/err"; then
  echo "expected teardown failure to fail closed" >&2
  exit 1
fi
grep -F 'compute_not_terminated' "$teardown/err" >/dev/null

ssh_failure="$TMP/ssh"
make_fixture "$ssh_failure"
: >"$ssh_failure/artifacts/command-status.log"
if run_finalizer "$ssh_failure" >"$ssh_failure/out" 2>"$ssh_failure/err"; then
  echo "expected missing SSH proof to fail closed" >&2
  exit 1
fi
grep -F 'ssh_recovery_not_proven' "$ssh_failure/err" >/dev/null

missing_builder="$TMP/builder"
make_fixture "$missing_builder"
python3 - "$missing_builder/summary.json" <<'PY'
import json, sys
path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["remote_summary"].pop("builder_proof")
open(path, "w", encoding="utf-8").write(json.dumps(data) + "\n")
PY
if run_finalizer "$missing_builder" >"$missing_builder/out" 2>"$missing_builder/err"; then
  echo "expected missing builder proof to fail closed" >&2
  exit 1
fi
grep -F 'builder_image_not_immutable' "$missing_builder/err" >/dev/null

echo "PASS test_aws_spot_artifact_finalize"

attempt_layout="$TMP/attempt-layout"
make_fixture "$attempt_layout"
mkdir -p "$attempt_layout/artifacts/attempt-0"
python3 - <<'PY' "$attempt_layout/summary.json" "$attempt_layout/artifacts/attempt-0"
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text())
data["artifact_dir"] = str(Path(sys.argv[2]))
path.write_text(json.dumps(data))
PY
python3 - <<'PY' "$attempt_layout/summary.json" "$attempt_layout/artifacts/attempt-0/command-stdout.log"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
remote = data["remote_summary"]
data["remote_summary"] = {}
Path(sys.argv[1]).write_text(json.dumps(data))
Path(sys.argv[2]).write_text(
    "ADL_AWS_REMOTE_SUMMARY_BEGIN\n" + json.dumps(remote) + "\nADL_AWS_REMOTE_SUMMARY_END\n"
)
PY
mv "$attempt_layout/artifacts/command-status.log" "$attempt_layout/artifacts/attempt-0/command-status.log"
python3 "$FINALIZER" \
  --summary "$attempt_layout/summary.json" \
  --artifact-dir "$attempt_layout/artifacts" \
  --wrapper-summary "$attempt_layout/wrapper.json" \
  --expected-source-commit "$source_commit" \
  --expected-image "$image" \
  --expected-cache-volume-id-sha256 "$cache_volume_hash" \
  --estimated-hourly-cost-usd 0.21 \
  --runner-exit-code 0 >/dev/null
python3 - <<'PY' "$attempt_layout/wrapper.json"
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))
assert data["status"] == "passed", data
assert data["self_verification"]["live_logs_verified"] is True, data
assert data["self_verification"]["immutable_builder_image_verified"] is True, data
PY

echo "PASS test_aws_spot_artifact_finalize_attempt_layout"
