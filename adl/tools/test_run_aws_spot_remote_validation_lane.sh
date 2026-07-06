#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
SETUP_SCRIPT="$ROOT/adl/tools/setup_aws_spot_remote_validation_github_resources.sh"
WORKFLOW="$ROOT/.github/workflows/aws-spot-remote-validation.yaml"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

account="123456789012"
account_hash="$(python3 - "$account" <<'PY'
import hashlib
import sys
print(hashlib.sha256(sys.argv[1].encode("utf-8")).hexdigest())
PY
)"

proof="$TMP/proof.json"
python3 - "$proof" "$account_hash" <<'PY'
import json
import sys
path, account_hash = sys.argv[1:3]
with open(path, "w", encoding="utf-8") as handle:
    json.dump({"account_identity": {"account_id_sha256": account_hash}}, handle)
PY

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"

cat >"$fake_bin/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" != "sts" || "$2" != "get-caller-identity" ]]; then
  echo "unexpected aws invocation: $*" >&2
  exit 1
fi
cat <<'JSON'
{
  "Account": "123456789012",
  "Arn": "arn:aws:iam::123456789012:user/test-user",
  "UserId": "AIDAEXAMPLE"
}
JSON
EOF

cat >"$fake_bin/adl-aws-remote-validation" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$@" >"${ADL_FAKE_AWS_REMOTE_ARGS:?}"
out=""
artifact_dir=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --out)
      out="${2:-}"
      shift 2
      ;;
    --artifact-dir)
      artifact_dir="${2:-}"
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done
mkdir -p "$(dirname "$out")" "$artifact_dir"
cat >"$out" <<'JSON'
{"schema":"adl.aws_remote_validation_run.v1","status":"passed"}
JSON
cat >"$artifact_dir/events.jsonl" <<'JSONL'
{"event":"fixture"}
JSONL
echo "fixture remote validation passed"
EOF
chmod +x "$fake_bin/aws" "$fake_bin/adl-aws-remote-validation"

ADL_AWS_CLI="$fake_bin/aws" \
bash "$SCRIPT" \
  --check-account \
  --expected-proof "$proof" \
  --git-ref origin/main >"$TMP/check.out"
grep -F "PASS account_profile_resolved profile=agent-logic-admin account_matches_retained_proof=true" "$TMP/check.out" >/dev/null
grep -F "DRY-RUN no EC2 resources launched" "$TMP/check.out" >/dev/null
grep -F "cache_volume=adl-aws-remote-validation-cache-volume cache_mount=/mnt/adl-cache ssh_tail_enabled=true" "$TMP/check.out" >/dev/null
if grep -F "$account" "$TMP/check.out" >/dev/null; then
  echo "account id leaked in account-check output" >&2
  exit 1
fi
if grep -F "arn:aws:iam" "$TMP/check.out" >/dev/null; then
  echo "arn leaked in account-check output" >&2
  exit 1
fi
if grep -F "AIDAEXAMPLE" "$TMP/check.out" >/dev/null; then
  echo "user id leaked in account-check output" >&2
  exit 1
fi

ADL_FAKE_AWS_REMOTE_ARGS="$TMP/args.txt" \
ADL_AWS_CLI="$fake_bin/aws" \
bash "$SCRIPT" \
  --run \
  --expected-proof "$proof" \
  --bin "$fake_bin/adl-aws-remote-validation" \
  --run-id fixture-run \
  --command "cargo test --manifest-path adl/Cargo.toml provider_communication -- --nocapture" \
  --git-ref origin/main \
  --out "$TMP/summary.json" \
  --artifact-dir "$TMP/artifacts" \
  --instance-type m7a.2xlarge \
  --json >"$TMP/run.out"

grep -F "fixture remote validation passed" "$TMP/run.out" >/dev/null
grep -Fx -- "run" "$TMP/args.txt" >/dev/null
grep -Fx -- "--profile" "$TMP/args.txt" >/dev/null
grep -Fx -- "agent-logic-admin" "$TMP/args.txt" >/dev/null
grep -Fx -- "--region" "$TMP/args.txt" >/dev/null
grep -Fx -- "us-west-2" "$TMP/args.txt" >/dev/null
grep -Fx -- "--issue" "$TMP/args.txt" >/dev/null
grep -Fx -- "4837" "$TMP/args.txt" >/dev/null
grep -Fx -- "--instance-type" "$TMP/args.txt" >/dev/null
grep -Fx -- "m7a.2xlarge" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-name" "$TMP/args.txt" >/dev/null
grep -Fx -- "adl-aws-remote-validation-cache-volume" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-size-gib" "$TMP/args.txt" >/dev/null
grep -Fx -- "100" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-type" "$TMP/args.txt" >/dev/null
grep -Fx -- "gp3" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-iops" "$TMP/args.txt" >/dev/null
grep -Fx -- "3000" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-throughput-mbps" "$TMP/args.txt" >/dev/null
grep -Fx -- "125" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-device-name" "$TMP/args.txt" >/dev/null
grep -Fx -- "/dev/sdf" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-mount-path" "$TMP/args.txt" >/dev/null
grep -Fx -- "/mnt/adl-cache" "$TMP/args.txt" >/dev/null
grep -Fx -- "--ssh-key-name" "$TMP/args.txt" >/dev/null
grep -Fx -- "adl-wp06-spot-ssh-debug-20260704" "$TMP/args.txt" >/dev/null
grep -Fx -- "--ssh-private-key-path" "$TMP/args.txt" >/dev/null
grep -Fx -- "$HOME/.ssh/adl-4603-ssh-debug-20260701.pem" "$TMP/args.txt" >/dev/null
grep -Fx -- "--ssh-user" "$TMP/args.txt" >/dev/null
grep -Fx -- "ec2-user" "$TMP/args.txt" >/dev/null
grep -Fx -- "--json" "$TMP/args.txt" >/dev/null
test -f "$TMP/summary.json"
test -f "$TMP/artifacts/events.jsonl"

if bash "$SCRIPT" --extra-arg --profile >"$TMP/extra.out" 2>"$TMP/extra.err"; then
  echo "expected --extra-arg to be rejected" >&2
  exit 1
fi
grep -F "unknown argument: --extra-arg" "$TMP/extra.err" >/dev/null

bad_proof="$TMP/bad-proof.json"
python3 - "$bad_proof" <<'PY'
import json
import sys
with open(sys.argv[1], "w", encoding="utf-8") as handle:
    json.dump({"account_identity": {"account_id_sha256": "not-the-right-hash"}}, handle)
PY

if ADL_AWS_CLI="$fake_bin/aws" bash "$SCRIPT" --check-account --expected-proof "$bad_proof" >"$TMP/bad.out" 2>"$TMP/bad.err"; then
  echo "expected account mismatch to fail closed" >&2
  exit 1
fi
grep -F "AWS profile account does not match retained Agent Logic proof" "$TMP/bad.err" >/dev/null

[ -f "$SETUP_SCRIPT" ]
[ -x "$SETUP_SCRIPT" ]
[ -f "$WORKFLOW" ]
grep -F -- "AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN" "$WORKFLOW" >/dev/null
grep -F -- "aws-actions/configure-aws-credentials@7474bc4690e29a8392af63c5b98e7449536d5c3a" "$WORKFLOW" >/dev/null
grep -F -- "group: aws-spot-remote-validation-ebs-cache" "$WORKFLOW" >/dev/null
grep -F -- "github.sha" "$WORKFLOW" >/dev/null
grep -F -- "git_ref must be a branch, tag, or SHA; HEAD is ambiguous" "$WORKFLOW" >/dev/null
grep -F -- "--profile env" "$WORKFLOW" >/dev/null
grep -F -- "--check-account" "$WORKFLOW" >/dev/null
grep -F -- "--json" "$WORKFLOW" >/dev/null
grep -F -- "Redact Spot artifact identities" "$WORKFLOW" >/dev/null
grep -F -- "Build Spot remote validation binary" "$WORKFLOW" >/dev/null
grep -F -- "adl-aws-remote-validation-cache-volume:/mnt/adl-cache" "$WORKFLOW" >/dev/null
grep -F -- "ssh tail" "$WORKFLOW" >/dev/null
grep -F -- "if-no-files-found: warn" "$WORKFLOW" >/dev/null
grep -F -- "ec2:RunInstances" "$SETUP_SCRIPT" >/dev/null
grep -F -- "ec2:CreateVolume" "$SETUP_SCRIPT" >/dev/null
grep -F -- "ssm:SendCommand" "$SETUP_SCRIPT" >/dev/null
grep -F -- "iam:PassRole" "$SETUP_SCRIPT" >/dev/null
grep -F -- "AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN" "$SETUP_SCRIPT" >/dev/null
grep -F -- "repo:{repo}:ref:refs/heads/main" "$SETUP_SCRIPT" >/dev/null
grep -F -- "repo:{repo}:ref:refs/heads/codex/*" "$SETUP_SCRIPT" >/dev/null

echo "PASS test_run_aws_spot_remote_validation_lane"
