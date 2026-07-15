#!/usr/bin/env bash
set -euo pipefail

IMAGE=""
EXPECTED_REF=""
EXPECTED_ARCH="x86_64"
COMMAND=""
MIN_CACHE_FREE_GIB="10"
LOW_SPACE_CLEAN_COMMAND='cargo clean --manifest-path adl/Cargo.toml'

usage() {
  cat <<'USAGE'
Usage:
  run_aws_spot_builder_image_validation.sh \
    --image <registry/repository@sha256:digest> \
    --expected-ref <40-hex-commit> \
    --command <shell-command> [options]

Options:
  --expected-architecture <arch>  Defaults to x86_64.
  --min-cache-free-gib <gib>      Minimum writable cache headroom. Defaults to 10.

This command runs on the ephemeral Spot host after the repository and retained
EBS cache are ready. It never builds the image or installs Rust validation
tools. Host Docker and AWS CLI packages may be installed when the selected AMI
does not already provide them.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --image) IMAGE="${2:-}"; shift 2 ;;
    --expected-ref) EXPECTED_REF="${2:-}"; shift 2 ;;
    --expected-architecture) EXPECTED_ARCH="${2:-}"; shift 2 ;;
    --command) COMMAND="${2:-}"; shift 2 ;;
    --min-cache-free-gib) MIN_CACHE_FREE_GIB="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "spot_builder_image_validation: unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [[ ! "$IMAGE" =~ @sha256:[0-9a-f]{64}$ ]]; then
  echo "spot_builder_image_validation: --image must be pinned by immutable sha256 digest" >&2
  exit 2
fi
if [[ ! "$EXPECTED_REF" =~ ^[0-9a-f]{40}$ ]]; then
  echo "spot_builder_image_validation: --expected-ref must be a full 40-hex commit" >&2
  exit 2
fi
if [[ -z "$COMMAND" ]]; then
  echo "spot_builder_image_validation: --command is required" >&2
  exit 2
fi
if [[ ! "$MIN_CACHE_FREE_GIB" =~ ^[0-9]+$ ]] || [[ "$MIN_CACHE_FREE_GIB" -lt 1 ]]; then
  echo "spot_builder_image_validation: --min-cache-free-gib must be a positive integer" >&2
  exit 2
fi

: "${ADL_REMOTE_REPO_DIR:?ADL_REMOTE_REPO_DIR is required}"
: "${ADL_RUN_ROOT:?ADL_RUN_ROOT is required}"
: "${ADL_CACHE_VOLUME_MOUNT_PATH:?ADL_CACHE_VOLUME_MOUNT_PATH is required}"

stage() {
  printf '%s stage=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" \
    | tee -a "$ADL_RUN_ROOT/progress.log" >&2
}

CURRENT_STAGE="verify_source"
stage "$CURRENT_STAGE"
RESOLVED_REF="$(git -C "$ADL_REMOTE_REPO_DIR" rev-parse HEAD)"
if [[ "$RESOLVED_REF" != "$EXPECTED_REF" ]]; then
  echo "spot_builder_image_validation: resolved source commit does not match expected commit" >&2
  exit 1
fi

CURRENT_STAGE="verify_cache"
stage "$CURRENT_STAGE"
CACHE_MOUNT="$ADL_CACHE_VOLUME_MOUNT_PATH"
mountpoint -q "$CACHE_MOUNT" || {
  echo "spot_builder_image_validation: retained cache path is not a mountpoint" >&2
  exit 1
}
CACHE_SOURCE="$(findmnt -n -o SOURCE --target "$CACHE_MOUNT")"
ROOT_SOURCE="$(findmnt -n -o SOURCE --target /)"
if [[ -z "$CACHE_SOURCE" || "$CACHE_SOURCE" == "$ROOT_SOURCE" ]]; then
  echo "spot_builder_image_validation: retained cache mount resolves to the root filesystem" >&2
  exit 1
fi
PROBE="$CACHE_MOUNT/.adl-write-probe-$$"
umask 077
: >"$PROBE"
rm -f "$PROBE"
CACHE_FREE_BYTES="$(df -PB1 "$CACHE_MOUNT" | awk 'NR==2 {print $4}')"
MIN_CACHE_FREE_BYTES="$((MIN_CACHE_FREE_GIB * 1024 * 1024 * 1024))"
CACHE_LOW_SPACE_RECOVERY=false
if [[ ! "$CACHE_FREE_BYTES" =~ ^[0-9]+$ ]] || [[ "$CACHE_FREE_BYTES" -lt "$MIN_CACHE_FREE_BYTES" ]]; then
  if [[ "$COMMAND" == "$LOW_SPACE_CLEAN_COMMAND" ]]; then
    CACHE_LOW_SPACE_RECOVERY=true
    echo "spot_builder_image_validation: low-space target cleanup recovery authorized" >&2
  else
    echo "spot_builder_image_validation: retained cache has insufficient free space" >&2
    exit 1
  fi
fi

CACHE_ROOT="$CACHE_MOUNT/adl-aws-remote-validation/shared"
# Keep the original warm-EBS identity established by #4837. A container is an
# execution environment, not a reason to fork Cargo's retained cache layout.
TARGET_DIR="$CACHE_ROOT/target"
SCCACHE_DIR="$CACHE_ROOT/sccache"
CARGO_HOME_DIR="$CACHE_ROOT/cargo-home"
TMP_DIR="$CACHE_ROOT/tmp"
CACHE_TARGET_PREEXISTING_ENTRIES=0
CACHE_TARGET_PREEXISTING_BYTES=0
if [[ -d "$TARGET_DIR" ]]; then
  CACHE_TARGET_PREEXISTING_ENTRIES="$(find "$TARGET_DIR" -mindepth 1 -print | wc -l | tr -d ' ')"
  CACHE_TARGET_PREEXISTING_KIB="$(du -sk "$TARGET_DIR" 2>/dev/null | awk '{print $1}')"
  CACHE_TARGET_PREEXISTING_BYTES="$((CACHE_TARGET_PREEXISTING_KIB * 1024))"
fi
mkdir -p "$TARGET_DIR" "$SCCACHE_DIR" "$CARGO_HOME_DIR" "$TMP_DIR"

CURRENT_STAGE="ensure_container_runtime"
stage "$CURRENT_STAGE"
if ! command -v docker >/dev/null 2>&1; then
  sudo dnf install -y docker >/tmp/adl-docker-install.log 2>&1 \
    || sudo yum install -y docker >/tmp/adl-docker-install.log 2>&1
fi
sudo systemctl enable --now docker >/tmp/adl-docker-service.log 2>&1
DOCKER=(sudo docker)

if ! command -v aws >/dev/null 2>&1; then
  sudo dnf install -y awscli >/tmp/adl-awscli-install.log 2>&1 \
    || sudo yum install -y awscli >/tmp/adl-awscli-install.log 2>&1
fi

CURRENT_STAGE="pull_immutable_image"
stage "$CURRENT_STAGE"
REGISTRY="${IMAGE%%/*}"
if [[ "$REGISTRY" == *.dkr.ecr.*.amazonaws.com ]]; then
  aws ecr get-login-password --region "${ADL_REGION:-us-west-2}" \
    | "${DOCKER[@]}" login --username AWS --password-stdin "$REGISTRY" >/tmp/adl-ecr-login.log
fi
"${DOCKER[@]}" pull "$IMAGE" >/tmp/adl-builder-image-pull.log
IMAGE_ID="$("${DOCKER[@]}" image inspect --format '{{.Id}}' "$IMAGE")"
IMAGE_ARCH="$("${DOCKER[@]}" image inspect --format '{{.Architecture}}' "$IMAGE")"
case "$EXPECTED_ARCH" in
  x86_64|amd64) EXPECTED_DOCKER_ARCH="amd64"; EXPECTED_UNAME_ARCH="x86_64" ;;
  aarch64|arm64) EXPECTED_DOCKER_ARCH="arm64"; EXPECTED_UNAME_ARCH="aarch64" ;;
  *) echo "spot_builder_image_validation: unsupported expected architecture" >&2; exit 2 ;;
esac
if [[ "$IMAGE_ARCH" != "$EXPECTED_DOCKER_ARCH" ]]; then
  echo "spot_builder_image_validation: builder image architecture mismatch" >&2
  exit 1
fi
if [[ "$IMAGE_ID" != sha256:* ]]; then
  echo "spot_builder_image_validation: pulled image did not resolve to a content digest" >&2
  exit 1
fi

CURRENT_STAGE="verify_builder_toolchain"
stage "$CURRENT_STAGE"
TOOLCHAIN_OUTPUT="$ADL_RUN_ROOT/builder-toolchain.log"
"${DOCKER[@]}" run --rm --entrypoint /bin/bash "$IMAGE" -lc "
  set -euo pipefail
  test \"\$(uname -m)\" = '$EXPECTED_UNAME_ARCH'
  rustc --version
  cargo --version
  cargo nextest --version
  sccache --version
  ld.lld --version | head -n 1
  aws --version
" >"$TOOLCHAIN_OUTPUT" 2>&1
for required in rustc cargo cargo-nextest sccache LLD aws-cli; do
  grep -F "$required" "$TOOLCHAIN_OUTPUT" >/dev/null || {
    echo "spot_builder_image_validation: builder toolchain verification missing $required" >&2
    exit 1
  }
done

CURRENT_STAGE="validation_command"
stage "$CURRENT_STAGE"
VALIDATION_START="$(date +%s)"
VALIDATION_UID="$(id -u)"
VALIDATION_GID="$(id -g)"
"${DOCKER[@]}" run --rm \
  --user "$VALIDATION_UID:$VALIDATION_GID" \
  --workdir /workspace \
  --volume "$ADL_REMOTE_REPO_DIR:/workspace" \
  --volume "$CACHE_ROOT:/cache-root" \
  --volume "$TMP_DIR:/tmp" \
  --volume "$ADL_RUN_ROOT:/run-output" \
  --env CARGO_HOME=/cache-root/cargo-home \
  --env CARGO_TARGET_DIR=/cache-root/target \
  --env SCCACHE_DIR=/cache-root/sccache \
  --env AWS_EC2_METADATA_DISABLED=true \
  --env TMPDIR=/tmp \
  --env RUSTC_WRAPPER=sccache \
  --env RUSTFLAGS= \
  --env CARGO_INCREMENTAL=0 \
  --entrypoint /bin/bash \
  "$IMAGE" -lc "set +e; $COMMAND; status=\$?; sccache --show-stats > /run-output/sccache-stats.log 2>&1 || true; exit \$status"
VALIDATION_END="$(date +%s)"

CURRENT_STAGE="write_builder_summary"
stage "$CURRENT_STAGE"
IMAGE_DIGEST="${IMAGE##*@}"
export RESOLVED_REF IMAGE_DIGEST IMAGE_ARCH CACHE_SOURCE CACHE_FREE_BYTES
export VALIDATION_START VALIDATION_END
export CACHE_TARGET_PREEXISTING_ENTRIES CACHE_TARGET_PREEXISTING_BYTES
export CACHE_LOW_SPACE_RECOVERY
python3 - "$ADL_RUN_ROOT/spot-builder-summary.json" <<'PY'
import hashlib
import json
import os
import sys

out = sys.argv[1]
payload = {
    "schema": "adl.aws_spot_builder_image_validation.v1",
    "status": "passed",
    "source_commit": os.environ["RESOLVED_REF"],
    "source_commit_verified": True,
    "builder_image_digest_sha256": hashlib.sha256(os.environ["IMAGE_DIGEST"].encode()).hexdigest(),
    "builder_image_immutable": True,
    "builder_image_architecture": os.environ["IMAGE_ARCH"],
    "toolchain_verified": True,
    "cache_mount_verified": True,
    "cache_mount_source_sha256": hashlib.sha256(os.environ["CACHE_SOURCE"].encode()).hexdigest(),
    "cache_writable": True,
    "cache_free_bytes": int(os.environ["CACHE_FREE_BYTES"]),
    "cache_target_preexisting_entries": int(os.environ["CACHE_TARGET_PREEXISTING_ENTRIES"]),
    "cache_target_preexisting_bytes": int(os.environ["CACHE_TARGET_PREEXISTING_BYTES"]),
    "cache_low_space_recovery": os.environ["CACHE_LOW_SPACE_RECOVERY"] == "true",
    "validation_seconds": int(os.environ["VALIDATION_END"]) - int(os.environ["VALIDATION_START"]),
    "host_validation_tools_installed": False,
}
with open(out, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2, sort_keys=True)
    handle.write("\n")
print("ADL_SPOT_BUILDER_PROOF=" + json.dumps(payload, sort_keys=True, separators=(",", ":")))
PY
