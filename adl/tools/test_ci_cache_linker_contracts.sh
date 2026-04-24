#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKFLOW="$ROOT_DIR/.github/workflows/ci.yaml"

python3 - "$WORKFLOW" <<'PY'
import pathlib
import sys

workflow = pathlib.Path(sys.argv[1]).read_text()

required_fragments = [
    "cache-directories: |\n            ~/.cache/sccache",
    "tool: sccache",
    'echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"',
    'echo "SCCACHE_DIR=$HOME/.cache/sccache" >> "$GITHUB_ENV"',
    "command -v ld.lld >/dev/null 2>&1",
    'echo "RUSTFLAGS=-C link-arg=-fuse-ld=lld" >> "$GITHUB_ENV"',
    'sccache --zero-stats || true',
    'sccache --show-stats || true',
]

missing = [frag for frag in required_fragments if frag not in workflow]
if missing:
    raise SystemExit(
        "ci cache/linker posture drifted; missing workflow fragments:\n- "
        + "\n- ".join(missing)
    )

for step in [
    "Install sccache",
    "Configure Rust acceleration",
    "Install sccache for coverage",
    "Configure Rust acceleration for coverage",
    "Rust acceleration stats",
    "Rust acceleration stats for coverage",
]:
    if f"- name: {step}" not in workflow:
        raise SystemExit(f"missing workflow step: {step}")

print("PASS test_ci_cache_linker_contracts")
PY
