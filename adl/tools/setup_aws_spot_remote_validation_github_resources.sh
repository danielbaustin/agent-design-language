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
PROFILE="${ADL_AWS_PROFILE:-agent-logic-admin}"
REGION="${ADL_AWS_REGION:-us-west-2}"
REPO="danielbaustin/agent-design-language"
GITHUB_ROLE_NAME="adl-spot-remote-validation-github-actions-role"
ARTIFACT_DIR=".adl/tmp/aws-spot-remote-validation-github-setup"
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
            "ec2:CancelSpotInstanceRequests",
            "ec2:CreateSecurityGroup",
            "ec2:CreateTags",
            "ec2:CreateVolume",
            "ec2:DeleteSecurityGroup",
            "ec2:DeleteVolume",
            "ec2:Describe*",
            "ec2:DetachVolume",
            "ec2:ModifyVolume",
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
        "Sid": "RemoteValidationEphemeralInstanceProfiles",
        "Effect": "Allow",
        "Action": [
            "iam:AddRoleToInstanceProfile",
            "iam:CreateInstanceProfile",
            "iam:CreateRole",
            "iam:DeleteInstanceProfile",
            "iam:DeleteRole",
            "iam:GetInstanceProfile",
            "iam:GetRole",
            "iam:PassRole",
            "iam:PutRolePolicy",
            "iam:RemoveRoleFromInstanceProfile",
        ],
        "Resource": [
            f"arn:aws:iam::{account_id}:role/adl-aws-remote-validation-*",
            f"arn:aws:iam::{account_id}:instance-profile/adl-aws-remote-validation-*",
        ],
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

printf 'PASS aws_spot_remote_validation_github_resources_ready region=%s profile=%s role_configured=true\n' "$REGION" "$PROFILE"
printf 'github_actions_config_path=%s\n' "$github_config"
