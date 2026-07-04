#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  setup_aws_codefriend_build_resources.sh --apply [options]

Options:
  --apply                         Create or update AWS resources.
  --check                         Report whether resources exist without mutating AWS.
  --profile <profile>             AWS CLI profile. Default: agent-logic-admin.
  --region <region>               AWS region. Default: us-west-2.
  --project-name <name>           CodeBuild project. Default: adl-codefriend-build.
  --repo <owner/name>             GitHub repository. Default: danielbaustin/agent-design-language.
  --source-location <url>         CodeBuild GitHub source URL.
  --compute-type <type>           CodeBuild compute type. Default: BUILD_GENERAL1_LARGE.
  --github-role-name <name>       OIDC role for GitHub Actions.
  --service-role-name <name>      CodeBuild service role.
  --artifact-dir <path>           Local setup artifact directory.
  -h, --help                      Show this help.

Creates the minimum Agent Logic AWS resources for the CodeFriend CodeBuild lane:
the CodeBuild service role, the GitHub Actions OIDC start-build role, and the
CodeBuild project. Output intentionally avoids account ids, ARNs, and secrets.
USAGE
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 2
}

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
AWS_CLI="${ADL_AWS_CLI:-aws}"
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
PROJECT_NAME="${ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT:-adl-codefriend-build}"
REPO="danielbaustin/agent-design-language"
SOURCE_LOCATION="https://github.com/danielbaustin/agent-design-language.git"
COMPUTE_TYPE="${ADL_AWS_CODEFRIEND_COMPUTE_TYPE:-BUILD_GENERAL1_LARGE}"
GITHUB_ROLE_NAME="adl-codefriend-github-actions-build-role"
SERVICE_ROLE_NAME="adl-codefriend-codebuild-service-role"
ARTIFACT_DIR=".adl/tmp/aws-codefriend-build-resource-setup"
MODE="check"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --apply)
      MODE="apply"
      shift
      ;;
    --check)
      MODE="check"
      shift
      ;;
    --profile)
      [ "$#" -ge 2 ] || die "--profile requires a value"
      PROFILE="$2"
      shift 2
      ;;
    --region)
      [ "$#" -ge 2 ] || die "--region requires a value"
      REGION="$2"
      shift 2
      ;;
    --project-name)
      [ "$#" -ge 2 ] || die "--project-name requires a value"
      PROJECT_NAME="$2"
      shift 2
      ;;
    --repo)
      [ "$#" -ge 2 ] || die "--repo requires owner/name"
      REPO="$2"
      shift 2
      ;;
    --source-location)
      [ "$#" -ge 2 ] || die "--source-location requires a value"
      SOURCE_LOCATION="$2"
      shift 2
      ;;
    --compute-type)
      [ "$#" -ge 2 ] || die "--compute-type requires a value"
      COMPUTE_TYPE="$2"
      shift 2
      ;;
    --github-role-name)
      [ "$#" -ge 2 ] || die "--github-role-name requires a value"
      GITHUB_ROLE_NAME="$2"
      shift 2
      ;;
    --service-role-name)
      [ "$#" -ge 2 ] || die "--service-role-name requires a value"
      SERVICE_ROLE_NAME="$2"
      shift 2
      ;;
    --artifact-dir)
      [ "$#" -ge 2 ] || die "--artifact-dir requires a value"
      ARTIFACT_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "unknown argument: $1"
      ;;
  esac
done

case "$REPO" in
  */*) ;;
  *) die "--repo must be owner/name" ;;
esac

mkdir -p "$ARTIFACT_DIR"

aws_args=(--profile "$PROFILE")

identity_json="$("$AWS_CLI" sts get-caller-identity "${aws_args[@]}" --output json)"
account_hash="$(
  python3 - <<'PY' "$identity_json"
import hashlib
import json
import sys

identity = json.loads(sys.argv[1])
account = str(identity.get("Account") or "")
print(hashlib.sha256(account.encode("utf-8")).hexdigest() if account else "")
PY
)"
[ -n "$account_hash" ] || die "AWS profile did not resolve to an account"

account_id="$(
  python3 - <<'PY' "$identity_json"
import json
import sys

print(json.loads(sys.argv[1]).get("Account", ""))
PY
)"
[ -n "$account_id" ] || die "AWS profile did not return an account id"

printf 'PASS account_profile_resolved profile=%s account_hash_available=true\n' "$PROFILE"

provider_arn=""
for arn in $("$AWS_CLI" iam list-open-id-connect-providers "${aws_args[@]}" --query 'OpenIDConnectProviderList[].Arn' --output text); do
  url="$("$AWS_CLI" iam get-open-id-connect-provider "${aws_args[@]}" --open-id-connect-provider-arn "$arn" --query 'Url' --output text 2>/dev/null || true)"
  if [ "$url" = "token.actions.githubusercontent.com" ]; then
    provider_arn="$arn"
    break
  fi
done

if [ -z "$provider_arn" ] && [ "$MODE" = "apply" ]; then
  provider_arn="$("$AWS_CLI" iam create-open-id-connect-provider \
    "${aws_args[@]}" \
    --url https://token.actions.githubusercontent.com \
    --client-id-list sts.amazonaws.com \
    --thumbprint-list 6938fd4d98bab03faadb97b34396831e3780aea1 \
    --query 'OpenIDConnectProviderArn' \
    --output text)"
fi

provider_exists=false
[ -n "$provider_arn" ] && provider_exists=true
printf 'aws_codefriend_oidc_provider_exists=%s\n' "$provider_exists"

project_found="$("$AWS_CLI" codebuild batch-get-projects \
  "${aws_args[@]}" \
  --region "$REGION" \
  --names "$PROJECT_NAME" \
  --query 'length(projects)' \
  --output text)"

service_role_exists=false
if "$AWS_CLI" iam get-role "${aws_args[@]}" --role-name "$SERVICE_ROLE_NAME" --query 'Role.RoleName' --output text >/dev/null 2>&1; then
  service_role_exists=true
fi

github_role_exists=false
if "$AWS_CLI" iam get-role "${aws_args[@]}" --role-name "$GITHUB_ROLE_NAME" --query 'Role.RoleName' --output text >/dev/null 2>&1; then
  github_role_exists=true
fi

printf 'aws_codefriend_service_role_exists=%s\n' "$service_role_exists"
printf 'aws_codefriend_github_role_exists=%s\n' "$github_role_exists"
printf 'aws_codefriend_codebuild_project_exists=%s\n' "$([ "$project_found" = "1" ] && printf true || printf false)"

if [ "$MODE" != "apply" ]; then
  printf 'DRY-RUN no AWS resources changed; pass --apply to create or update\n'
  exit 0
fi

[ -n "$provider_arn" ] || die "GitHub Actions OIDC provider is unavailable"

service_trust="$ARTIFACT_DIR/codebuild-service-trust.json"
service_policy="$ARTIFACT_DIR/codebuild-service-policy.json"
github_trust="$ARTIFACT_DIR/github-actions-trust.json"
github_policy="$ARTIFACT_DIR/github-actions-codebuild-policy.json"
project_json="$ARTIFACT_DIR/codebuild-project.json"

python3 - <<'PY' "$service_trust" "$service_policy" "$github_trust" "$github_policy" "$project_json" "$account_id" "$provider_arn" "$REPO" "$REGION" "$PROJECT_NAME" "$SOURCE_LOCATION" "$SERVICE_ROLE_NAME" "$COMPUTE_TYPE"
import json
import sys
from pathlib import Path

(
    service_trust_path,
    service_policy_path,
    github_trust_path,
    github_policy_path,
    project_path,
    account_id,
    provider_arn,
    repo,
    region,
    project_name,
    source_location,
    service_role_name,
    compute_type,
) = sys.argv[1:14]

def write(path, payload):
    Path(path).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")

write(service_trust_path, {
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Allow",
        "Principal": {"Service": "codebuild.amazonaws.com"},
        "Action": "sts:AssumeRole",
    }],
})

write(service_policy_path, {
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Allow",
        "Action": [
            "logs:CreateLogGroup",
            "logs:CreateLogStream",
            "logs:PutLogEvents",
        ],
        "Resource": [
            f"arn:aws:logs:{region}:{account_id}:log-group:/aws/codebuild/{project_name}",
            f"arn:aws:logs:{region}:{account_id}:log-group:/aws/codebuild/{project_name}:*",
        ],
    }],
})

write(github_trust_path, {
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Allow",
        "Principal": {"Federated": provider_arn},
        "Action": "sts:AssumeRoleWithWebIdentity",
        "Condition": {
            "StringEquals": {
                "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
            },
            "StringLike": {
                "token.actions.githubusercontent.com:sub": f"repo:{repo}:*",
            },
        },
    }],
})

write(github_policy_path, {
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Allow",
        "Action": [
            "codebuild:BatchGetBuilds",
            "codebuild:BatchGetProjects",
            "codebuild:StartBuild",
        ],
        "Resource": f"arn:aws:codebuild:{region}:{account_id}:project/{project_name}",
    }],
})

buildspec = """version: 0.2
env:
  shell: bash
phases:
  install:
    commands:
      - set -euo pipefail
      - |
        if ! command -v cargo >/dev/null 2>&1; then
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
        fi
      - . "$HOME/.cargo/env"
      - rustc --version
      - cargo --version
  build:
    commands:
      - set -euo pipefail
      - . "$HOME/.cargo/env"
      - test -n "${ADL_CODEFRIEND_BUILD_COMMAND:-}"
      - bash -lc "$ADL_CODEFRIEND_BUILD_COMMAND"
"""

write(project_path, {
    "name": project_name,
    "description": "ADL CodeFriend GitHub Actions triggered build lane",
    "source": {
        "type": "GITHUB",
        "location": source_location,
        "gitCloneDepth": 1,
        "buildspec": buildspec,
    },
    "artifacts": {"type": "NO_ARTIFACTS"},
    "environment": {
        "type": "LINUX_CONTAINER",
        "image": "aws/codebuild/standard:7.0",
        "computeType": compute_type,
        "privilegedMode": False,
        "environmentVariables": [
            {"name": "ADL_CODEFRIEND_BUILD_COMMAND", "value": "bash adl/tools/run_pr_fast_test_lane.sh", "type": "PLAINTEXT"},
        ],
    },
    "serviceRole": f"arn:aws:iam::{account_id}:role/{service_role_name}",
    "timeoutInMinutes": 45,
    "queuedTimeoutInMinutes": 30,
    "logsConfig": {
        "cloudWatchLogs": {
            "status": "ENABLED",
            "groupName": f"/aws/codebuild/{project_name}",
        },
    },
    "tags": [
        {"key": "Project", "value": "ADL"},
        {"key": "Lane", "value": "CodeFriendBuild"},
        {"key": "Issue", "value": "4838"},
        {"key": "ManagedBy", "value": "adl-tools"},
    ],
})
PY

if [ "$service_role_exists" != "true" ]; then
  "$AWS_CLI" iam create-role \
    "${aws_args[@]}" \
    --role-name "$SERVICE_ROLE_NAME" \
    --assume-role-policy-document "file://$service_trust" \
    --description "ADL CodeFriend CodeBuild service role" \
    --tags Key=Project,Value=ADL Key=Lane,Value=CodeFriendBuild Key=Issue,Value=4838 Key=ManagedBy,Value=adl-tools \
    --query 'Role.RoleName' \
    --output text >/dev/null
else
  "$AWS_CLI" iam update-assume-role-policy \
    "${aws_args[@]}" \
    --role-name "$SERVICE_ROLE_NAME" \
    --policy-document "file://$service_trust"
fi

"$AWS_CLI" iam put-role-policy \
  "${aws_args[@]}" \
  --role-name "$SERVICE_ROLE_NAME" \
  --policy-name adl-codefriend-codebuild-service-policy \
  --policy-document "file://$service_policy"

if [ "$github_role_exists" != "true" ]; then
  "$AWS_CLI" iam create-role \
    "${aws_args[@]}" \
    --role-name "$GITHUB_ROLE_NAME" \
    --assume-role-policy-document "file://$github_trust" \
    --description "ADL CodeFriend GitHub Actions CodeBuild start role" \
    --tags Key=Project,Value=ADL Key=Lane,Value=CodeFriendBuild Key=Issue,Value=4838 Key=ManagedBy,Value=adl-tools \
    --query 'Role.RoleName' \
    --output text >/dev/null
else
  "$AWS_CLI" iam update-assume-role-policy \
    "${aws_args[@]}" \
    --role-name "$GITHUB_ROLE_NAME" \
    --policy-document "file://$github_trust"
fi

"$AWS_CLI" iam put-role-policy \
  "${aws_args[@]}" \
  --role-name "$GITHUB_ROLE_NAME" \
  --policy-name adl-codefriend-start-build-policy \
  --policy-document "file://$github_policy"

if [ "$project_found" = "1" ]; then
  "$AWS_CLI" codebuild update-project \
    "${aws_args[@]}" \
    --region "$REGION" \
    --cli-input-json "file://$project_json" \
    --query 'project.name' \
    --output text >/dev/null
else
  "$AWS_CLI" codebuild create-project \
    "${aws_args[@]}" \
    --region "$REGION" \
    --cli-input-json "file://$project_json" \
    --query 'project.name' \
    --output text >/dev/null
fi

github_role_arn="$("$AWS_CLI" iam get-role "${aws_args[@]}" --role-name "$GITHUB_ROLE_NAME" --query 'Role.Arn' --output text)"
github_config="$ARTIFACT_DIR/github-actions-config.env"
python3 - <<'PY' "$github_config" "$PROJECT_NAME" "$REGION" "$github_role_arn" "$account_hash"
import sys
from pathlib import Path

path, project_name, region, role_arn, account_hash = sys.argv[1:6]
Path(path).write_text(
    "\n".join([
        f"AWS_CODEFRIEND_CODEBUILD_PROJECT={project_name}",
        f"AWS_CODEFRIEND_REGION={region}",
        f"AWS_CODEFRIEND_BUILD_ROLE_ARN={role_arn}",
        f"AWS_CODEFRIEND_ACCOUNT_SHA256={account_hash}",
        "",
    ])
)
PY
chmod 600 "$github_config"

printf 'PASS aws_codefriend_resources_ready project=%s region=%s profile=%s compute_type=%s\n' "$PROJECT_NAME" "$REGION" "$PROFILE" "$COMPUTE_TYPE"
printf 'github_actions_config_path=%s\n' "$github_config"
