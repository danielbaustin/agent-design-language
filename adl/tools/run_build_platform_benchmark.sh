#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  run_build_platform_benchmark.sh --platform <name> [options]

Options:
  --platform <name>        Platform label: wuji, nessus, aws_spot, codebuild.
  --cache-posture <text>   Cache posture label recorded in the summary.
  --out <path>             Summary JSON path. Default: .adl/tmp/build-platform-benchmark/<platform>/summary.json.
  --artifact-dir <path>    Log directory. Default: beside --out.
  -h, --help               Show this help.

Runs the WP-06 comparable Rust build/test workload:
  cargo build --manifest-path <adl>/Cargo.toml --locked --bin adl-pr-doctor
  cargo test  --manifest-path <adl>/Cargo.toml --locked --lib provider_communication -- --nocapture
USAGE
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 2
}

PLATFORM=""
CACHE_POSTURE="unspecified"
OUT_PATH=""
ARTIFACT_DIR=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --platform)
      [ "$#" -ge 2 ] || die "--platform requires a value"
      PLATFORM="$2"
      shift 2
      ;;
    --cache-posture)
      [ "$#" -ge 2 ] || die "--cache-posture requires a value"
      CACHE_POSTURE="$2"
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || die "--out requires a value"
      OUT_PATH="$2"
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

[ -n "$PLATFORM" ] || die "--platform is required"

if [ -f "adl/Cargo.toml" ]; then
  REPO_ROOT="$(pwd)"
  ADL_MANIFEST="adl/Cargo.toml"
elif [ -f "Cargo.toml" ] && [ "$(basename "$(pwd)")" = "adl" ]; then
  REPO_ROOT="$(cd .. && pwd)"
  ADL_MANIFEST="Cargo.toml"
else
  die "run from the ADL repo root or adl crate directory"
fi

if [ -z "$OUT_PATH" ]; then
  OUT_PATH="$REPO_ROOT/.adl/tmp/build-platform-benchmark/$PLATFORM/summary.json"
fi
if [ -z "$ARTIFACT_DIR" ]; then
  ARTIFACT_DIR="$(dirname "$OUT_PATH")"
fi

mkdir -p "$ARTIFACT_DIR" "$(dirname "$OUT_PATH")"
BUILD_STDOUT="$ARTIFACT_DIR/build.stdout.log"
BUILD_STDERR="$ARTIFACT_DIR/build.stderr.log"
TEST_STDOUT="$ARTIFACT_DIR/test.stdout.log"
TEST_STDERR="$ARTIFACT_DIR/test.stderr.log"

BUILD_COMMAND="cargo build --manifest-path $ADL_MANIFEST --locked --bin adl-pr-doctor"
TEST_COMMAND="cargo test --manifest-path $ADL_MANIFEST --locked --lib provider_communication -- --nocapture"

START_EPOCH="$(date +%s)"
/usr/bin/time -p cargo build --manifest-path "$ADL_MANIFEST" --locked --bin adl-pr-doctor >"$BUILD_STDOUT" 2>"$BUILD_STDERR"
AFTER_BUILD_EPOCH="$(date +%s)"
/usr/bin/time -p cargo test --manifest-path "$ADL_MANIFEST" --locked --lib provider_communication -- --nocapture >"$TEST_STDOUT" 2>"$TEST_STDERR"
END_EPOCH="$(date +%s)"

python3 - <<'PY' "$OUT_PATH" "$PLATFORM" "$CACHE_POSTURE" "$BUILD_COMMAND" "$TEST_COMMAND" "$START_EPOCH" "$AFTER_BUILD_EPOCH" "$END_EPOCH" "$BUILD_STDOUT" "$BUILD_STDERR" "$TEST_STDOUT" "$TEST_STDERR"
import json
import re
import sys
from pathlib import Path

(
    out_path,
    platform,
    cache_posture,
    build_command,
    test_command,
    start_epoch,
    after_build_epoch,
    end_epoch,
    build_stdout,
    build_stderr,
    test_stdout,
    test_stderr,
) = sys.argv[1:13]

def real_seconds(path):
    text = Path(path).read_text(encoding="utf-8", errors="replace")
    match = re.search(r"^real\s+([0-9.]+)$", text, re.MULTILINE)
    return float(match.group(1)) if match else None

build_real = real_seconds(build_stderr)
test_real = real_seconds(test_stderr)
summary = {
    "schema": "adl.build_platform_benchmark.v1",
    "platform": platform,
    "cache_posture": cache_posture,
    "status": "passed",
    "build_command": build_command,
    "test_command": test_command,
    "build_elapsed_seconds": int(after_build_epoch) - int(start_epoch),
    "test_elapsed_seconds": int(end_epoch) - int(after_build_epoch),
    "total_elapsed_seconds": int(end_epoch) - int(start_epoch),
    "build_real_seconds": build_real,
    "test_real_seconds": test_real,
    "logs": {
        "build_stdout": build_stdout,
        "build_stderr": build_stderr,
        "test_stdout": test_stdout,
        "test_stderr": test_stderr,
    },
}
Path(out_path).write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, indent=2, sort_keys=True))
print(
    "ADL_BUILD_PLATFORM_BENCHMARK "
    f"platform={platform} build_seconds={summary['build_elapsed_seconds']} "
    f"test_seconds={summary['test_elapsed_seconds']} total_seconds={summary['total_elapsed_seconds']} "
    "status=passed"
)
PY
