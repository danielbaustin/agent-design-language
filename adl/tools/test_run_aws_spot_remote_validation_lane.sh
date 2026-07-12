#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_remote_validation_lane.sh"
SETUP_SCRIPT="$ROOT/adl/tools/setup_aws_spot_remote_validation_github_resources.sh"
WORKFLOW="$ROOT/.github/workflows/aws-spot-remote-validation.yaml"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

account="123456789012"
builder_digest="sha256:$(printf 'a%.0s' {1..64})"
builder_image="$account.dkr.ecr.us-west-2.amazonaws.com/adl-builder@$builder_digest"
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
    json.dump({
        "account_identity": {"account_id_sha256": account_hash},
        "cache_volume": {"volume_id": "vol-0123456789abcdef0"},
        "launch_surface": {"subnet_id": "subnet-0123456789abcdef0"},
    }, handle)
PY

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
test_ssh_key="$TMP/spot-key"
ssh-keygen -q -t ed25519 -N '' -f "$test_ssh_key"
chmod 600 "$test_ssh_key"

cat >"$fake_bin/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == "sts get-caller-identity" ]]; then
  cat <<'JSON'
{
  "Account": "123456789012",
  "Arn": "arn:aws:iam::123456789012:user/test-user",
  "UserId": "AIDAEXAMPLE"
}
JSON
elif [[ "$1 $2" == "ec2 describe-volumes" ]]; then
  case "$*" in
    *'Volumes[0].State'*) echo available ;;
    *'Volumes[0].Tags'*) echo adl-aws-remote-validation-cache-volume ;;
    *'Volumes[0].AvailabilityZone'*) echo us-west-2a ;;
    *'Volumes[0].Size'*) echo 100 ;;
    *'Volumes[0].VolumeType'*) echo gp3 ;;
    *'Volumes[0].Iops'*) echo 3000 ;;
    *'Volumes[0].Throughput'*) echo 125 ;;
    *'length(Volumes)'*) echo 1 ;;
    *) echo unexpected-volume-query >&2; exit 1 ;;
  esac
elif [[ "$1 $2" == "ec2 describe-subnets" ]]; then
  echo us-west-2a
elif [[ "$1 $2" == "ssm get-parameter" ]]; then
  echo ami-0123456789abcdef0
else
  echo "unexpected aws invocation: $*" >&2
  exit 1
fi
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
status="${ADL_FAKE_AWS_REMOTE_STATUS:-passed}"
cat >"$out" <<JSON
{
  "schema_version":"adl.aws_remote_validation_run.v1",
  "status":"$status",
  "launch":{"purchase_option":"spot","instance_id":"i-0123456789abcdef0"},
  "cache_volume":{"volume_id":"vol-0123456789abcdef0","created":false,"attachment_state":"attached","mount_path":"/mnt/adl-cache"},
  "cleanup":{"termination_attempted":true,"final_instance_state":"terminated","termination_error":null},
  "launch_surface":{"ssh_debug_enabled":true,"vpc_id":"vpc-0123456789abcdef0","subnet_id":"subnet-0123456789abcdef0","security_group_id":"sg-0123456789abcdef0"},
  "timings":{"total_seconds":120,"launch_seconds":20,"ssm_ready_seconds":10,"remote_command_seconds":80,"teardown_seconds":10},
  "remote_summary":{"builder_proof":{"builder_image_immutable":true,"builder_image_digest_sha256":"${ADL_FAKE_EXPECTED_IMAGE_DIGEST_HASH:?}","toolchain_verified":true,"source_commit_verified":true,"source_commit":"${ADL_FAKE_EXPECTED_SOURCE:?}","cache_mount_verified":true,"cache_writable":true,"host_validation_tools_installed":false,"builder_image_architecture":"amd64","cache_target_preexisting_entries":5,"cache_target_preexisting_bytes":4096,"cache_free_bytes":90000000000,"validation_seconds":73}}
}
JSON
cat >"$artifact_dir/command-status.log" <<'LOG'
2026-07-09T00:00:10Z status=ssh_debug_ready instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
2026-07-09T00:00:11Z status=ssh_tail_started instance_id=i-0123456789abcdef0 public_ip=192.0.2.10
LOG
cat >"$artifact_dir/command-stdout.log" <<JSON
ADL_AWS_REMOTE_SUMMARY_BEGIN
{"builder_proof":{"builder_image_immutable":true,"builder_image_digest_sha256":"${ADL_FAKE_EXPECTED_IMAGE_DIGEST_HASH}","toolchain_verified":true,"source_commit_verified":true,"source_commit":"${ADL_FAKE_EXPECTED_SOURCE}","cache_mount_verified":true,"cache_writable":true,"host_validation_tools_installed":false,"builder_image_architecture":"amd64","cache_target_preexisting_entries":5,"cache_target_preexisting_bytes":4096,"cache_free_bytes":90000000000,"validation_seconds":73}}
ADL_AWS_REMOTE_SUMMARY_END
JSON
if [[ "$status" == "resumed_after_interruption" ]]; then
  cat >"$artifact_dir/resume-state.json" <<'JSON'
{
  "schema_version": "adl.aws_remote_validation_resume_state.v1",
  "issue": 4974,
  "run_id": "fixture-run-resumed",
  "repo_url": "https://github.com/danielbaustin/agent-design-language.git",
  "git_ref": "origin/main",
  "command": "cargo test --manifest-path adl/Cargo.toml provider_communication -- --nocapture",
  "output_summary_ref": "summary.json",
  "artifact_root_ref": ".",
  "started_at": "2026-07-09T00:00:00Z",
  "updated_at": "2026-07-09T00:01:00Z",
  "max_spot_retries": 2,
  "interrupted_attempts": 1,
  "next_action": "complete",
  "final_status": "resumed_after_interruption",
  "attempts": [
    {
      "attempt_index": 0,
      "started_at": "2026-07-09T00:00:00Z",
      "finished_at": "2026-07-09T00:00:30Z",
      "summary_path": "attempt-0/summary.json",
      "artifact_dir": "attempt-0",
      "status": "interrupted_by_aws",
      "failure_reason": "AWS Spot interruption notice received",
      "launch_purchase_option": "spot",
      "launch_instance_type": "m7a.2xlarge",
      "launch_initial_state": "pending",
      "launch_instance_id_sha256": "0123456789abcdef",
      "provider_interruption_confirmed": true,
      "retryable": true,
      "next_action": "retry_after_interruption_1"
    },
    {
      "attempt_index": 1,
      "started_at": "2026-07-09T00:00:31Z",
      "finished_at": "2026-07-09T00:01:00Z",
      "summary_path": "attempt-1/summary.json",
      "artifact_dir": "attempt-1",
      "status": "passed",
      "failure_reason": null,
      "launch_purchase_option": "spot",
      "launch_instance_type": "m7a.2xlarge",
      "launch_initial_state": "pending",
      "launch_instance_id_sha256": "fedcba9876543210",
      "provider_interruption_confirmed": false,
      "retryable": false,
      "next_action": "finalize"
    }
  ]
}
JSON
fi
cat >"$artifact_dir/events.jsonl" <<'JSONL'
{"event":"fixture"}
JSONL
echo "fixture remote validation passed"
EOF

cat >"$fake_bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
method="GET"
out=""
data=""
url=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -X)
      method="${2:-}"
      shift 2
      ;;
    -o)
      out="${2:-}"
      shift 2
      ;;
    --data-binary)
      data="${2:-}"
      shift 2
      ;;
    -H|-w)
      shift 2
      ;;
    --config)
      shift 2
      ;;
    -sS)
      shift
      ;;
    http://*|https://*)
      url="$1"
      shift
      ;;
    *)
      shift
      ;;
  esac
done
mkdir -p "$(dirname "${ADL_FAKE_GITHUB_API_LOG:?}")"
printf 'method=%s url=%s data=%s\n' "$method" "$url" "$data" >>"$ADL_FAKE_GITHUB_API_LOG"
if [[ -n "$out" ]]; then
  printf '{}\n' >"$out"
fi
case "$method" in
  GET)
    if [[ "$url" == *"/actions/variables/ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR" ]]; then
      printf '200'
    else
      printf '404'
    fi
    ;;
  POST)
    printf '201'
    ;;
  PATCH)
    printf '204'
    ;;
  *)
    printf '500'
    ;;
esac
EOF
chmod +x "$fake_bin/aws" "$fake_bin/adl-aws-remote-validation" "$fake_bin/curl"

ADL_FAKE_GITHUB_API_LOG="$TMP/github-api.log" \
ADL_GITHUB_API_BIN="$fake_bin/curl" \
ADL_GITHUB_API_URL="https://api.github.test" \
GITHUB_TOKEN="test-token" \
bash "$SETUP_SCRIPT" \
  --apply \
  --github-vars-only \
  --region us-west-2 \
  --repo danielbaustin/agent-design-language \
  --ssh-allowed-cidr 203.0.113.10/32 \
  --artifact-dir "$TMP/github-setup" >"$TMP/github-setup.out"

grep -F "PASS github_repository_variable name=AWS_SPOT_REMOTE_VALIDATION_REGION configured=true action=POST" "$TMP/github-setup.out" >/dev/null
grep -F "PASS github_repository_variable name=ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR configured=true action=PATCH" "$TMP/github-setup.out" >/dev/null
grep -F "url=https://api.github.test/repos/danielbaustin/agent-design-language/actions/variables/AWS_SPOT_REMOTE_VALIDATION_REGION" "$TMP/github-api.log" >/dev/null
grep -F "url=https://api.github.test/repos/danielbaustin/agent-design-language/actions/variables/ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR" "$TMP/github-api.log" >/dev/null
grep -F "url=https://api.github.test/repos/danielbaustin/agent-design-language/actions/variables data=@$TMP/github-setup/github-variable-AWS_SPOT_REMOTE_VALIDATION_REGION.json" "$TMP/github-api.log" >/dev/null
grep -F "method=PATCH url=https://api.github.test/repos/danielbaustin/agent-design-language/actions/variables/ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR data=@$TMP/github-setup/github-variable-ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR.json" "$TMP/github-api.log" >/dev/null
python3 - "$TMP/github-setup/github-variable-ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR.json" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
assert payload == {
    "name": "ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR",
    "value": "203.0.113.10/32",
}
PY

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

ADL_AWS_CLI="$fake_bin/aws" \
bash "$SCRIPT" preflight \
  --expected-proof "$proof" \
  --builder-image "$builder_image" \
  --estimated-hourly-cost-usd 0.15 \
  --ssh-private-key-path "$test_ssh_key" \
  --git-ref origin/main >"$TMP/preflight.out"
python3 - "$TMP/preflight.out" <<'PY'
import json
import sys
text = open(sys.argv[1], encoding="utf-8").read()
payload = json.loads(text[text.index("{"):])
assert payload["status"] == "ready"
assert payload["account_matches_retained_proof"] is True
assert payload["builder_image_immutable"] is True
assert payload["retained_cache_available"] is True
assert payload["ssh_recovery_configured"] is True
assert payload["aws_resources_created"] is False
PY

ADL_FAKE_AWS_REMOTE_ARGS="$TMP/args.txt" \
ADL_FAKE_EXPECTED_SOURCE="$(git -C "$ROOT" rev-parse origin/main)" \
ADL_FAKE_EXPECTED_IMAGE_DIGEST_HASH="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$builder_digest")" \
ADL_AWS_CLI="$fake_bin/aws" \
bash "$SCRIPT" \
  --run \
  --expected-proof "$proof" \
  --bin "$fake_bin/adl-aws-remote-validation" \
  --run-id fixture-run \
  --builder-image "$builder_image" \
  --estimated-hourly-cost-usd 0.15 \
  --ssh-private-key-path "$test_ssh_key" \
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
grep -Fx -- "5191" "$TMP/args.txt" >/dev/null
grep -Fx -- "--instance-type" "$TMP/args.txt" >/dev/null
grep -Fx -- "m7a.2xlarge" "$TMP/args.txt" >/dev/null
grep -Fx -- "--cache-volume-id" "$TMP/args.txt" >/dev/null
grep -Fx -- "vol-0123456789abcdef0" "$TMP/args.txt" >/dev/null
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
grep -Fx -- "$test_ssh_key" "$TMP/args.txt" >/dev/null
grep -Fx -- "--ssh-user" "$TMP/args.txt" >/dev/null
grep -Fx -- "ec2-user" "$TMP/args.txt" >/dev/null
grep -Fx -- "--json" "$TMP/args.txt" >/dev/null
python3 - "$TMP/args.txt" "$builder_image" <<'PY'
import shlex
import sys

args = open(sys.argv[1], encoding="utf-8").read().splitlines()
command = args[args.index("--command") + 1]
parts = shlex.split(command)
assert parts[:2] == ["bash", "adl/tools/run_aws_spot_builder_image_validation.sh"]
assert parts[parts.index("--image") + 1] == sys.argv[2]
assert len(parts[parts.index("--expected-ref") + 1]) == 40
assert parts[parts.index("--expected-architecture") + 1] == "x86_64"
assert parts[parts.index("--min-cache-free-gib") + 1] == "10"
assert parts[parts.index("--command") + 1].startswith("cargo test")
PY
test -f "$TMP/summary.json"
test -f "$TMP/artifacts/events.jsonl"
test -f "$TMP/artifacts/wrapper-final-summary.json"
python3 - "$TMP/artifacts/wrapper-final-summary.json" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
assert payload["schema"] == "adl.aws_spot_remote_validation_wrapper_summary.v2"
assert payload["status"] == "passed"
assert payload["runner_exit_code"] == 0
assert payload["self_verification"]["passed"] is True
assert payload["self_verification"]["host_validation_tools_installed"] is False
assert payload["resumed_after_interruption"] is False
PY

ADL_FAKE_AWS_REMOTE_ARGS="$TMP/launch-args.txt" \
ADL_FAKE_EXPECTED_SOURCE="$(git -C "$ROOT" rev-parse origin/main)" \
ADL_FAKE_EXPECTED_IMAGE_DIGEST_HASH="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$builder_digest")" \
ADL_AWS_CLI="$fake_bin/aws" \
bash "$SCRIPT" launch \
  --expected-proof "$proof" \
  --bin "$fake_bin/adl-aws-remote-validation" \
  --run-id fixture-launch \
  --builder-image "$builder_image" \
  --estimated-hourly-cost-usd 0.15 \
  --ssh-private-key-path "$test_ssh_key" \
  --command "cargo nextest run --workspace" \
  --git-ref origin/main \
  --out "$TMP/launch-summary.json" \
  --artifact-dir "$TMP/launch-artifacts" \
  --instance-type m7a.2xlarge \
  --json >"$TMP/launch.out"
grep -F 'status=launched run_id=fixture-launch' "$TMP/launch.out" >/dev/null
for _ in $(seq 1 50); do
  [[ -f "$TMP/launch-artifacts/wrapper-final-summary.json" ]] && break
  sleep 0.1
done
test -f "$TMP/launch-artifacts/wrapper-final-summary.json"
bash "$SCRIPT" status \
  --run-id fixture-launch \
  --out "$TMP/launch-summary.json" \
  --artifact-dir "$TMP/launch-artifacts" >"$TMP/launch-status.out"
grep -F '"status": "passed"' "$TMP/launch-status.out" >/dev/null

ADL_FAKE_AWS_REMOTE_STATUS=resumed_after_interruption \
ADL_FAKE_AWS_REMOTE_ARGS="$TMP/resume-args.txt" \
ADL_FAKE_EXPECTED_SOURCE="$(git -C "$ROOT" rev-parse origin/main)" \
ADL_FAKE_EXPECTED_IMAGE_DIGEST_HASH="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(sys.argv[1].encode()).hexdigest())' "$builder_digest")" \
ADL_AWS_CLI="$fake_bin/aws" \
bash "$SCRIPT" \
  --run \
  --expected-proof "$proof" \
  --bin "$fake_bin/adl-aws-remote-validation" \
  --run-id fixture-run-resumed \
  --builder-image "$builder_image" \
  --estimated-hourly-cost-usd 0.15 \
  --ssh-private-key-path "$test_ssh_key" \
  --command "cargo test --manifest-path adl/Cargo.toml provider_communication -- --nocapture" \
  --git-ref origin/main \
  --out "$TMP/resumed-summary.json" \
  --artifact-dir "$TMP/resumed-artifacts" \
  --instance-type m7a.2xlarge \
  --json >"$TMP/resumed-run.out" 2>"$TMP/resumed-run.err"

python3 - "$TMP/resumed-artifacts/wrapper-final-summary.json" "$TMP/resumed-artifacts/resume-state.json" <<'PY'
import json
import sys
wrapper = json.load(open(sys.argv[1], encoding="utf-8"))
resume = json.load(open(sys.argv[2], encoding="utf-8"))
assert wrapper["status"] == "passed"
assert wrapper["runner_exit_code"] == 0
assert wrapper["attempt_count"] == 2
assert wrapper["interrupted_attempt_count"] == 1
assert wrapper["resumed_after_interruption"] is True
assert resume["attempts"][0]["status"] == "interrupted_by_aws"
assert "launch_instance_id" not in resume["attempts"][0]
assert resume["attempts"][0]["summary_path"] == "attempt-0/summary.json"
text = json.dumps(resume)
assert "123456789012" not in text
assert "arn:aws:" not in text
PY

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
grep -F -- "Verify Spot artifact redaction" "$WORKFLOW" >/dev/null
grep -F -- "AWS_SPOT_REMOTE_VALIDATION_SSH_PRIVATE_KEY_B64" "$WORKFLOW" >/dev/null
grep -F -- "ssh-keygen -y -P ''" "$WORKFLOW" >/dev/null
grep -F -- "include-hidden-files: false" "$WORKFLOW" >/dev/null
grep -F -- "Build Spot remote validation binary" "$WORKFLOW" >/dev/null
grep -F -- "tools/aws_remote_validation/Cargo.toml" "$WORKFLOW" >/dev/null
grep -F -- "adl-aws-remote-validation-cache-volume:/mnt/adl-cache" "$WORKFLOW" >/dev/null
grep -F -- "ssh tail" "$WORKFLOW" >/dev/null
grep -F -- "ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR" "$WORKFLOW" >/dev/null
grep -F -- "if-no-files-found: warn" "$WORKFLOW" >/dev/null
grep -F -- "ec2:RunInstances" "$SETUP_SCRIPT" >/dev/null
if grep -F -- '"ec2:CreateVolume"' "$SETUP_SCRIPT" >/dev/null; then
  echo "GitHub Spot role must not create retained cache volumes" >&2
  exit 1
fi
if grep -F -- '"ec2:DeleteVolume"' "$SETUP_SCRIPT" >/dev/null; then
  echo "GitHub Spot role must not delete retained cache volumes" >&2
  exit 1
fi
grep -F -- "ssm:SendCommand" "$SETUP_SCRIPT" >/dev/null
grep -F -- "iam:PassRole" "$SETUP_SCRIPT" >/dev/null
grep -F -- "AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN" "$SETUP_SCRIPT" >/dev/null
grep -F -- "AWS_SPOT_REMOTE_VALIDATION_REGION" "$SETUP_SCRIPT" >/dev/null
grep -F -- "ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR" "$SETUP_SCRIPT" >/dev/null
grep -F -- "repo:{repo}:ref:refs/heads/main" "$SETUP_SCRIPT" >/dev/null
grep -F -- "repo:{repo}:ref:refs/heads/codex/*" "$SETUP_SCRIPT" >/dev/null
grep -F -- "AdlAwsRemoteValidationBuilderImageEcrRead" "$ROOT/adl/src/aws_remote_validation.rs" >/dev/null
grep -F -- "ecr:GetAuthorizationToken" "$ROOT/adl/src/aws_remote_validation.rs" >/dev/null
grep -F -- "repository/adl-builder" "$ROOT/adl/src/aws_remote_validation.rs" >/dev/null

echo "PASS test_run_aws_spot_remote_validation_lane"
