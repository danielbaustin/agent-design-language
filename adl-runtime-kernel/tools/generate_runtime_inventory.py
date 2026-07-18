#!/usr/bin/env python3
"""Generate the tracked Runtime v3 implementation inventory."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
GENERATOR_PATH = "adl-runtime-kernel/tools/generate_runtime_inventory.py"
MANIFEST_PATH = "adl-runtime-kernel/Cargo.toml"
OUTPUT_PATH = "docs/architecture/runtime_v3_current_inventory.v1.json"
PARITY_BASELINE_PATH = "docs/architecture/runtime_v3_baseline_modules.v1.json"
AUXILIARY_IMPLEMENTATION_PATHS = ["adl-runtime/src/guardian.rs"]

TEST_ATTRIBUTE_RE = re.compile(
    r"^\s*#\s*\[\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*test"
    r"(?:\s*\([^]]*\))?\s*\]\s*$"
)


def fail(message: str) -> None:
    print(f"runtime_inventory: {message}", file=sys.stderr)
    raise SystemExit(2)


def tracked_files(root: Path) -> list[str]:
    result = subprocess.run(
        ["git", "-C", str(root), "ls-files", "-z"],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        fail(f"git ls-files failed: {result.stderr.decode().strip()}")
    return sorted(
        path.decode()
        for path in result.stdout.split(b"\0")
        if path
    )


def require_tracked(tracked: set[str], path: str) -> None:
    if path not in tracked:
        fail(f"required input is not tracked: {path}")


def logical_line_count(path: Path) -> int:
    return len(path.read_text(encoding="utf-8").splitlines())


def direct_dependency_names(manifest: dict[str, Any]) -> list[str]:
    names = set(manifest.get("dependencies", {}))
    for target in manifest.get("target", {}).values():
        if isinstance(target, dict):
            dependencies = target.get("dependencies", {})
            if isinstance(dependencies, dict):
                names.update(dependencies)
    return sorted(names)


def build_inventory(root: Path = ROOT) -> dict[str, Any]:
    tracked = set(tracked_files(root))
    require_tracked(tracked, MANIFEST_PATH)
    require_tracked(tracked, PARITY_BASELINE_PATH)
    for path in AUXILIARY_IMPLEMENTATION_PATHS:
        require_tracked(tracked, path)

    implementation_files = sorted(
        path
        for path in tracked
        if path.startswith("adl-runtime-kernel/src/") and path.endswith(".rs")
    )
    crate_rust_files = sorted(
        path
        for path in tracked
        if (
            path.startswith("adl-runtime-kernel/src/")
            or path.startswith("adl-runtime-kernel/tests/")
        )
        and path.endswith(".rs")
    )
    if not implementation_files:
        fail("no tracked Runtime v3 implementation files found")

    manifest = tomllib.loads((root / MANIFEST_PATH).read_text(encoding="utf-8"))
    dependencies = direct_dependency_names(manifest)

    test_attribute_count = 0
    for path in crate_rust_files:
        lines = (root / path).read_text(encoding="utf-8").splitlines()
        test_attribute_count += sum(bool(TEST_ATTRIBUTE_RE.match(line)) for line in lines)

    baseline = json.loads((root / PARITY_BASELINE_PATH).read_text(encoding="utf-8"))
    if baseline.get("schema") != "adl.runtime_v3.baseline_modules.v1":
        fail(f"unsupported parity baseline schema in {PARITY_BASELINE_PATH}")
    modules = baseline.get("modules")
    if not isinstance(modules, list) or not all(isinstance(path, str) for path in modules):
        fail(f"{PARITY_BASELINE_PATH} must contain a string modules array")
    if len(modules) != len(set(modules)):
        fail(f"{PARITY_BASELINE_PATH} contains duplicate module paths")

    kernel_loc = sum(logical_line_count(root / path) for path in implementation_files)
    auxiliary_loc = sum(
        logical_line_count(root / path) for path in AUXILIARY_IMPLEMENTATION_PATHS
    )
    return {
        "schema": "adl.runtime_v3.current_inventory.v1",
        "generator": GENERATOR_PATH,
        "tracked_files_only": True,
        "rust_implementation": {
            "root": "adl-runtime-kernel/src",
            "files": implementation_files,
            "file_count": len(implementation_files),
            "implementation_loc": kernel_loc,
            "measurement": (
                "logical lines across tracked Rust files under the implementation root"
            ),
        },
        "selected_auxiliary_surface": {
            "files": AUXILIARY_IMPLEMENTATION_PATHS,
            "implementation_loc": auxiliary_loc,
            "combined_with_kernel_loc": kernel_loc + auxiliary_loc,
            "budget_disposition": (
                "The 12,000-line challenge applies to the independent Runtime v3 kernel. "
                "The selected shared guardian is reported separately because its source "
                "contains the reusable process contract and co-located unit tests."
            ),
        },
        "direct_dependencies": {
            "manifest": MANIFEST_PATH,
            "names": dependencies,
            "count": len(dependencies),
            "scope": "[dependencies] plus target-specific [dependencies]; excludes dev-dependencies",
        },
        "rust_tests": {
            "roots": ["adl-runtime-kernel/src", "adl-runtime-kernel/tests"],
            "attribute_count": test_attribute_count,
            "measurement": (
                "tracked Rust attributes named test, including qualified forms such as "
                "tokio::test"
            ),
        },
        "parity_baseline": {
            "artifact": PARITY_BASELINE_PATH,
            "schema": baseline["schema"],
            "module_count": len(modules),
        },
    }


def render_inventory(inventory: dict[str, Any]) -> str:
    return json.dumps(inventory, indent=2, ensure_ascii=True) + "\n"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if the tracked inventory artifact is stale",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    rendered = render_inventory(build_inventory())
    output = ROOT / OUTPUT_PATH

    if args.check:
        if not output.exists() or output.read_text(encoding="utf-8") != rendered:
            print(f"FAIL: {OUTPUT_PATH} is stale; rerun {GENERATOR_PATH}")
            return 1
        print(f"PASS: {OUTPUT_PATH} is current")
        return 0

    output.write_text(rendered, encoding="utf-8")
    print(f"WROTE {OUTPUT_PATH}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
