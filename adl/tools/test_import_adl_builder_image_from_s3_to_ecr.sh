#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/import_adl_builder_image_from_s3_to_ecr.sh"

assert_has() {
  local path="$1"
  local needle="$2"
  if ! grep -F -- "$needle" "$path" >/dev/null; then
    echo "expected $path to contain: $needle" >&2
    exit 1
  fi
}

bash -n "$SCRIPT"
assert_has "$SCRIPT" "privilegedMode"
assert_has "$SCRIPT" "--ensure-role-policy"
assert_has "$SCRIPT" "aws iam put-role-policy"
assert_has "$SCRIPT" "AuthorizeEcrPush"
assert_has "$SCRIPT" "ReadAdlBuilderImageTransitObject"
assert_has "$SCRIPT" "PushAdlBuilderImageToEcr"
assert_has "$SCRIPT" 'arn:aws:ecr:$image_region:$image_account:repository/$image_repository'
assert_has "$SCRIPT" 'aws s3 cp "$ADL_BUILDER_IMAGE_TAR_S3_URI" /tmp/adl-builder-image.tar'
assert_has "$SCRIPT" "docker load -i /tmp/adl-builder-image.tar"
assert_has "$SCRIPT" 'docker push "$ADL_BUILDER_IMAGE_URI"'
assert_has "$SCRIPT" "adl-codefriend-build"
assert_has "$SCRIPT" 'groupName": "/aws/codebuild/adl-codefriend-build"'
assert_has "$SCRIPT" "BUILD_GENERAL1_LARGE"

echo "PASS test_import_adl_builder_image_from_s3_to_ecr"
