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
  --compute-type <type>           CodeBuild compute type. Default: BUILD_GENERAL1_XLARGE.
  --image-uri <uri>               CodeBuild environment image. Default: ADL_AWS_CODEFRIEND_IMAGE,
                                  then adl-builder:v0.91.7-fixed.
  --cache-bucket <bucket>         S3 cache bucket. Default: adl-codefriend-build-cache.
  --cache-prefix <prefix>         S3 cache prefix. Default: codebuild/cache.
  --github-role-name <name>       OIDC role for GitHub Actions.
  --service-role-name <name>      CodeBuild service role.
  --artifact-dir <path>           Local setup artifact directory.
  -h, --help                      Show this help.

Creates the minimum Agent Logic AWS resources for the CodeFriend CodeBuild lane:
the CodeBuild service role, the GitHub Actions OIDC start-build role, and the
CodeBuild project plus its bounded-retention CloudWatch log group. Output
intentionally avoids account ids, ARNs, and secrets.
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
COMPUTE_TYPE="${ADL_AWS_CODEFRIEND_COMPUTE_TYPE:-BUILD_GENERAL1_XLARGE}"
IMAGE_URI="${ADL_AWS_CODEFRIEND_IMAGE:-adl-builder:v0.91.7-fixed}"
CACHE_BUCKET="${ADL_AWS_CODEFRIEND_CACHE_BUCKET:-adl-codefriend-build-cache}"
CACHE_PREFIX="${ADL_AWS_CODEFRIEND_CACHE_PREFIX:-codebuild/cache}"
GITHUB_ROLE_NAME="adl-codefriend-github-actions-build-role"
SERVICE_ROLE_NAME="adl-codefriend-codebuild-service-role"
LOG_RETENTION_DAYS="${ADL_AWS_CODEFRIEND_LOG_RETENTION_DAYS:-30}"
ARTIFACT_DIR=".adl/local-artifacts/aws-codefriend-build-resource-setup"
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
    --image-uri)
      [ "$#" -ge 2 ] || die "--image-uri requires a value"
      IMAGE_URI="$2"
      shift 2
      ;;
    --cache-bucket)
      [ "$#" -ge 2 ] || die "--cache-bucket requires a value"
      CACHE_BUCKET="$2"
      shift 2
      ;;
    --cache-prefix)
      [ "$#" -ge 2 ] || die "--cache-prefix requires a value"
      CACHE_PREFIX="$2"
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

LOG_GROUP="/aws/codebuild/${PROJECT_NAME}"

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

if [[ "$IMAGE_URI" == adl-builder:* ]]; then
  IMAGE_URI="${account_id}.dkr.ecr.${REGION}.amazonaws.com/${IMAGE_URI}"
fi
if [[ "$IMAGE_URI" == *.dkr.ecr.*/*:* && "$IMAGE_URI" != *@sha256:* ]]; then
  image_registry="${IMAGE_URI%%/*}"
  image_repository_tag="${IMAGE_URI#*/}"
  image_repository="${image_repository_tag%:*}"
  image_tag="${image_repository_tag##*:}"
  image_digest="$("$AWS_CLI" ecr describe-images "${aws_args[@]}" --region "$REGION" --repository-name "$image_repository" --image-ids "imageTag=$image_tag" --query 'imageDetails[0].imageDigest' --output text)"
  [[ "$image_digest" == sha256:* ]] || die "CodeBuild builder image tag did not resolve to an immutable ECR digest"
  IMAGE_URI="$image_registry/$image_repository@$image_digest"
fi

printf 'PASS account_profile_resolved profile=%s account_hash_available=true\n' "$PROFILE"

cache_bucket_exists=false
if "$AWS_CLI" s3api head-bucket "${aws_args[@]}" --bucket "$CACHE_BUCKET" >/dev/null 2>&1; then
  cache_bucket_exists=true
elif [ "$MODE" = "apply" ]; then
  if [ "$REGION" = "us-east-1" ]; then
    "$AWS_CLI" s3api create-bucket "${aws_args[@]}" --bucket "$CACHE_BUCKET" >/dev/null
  else
    "$AWS_CLI" s3api create-bucket \
      "${aws_args[@]}" \
      --bucket "$CACHE_BUCKET" \
      --create-bucket-configuration "LocationConstraint=$REGION" >/dev/null
  fi
  "$AWS_CLI" s3api put-public-access-block \
    "${aws_args[@]}" \
    --bucket "$CACHE_BUCKET" \
    --public-access-block-configuration BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true
  "$AWS_CLI" s3api put-bucket-versioning \
    "${aws_args[@]}" \
    --bucket "$CACHE_BUCKET" \
    --versioning-configuration Status=Suspended
  cache_bucket_exists=true
fi
printf 'aws_codefriend_cache_bucket_exists=%s\n' "$cache_bucket_exists"

log_group_exists=false
log_group_name="$($AWS_CLI logs describe-log-groups "${aws_args[@]}" --region "$REGION" --log-group-name-prefix "$LOG_GROUP" --query "logGroups[?logGroupName=='${LOG_GROUP}'].logGroupName | [0]" --output text)"
if [ "$log_group_name" = "$LOG_GROUP" ]; then
  log_group_exists=true
elif [ "$MODE" = "apply" ]; then
  "$AWS_CLI" logs create-log-group "${aws_args[@]}" --region "$REGION" --log-group-name "$LOG_GROUP"
  log_group_exists=true
fi
if [ "$MODE" = "apply" ]; then
  "$AWS_CLI" logs put-retention-policy "${aws_args[@]}" --region "$REGION" --log-group-name "$LOG_GROUP" --retention-in-days "$LOG_RETENTION_DAYS"
fi
printf 'aws_codefriend_log_group_exists=%s retention_days=%s\n' "$log_group_exists" "$LOG_RETENTION_DAYS"

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

python3 - <<'PY' "$service_trust" "$service_policy" "$github_trust" "$github_policy" "$project_json" "$account_id" "$provider_arn" "$REPO" "$REGION" "$PROJECT_NAME" "$SOURCE_LOCATION" "$SERVICE_ROLE_NAME" "$COMPUTE_TYPE" "$IMAGE_URI" "$CACHE_BUCKET" "$CACHE_PREFIX"
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
    image_uri,
    cache_bucket,
    cache_prefix,
) = sys.argv[1:17]

def write(path, payload):
    Path(path).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")

image_pull_credentials_type = "SERVICE_ROLE" if ".dkr.ecr." in image_uri else "CODEBUILD"

service_policy_statements = [{
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
}, {
    "Effect": "Allow",
    "Action": ["s3:GetObject", "s3:PutObject", "s3:DeleteObject", "s3:GetObjectVersion"],
    "Resource": f"arn:aws:s3:::{cache_bucket}/{cache_prefix}/*",
}, {
    "Effect": "Allow",
    "Action": ["s3:ListBucket", "s3:GetBucketLocation"],
    "Resource": f"arn:aws:s3:::{cache_bucket}",
    "Condition": {"StringLike": {"s3:prefix": [cache_prefix, f"{cache_prefix}/*"]}},
}]

if image_pull_credentials_type == "SERVICE_ROLE":
    registry, _, repository_with_tag = image_uri.partition("/")
    image_account = registry.split(".", 1)[0] if registry else account_id
    repository = repository_with_tag.split("@", 1)[0].split(":", 1)[0]
    repository_resource = (
        f"arn:aws:ecr:{region}:{image_account}:repository/{repository}"
        if repository
        else f"arn:aws:ecr:{region}:{image_account}:repository/*"
    )
    service_policy_statements.extend([{
        "Effect": "Allow",
        "Action": ["ecr:GetAuthorizationToken"],
        "Resource": "*",
    }, {
        "Effect": "Allow",
        "Action": [
            "ecr:BatchCheckLayerAvailability",
            "ecr:BatchGetImage",
            "ecr:GetDownloadUrlForLayer",
        ],
        "Resource": repository_resource,
    }])

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
    "Statement": service_policy_statements,
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
                "token.actions.githubusercontent.com:sub": [
                    f"repo:{repo}:ref:refs/heads/main",
                    f"repo:{repo}:ref:refs/heads/codex/*",
                ],
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
            "codebuild:StopBuild",
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
      - export PATH="/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin"
      - export NO_PROXY="127.0.0.1,localhost,${NO_PROXY:-}"
      - export no_proxy="127.0.0.1,localhost,${no_proxy:-}"
      - |
        require_tool() {
          tool="$1"
          if ! command -v "$tool" >/dev/null 2>&1; then
            echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=missing_tool tool=$tool" >&2
            exit 42
          fi
        }
        for tool in rustc cargo cargo-nextest sccache ld.lld zstd aws git; do
          require_tool "$tool"
        done
        case "${ADL_CODEFRIEND_EXPECTED_IMAGE:-}" in
          *@sha256:*) ;;
          *) echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=wrong_image expected_image_not_digest_pinned" >&2; exit 43 ;;
        esac
        printf '%s\\n' "${CODEBUILD_RESOLVED_SOURCE_VERSION:-}" | grep -Eq '^[0-9a-f]{40}$' || {
          echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=wrong_ref resolved_source_sha_invalid" >&2
          exit 44
        }
        if [ -n "${ADL_CODEFRIEND_EXPECTED_SOURCE_SHA:-}" ] && [ "$CODEBUILD_RESOLVED_SOURCE_VERSION" != "$ADL_CODEFRIEND_EXPECTED_SOURCE_SHA" ]; then
          echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=wrong_ref expected_source_sha_mismatch" >&2
          exit 44
        fi
        echo "ADL_CODEFRIEND_TOOLCHAIN rustc=$(rustc --version)"
        echo "ADL_CODEFRIEND_TOOLCHAIN cargo=$(cargo --version)"
        echo "ADL_CODEFRIEND_TOOLCHAIN nextest=$(cargo-nextest --version)"
        echo "ADL_CODEFRIEND_TOOLCHAIN sccache=$(sccache --version)"
        echo "ADL_CODEFRIEND_TOOLCHAIN lld=$(ld.lld --version | head -n 1)"
        echo "ADL_CODEFRIEND_TOOLCHAIN zstd=$(zstd --version | head -n 1)"
        echo "ADL_CODEFRIEND_TOOLCHAIN_SOURCE image=prebuilt per_job_install=false"
        echo "ADL_CODEFRIEND_PREFLIGHT status=passed image_digest_pinned=true source_sha_verified=true"
      - rm -rf /codebuild/adl-source
      - mkdir -p /codebuild/adl-source /codebuild/adl-target
      - tar -C "$CODEBUILD_SRC_DIR" -cf - . | tar -C /codebuild/adl-source -xf -
      - cd /codebuild/adl-source
      - |
        [ "$(pwd -P)" = "/codebuild/adl-source" ] || {
          echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=cache_configuration source_path_invalid" >&2
          exit 45
        }
        [ -d /codebuild/adl-target ] && [ -w /codebuild/adl-target ] || {
          echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=cache_configuration target_path_unwritable" >&2
          exit 45
        }
      - mkdir -p "$HOME/.cargo/registry" "$HOME/.cargo/git" /codebuild/adl-target
      - export CARGO_TARGET_DIR="/codebuild/adl-target"
      - export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-18}"
      - export SCCACHE_CACHE_SIZE="${SCCACHE_CACHE_SIZE:-20G}"
      - export SCCACHE_BUCKET="__SCCACHE_BUCKET__"
      - export SCCACHE_REGION="__SCCACHE_REGION__"
      - export SCCACHE_S3_KEY_PREFIX="__SCCACHE_PREFIX__/sccache/x86_64-unknown-linux-gnu"
      - export ADL_CODEFRIEND_TARGET_CACHE_MODE="${ADL_CODEFRIEND_TARGET_CACHE_MODE:-s3-tar}"
      - export ADL_CODEFRIEND_TARGET_CACHE_BUCKET="${ADL_CODEFRIEND_TARGET_CACHE_BUCKET:-__SCCACHE_BUCKET__}"
      - export ADL_CODEFRIEND_TARGET_CACHE_PREFIX="${ADL_CODEFRIEND_TARGET_CACHE_PREFIX:-__SCCACHE_PREFIX__/target/x86_64-unknown-linux-gnu}"
      - eval "$(aws configure export-credentials --format env)"
      - export CARGO_INCREMENTAL=0
      - export RUSTFLAGS="-C link-arg=-fuse-ld=lld --remap-path-prefix=/codebuild/adl-source=/workspace --remap-path-prefix=/root=/home"
      - export RUSTC_WRAPPER=sccache
      - ADL_RUST_CACHE_TARGET_DIR="/codebuild/adl-target" ADL_RUST_CACHE_SCCACHE_DIR="${SCCACHE_DIR:-$HOME/.cache/sccache}" ADL_RUST_CACHE_SCCACHE_SIZE="${SCCACHE_CACHE_SIZE:-20G}" ADL_RUST_CACHE_REQUIRE_SCCACHE=1 ADL_RUST_CACHE_REQUIRE_LLD=1 ADL_RUST_CACHE_USE_LLD=1 bash adl/tools/rust_cache_env.sh write-shell-env /tmp/adl-rust-cache-env.sh
      - . /tmp/adl-rust-cache-env.sh
      - sccache --start-server || true
      - sccache --zero-stats || true
  build:
    commands:
      - set -euo pipefail
      - export PATH="/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin"
      - export NO_PROXY="127.0.0.1,localhost,${NO_PROXY:-}"
      - export no_proxy="127.0.0.1,localhost,${no_proxy:-}"
      - cd /codebuild/adl-source
      - export CARGO_TARGET_DIR="/codebuild/adl-target"
      - export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-18}"
      - export SCCACHE_CACHE_SIZE="${SCCACHE_CACHE_SIZE:-20G}"
      - export SCCACHE_BUCKET="__SCCACHE_BUCKET__"
      - export SCCACHE_REGION="__SCCACHE_REGION__"
      - export SCCACHE_S3_KEY_PREFIX="__SCCACHE_PREFIX__/sccache/x86_64-unknown-linux-gnu"
      - export ADL_CODEFRIEND_TARGET_CACHE_MODE="${ADL_CODEFRIEND_TARGET_CACHE_MODE:-s3-tar}"
      - export ADL_CODEFRIEND_TARGET_CACHE_BUCKET="${ADL_CODEFRIEND_TARGET_CACHE_BUCKET:-__SCCACHE_BUCKET__}"
      - export ADL_CODEFRIEND_TARGET_CACHE_PREFIX="${ADL_CODEFRIEND_TARGET_CACHE_PREFIX:-__SCCACHE_PREFIX__/target/x86_64-unknown-linux-gnu}"
      - eval "$(aws configure export-credentials --format env)"
      - export CARGO_INCREMENTAL=0
      - export RUSTFLAGS="-C link-arg=-fuse-ld=lld --remap-path-prefix=/codebuild/adl-source=/workspace --remap-path-prefix=/root=/home"
      - export RUSTC_WRAPPER=sccache
      - ADL_RUST_CACHE_TARGET_DIR="/codebuild/adl-target" ADL_RUST_CACHE_SCCACHE_DIR="${SCCACHE_DIR:-$HOME/.cache/sccache}" ADL_RUST_CACHE_SCCACHE_SIZE="${SCCACHE_CACHE_SIZE:-20G}" ADL_RUST_CACHE_REQUIRE_SCCACHE=1 ADL_RUST_CACHE_REQUIRE_LLD=1 ADL_RUST_CACHE_USE_LLD=1 bash adl/tools/rust_cache_env.sh write-shell-env /tmp/adl-rust-cache-env.sh
      - . /tmp/adl-rust-cache-env.sh
      - test -n "${ADL_CODEFRIEND_BUILD_COMMAND:-}"
      - |
        if [ -z "${ADL_CODEFRIEND_TARGET_CACHE_KEY:-}" ]; then
          lock_hash="$(sha256sum adl/Cargo.lock | awk '{print $1}')"
          source_key="${CODEBUILD_RESOLVED_SOURCE_VERSION}"
          compatibility_hash="$(printf '%s\\n' "${ADL_CODEFRIEND_EXPECTED_IMAGE}" "$(rustc --version)" "${RUSTFLAGS}" "${CARGO_INCREMENTAL}" | sha256sum | awk '{print $1}')"
          ADL_CODEFRIEND_TARGET_CACHE_KEY="v2-${source_key}-${lock_hash}-${compatibility_hash}"
          export ADL_CODEFRIEND_TARGET_CACHE_KEY
        fi
        ADL_CODEFRIEND_TARGET_CACHE_URI="s3://${ADL_CODEFRIEND_TARGET_CACHE_BUCKET}/${ADL_CODEFRIEND_TARGET_CACHE_PREFIX}/${ADL_CODEFRIEND_TARGET_CACHE_KEY}.tar.zst"
        ADL_CODEFRIEND_TARGET_CACHE_OBJECT_KEY="${ADL_CODEFRIEND_TARGET_CACHE_PREFIX}/${ADL_CODEFRIEND_TARGET_CACHE_KEY}.tar.zst"
        export ADL_CODEFRIEND_TARGET_CACHE_URI
        export ADL_CODEFRIEND_TARGET_CACHE_OBJECT_KEY
        echo "ADL_CODEFRIEND_TARGET_CACHE mode=${ADL_CODEFRIEND_TARGET_CACHE_MODE} key=${ADL_CODEFRIEND_TARGET_CACHE_KEY}"
        if [ "${ADL_CODEFRIEND_TARGET_CACHE_MODE}" = "s3-tar" ]; then
          if expected_cache_checksum="$(aws s3api get-object --bucket "$ADL_CODEFRIEND_TARGET_CACHE_BUCKET" --key "$ADL_CODEFRIEND_TARGET_CACHE_OBJECT_KEY" --query 'Metadata.sha256' --output text /tmp/adl-codefriend-target-cache.tar.zst 2>/tmp/adl-codefriend-target-cache-restore.log)" &&
             [ -n "$expected_cache_checksum" ] && [ "$expected_cache_checksum" != "None" ]; then
            actual_cache_checksum="$(sha256sum /tmp/adl-codefriend-target-cache.tar.zst | awk '{print $1}')"
            if [ "$actual_cache_checksum" != "$expected_cache_checksum" ]; then
              echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=cache_configuration target_cache_checksum_failed" >&2
              exit 45
            fi
            rm -rf /codebuild/adl-target
            mkdir -p /codebuild/adl-target
            if tar -I 'zstd -d -T0' -xf /tmp/adl-codefriend-target-cache.tar.zst -C /codebuild; then
              echo "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=hit checksum=verified"
            else
              echo "ADL_CODEFRIEND_PREFLIGHT status=failed classification=cache_configuration target_cache_extract_failed" >&2
              exit 45
            fi
          else
            echo "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=miss"
          fi
        elif [ "${ADL_CODEFRIEND_TARGET_CACHE_MODE}" = "local" ]; then
          mkdir -p /codebuild/adl-target
          echo "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=local-cache"
        else
          echo "ADL_CODEFRIEND_TARGET_CACHE_RESTORE status=disabled"
        fi
      - |
        publish_target_cache() {
          local source="$1"
          if [ "${ADL_CODEFRIEND_TARGET_CACHE_MODE}" = "s3-tar" ] && [ -d "$CARGO_TARGET_DIR" ]; then
            cache_upload_suffix="${CODEBUILD_BUILD_ID##*:}"
            cache_upload_uri="${ADL_CODEFRIEND_TARGET_CACHE_URI}.upload-${cache_upload_suffix}"
            tar -I 'zstd -T0 -1' -cf /tmp/adl-codefriend-target-cache.tar.zst -C /codebuild adl-target
            cache_checksum="$(sha256sum /tmp/adl-codefriend-target-cache.tar.zst | awk '{print $1}')"
            aws s3 cp /tmp/adl-codefriend-target-cache.tar.zst "$cache_upload_uri" --metadata "sha256=${cache_checksum}" >/tmp/adl-codefriend-target-cache-save.log
            aws s3 cp "$cache_upload_uri" "$ADL_CODEFRIEND_TARGET_CACHE_URI" >>/tmp/adl-codefriend-target-cache-save.log
            aws s3 rm "$cache_upload_uri" >>/tmp/adl-codefriend-target-cache-save.log
            echo "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=uploaded checksum=verified atomic=true source=${source}"
          elif [ "${ADL_CODEFRIEND_TARGET_CACHE_MODE}" = "local" ]; then
            echo "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=local-cache source=${source}"
          else
            echo "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=skipped source=${source}"
          fi
        }
        prepared="false"
        if [ -n "${ADL_CODEFRIEND_BUILD_PREPARE_COMMAND:-}" ]; then
          echo "ADL_CODEFRIEND_BUILD_PREPARE status=started"
          set +e
          bash -lc "$ADL_CODEFRIEND_BUILD_PREPARE_COMMAND"
          prepare_status=$?
          set -e
          if [ "$prepare_status" -ne 0 ]; then
            echo "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=skipped-prepare-failed"
            exit "$prepare_status"
          fi
          echo "ADL_CODEFRIEND_BUILD_PREPARE status=completed"
          prepared="true"
          publish_target_cache prepare
        fi
        echo "ADL_CODEFRIEND_BUILD_COMMAND status=started"
        set +e
        bash -lc "$ADL_CODEFRIEND_BUILD_COMMAND"
        command_status=$?
        set -e
        echo "ADL_CODEFRIEND_BUILD_COMMAND status=completed exit_code=${command_status}"
        if [ "$prepared" = "false" ]; then
          if [ "$command_status" -eq 0 ]; then
            publish_target_cache command
          else
            echo "ADL_CODEFRIEND_TARGET_CACHE_SAVE status=skipped-command-failed"
          fi
        fi
        exit "$command_status"
  post_build:
    commands:
      - if command -v sccache >/dev/null 2>&1; then sccache --show-stats || true; fi
cache:
  paths:
    - '/root/.cargo/registry/**/*'
    - '/root/.cargo/git/**/*'
"""

buildspec = (
    buildspec
    .replace("__SCCACHE_BUCKET__", cache_bucket)
    .replace("__SCCACHE_REGION__", region)
    .replace("__SCCACHE_PREFIX__", cache_prefix)
)

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
    "cache": {
        "type": "LOCAL",
        "modes": ["LOCAL_SOURCE_CACHE", "LOCAL_CUSTOM_CACHE"],
    },
    "environment": {
        "type": "LINUX_CONTAINER",
        "image": image_uri,
        "computeType": compute_type,
        "imagePullCredentialsType": image_pull_credentials_type,
        "privilegedMode": False,
        "environmentVariables": [
            {"name": "ADL_CODEFRIEND_BUILD_COMMAND", "value": "bash adl/tools/run_pr_fast_test_lane.sh", "type": "PLAINTEXT"},
            {"name": "ADL_CODEFRIEND_EXPECTED_IMAGE", "value": image_uri, "type": "PLAINTEXT"},
            {"name": "CARGO_BUILD_JOBS", "value": "18", "type": "PLAINTEXT"},
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
python3 - <<'PY' "$github_config" "$PROJECT_NAME" "$REGION" "$github_role_arn" "$account_hash" "$CACHE_BUCKET" "$CACHE_PREFIX"
import sys
from pathlib import Path

path, project_name, region, role_arn, account_hash, cache_bucket, cache_prefix = sys.argv[1:8]
Path(path).write_text(
    "\n".join([
        f"AWS_CODEFRIEND_CODEBUILD_PROJECT={project_name}",
        f"AWS_CODEFRIEND_REGION={region}",
        f"AWS_CODEFRIEND_CACHE_BUCKET={cache_bucket}",
        f"AWS_CODEFRIEND_CACHE_PREFIX={cache_prefix}",
        f"AWS_CODEFRIEND_BUILD_ROLE_ARN={role_arn}",
        f"AWS_CODEFRIEND_ACCOUNT_SHA256={account_hash}",
        "",
    ])
)
PY
chmod 600 "$github_config"

printf 'PASS aws_codefriend_resources_ready project=%s region=%s profile=%s compute_type=%s cache_bucket=%s cache_prefix=%s\n' "$PROJECT_NAME" "$REGION" "$PROFILE" "$COMPUTE_TYPE" "$CACHE_BUCKET" "$CACHE_PREFIX"
printf 'github_actions_config_path=%s\n' "$github_config"
