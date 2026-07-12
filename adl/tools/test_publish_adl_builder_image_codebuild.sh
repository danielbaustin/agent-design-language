#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/publish_adl_builder_image_codebuild.sh"

bash -n "$SCRIPT"
for token in \
  'BUILD_GENERAL1_XLARGE' \
  'privilegedMode' \
  'adl/docker/adl-builder/Dockerfile' \
  'cargo nextest --version' \
  'cargo llvm-cov --version' \
  'llvm-tools-preview' \
  'docker push' \
  'ecr:PutImage' \
  'immutable_digest_verified=true'; do
  grep -F "$token" "$SCRIPT" >/dev/null
done

echo "PASS test_publish_adl_builder_image_codebuild"
