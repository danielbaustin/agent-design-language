#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

usage() {
  cat <<'EOF'
Usage:
  adl/tools/setup_adl_builder_image.sh [options]

Options:
  --image-uri <uri>          Explicit image URI. Required for --write-env unless --ecr-repository is used.
  --ecr-repository <name>    ECR repository name. Defaults to adl-builder.
  --tag <tag>                Image tag. Defaults to v0.91.7.
  --region <region>          AWS region. Defaults to us-west-2.
  --aws-profile <profile>    AWS profile. Defaults to agent-logic-admin.
  --docker-bin <path>        Docker-compatible CLI. Defaults to docker.
  --docker-config <path>     Docker config directory for registry login. Defaults to .adl/local/docker-config.
  --platform <platform>      Docker build platform. Defaults to linux/amd64.
  --dockerfile <path>        Dockerfile path. Defaults to adl/docker/adl-builder/Dockerfile.
  --context <path>           Docker build context. Defaults to repository root.
  --ensure-ecr               Create the ECR repository when missing.
  --build                    Build the local image tag.
  --push                     Login and push to ECR. Implies --build unless --skip-build is set.
  --skip-build               Do not build before push.
  --write-env <path>         Write lane environment exports for CodeBuild, Spot, Nessus, and local use.
  --print-config             Print non-secret configuration.
  --help                     Show this help.

Environment overrides:
  ADL_BUILDER_IMAGE
  ADL_BUILDER_IMAGE_TAG
  ADL_BUILDER_ECR_REPOSITORY
  ADL_AWS_REGION
  AWS_PROFILE
  DOCKER_BIN
  DOCKER_CONFIG
EOF
}

IMAGE_URI="${ADL_BUILDER_IMAGE:-}"
ECR_REPOSITORY="${ADL_BUILDER_ECR_REPOSITORY:-adl-builder}"
TAG="${ADL_BUILDER_IMAGE_TAG:-v0.91.7}"
REGION="${ADL_AWS_REGION:-us-west-2}"
AWS_PROFILE_VALUE="${AWS_PROFILE:-agent-logic-admin}"
DOCKER_BIN="${DOCKER_BIN:-docker}"
DOCKER_CONFIG_DIR="${DOCKER_CONFIG:-$ROOT_DIR/.adl/local/docker-config}"
PLATFORM="linux/amd64"
DOCKERFILE="$ROOT_DIR/adl/docker/adl-builder/Dockerfile"
CONTEXT="$ROOT_DIR"
ENSURE_ECR=false
BUILD=false
PUSH=false
SKIP_BUILD=false
WRITE_ENV=""
PRINT_CONFIG=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --image-uri)
      IMAGE_URI="${2:-}"
      shift 2
      ;;
    --ecr-repository)
      ECR_REPOSITORY="${2:-}"
      shift 2
      ;;
    --tag)
      TAG="${2:-}"
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
    --docker-bin)
      DOCKER_BIN="${2:-}"
      shift 2
      ;;
    --docker-config)
      DOCKER_CONFIG_DIR="${2:-}"
      shift 2
      ;;
    --platform)
      PLATFORM="${2:-}"
      shift 2
      ;;
    --dockerfile)
      DOCKERFILE="${2:-}"
      shift 2
      ;;
    --context)
      CONTEXT="${2:-}"
      shift 2
      ;;
    --ensure-ecr)
      ENSURE_ECR=true
      shift
      ;;
    --build)
      BUILD=true
      shift
      ;;
    --push)
      PUSH=true
      shift
      ;;
    --skip-build)
      SKIP_BUILD=true
      shift
      ;;
    --write-env)
      WRITE_ENV="${2:-}"
      shift 2
      ;;
    --print-config)
      PRINT_CONFIG=true
      shift
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      echo "setup_adl_builder_image: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    echo "setup_adl_builder_image: required command not found: $name" >&2
    exit 2
  fi
}

resolve_ecr_image_uri() {
  require_cmd aws
  local account_id
  account_id="$(AWS_PROFILE="$AWS_PROFILE_VALUE" aws sts get-caller-identity --query Account --output text)"
  printf '%s.dkr.ecr.%s.amazonaws.com/%s:%s\n' "$account_id" "$REGION" "$ECR_REPOSITORY" "$TAG"
}

if [[ -z "$IMAGE_URI" && ( "$ENSURE_ECR" == true || "$PUSH" == true ) ]]; then
  IMAGE_URI="$(resolve_ecr_image_uri)"
fi

if [[ -z "$IMAGE_URI" ]]; then
  IMAGE_URI="adl-builder:$TAG"
fi

if [[ "$PRINT_CONFIG" == true ]]; then
  printf 'builder_image=%s\n' "$IMAGE_URI"
  printf 'dockerfile=%s\n' "${DOCKERFILE#"$ROOT_DIR"/}"
  printf 'docker_config=%s\n' "${DOCKER_CONFIG_DIR#"$ROOT_DIR"/}"
  printf 'context=%s\n' "${CONTEXT#"$ROOT_DIR"/}"
  printf 'platform=%s\n' "$PLATFORM"
  printf 'ecr_repository=%s\n' "$ECR_REPOSITORY"
  printf 'region=%s\n' "$REGION"
  printf 'aws_profile=%s\n' "$AWS_PROFILE_VALUE"
fi

if [[ "$ENSURE_ECR" == true ]]; then
  require_cmd aws
  if ! AWS_PROFILE="$AWS_PROFILE_VALUE" aws ecr describe-repositories --region "$REGION" --repository-names "$ECR_REPOSITORY" >/dev/null 2>&1; then
    AWS_PROFILE="$AWS_PROFILE_VALUE" aws ecr create-repository --region "$REGION" --repository-name "$ECR_REPOSITORY" >/dev/null
  fi
fi

if [[ "$PUSH" == true && "$SKIP_BUILD" != true ]]; then
  BUILD=true
fi

if [[ "$BUILD" == true ]]; then
  require_cmd "$DOCKER_BIN"
  mkdir -p "$DOCKER_CONFIG_DIR"
  export DOCKER_CONFIG="$DOCKER_CONFIG_DIR"
  "$DOCKER_BIN" build --platform "$PLATFORM" -f "$DOCKERFILE" -t "$IMAGE_URI" "$CONTEXT"
fi

if [[ "$PUSH" == true ]]; then
  require_cmd aws
  require_cmd "$DOCKER_BIN"
  mkdir -p "$DOCKER_CONFIG_DIR"
  export DOCKER_CONFIG="$DOCKER_CONFIG_DIR"
  registry="${IMAGE_URI%%/*}"
  AWS_PROFILE="$AWS_PROFILE_VALUE" aws ecr get-login-password --region "$REGION" \
    | "$DOCKER_BIN" login --username AWS --password-stdin "$registry"
  "$DOCKER_BIN" push "$IMAGE_URI"
fi

if [[ -n "$WRITE_ENV" ]]; then
  mkdir -p "$(dirname "$WRITE_ENV")"
  {
    printf 'ADL_BUILDER_IMAGE=%q\n' "$IMAGE_URI"
    printf 'ADL_AWS_CODEFRIEND_IMAGE=%q\n' "$IMAGE_URI"
    printf 'ADL_AWS_SPOT_BUILDER_IMAGE=%q\n' "$IMAGE_URI"
    printf 'ADL_NESSUS_BUILDER_IMAGE=%q\n' "$IMAGE_URI"
    printf 'ADL_LOCAL_BUILDER_IMAGE=%q\n' "$IMAGE_URI"
  } >"$WRITE_ENV"
fi
