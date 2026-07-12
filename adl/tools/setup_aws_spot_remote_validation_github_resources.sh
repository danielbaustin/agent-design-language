#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  setup_aws_spot_remote_validation_github_resources.sh --apply [options]

Options:
  --apply                         Create or update AWS resources.
  --check                         Report whether resources exist without mutating AWS.
  --profile <profile>             AWS CLI profile. Default: agent-logic-admin.
  --region <region>               AWS region. Default: us-west-2.
  --repo <owner/name>             GitHub repository. Default: danielbaustin/agent-design-language.
  --github-role-name <name>       OIDC role for GitHub Actions.
  --ssh-allowed-cidr <cidr>       Optional SSH debug source CIDR repository variable.
  --github-vars-only              Only create/update GitHub repository variables.
  --artifact-dir <path>           Local setup artifact directory.
  -h, --help                      Show this help.

Creates the Agent Logic GitHub Actions OIDC role used to start the AWS Spot
remote validation lane. Output intentionally avoids account ids, ARNs, and
secrets; the generated env file is chmod 600 and is for operator setup only.
USAGE
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 2
}

AWS_CLI="${ADL_AWS_CLI:-aws}"
GITHUB_API_BIN="${ADL_GITHUB_API_BIN:-curl}"
GITHUB_API_URL="${ADL_GITHUB_API_URL:-https://api.github.com}"
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
REPO="danielbaustin/agent-design-language"
GITHUB_ROLE_NAME="adl-spot-remote-validation-github-actions-role"
SSH_ALLOWED_CIDR="${ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR:-}"
ARTIFACT_DIR=".adl/tmp/aws-spot-remote-validation-github-setup"
MODE="check"
GITHUB_VARS_ONLY=false

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
    --repo)
      [ "$#" -ge 2 ] || die "--repo requires owner/name"
      REPO="$2"
      shift 2
      ;;
    --github-role-name)
      [ "$#" -ge 2 ] || die "--github-role-name requires a value"
      GITHUB_ROLE_NAME="$2"
      shift 2
      ;;
    --ssh-allowed-cidr)
      [ "$#" -ge 2 ] || die "--ssh-allowed-cidr requires a value"
      SSH_ALLOWED_CIDR="$2"
      shift 2
      ;;
    --github-vars-only)
      GITHUB_VARS_ONLY=true
      shift
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

github_token() {
  if [ -n "${GITHUB_TOKEN:-}" ]; then
    printf '%s' "$GITHUB_TOKEN"
    return 0
  fi
  if [ -n "${GH_TOKEN:-}" ]; then
    printf '%s' "$GH_TOKEN"
    return 0
  fi
  if [ -n "${ADL_GITHUB_TOKEN_FILE:-}" ] && [ -r "$ADL_GITHUB_TOKEN_FILE" ]; then
    sed -n '1p' "$ADL_GITHUB_TOKEN_FILE"
    return 0
  fi
  if [ -r "$HOME/keys/github.token" ]; then
    sed -n '1p' "$HOME/keys/github.token"
    return 0
  fi
  return 1
}

github_variable_status() {
  variable_name="$1"
  token="$2"
  status_path="$ARTIFACT_DIR/github-variable-${variable_name}.status"
  body_path="$ARTIFACT_DIR/github-variable-${variable_name}.body"
  curl_config_path="$(github_curl_config "$variable_name" "lookup" "$token")"
  "$GITHUB_API_BIN" \
    --config "$curl_config_path" \
    -sS \
    -o "$body_path" \
    -w '%{http_code}' \
    "$GITHUB_API_URL/repos/$REPO/actions/variables/$variable_name" >"$status_path"
  sed -n '1p' "$status_path"
}

github_curl_config() {
  variable_name="$1"
  purpose="$2"
  token="$3"
  config_path="$ARTIFACT_DIR/github-variable-${variable_name}-${purpose}.curl"
  python3 - <<'PY' "$config_path" "$token"
import os
import sys
from pathlib import Path

path, token = sys.argv[1:3]
Path(path).write_text(
    "\n".join([
        'header = "Accept: application/vnd.github+json"',
        f'header = "Authorization: Bearer {token}"',
        'header = "X-GitHub-Api-Version: 2022-11-28"',
        "",
    ]),
    encoding="utf-8",
)
os.chmod(path, 0o600)
PY
  printf '%s' "$config_path"
}

github_variable_payload() {
  variable_name="$1"
  variable_value="$2"
  payload_path="$ARTIFACT_DIR/github-variable-${variable_name}.json"
  python3 - <<'PY' "$payload_path" "$variable_name" "$variable_value"
import json
import sys
from pathlib import Path

path, name, value = sys.argv[1:4]
Path(path).write_text(json.dumps({"name": name, "value": value}, separators=(",", ":")) + "\n")
PY
  printf '%s' "$payload_path"
}

upsert_github_variable() {
  variable_name="$1"
  variable_value="$2"
  if [ -z "$variable_value" ]; then
    printf 'SKIP github_repository_variable name=%s reason=empty_value\n' "$variable_name"
    return 0
  fi
  token="$(github_token || true)"
  if [ -z "$token" ]; then
    printf 'WARN github_repository_variable name=%s configured=false reason=missing_github_token\n' "$variable_name" >&2
    return 0
  fi

  status="$(github_variable_status "$variable_name" "$token")"
  payload_path="$(github_variable_payload "$variable_name" "$variable_value")"
  response_path="$ARTIFACT_DIR/github-variable-${variable_name}-upsert.response"
  case "$status" in
    200)
      method="PATCH"
      url="$GITHUB_API_URL/repos/$REPO/actions/variables/$variable_name"
      ;;
    404)
      method="POST"
      url="$GITHUB_API_URL/repos/$REPO/actions/variables"
      ;;
    *)
      die "GitHub repository variable lookup failed for $variable_name with HTTP $status"
      ;;
  esac
  curl_config_path="$(github_curl_config "$variable_name" "upsert" "$token")"
  upsert_status="$("$GITHUB_API_BIN" \
    --config "$curl_config_path" \
    -sS \
    -o "$response_path" \
    -w '%{http_code}' \
    -X "$method" \
    -H "Content-Type: application/json" \
    --data-binary "@$payload_path" \
    "$url")"
  case "$method:$upsert_status" in
    PATCH:204|POST:201)
      printf 'PASS github_repository_variable name=%s configured=true action=%s\n' "$variable_name" "$method"
      ;;
    *)
      die "GitHub repository variable upsert failed for $variable_name with HTTP $upsert_status"
      ;;
  esac
}

configure_github_variables() {
  upsert_github_variable "AWS_SPOT_REMOTE_VALIDATION_REGION" "$REGION"
  upsert_github_variable "ADL_AWS_REMOTE_VALIDATION_SSH_ALLOWED_CIDR" "$SSH_ALLOWED_CIDR"
}

if [ "$GITHUB_VARS_ONLY" = "true" ]; then
  [ "$MODE" = "apply" ] || die "--github-vars-only requires --apply"
  configure_github_variables
  exit 0
fi

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
printf 'aws_spot_remote_validation_oidc_provider_exists=%s\n' "$provider_exists"

github_role_exists=false
if "$AWS_CLI" iam get-role "${aws_args[@]}" --role-name "$GITHUB_ROLE_NAME" --query 'Role.RoleName' --output text >/dev/null 2>&1; then
  github_role_exists=true
fi
printf 'aws_spot_remote_validation_github_role_exists=%s\n' "$github_role_exists"

if [ "$MODE" != "apply" ]; then
  printf 'DRY-RUN no AWS resources changed; pass --apply to create or update\n'
  exit 0
fi

[ -n "$provider_arn" ] || die "GitHub Actions OIDC provider is unavailable"

github_trust="$ARTIFACT_DIR/github-actions-trust.json"
github_policy="$ARTIFACT_DIR/github-actions-spot-remote-validation-policy.json"

python3 - <<'PY' "$github_trust" "$github_policy" "$account_id" "$provider_arn" "$REPO" "$REGION"
import json
import sys
from pathlib import Path

trust_path, policy_path, account_id, provider_arn, repo, region = sys.argv[1:7]

def write(path, payload):
    Path(path).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")

write(trust_path, {
    "Version": "2012-10-17",
    "Statement": [{
        "Effect": "Allow",
        "Principal": {"Federated": provider_arn},
        "Action": "sts:AssumeRoleWithWebIdentity",
        "Condition": {
            "StringEquals": {"token.actions.githubusercontent.com:aud": "sts.amazonaws.com"},
            "StringLike": {
                "token.actions.githubusercontent.com:sub": [
                    f"repo:{repo}:ref:refs/heads/main",
                    f"repo:{repo}:ref:refs/heads/codex/*",
                ],
            },
        },
    }],
})

write(policy_path, {
    "Version": "2012-10-17",
    "Statement": [{
        "Sid": "RemoteValidationEc2Lifecycle",
        "Effect": "Allow",
        "Action": [
            "ec2:AuthorizeSecurityGroupIngress",
            "ec2:AttachVolume",
            "ec2:CancelSpotInstanceRequests",
            "ec2:CreateSecurityGroup",
            "ec2:CreateTags",
            "ec2:DeleteSecurityGroup",
            "ec2:Describe*",
            "ec2:DetachVolume",
            "ec2:RequestSpotInstances",
            "ec2:RevokeSecurityGroupIngress",
            "ec2:RunInstances",
            "ec2:TerminateInstances",
        ],
        "Resource": "*",
    }, {
        "Sid": "RemoteValidationSsmCommands",
        "Effect": "Allow",
        "Action": [
            "ssm:CancelCommand",
            "ssm:DescribeInstanceInformation",
            "ssm:GetCommandInvocation",
            "ssm:ListCommandInvocations",
            "ssm:SendCommand",
        ],
        "Resource": "*",
    }, {
        "Sid": "ResolveAmazonLinuxAmi",
        "Effect": "Allow",
        "Action": ["ssm:GetParameter"],
        "Resource": f"arn:aws:ssm:{region}::parameter/aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64",
    }, {
        "Sid": "RemoteValidationEphemeralInstanceProfiles",
        "Effect": "Allow",
        "Action": [
            "iam:AddRoleToInstanceProfile",
            "iam:AttachRolePolicy",
            "iam:CreateInstanceProfile",
            "iam:CreateRole",
            "iam:DeleteInstanceProfile",
            "iam:DeleteRole",
            "iam:DeleteRolePolicy",
            "iam:DetachRolePolicy",
            "iam:GetInstanceProfile",
            "iam:GetRole",
            "iam:PassRole",
            "iam:PutRolePolicy",
            "iam:RemoveRoleFromInstanceProfile",
            "iam:TagInstanceProfile",
            "iam:TagRole",
        ],
        "Resource": [
            f"arn:aws:iam::{account_id}:role/ADLAwsRemoteValidationRole-*",
            f"arn:aws:iam::{account_id}:instance-profile/ADLAwsRemoteValidationProfile-*",
        ],
    }, {
        "Sid": "ResolveImmutableBuilderImage",
        "Effect": "Allow",
        "Action": ["ecr:GetAuthorizationToken"],
        "Resource": "*",
    }, {
        "Sid": "DescribeAdlBuilderImage",
        "Effect": "Allow",
        "Action": ["ecr:DescribeImages"],
        "Resource": f"arn:aws:ecr:{region}:{account_id}:repository/adl-builder",
    }, {
        "Sid": "RemoteValidationReadCostQuotaBudget",
        "Effect": "Allow",
        "Action": [
            "budgets:ViewBudget",
            "ce:GetCostAndUsage",
            "servicequotas:GetServiceQuota",
        ],
        "Resource": "*",
    }],
})
PY

if [ "$github_role_exists" != "true" ]; then
  "$AWS_CLI" iam create-role \
    "${aws_args[@]}" \
    --role-name "$GITHUB_ROLE_NAME" \
    --assume-role-policy-document "file://$github_trust" \
    --description "ADL Spot remote validation GitHub Actions role" \
    --tags Key=Project,Value=ADL Key=Lane,Value=SpotRemoteValidation Key=Issue,Value=4837 Key=ManagedBy,Value=adl-tools \
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
  --policy-name adl-spot-remote-validation-policy \
  --policy-document "file://$github_policy"

github_role_arn="$("$AWS_CLI" iam get-role "${aws_args[@]}" --role-name "$GITHUB_ROLE_NAME" --query 'Role.Arn' --output text)"
github_config="$ARTIFACT_DIR/github-actions-config.env"
python3 - <<'PY' "$github_config" "$REGION" "$github_role_arn" "$account_hash"
import sys
from pathlib import Path

path, region, role_arn, account_hash = sys.argv[1:5]
Path(path).write_text(
    "\n".join([
        f"AWS_SPOT_REMOTE_VALIDATION_REGION={region}",
        f"AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN={role_arn}",
        f"AWS_SPOT_REMOTE_VALIDATION_ACCOUNT_SHA256={account_hash}",
        "",
    ])
)
PY
chmod 600 "$github_config"
configure_github_variables

printf 'PASS aws_spot_remote_validation_github_resources_ready region=%s profile=%s role_configured=true\n' "$REGION" "$PROFILE"
printf 'github_actions_config_path=%s\n' "$github_config"
