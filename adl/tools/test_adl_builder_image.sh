#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/setup_adl_builder_image.sh"
DOCKERFILE="$ROOT/adl/docker/adl-builder/Dockerfile"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_has() {
  local path="$1"
  local needle="$2"
  if ! grep -F "$needle" "$path" >/dev/null; then
    echo "expected $path to contain: $needle" >&2
    exit 1
  fi
}

assert_has "$DOCKERFILE" "FROM ubuntu:24.04"
assert_has "$DOCKERFILE" "ARG SCCACHE_VERSION=v0.16.0"
assert_has "$DOCKERFILE" "ARG CARGO_NEXTEST_VERSION=0.9.140"
assert_has "$DOCKERFILE" "CARGO_NEXTEST_X86_64_SHA256=4ee9aaa0d0171a985a5d0eb735b87355894c1c455972e9674fb9fdbd1387c9a3"
assert_has "$DOCKERFILE" "clang"
assert_has "$DOCKERFILE" "lld"
assert_has "$DOCKERFILE" "time"
assert_has "$DOCKERFILE" "awscli-exe-linux"
assert_has "$DOCKERFILE" "rustup component add rustfmt clippy"
assert_has "$DOCKERFILE" "nextest-rs/nextest"
assert_not_latest() {
  if grep -F "releases/latest" "$DOCKERFILE" >/dev/null; then
    echo "builder image must not resolve cargo-nextest from releases/latest" >&2
    exit 1
  fi
}
assert_not_latest
assert_has "$DOCKERFILE" "sha256sum -c -"
assert_has "$DOCKERFILE" "cargo nextest --version"
assert_has "$DOCKERFILE" "/usr/local/share/adl-builder-toolchain.txt"
assert_has "$DOCKERFILE" "RUSTC_WRAPPER=sccache"
assert_has "$DOCKERFILE" "CARGO_INCREMENTAL=0"
assert_has "$DOCKERFILE" "ENTRYPOINT [\"/bin/bash\", \"-lc\"]"

bash -n "$SCRIPT"

config="$TMP/config.txt"
bash "$SCRIPT" --image-uri "example.invalid/adl-builder:test" --print-config >"$config"
assert_has "$config" "builder_image=example.invalid/adl-builder:test"
assert_has "$config" "dockerfile=adl/docker/adl-builder/Dockerfile"
assert_has "$config" "docker_config=.adl/local/docker-config"
assert_has "$config" "platform=linux/amd64"
assert_has "$config" "aws_profile=agent-logic-admin"

default_config="$TMP/default-config.txt"
bash "$SCRIPT" --print-config >"$default_config"
assert_has "$default_config" "builder_image=adl-builder:v0.91.7-fixed"

env_file="$TMP/builder.env"
bash "$SCRIPT" --image-uri "example.invalid/adl-builder:test" --write-env "$env_file"
assert_has "$env_file" "ADL_BUILDER_IMAGE=example.invalid/adl-builder:test"
assert_has "$env_file" "ADL_AWS_CODEFRIEND_IMAGE=example.invalid/adl-builder:test"
assert_has "$env_file" "ADL_AWS_SPOT_BUILDER_IMAGE=example.invalid/adl-builder:test"
assert_has "$env_file" "ADL_NESSUS_BUILDER_IMAGE=example.invalid/adl-builder:test"
assert_has "$env_file" "ADL_LOCAL_BUILDER_IMAGE=example.invalid/adl-builder:test"

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"$FAKE_DOCKER_LOG"
EOF
chmod +x "$fake_bin/docker"

FAKE_DOCKER_LOG="$TMP/docker.log" PATH="$fake_bin:$PATH" \
  bash "$SCRIPT" \
    --docker-bin docker \
    --image-uri "example.invalid/adl-builder:test" \
    --platform linux/amd64 \
    --build
assert_has "$TMP/docker.log" "build --platform linux/amd64 -f $DOCKERFILE -t example.invalid/adl-builder:test $ROOT"

echo "PASS test_adl_builder_image"
