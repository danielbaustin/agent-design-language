#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  adl/tools/import_adl_builder_image_from_s3_to_ecr.sh [options]

Options:
  --s3-uri <s3://bucket/key>       Docker image tar object to import. Required.
  --image-uri <uri>                ECR image URI to push. Required.
  --project-name <name>            CodeBuild importer project. Defaults to adl-builder-image-import.
  --service-role <arn>             CodeBuild service role. Defaults to the adl-codefriend-build role.
  --region <region>                AWS region. Defaults to us-west-2.
  --aws-profile <profile>          AWS profile. Defaults to agent-logic-admin.
  --compute-type <type>            CodeBuild compute type. Defaults to BUILD_GENERAL1_LARGE.
  --ensure-role-policy             Attach/update the narrow importer policy on the CodeBuild role.
  --create-project                 Create or update the importer project.
  --start                          Start the import build and wait for completion.
  --help                           Show this help.

The importer project uses privileged Docker only for image load/tag/push. The
image tar moves through S3 so upload retry and multipart behavior are owned by
the S3 transfer path, not Docker's direct ECR push client.
EOF
}

S3_URI=""
IMAGE_URI=""
PROJECT_NAME="${ADL_BUILDER_IMAGE_IMPORT_PROJECT:-adl-builder-image-import}"
SERVICE_ROLE="${ADL_BUILDER_IMAGE_IMPORT_SERVICE_ROLE:-}"
REGION="${ADL_AWS_REGION:-us-west-2}"
AWS_PROFILE_VALUE="${AWS_PROFILE:-agent-logic-admin}"
COMPUTE_TYPE="${ADL_BUILDER_IMAGE_IMPORT_COMPUTE_TYPE:-BUILD_GENERAL1_LARGE}"
CREATE_PROJECT=false
START=false
ENSURE_ROLE_POLICY=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --s3-uri)
      S3_URI="${2:-}"
      shift 2
      ;;
    --image-uri)
      IMAGE_URI="${2:-}"
      shift 2
      ;;
    --project-name)
      PROJECT_NAME="${2:-}"
      shift 2
      ;;
    --service-role)
      SERVICE_ROLE="${2:-}"
      shift 2
      ;;
    --region)
      REGION="${2:-}"
      shift 2
      ;;
    --aws-profile)
      AWS_PROFILE_VALUE="${2:-}"
      shift 2
      ;;
    --compute-type)
      COMPUTE_TYPE="${2:-}"
      shift 2
      ;;
    --create-project)
      CREATE_PROJECT=true
      shift
      ;;
    --ensure-role-policy)
      ENSURE_ROLE_POLICY=true
      shift
      ;;
    --start)
      START=true
      shift
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      echo "import_adl_builder_image: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$S3_URI" || -z "$IMAGE_URI" ]]; then
  echo "import_adl_builder_image: --s3-uri and --image-uri are required" >&2
  usage >&2
  exit 2
fi

if [[ -z "$SERVICE_ROLE" ]]; then
  SERVICE_ROLE="$(AWS_PROFILE="$AWS_PROFILE_VALUE" aws codebuild batch-get-projects \
    --region "$REGION" \
    --names adl-codefriend-build \
    --query 'projects[0].serviceRole' \
    --output text)"
fi

if [[ -z "$SERVICE_ROLE" || "$SERVICE_ROLE" == "None" ]]; then
  echo "import_adl_builder_image: unable to resolve CodeBuild service role" >&2
  exit 2
fi

if [[ "$SERVICE_ROLE" == arn:aws:iam::*:role/* ]]; then
  SERVICE_ROLE_NAME="${SERVICE_ROLE##*/}"
else
  SERVICE_ROLE_NAME="$SERVICE_ROLE"
fi

s3_without_scheme="${S3_URI#s3://}"
s3_bucket="${s3_without_scheme%%/*}"
s3_key="${s3_without_scheme#*/}"
s3_prefix="${s3_key%/*}"
image_registry="${IMAGE_URI%%/*}"
image_repo_tag="${IMAGE_URI#*/}"
image_repository="${image_repo_tag%%:*}"
image_account="${image_registry%%.*}"
image_region="${image_registry#*.dkr.ecr.}"
image_region="${image_region%%.amazonaws.com}"

if [[ -z "$s3_bucket" || "$s3_bucket" == "$S3_URI" || -z "$s3_key" || "$s3_key" == "$s3_without_scheme" ]]; then
  echo "import_adl_builder_image: --s3-uri must be an s3://bucket/key URI" >&2
  exit 2
fi

if [[ -z "$image_account" || -z "$image_region" || "$image_region" == "$image_registry" || -z "$image_repository" ]]; then
  echo "import_adl_builder_image: --image-uri must be an ECR image URI" >&2
  exit 2
fi

buildspec="$(cat <<'YAML'
version: 0.2
phases:
  build:
    commands:
      - |
        bash -lc '
          set -euo pipefail
          test -n "${ADL_BUILDER_IMAGE_TAR_S3_URI:-}"
          test -n "${ADL_BUILDER_IMAGE_URI:-}"
          aws s3 cp "$ADL_BUILDER_IMAGE_TAR_S3_URI" /tmp/adl-builder-image.tar
          docker load -i /tmp/adl-builder-image.tar
          docker image ls
          docker tag "$ADL_BUILDER_IMAGE_URI" "$ADL_BUILDER_IMAGE_URI"
          registry="${ADL_BUILDER_IMAGE_URI%%/*}"
          aws ecr get-login-password --region "$AWS_DEFAULT_REGION" | docker login --username AWS --password-stdin "$registry"
          docker push "$ADL_BUILDER_IMAGE_URI"
        '
YAML
)"

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/adl-builder-image-import.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT
project_json="$tmp_dir/project.json"
policy_json="$tmp_dir/importer-role-policy.json"

cat >"$policy_json" <<JSON
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "ReadAdlBuilderImageTransitObject",
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:GetObjectVersion"
      ],
      "Resource": "arn:aws:s3:::$s3_bucket/$s3_prefix/*"
    },
    {
      "Sid": "AuthorizeEcrPush",
      "Effect": "Allow",
      "Action": [
        "ecr:GetAuthorizationToken"
      ],
      "Resource": "*"
    },
    {
      "Sid": "PushAdlBuilderImageToEcr",
      "Effect": "Allow",
      "Action": [
        "ecr:BatchCheckLayerAvailability",
        "ecr:BatchGetImage",
        "ecr:CompleteLayerUpload",
        "ecr:DescribeImages",
        "ecr:DescribeRepositories",
        "ecr:InitiateLayerUpload",
        "ecr:PutImage",
        "ecr:UploadLayerPart"
      ],
      "Resource": "arn:aws:ecr:$image_region:$image_account:repository/$image_repository"
    }
  ]
}
JSON

cat >"$project_json" <<JSON
{
  "name": "$PROJECT_NAME",
  "description": "Import ADL builder image tar from S3 and push to ECR",
  "source": {
    "type": "NO_SOURCE",
    "buildspec": $(python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))' <<<"$buildspec")
  },
  "artifacts": {
    "type": "NO_ARTIFACTS"
  },
  "environment": {
    "type": "LINUX_CONTAINER",
    "image": "aws/codebuild/standard:7.0",
    "computeType": "$COMPUTE_TYPE",
    "privilegedMode": true,
    "environmentVariables": []
  },
  "logsConfig": {
    "cloudWatchLogs": {
      "status": "ENABLED",
      "groupName": "/aws/codebuild/adl-codefriend-build",
      "streamName": "$PROJECT_NAME"
    }
  },
  "serviceRole": "$SERVICE_ROLE",
  "timeoutInMinutes": 60
}
JSON

if [[ "$CREATE_PROJECT" == true ]]; then
  if AWS_PROFILE="$AWS_PROFILE_VALUE" aws codebuild batch-get-projects \
    --region "$REGION" \
    --names "$PROJECT_NAME" \
    --query 'projects[0].name' \
    --output text | grep -Fx "$PROJECT_NAME" >/dev/null; then
    AWS_PROFILE="$AWS_PROFILE_VALUE" aws codebuild update-project \
      --region "$REGION" \
      --cli-input-json "file://$project_json" >/dev/null
  else
    AWS_PROFILE="$AWS_PROFILE_VALUE" aws codebuild create-project \
      --region "$REGION" \
      --cli-input-json "file://$project_json" >/dev/null
  fi
fi

if [[ "$ENSURE_ROLE_POLICY" == true ]]; then
  AWS_PROFILE="$AWS_PROFILE_VALUE" aws iam put-role-policy \
    --role-name "$SERVICE_ROLE_NAME" \
    --policy-name adl-builder-image-import \
    --policy-document "file://$policy_json"
fi

if [[ "$START" == true ]]; then
  build_id="$(AWS_PROFILE="$AWS_PROFILE_VALUE" aws codebuild start-build \
    --region "$REGION" \
    --project-name "$PROJECT_NAME" \
    --environment-variables-override \
      "name=ADL_BUILDER_IMAGE_TAR_S3_URI,value=$S3_URI,type=PLAINTEXT" \
      "name=ADL_BUILDER_IMAGE_URI,value=$IMAGE_URI,type=PLAINTEXT" \
    --query 'build.id' \
    --output text)"
  printf 'build_id=%s\n' "$build_id"
  while true; do
    status="$(AWS_PROFILE="$AWS_PROFILE_VALUE" aws codebuild batch-get-builds \
      --region "$REGION" \
      --ids "$build_id" \
      --query 'builds[0].buildStatus' \
      --output text)"
    printf 'status=%s\n' "$status"
    case "$status" in
      SUCCEEDED)
        exit 0
        ;;
      FAILED|FAULT|STOPPED|TIMED_OUT)
        exit 1
        ;;
    esac
    sleep 15
  done
fi
