#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILE="agent-logic-admin"
REGION="us-west-2"
PROJECT="adl-builder-image-build"
REPOSITORY="adl-builder"
TAG=""
GIT_REF=""
EXPECTED_PROOF="$ROOT/docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json"
RUN=false

usage() {
  cat <<'USAGE'
Usage:
  publish_adl_builder_image_codebuild.sh --tag <tag> --git-ref <pushed-ref> [--run]

Creates or updates the purpose-specific CodeBuild image publisher. Without
  --run it prints a redacted plan. The publisher builds the ADL Dockerfile once,
verifies required tools inside the image, pushes the candidate tag to Agent
Logic ECR, and reports its immutable digest.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --tag) TAG="${2:-}"; shift 2 ;;
    --git-ref) GIT_REF="${2:-}"; shift 2 ;;
    --profile) PROFILE="${2:-}"; shift 2 ;;
    --region) REGION="${2:-}"; shift 2 ;;
    --project-name) PROJECT="${2:-}"; shift 2 ;;
    --expected-proof) EXPECTED_PROOF="${2:-}"; shift 2 ;;
    --run) RUN=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "publish_adl_builder_image_codebuild: unknown argument: $1" >&2; exit 2 ;;
  esac
done

[[ "$TAG" =~ ^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$ ]] || {
  echo "publish_adl_builder_image_codebuild: --tag is required and must be an ECR tag" >&2
  exit 2
}
[[ -n "$GIT_REF" && "$GIT_REF" != HEAD ]] || {
  echo "publish_adl_builder_image_codebuild: --git-ref must be a pushed branch, tag, or commit" >&2
  exit 2
}

identity="$(aws --profile "$PROFILE" sts get-caller-identity --output json)"
account="$(python3 -c 'import json,sys; print(json.loads(sys.argv[1])["Account"])' "$identity")"
account_hash="$(printf '%s' "$account" | shasum -a 256 | awk '{print $1}')"
expected_account_hash="$(python3 - "$EXPECTED_PROOF" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
try:
    proof = json.loads(path.read_text(encoding="utf-8"))
except (OSError, json.JSONDecodeError) as exc:
    raise SystemExit(f"publish_adl_builder_image_codebuild: unable to read retained proof: {exc}")
identity = proof.get("account_identity") or {}
account_hash = identity.get("account_id_sha256")
if not isinstance(account_hash, str) or len(account_hash) != 64:
    raise SystemExit("publish_adl_builder_image_codebuild: retained proof is missing account identity hash")
print(account_hash)
PY
)"
[[ "$account_hash" == "$expected_account_hash" ]] || {
  echo "publish_adl_builder_image_codebuild: AWS profile account does not match retained Agent Logic proof" >&2
  exit 2
}
registry="$account.dkr.ecr.$REGION.amazonaws.com"
image="$registry/$REPOSITORY:$TAG"
service_role="$(aws --profile "$PROFILE" --region "$REGION" codebuild batch-get-projects --names adl-codefriend-build --query 'projects[0].serviceRole' --output text)"
[[ "$service_role" == arn:aws:iam::*:role/* ]] || {
  echo "publish_adl_builder_image_codebuild: unable to resolve CodeBuild service role" >&2
  exit 2
}

echo "PASS account_profile_resolved profile=$PROFILE account_matches_retained_proof=true"
echo "builder_image_publish_plan project=$PROJECT region=$REGION repository=$REPOSITORY tag=$TAG git_ref=$GIT_REF compute=BUILD_GENERAL1_XLARGE"
if [[ "$RUN" != true ]]; then
  echo "DRY-RUN no CodeBuild project changed and no image built; pass --run to execute"
  exit 0
fi

tmp="$(mktemp -d "${TMPDIR:-/tmp}/adl-builder-codebuild.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT
project_json="$tmp/project.json"
policy_json="$tmp/policy.json"

python3 - <<'PY' "$project_json" "$PROJECT" "$service_role" "$image" "$REGION"
import json
import sys
from pathlib import Path

path, project, role, image, region = sys.argv[1:6]
buildspec = r'''version: 0.2
phases:
  pre_build:
    commands:
      - registry="${ADL_BUILDER_IMAGE_URI%%/*}"
      - aws ecr get-login-password --region "$AWS_DEFAULT_REGION" | docker login --username AWS --password-stdin "$registry"
  build:
    commands:
      - docker build --platform linux/amd64 -f adl/docker/adl-builder/Dockerfile -t "$ADL_BUILDER_IMAGE_URI" .
      - docker run --rm "$ADL_BUILDER_IMAGE_URI" 'cargo nextest --version && cargo llvm-cov --version && rustup component list --installed | grep -E "^llvm-tools-" && sccache --version && ld.lld --version && aws --version'
      - docker push "$ADL_BUILDER_IMAGE_URI"
  post_build:
    commands:
      - aws ecr describe-images --repository-name adl-builder --image-ids imageTag="${ADL_BUILDER_IMAGE_URI##*:}" --query 'imageDetails[0].imageDigest' --output text
'''
payload = {
    "name": project,
    "description": "Build, verify, and publish the immutable ADL validation image",
    "source": {
        "type": "GITHUB",
        "location": "https://github.com/danielbaustin/agent-design-language.git",
        "gitCloneDepth": 1,
        "buildspec": buildspec,
    },
    "artifacts": {"type": "NO_ARTIFACTS"},
    "environment": {
        "type": "LINUX_CONTAINER",
        "image": "aws/codebuild/standard:7.0",
        "computeType": "BUILD_GENERAL1_XLARGE",
        "privilegedMode": True,
        "environmentVariables": [
            {"name": "ADL_BUILDER_IMAGE_URI", "value": image, "type": "PLAINTEXT"},
            {"name": "AWS_DEFAULT_REGION", "value": region, "type": "PLAINTEXT"},
        ],
    },
    "serviceRole": role,
    "logsConfig": {
        "cloudWatchLogs": {
            "status": "ENABLED",
            "groupName": "/aws/codebuild/adl-codefriend-build",
            "streamName": "adl-builder-image-build",
        }
    },
    "timeoutInMinutes": 60,
}
Path(path).write_text(json.dumps(payload, indent=2) + "\n")
PY

cat >"$policy_json" <<JSON
{
  "Version": "2012-10-17",
  "Statement": [
    {"Effect":"Allow","Action":["ecr:GetAuthorizationToken"],"Resource":"*"},
    {"Effect":"Allow","Action":["ecr:BatchCheckLayerAvailability","ecr:BatchGetImage","ecr:CompleteLayerUpload","ecr:DescribeImages","ecr:GetDownloadUrlForLayer","ecr:InitiateLayerUpload","ecr:PutImage","ecr:UploadLayerPart"],"Resource":"arn:aws:ecr:$REGION:$account:repository/$REPOSITORY"}
  ]
}
JSON

role_name="${service_role##*/}"
aws --profile "$PROFILE" iam put-role-policy --role-name "$role_name" --policy-name adl-builder-image-build --policy-document "file://$policy_json"
if aws --profile "$PROFILE" --region "$REGION" codebuild batch-get-projects --names "$PROJECT" --query 'projects[0].name' --output text | grep -Fx "$PROJECT" >/dev/null; then
  aws --profile "$PROFILE" --region "$REGION" codebuild update-project --cli-input-json "file://$project_json" >/dev/null
else
  aws --profile "$PROFILE" --region "$REGION" codebuild create-project --cli-input-json "file://$project_json" >/dev/null
fi

build_id="$(aws --profile "$PROFILE" --region "$REGION" codebuild start-build --project-name "$PROJECT" --source-version "$GIT_REF" --query 'build.id' --output text)"
echo "builder_image_build_started=true"
while true; do
  status="$(aws --profile "$PROFILE" --region "$REGION" codebuild batch-get-builds --ids "$build_id" --query 'builds[0].buildStatus' --output text)"
  echo "builder_image_build_status=$status"
  case "$status" in
    SUCCEEDED) break ;;
    FAILED|FAULT|STOPPED|TIMED_OUT) exit 1 ;;
  esac
  sleep 15
done
digest="$(aws --profile "$PROFILE" --region "$REGION" ecr describe-images --repository-name "$REPOSITORY" --image-ids imageTag="$TAG" --query 'imageDetails[0].imageDigest' --output text)"
[[ "$digest" =~ ^sha256:[0-9a-f]{64}$ ]]
echo "PASS builder_image_published tag=$TAG immutable_digest_verified=true"
