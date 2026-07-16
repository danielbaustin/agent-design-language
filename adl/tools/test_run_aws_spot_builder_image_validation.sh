#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_builder_image_validation.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

FAKE_BIN="$TMP/fake-bin"
RUN_ROOT="$TMP/run"
CACHE_MOUNT="$TMP/cache"
mkdir -p "$FAKE_BIN" "$RUN_ROOT" "$CACHE_MOUNT"

cat >"$FAKE_BIN/mountpoint" <<'EOF'
#!/usr/bin/env bash
[[ "${ADL_FAKE_MOUNT_OK:-1}" == "1" ]]
EOF

cat >"$FAKE_BIN/findmnt" <<'EOF'
#!/usr/bin/env bash
case "${*: -1}" in
  /) echo /dev/root ;;
  *) echo /dev/fake-retained-cache ;;
esac
EOF

cat >"$FAKE_BIN/df" <<'EOF'
#!/usr/bin/env bash
printf 'Filesystem 1-blocks Used Available Capacity Mounted on\n'
printf '/dev/fake 100000000000 1 %s 1%% /cache\n' "${ADL_FAKE_CACHE_FREE_BYTES:-90000000000}"
EOF

cat >"$FAKE_BIN/systemctl" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF

cat >"$FAKE_BIN/sudo" <<'EOF'
#!/usr/bin/env bash
exec "$@"
EOF

cat >"$FAKE_BIN/aws" <<'EOF'
#!/usr/bin/env bash
if [[ "$1 $2" == "ecr get-login-password" ]]; then
  echo fake-password
  exit 0
fi
echo "unexpected aws command: $*" >&2
exit 1
EOF

cat >"$FAKE_BIN/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in
  login) cat >/dev/null; exit 0 ;;
  pull) exit 0 ;;
  image)
    if [[ "$2" != "inspect" ]]; then exit 2; fi
    case "$4" in
      *Architecture*) echo "${ADL_FAKE_IMAGE_ARCH:-amd64}" ;;
      *) echo sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ;;
    esac
    exit 0
    ;;
  run)
    args="$*"
    if [[ "$args" == *"rustc --version"* ]]; then
      if [[ "${ADL_FAKE_TOOLCHAIN_OK:-1}" != "1" ]]; then
        echo "rustc 1.96.0"
        exit 0
      fi
      cat <<'TOOLS'
rustc 1.96.0
cargo 1.96.0
cargo-nextest 0.9.140
sccache 0.16.0
Ubuntu LLD 18.1.3
aws-cli/2.35.15
TOOLS
      exit 0
    fi
    [[ "$args" == *"--env RUSTFLAGS= --env CARGO_INCREMENTAL=0"* ]] || {
      echo "validation container did not preserve the known-good Rust flags" >&2
      exit 2
    }
    [[ "$args" == *"--user "* && "$args" == *"AWS_EC2_METADATA_DISABLED=true"* ]] || {
      echo "validation container did not isolate root permissions and EC2 role discovery" >&2
      exit 2
    }
    [[ "$args" == *":/cache-root"* && "$args" == *"CARGO_TARGET_DIR=/cache-root/target"* && "$args" == *"SCCACHE_DIR=/cache-root/sccache"* && "$args" == *"CARGO_HOME=/cache-root/cargo-home"* ]] || {
      echo "validation container did not preserve the known-good cache layout" >&2
      exit 2
    }
    [[ "$args" == *"/adl-aws-remote-validation/shared/tmp:/tmp"* && "$args" == *"TMPDIR=/tmp"* ]] || {
      echo "validation container did not mount EBS-backed temp space" >&2
      exit 2
    }
    run_root=""
    previous=""
    for arg in "$@"; do
      if [[ "$previous" == "--volume" && "$arg" == *:/run-output ]]; then
        run_root="${arg%:/run-output}"
      fi
      previous="$arg"
    done
    [[ -n "$run_root" ]]
    echo 'Compile requests                     10' >"$run_root/sccache-stats.log"
    exit "${ADL_FAKE_VALIDATION_EXIT:-0}"
    ;;
esac
echo "unexpected docker command: $*" >&2
exit 1
EOF
chmod +x "$FAKE_BIN"/*

commit="$(git -C "$ROOT" rev-parse HEAD)"
digest="sha256:$(printf 'a%.0s' {1..64})"
image="123456789012.dkr.ecr.us-west-2.amazonaws.com/adl-builder@$digest"

run_fixture() {
  local command="${1:-cargo nextest run --workspace}"
  PATH="$FAKE_BIN:$PATH" \
  ADL_REMOTE_REPO_DIR="$ROOT" \
  ADL_RUN_ROOT="$RUN_ROOT" \
  ADL_CACHE_VOLUME_MOUNT_PATH="$CACHE_MOUNT" \
  ADL_REGION=us-west-2 \
  bash "$SCRIPT" \
    --image "$image" \
    --expected-ref "$commit" \
    --command "$command"
}

run_fixture >"$TMP/pass.out" 2>"$TMP/pass.err"
grep -F 'ADL_SPOT_BUILDER_PROOF=' "$TMP/pass.out" >/dev/null
test -f "$RUN_ROOT/validation-command.stdout.log"
test -f "$RUN_ROOT/validation-command.stderr.log"
python3 - "$RUN_ROOT/spot-builder-summary.json" "$commit" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
assert payload["status"] == "passed"
assert payload["source_commit"] == sys.argv[2]
assert payload["source_commit_verified"] is True
assert payload["builder_image_immutable"] is True
assert payload["toolchain_verified"] is True
assert payload["cache_mount_verified"] is True
assert payload["host_validation_tools_installed"] is False
assert payload["cache_mount_source_sha256"]
PY

if PATH="$FAKE_BIN:$PATH" ADL_REMOTE_REPO_DIR="$ROOT" ADL_RUN_ROOT="$RUN_ROOT" \
  ADL_CACHE_VOLUME_MOUNT_PATH="$CACHE_MOUNT" bash "$SCRIPT" \
  --image 'example.invalid/adl-builder:mutable' --expected-ref "$commit" --command true \
  >"$TMP/mutable.out" 2>"$TMP/mutable.err"; then
  echo "expected mutable image to fail" >&2
  exit 1
fi
grep -F 'immutable sha256 digest' "$TMP/mutable.err" >/dev/null

wrong_ref="$(printf 'f%.0s' {1..40})"
if PATH="$FAKE_BIN:$PATH" ADL_REMOTE_REPO_DIR="$ROOT" ADL_RUN_ROOT="$RUN_ROOT" \
  ADL_CACHE_VOLUME_MOUNT_PATH="$CACHE_MOUNT" bash "$SCRIPT" \
  --image "$image" --expected-ref "$wrong_ref" --command true \
  >"$TMP/ref.out" 2>"$TMP/ref.err"; then
  echo "expected wrong source ref to fail" >&2
  exit 1
fi
grep -F 'resolved source commit does not match' "$TMP/ref.err" >/dev/null

if ADL_FAKE_MOUNT_OK=0 run_fixture >"$TMP/mount.out" 2>"$TMP/mount.err"; then
  echo "expected missing cache mount to fail" >&2
  exit 1
fi
grep -F 'not a mountpoint' "$TMP/mount.err" >/dev/null

if ADL_FAKE_CACHE_FREE_BYTES=1024 run_fixture >"$TMP/space.out" 2>"$TMP/space.err"; then
  echo "expected insufficient cache headroom to fail" >&2
  exit 1
fi
grep -F 'insufficient free space' "$TMP/space.err" >/dev/null

ADL_FAKE_CACHE_FREE_BYTES=1024 run_fixture \
  'cargo clean --manifest-path adl/Cargo.toml' \
  >"$TMP/clean.out" 2>"$TMP/clean.err"
grep -F 'low-space target cleanup recovery authorized' "$TMP/clean.err" >/dev/null

if ADL_FAKE_TOOLCHAIN_OK=0 run_fixture >"$TMP/tool.out" 2>"$TMP/tool.err"; then
  echo "expected missing builder tool to fail" >&2
  exit 1
fi
grep -F 'builder toolchain verification missing' "$TMP/tool.err" >/dev/null

if ADL_FAKE_IMAGE_ARCH=arm64 run_fixture >"$TMP/arch.out" 2>"$TMP/arch.err"; then
  echo "expected wrong image architecture to fail" >&2
  exit 1
fi
grep -F 'builder image architecture mismatch' "$TMP/arch.err" >/dev/null

if ADL_FAKE_VALIDATION_EXIT=17 run_fixture >"$TMP/validation.out" 2>"$TMP/validation.err"; then
  echo "expected validation failure to propagate" >&2
  exit 1
fi
grep -F 'ADL_SPOT_BUILDER_PROOF=' "$TMP/validation.out" >/dev/null
python3 - "$RUN_ROOT/spot-builder-summary.json" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
assert payload["status"] == "failed"
assert payload["validation_exit_code"] == 17
assert payload["builder_image_immutable"] is True
assert payload["toolchain_verified"] is True
assert payload["cache_mount_verified"] is True
PY

echo "PASS test_run_aws_spot_builder_image_validation"
