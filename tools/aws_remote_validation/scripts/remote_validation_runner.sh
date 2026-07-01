#!/usr/bin/env bash
set -euo pipefail

: "${ADL_RUN_ID:?ADL_RUN_ID is required}"
: "${ADL_REMOTE_REPO_DIR:?ADL_REMOTE_REPO_DIR is required}"
: "${ADL_REMOTE_COMMAND:?ADL_REMOTE_COMMAND is required}"

RUN_ROOT="${ADL_RUN_ROOT:-/tmp/adl-aws-remote-validation/${ADL_RUN_ID}}"
PROGRESS_ROOT="${ADL_PROGRESS_ROOT:-$RUN_ROOT}"
WORK_ROOT="$RUN_ROOT"

if [ "${ADL_CACHE_VOLUME_ENABLED:-0}" = "1" ]; then
  CACHE_VOLUME_DEVICE_NAME="${ADL_CACHE_VOLUME_DEVICE_NAME:?ADL_CACHE_VOLUME_DEVICE_NAME is required}"
  CACHE_VOLUME_MOUNT_PATH="${ADL_CACHE_VOLUME_MOUNT_PATH:?ADL_CACHE_VOLUME_MOUNT_PATH is required}"
  CURRENT_STAGE="prepare_cache_volume"
  mkdir -p "$PROGRESS_ROOT"
  ROOT_SOURCE="$(findmnt -n -o SOURCE / || true)"
  ROOT_DISK="$(lsblk -no PKNAME "$ROOT_SOURCE" 2>/dev/null | head -n 1 || true)"
  resolve_cache_device() {
    local attempt candidate basename
    for attempt in $(seq 1 60); do
      for candidate in "$CACHE_VOLUME_DEVICE_NAME" /dev/nvme1n1 /dev/nvme2n1 /dev/xvdf /dev/xvdg; do
        [ -b "$candidate" ] || continue
        basename="$(basename "$candidate")"
        if [ -n "$ROOT_DISK" ] && [ "$basename" = "$ROOT_DISK" ]; then
          continue
        fi
        readlink -f "$candidate" 2>/dev/null || printf '%s\n' "$candidate"
        return 0
      done
      sleep 2
    done
    return 1
  }
  CACHE_DEVICE="$(resolve_cache_device)"
  sudo mkdir -p "$CACHE_VOLUME_MOUNT_PATH"
  if ! sudo blkid "$CACHE_DEVICE" >/dev/null 2>&1; then
    sudo mkfs.ext4 -F "$CACHE_DEVICE" >/tmp/adl-cache-volume-format.log 2>&1
  fi
  CACHE_UUID="$(sudo blkid -s UUID -o value "$CACHE_DEVICE")"
  if ! grep -q "$CACHE_UUID" /etc/fstab 2>/dev/null; then
    echo "UUID=$CACHE_UUID $CACHE_VOLUME_MOUNT_PATH ext4 defaults,nofail 0 2" | sudo tee -a /etc/fstab >/dev/null
  fi
  sudo mountpoint -q "$CACHE_VOLUME_MOUNT_PATH" || sudo mount "$CACHE_VOLUME_MOUNT_PATH"
  CACHE_OWNER_USER="$(id -un)"
  CACHE_OWNER_GROUP="$(id -gn)"
  sudo chown -R "$CACHE_OWNER_USER":"$CACHE_OWNER_GROUP" "$CACHE_VOLUME_MOUNT_PATH"
  WORK_ROOT="$CACHE_VOLUME_MOUNT_PATH/adl-aws-remote-validation/${ADL_RUN_ID}"
fi

TARGET_DIR="$WORK_ROOT/target"
SCCACHE_DIR="$WORK_ROOT/sccache"
mkdir -p "$RUN_ROOT" "$PROGRESS_ROOT" "$TARGET_DIR" "$SCCACHE_DIR"

BOOTSTRAP_START="$(date +%s)"
CURRENT_STAGE="bootstrap"

emit_debug_log() {
  local label="$1"
  local path="$2"
  if [ -f "$path" ]; then
    local line_count
    line_count="$(wc -l < "$path" 2>/dev/null || echo 0)"
    echo "ADL_REMOTE_LOG_BEGIN:$label" >&2
    sed -n '1,160p' "$path" >&2 || true
    if [ "$line_count" -gt 160 ]; then
      echo "ADL_REMOTE_LOG_MIDDLE_ELIDED:$label:$line_count" >&2
      tail -n 160 "$path" >&2 || true
    fi
    echo "ADL_REMOTE_LOG_END:$label" >&2
  fi
}

log_progress() {
  local message="$1"
  local timestamp
  timestamp="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  mkdir -p "$PROGRESS_ROOT"
  printf '%s %s\n' "$timestamp" "$message" | tee -a "$PROGRESS_ROOT/progress.log" >&2
}

on_error() {
  local exit_code="$?"
  echo "ADL_REMOTE_FAILURE_STAGE=$CURRENT_STAGE" >&2
  emit_debug_log rustup /tmp/adl-rustup.log
  emit_debug_log build_toolchain /tmp/adl-build-toolchain.log
  emit_debug_log sccache_install /tmp/adl-sccache-install.log
  emit_debug_log nextest_install /tmp/adl-nextest-install.log
  emit_debug_log command_stdout "$RUN_ROOT/command.log"
  emit_debug_log command_stderr "$RUN_ROOT/command.err"
  emit_debug_log sccache_stats "$RUN_ROOT/sccache-stats.log"
  exit "$exit_code"
}
trap on_error ERR

release_target_triple() {
  local arch
  arch="$(uname -m)"
  case "$arch" in
    x86_64) printf '%s\n' "x86_64-unknown-linux-musl" ;;
    aarch64|arm64) printf '%s\n' "aarch64-unknown-linux-musl" ;;
    *) return 1 ;;
  esac
}

install_github_release_binary() {
  local repo_name binary_name target api_url asset_url archive_path extract_dir release_bin
  repo_name="$1"
  binary_name="$2"
  if [ -n "${3:-}" ]; then
    target="$3"
  else
    target="$(release_target_triple)" || return 1
  fi
  api_url="https://api.github.com/repos/$repo_name/releases/latest"
  asset_url="$(curl -fsSL "$api_url" | python3 -c 'import json, sys
repo = sys.argv[1]
binary = sys.argv[2]
target = sys.argv[3]
data = json.load(sys.stdin)
for asset in data.get("assets", []):
    url = asset.get("browser_download_url", "")
    if binary in url and target in url and url.endswith(".tar.gz"):
        print(url)
        break
' "$repo_name" "$binary_name" "$target")"
  [ -n "$asset_url" ] || return 1
  archive_path="/tmp/adl-$binary_name-release.tar.gz"
  extract_dir="/tmp/adl-$binary_name-release"
  curl -fsSL "$asset_url" -o "$archive_path"
  rm -rf "$extract_dir"
  mkdir -p "$extract_dir"
  tar -xzf "$archive_path" -C "$extract_dir"
  release_bin="$(find "$extract_dir" -type f -name "$binary_name" | head -n 1)"
  [ -n "$release_bin" ] || return 1
  install -m 0755 "$release_bin" "$HOME/.cargo/bin/$binary_name"
}

install_sccache_release() {
  local target
  target="$(release_target_triple)" || return 1
  case "$target" in
    x86_64-unknown-linux-musl) target="x86_64-unknown-linux-gnu" ;;
    aarch64-unknown-linux-musl) target="aarch64-unknown-linux-gnu" ;;
    *) return 1 ;;
  esac
  install_github_release_binary "mozilla/sccache" "sccache" "$target"
}

ensure_aws_cli() {
  if command -v aws >/dev/null 2>&1; then
    return 0
  fi
  sudo dnf install -y awscli >/tmp/adl-awscli-install.log 2>&1 \
    || sudo yum install -y awscli >/tmp/adl-awscli-install.log 2>&1
}

archive_installed_binary() {
  local binary_name archive_path package_dir
  binary_name="$1"
  archive_path="$2"
  package_dir="/tmp/adl-$binary_name-package"
  rm -rf "$package_dir"
  mkdir -p "$package_dir"
  cp "$HOME/.cargo/bin/$binary_name" "$package_dir/$binary_name"
  tar -czf "$archive_path" -C "$package_dir" "$binary_name"
}

install_binary_from_tarball_url() {
  local binary_name tarball_url extract_dir archive_path release_bin
  binary_name="$1"
  tarball_url="$2"
  [ -n "$tarball_url" ] || return 1
  archive_path="/tmp/adl-$binary_name-cache.tar.gz"
  extract_dir="/tmp/adl-$binary_name-cache"
  curl -fsSL "$tarball_url" -o "$archive_path"
  rm -rf "$extract_dir"
  mkdir -p "$extract_dir"
  tar -xzf "$archive_path" -C "$extract_dir"
  release_bin="$(find "$extract_dir" -type f -name "$binary_name" | head -n 1)"
  [ -n "$release_bin" ] || return 1
  install -m 0755 "$release_bin" "$HOME/.cargo/bin/$binary_name"
}

install_binary_from_s3_cache() {
  local binary_name bucket prefix object_uri archive_path tool_prefix
  binary_name="$1"
  bucket="$2"
  prefix="$3"
  [ -n "$bucket" ] || return 1
  ensure_aws_cli || return 1
  archive_path="/tmp/adl-$binary_name-cache.tar.gz"
  tool_prefix="$prefix/tools"
  object_uri="s3://$bucket/$tool_prefix/$binary_name.tar.gz"
  aws s3 cp "$object_uri" "$archive_path" >/tmp/adl-$binary_name-s3-download.log 2>&1 || return 1
  install_binary_from_tarball_url "$binary_name" "file://$archive_path"
}

upload_binary_to_s3_cache() {
  local binary_name bucket prefix archive_path object_uri tool_prefix
  binary_name="$1"
  bucket="$2"
  prefix="$3"
  [ -n "$bucket" ] || return 0
  ensure_aws_cli || return 1
  archive_path="/tmp/adl-$binary_name-upload.tar.gz"
  tool_prefix="$prefix/tools"
  object_uri="s3://$bucket/$tool_prefix/$binary_name.tar.gz"
  archive_installed_binary "$binary_name" "$archive_path" || return 1
  aws s3 cp "$archive_path" "$object_uri"
}

verify_sccache_binary() {
  command -v sccache >/dev/null 2>&1 || return 1
  sccache --version >/dev/null 2>&1 || return 1
  sccache --start-server >/dev/null 2>&1 || return 1
  sccache --zero-stats >/dev/null 2>&1 || return 1
}

remove_installed_binary() {
  local binary_name
  binary_name="$1"
  rm -f "$HOME/.cargo/bin/$binary_name"
}

verify_nextest_binary() {
  cargo nextest --version >/dev/null 2>&1
}

install_nextest_release() {
  local target
  target="$(release_target_triple)" || return 1
  case "$target" in
    x86_64-unknown-linux-musl) target="x86_64-unknown-linux-gnu" ;;
    aarch64-unknown-linux-musl) target="aarch64-unknown-linux-gnu" ;;
    *) return 1 ;;
  esac
  install_github_release_binary "nextest-rs/nextest" "cargo-nextest" "$target"
}

export HOME="${HOME:-/root}"
CACHE_BUCKET="${ADL_CACHE_BUCKET:-}"
CACHE_PREFIX="${ADL_CACHE_PREFIX:-}"
SCCACHE_TARBALL_URL="${ADL_SCCACHE_TARBALL_URL:-}"
NEXTEST_TARBALL_URL="${ADL_NEXTEST_TARBALL_URL:-}"
NEEDS_NEXTEST="${ADL_NEEDS_NEXTEST:-0}"
REGION="${ADL_REGION:-us-west-2}"

CURRENT_STAGE="ensure_build_toolchain"
log_progress "stage=ensure_build_toolchain"
if ! command -v cc >/dev/null 2>&1; then
  sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config openssl-devel >/tmp/adl-build-toolchain.log 2>&1 \
    || sudo yum install -y gcc gcc-c++ make pkgconfig openssl-devel >/tmp/adl-build-toolchain.log 2>&1
fi

CURRENT_STAGE="ensure_rustup"
log_progress "stage=ensure_rustup"
if ! command -v cargo >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal >/tmp/adl-rustup.log 2>&1
fi
if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$TARGET_DIR"
export SCCACHE_DIR="$SCCACHE_DIR"
if [ -n "$CACHE_BUCKET" ]; then
  export SCCACHE_BUCKET="$CACHE_BUCKET"
  export SCCACHE_REGION="$REGION"
  export SCCACHE_S3_KEY_PREFIX="$CACHE_PREFIX/sccache"
fi

CURRENT_STAGE="ensure_sccache"
log_progress "stage=ensure_sccache"
if ! command -v sccache >/dev/null 2>&1; then
  SCCACHE_CACHE_HIT=0
  if install_binary_from_s3_cache sccache "$CACHE_BUCKET" "$CACHE_PREFIX" >/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_binary_from_tarball_url sccache "$SCCACHE_TARBALL_URL" >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_sccache_release >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    :
  else
    remove_installed_binary sccache
    cargo install sccache --locked --force >>/tmp/adl-sccache-install.log 2>&1
    verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1
  fi
  if [ "$SCCACHE_CACHE_HIT" -eq 0 ]; then
    upload_binary_to_s3_cache sccache "$CACHE_BUCKET" "$CACHE_PREFIX" >>/tmp/adl-sccache-install.log 2>&1 || true
  fi
fi

CURRENT_STAGE="ensure_nextest"
log_progress "stage=ensure_nextest"
if [ "$NEEDS_NEXTEST" = "1" ] && ! cargo nextest --version >/dev/null 2>&1; then
  NEXTEST_CACHE_HIT=0
  if install_binary_from_s3_cache cargo-nextest "$CACHE_BUCKET" "$CACHE_PREFIX" >/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_binary_from_tarball_url cargo-nextest "$NEXTEST_TARBALL_URL" >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_nextest_release >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    :
  else
    cargo install cargo-nextest --locked >>/tmp/adl-nextest-install.log 2>&1
    verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1
  fi
  if [ "$NEXTEST_CACHE_HIT" -eq 0 ]; then
    upload_binary_to_s3_cache cargo-nextest "$CACHE_BUCKET" "$CACHE_PREFIX" >>/tmp/adl-nextest-install.log 2>&1 || true
  fi
fi
export RUSTC_WRAPPER="sccache"

RESOLVED_COMMIT="$(git -C "$ADL_REMOTE_REPO_DIR" rev-parse HEAD)"
RUSTC_VERSION="$(rustc --version 2>/dev/null || true)"
CARGO_VERSION="$(cargo --version 2>/dev/null || true)"
SCCACHE_VERSION="$(sccache --version 2>/dev/null || true)"
sccache --start-server >/dev/null 2>&1 || true
sccache --zero-stats >/dev/null 2>&1 || true
SCCACHE_DEGRADED=0
SCCACHE_DEGRADED_REASON=""

watch_sccache_health() {
  while true; do
    if ! sccache --show-stats >/dev/null 2>&1; then
      printf '%s sccache_watch_restart\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$RUN_ROOT/sccache-watch.log"
      sccache --start-server >/dev/null 2>&1 || true
    fi
    sleep 5
  done
}
watch_sccache_health >/tmp/adl-sccache-watch.log 2>&1 &
SCCACHE_WATCH_PID="$!"

INTERRUPTION_NOTICE=""
watch_spot_notice() {
  while true; do
    TOKEN="$(curl -sS -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 60' || true)"
    if [ -n "$TOKEN" ]; then
      NOTICE="$(curl -fsS -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/spot/instance-action || true)"
    else
      NOTICE="$(curl -fsS http://169.254.169.254/latest/meta-data/spot/instance-action || true)"
    fi
    if [ -n "$NOTICE" ]; then
      printf '%s\n' "$NOTICE" > "$RUN_ROOT/spot-interruption.log"
      break
    fi
    sleep 5
  done
}
watch_spot_notice >/tmp/adl-spot-watch.log 2>&1 &
WATCH_PID="$!"

BOOTSTRAP_END="$(date +%s)"
COMMAND_START="$(date +%s)"
CURRENT_STAGE="validation_command"
log_progress "stage=validation_command command=${ADL_REMOTE_COMMAND}"
set +e
( cd "$ADL_REMOTE_REPO_DIR" && bash -lc "$ADL_REMOTE_COMMAND" ) >"$RUN_ROOT/command.log" 2>"$RUN_ROOT/command.err"
COMMAND_EXIT="$?"
set -e
COMMAND_END="$(date +%s)"
kill "$WATCH_PID" >/dev/null 2>&1 || true
wait "$WATCH_PID" >/dev/null 2>&1 || true
kill "$SCCACHE_WATCH_PID" >/dev/null 2>&1 || true
wait "$SCCACHE_WATCH_PID" >/dev/null 2>&1 || true
sccache --show-stats >"$RUN_ROOT/sccache-stats.log" 2>&1 || true
[ -f "$RUN_ROOT/spot-interruption.log" ] && INTERRUPTION_NOTICE="$(cat "$RUN_ROOT/spot-interruption.log")"
if grep -Fq "sccache: warning: The server looks like it shut down unexpectedly" "$RUN_ROOT/command.err"; then
  SCCACHE_DEGRADED=1
  SCCACHE_DEGRADED_REASON="server_shut_down_unexpectedly"
elif grep -Fq "sccache: error:" "$RUN_ROOT/command.err"; then
  SCCACHE_DEGRADED=1
  SCCACHE_DEGRADED_REASON="client_or_server_error"
fi
if [ ! -s "$RUN_ROOT/sccache-stats.log" ]; then
  SCCACHE_DEGRADED=1
  if [ -z "$SCCACHE_DEGRADED_REASON" ]; then
    SCCACHE_DEGRADED_REASON="missing_stats"
  fi
fi

export ADL_RUN_ROOT="$RUN_ROOT"
export COMMAND_EXIT BOOTSTRAP_START BOOTSTRAP_END COMMAND_START COMMAND_END
export INTERRUPTION_NOTICE RESOLVED_COMMIT RUSTC_VERSION CARGO_VERSION SCCACHE_VERSION
export SCCACHE_DEGRADED SCCACHE_DEGRADED_REASON
python3 - <<'PY'
import json
import os
from pathlib import Path
run_root = Path(os.environ["ADL_RUN_ROOT"])
payload = {
  "status": "passed" if int(os.environ["COMMAND_EXIT"]) == 0 else "failed",
  "bootstrap_seconds": int(os.environ["BOOTSTRAP_END"]) - int(os.environ["BOOTSTRAP_START"]),
  "command_seconds": int(os.environ["COMMAND_END"]) - int(os.environ["COMMAND_START"]),
  "interruption_detected": bool(os.environ.get("INTERRUPTION_NOTICE", "")),
  "interruption_notice": os.environ.get("INTERRUPTION_NOTICE") or None,
  "resolved_commit": os.environ.get("RESOLVED_COMMIT") or None,
  "rustc_version": os.environ.get("RUSTC_VERSION") or None,
  "cargo_version": os.environ.get("CARGO_VERSION") or None,
  "sccache_version": os.environ.get("SCCACHE_VERSION") or None,
  "sccache_degraded": os.environ.get("SCCACHE_DEGRADED") == "1",
  "sccache_degraded_reason": os.environ.get("SCCACHE_DEGRADED_REASON") or None,
  "sccache_stats": {"raw_excerpt": run_root.joinpath("sccache-stats.log").read_text(errors="replace").splitlines()[:16] if run_root.joinpath("sccache-stats.log").exists() else []}
}
print("ADL_AWS_REMOTE_SUMMARY_BEGIN")
print(json.dumps(payload))
print("ADL_AWS_REMOTE_SUMMARY_END")
PY
cat "$RUN_ROOT/command.log"
cat "$RUN_ROOT/command.err" >&2
exit "$COMMAND_EXIT"
