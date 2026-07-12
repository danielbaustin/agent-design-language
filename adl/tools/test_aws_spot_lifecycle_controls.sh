#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
TMP_ROOT="${TMPDIR:-$ROOT/.adl/tmp}"
mkdir -p "$TMP_ROOT"
TMP="$(mktemp -d "$TMP_ROOT/aws-spot-lifecycle-test.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

account="123456789012"
account_hash="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$account")"
proof="$TMP/proof.json"
printf '{"account_identity":{"account_id_sha256":"%s"}}\n' "$account_hash" >"$proof"

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
args=" $* "
if [[ "$args" == *" sts get-caller-identity "* ]]; then
  printf '{"Account":"123456789012","Arn":"arn:aws:iam::123456789012:role/test","UserId":"AIDAEXAMPLE"}\n'
elif [[ "$args" == *" ec2 describe-instances "* ]]; then
  printf '%s\n' "${ADL_FAKE_RUN_ID:-fixture-controls}"
elif [[ "$args" == *" ec2 terminate-instances "* ]]; then
  printf '{"TerminatingInstances":[]}\n'
elif [[ "$args" == *" ec2 wait instance-terminated "* ]]; then
  exit 0
elif [[ "$args" == *" ec2 describe-volumes "* ]]; then
  printf 'available\n'
else
  echo "unexpected aws invocation: $*" >&2
  exit 1
fi
EOF
cat >"$fake_bin/ssh" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$@" >"${ADL_FAKE_SSH_ARGS:?}"
EOF
chmod +x "$fake_bin/aws" "$fake_bin/ssh"

key="$TMP/key"
ssh-keygen -q -t ed25519 -N '' -f "$key"
chmod 600 "$key"

run_id="fixture-controls"
out="$TMP/$run_id/summary.json"
artifacts="$TMP/$run_id/artifacts"
mkdir -p "$artifacts"
mkdir -p "$artifacts/attempt-1"
cat >"$artifacts/attempt-1/command-status.log" <<'LOG'
status=ssh_debug_ready instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
status=ssh_tail_started instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
LOG
cat >"$artifacts/attempt-1/remote-tail.log" <<'LOG'
instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
LOG
cat >"$artifacts/manager.stderr.log" <<'LOG'
instance_id=i-0123456789abcdef0 account=123456789012 public_ip=192.0.2.10
LOG

common=(
  --run-id "$run_id"
  --out "$out"
  --artifact-dir "$artifacts"
  --expected-proof "$proof"
  --profile agent-logic-admin
)

ADL_FAKE_SSH_ARGS="$TMP/ssh.args" ADL_SSH_BIN="$fake_bin/ssh" \
  bash "$SCRIPT" ssh "${common[@]}" --ssh-private-key-path "$key"
grep -Fx -- '-o' "$TMP/ssh.args" >/dev/null
grep -Fx -- "ec2-user@192.0.2.10" "$TMP/ssh.args" >/dev/null

ADL_AWS_CLI="$fake_bin/aws" ADL_FAKE_RUN_ID="$run_id" \
  bash "$SCRIPT" stop "${common[@]}" >"$TMP/stop.out"
grep -F 'status=terminated' "$TMP/stop.out" >/dev/null
grep -F 'retained_cache_preserved=true' "$TMP/stop.out" >/dev/null

ADL_AWS_CLI="$fake_bin/aws" ADL_FAKE_RUN_ID="$run_id" \
  bash "$SCRIPT" cleanup "${common[@]}" >"$TMP/cleanup.out"
grep -F 'status=clean' "$TMP/cleanup.out" >/dev/null
grep -F 'cache_state=available' "$TMP/cleanup.out" >/dev/null

bash "$SCRIPT" logs "${common[@]}" >"$TMP/logs.out"
if rg -n '123456789012|i-0123456789abcdef0|192\.0\.2\.10' "$TMP/logs.out" >/dev/null; then
  echo "logs action leaked an AWS identity" >&2
  exit 1
fi
grep -F '<ec2-instance-id-redacted>' "$TMP/logs.out" >/dev/null

cat >"$artifacts/wrapper-final-summary.json" <<'JSON'
{"schema":"adl.aws_spot_remote_validation_wrapper_summary.v2","status":"passed"}
JSON
bash "$SCRIPT" status "${common[@]}" >"$TMP/status.out"
grep -F '"status":"passed"' "$TMP/status.out" >/dev/null

if ADL_AWS_CLI="$fake_bin/aws" ADL_FAKE_RUN_ID=wrong-run \
  bash "$SCRIPT" stop "${common[@]}" >"$TMP/wrong.out" 2>"$TMP/wrong.err"; then
  echo "expected run-id mismatch to refuse termination" >&2
  exit 1
fi
grep -F 'run-id tag mismatch' "$TMP/wrong.err" >/dev/null

echo "PASS test_aws_spot_lifecycle_controls"
