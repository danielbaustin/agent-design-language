#!/usr/bin/env python3
"""Focused tests for the deterministic Runtime v3 inventory."""

from __future__ import annotations

import importlib.util
import json
import subprocess
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
GENERATOR = Path(__file__).with_name("generate_runtime_inventory.py")
OUTPUT = ROOT / "docs/architecture/runtime_v3_current_inventory.v1.json"
BASELINE = ROOT / "docs/architecture/runtime_v3_baseline_modules.v1.json"

SPEC = importlib.util.spec_from_file_location("runtime_inventory", GENERATOR)
assert SPEC is not None and SPEC.loader is not None
runtime_inventory = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runtime_inventory)


class RuntimeInventoryTest(unittest.TestCase):
    def test_generated_artifact_matches_deterministic_inventory(self) -> None:
        first = runtime_inventory.render_inventory(
            runtime_inventory.build_inventory(ROOT)
        )
        second = runtime_inventory.render_inventory(
            runtime_inventory.build_inventory(ROOT)
        )
        self.assertEqual(first, second)
        self.assertEqual(first, OUTPUT.read_text(encoding="utf-8"))

    def test_inventory_uses_declared_tracked_sources(self) -> None:
        inventory = runtime_inventory.build_inventory(ROOT)
        tracked = set(runtime_inventory.tracked_files(ROOT))
        implementation = inventory["rust_implementation"]

        self.assertTrue(implementation["files"])
        self.assertEqual(implementation["files"], sorted(implementation["files"]))
        self.assertTrue(set(implementation["files"]).issubset(tracked))
        self.assertEqual(
            implementation["implementation_loc"],
            sum(
                len((ROOT / path).read_text(encoding="utf-8").splitlines())
                for path in implementation["files"]
            ),
        )
        auxiliary = inventory["selected_auxiliary_surface"]
        self.assertEqual(auxiliary["files"], runtime_inventory.AUXILIARY_IMPLEMENTATION_PATHS)
        self.assertEqual(
            auxiliary["combined_with_kernel_loc"],
            implementation["implementation_loc"] + auxiliary["implementation_loc"],
        )

        dependencies = inventory["direct_dependencies"]
        self.assertEqual(dependencies["names"], sorted(dependencies["names"]))
        self.assertEqual(dependencies["count"], len(dependencies["names"]))
        self.assertIn("libc", dependencies["names"])
        self.assertNotIn("tempfile", dependencies["names"])

        test_files = sorted(
            path
            for path in tracked
            if (
                path.startswith("adl-runtime-kernel/src/")
                or path.startswith("adl-runtime-kernel/tests/")
            )
            and path.endswith(".rs")
        )
        self.assertEqual(
            inventory["rust_tests"]["attribute_count"],
            sum(
                bool(runtime_inventory.TEST_ATTRIBUTE_RE.match(line))
                for path in test_files
                for line in (ROOT / path).read_text(encoding="utf-8").splitlines()
            ),
        )

        baseline = json.loads(BASELINE.read_text(encoding="utf-8"))
        self.assertIn(runtime_inventory.PARITY_BASELINE_PATH, tracked)
        self.assertEqual(
            inventory["parity_baseline"]["module_count"],
            len(baseline["modules"]),
        )
        self.assertEqual(
            len(baseline["modules"]),
            len(set(baseline["modules"])),
        )

    def test_check_mode_accepts_current_artifact(self) -> None:
        result = subprocess.run(
            [sys.executable, str(GENERATOR), "--check"],
            cwd=ROOT,
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
