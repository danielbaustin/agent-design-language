#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKFLOW="$ROOT_DIR/.github/workflows/ci.yaml"

python3 - "$WORKFLOW" <<'PY'
import pathlib
import re
import sys

workflow = pathlib.Path(sys.argv[1]).read_text()

def step_run(name: str) -> str:
    pattern = re.compile(
        rf"^\s*-\s+name:\s+{re.escape(name)}\s*$"
        rf"(?:\n^\s+.*$)*?"
        rf"\n^\s+run:\s+(.+)$",
        re.MULTILINE,
    )
    match = pattern.search(workflow)
    if not match:
        raise SystemExit(f"missing workflow step: {name}")
    return match.group(1).strip()

ordinary_test = step_run("test")
if ordinary_test != "cargo nextest run --status-level pass --final-status-level slow --slow-time 60s":
    raise SystemExit(
        "ordinary adl-ci test lane must be 'cargo nextest run --status-level pass --final-status-level slow --slow-time 60s' without --all-features; "
        f"found: {ordinary_test}"
    )

ordinary_doc_test = step_run("doc test")
if ordinary_doc_test != "cargo test --doc":
    raise SystemExit(
        "ordinary adl-ci doc-test lane must be 'cargo test --doc' without --all-features; "
        f"found: {ordinary_doc_test}"
    )

coverage_run = step_run("Coverage run and summary (json)")
expected_coverage = (
    "cargo llvm-cov --workspace --all-features --json --summary-only "
    "--output-path coverage-summary.json"
)
if coverage_run != expected_coverage:
    raise SystemExit(
        "authoritative coverage lane must retain full all-features llvm-cov command; "
        f"found: {coverage_run}"
    )

print("PASS test_ci_runtime_contracts")
PY
