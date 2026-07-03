#!/usr/bin/env python3
"""Focused contract tests for warm_rust_dependency_cache.py."""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
HELPER = ROOT / "adl" / "tools" / "warm_rust_dependency_cache.py"


def run_helper(*args: str) -> dict:
    result = subprocess.run(
        [sys.executable, str(HELPER), *args, "--json"],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            f"helper failed with {result.returncode}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return json.loads(result.stdout)


def write_fixture(root: Path) -> tuple[Path, Path, Path]:
    project = root / "project"
    adl = project / "adl"
    member = adl / "crates" / "helper"
    source_target = root / "source-target"
    deps = source_target / "debug" / "deps"
    build = source_target / "debug" / "build"
    fingerprint = source_target / "debug" / ".fingerprint"

    member.mkdir(parents=True)
    deps.mkdir(parents=True)
    build.mkdir(parents=True)
    fingerprint.mkdir(parents=True)

    (adl / "Cargo.toml").write_text(
        "\n".join(
            [
                "[package]",
                'name = "adl"',
                'version = "0.1.0"',
                'edition = "2021"',
                "",
                "[workspace]",
                'members = ["crates/helper"]',
                "",
            ]
        )
    )
    (member / "Cargo.toml").write_text(
        "\n".join(
            [
                "[package]",
                'name = "helper"',
                'version = "0.1.0"',
                'edition = "2021"',
                "",
            ]
        )
    )
    (adl / "Cargo.lock").write_text(
        "\n".join(
            [
                "version = 4",
                "",
                "[[package]]",
                'name = "adl"',
                'version = "0.1.0"',
                "",
                "[[package]]",
                'name = "helper"',
                'version = "0.1.0"',
                "",
                "[[package]]",
                'name = "serde"',
                'version = "1.0.0"',
                "",
                "[[package]]",
                'name = "proc-macro2"',
                'version = "1.0.0"',
                "",
            ]
        )
    )

    for name, content in {
        "libserde-abc.rlib": "serde rlib",
        "serde-abc.d": "serde depinfo",
        "libproc_macro2-def.rmeta": "proc macro metadata",
        "libadl-aaa.rlib": "workspace root output",
        "libhelper-bbb.rlib": "workspace member output",
        "unmatched-file": "not a cargo dependency artifact",
    }.items():
        (deps / name).write_text(content)

    (build / "libserde-build-output").write_text("must not link build output")
    serde_fingerprint = fingerprint / "serde-abc"
    helper_fingerprint = fingerprint / "helper-bbb"
    unmatched_fingerprint = fingerprint / "unmatched-ccc"
    serde_fingerprint.mkdir()
    helper_fingerprint.mkdir()
    unmatched_fingerprint.mkdir()
    (serde_fingerprint / "lib-serde").write_text("serde fingerprint")
    (serde_fingerprint / "dep-lib-serde").write_text("serde dep fingerprint")
    (serde_fingerprint / "serde-abc.d").write_text("must skip dep info")
    (helper_fingerprint / "lib-helper").write_text("must not link workspace fingerprint")
    (unmatched_fingerprint / "lib-unmatched").write_text("must not link unmatched fingerprint")
    return adl / "Cargo.toml", source_target, root / "dest-target"


def assert_same_inode(a: Path, b: Path) -> None:
    a_stat = a.stat()
    b_stat = b.stat()
    if (a_stat.st_dev, a_stat.st_ino) != (b_stat.st_dev, b_stat.st_ino):
        raise AssertionError(f"expected hardlink inode match: {a} {b}")


def test_links_only_external_dependency_deps() -> None:
    with tempfile.TemporaryDirectory() as temp:
        manifest, source_target, dest_target = write_fixture(Path(temp))
        summary = run_helper(
            "--source-target",
            str(source_target),
            "--dest-target",
            str(dest_target),
            "--manifest-path",
            str(manifest),
        )

        if summary["status"] != "ok":
            raise AssertionError(summary)
        if summary["linked_files"] != 4:
            raise AssertionError(f"expected 2 linked dependency files: {summary}")
        if summary["linked_fingerprint_files"] != 2:
            raise AssertionError(f"expected dependency fingerprint files to be linked: {summary}")
        if summary["skipped_dep_info"] != 2:
            raise AssertionError(f"expected dep-info files to be skipped: {summary}")

        for name in ["libserde-abc.rlib", "libproc_macro2-def.rmeta"]:
            assert_same_inode(source_target / "debug" / "deps" / name, dest_target / "debug" / "deps" / name)

        for name in ["serde-abc.d", "libadl-aaa.rlib", "libhelper-bbb.rlib", "unmatched-file"]:
            if (dest_target / "debug" / "deps" / name).exists():
                raise AssertionError(f"unexpected workspace or unmatched artifact linked: {name}")

        if (dest_target / "debug" / "build").exists():
            raise AssertionError("build directory must not be linked")
        for name in ["lib-serde", "dep-lib-serde"]:
            assert_same_inode(
                source_target / "debug" / ".fingerprint" / "serde-abc" / name,
                dest_target / "debug" / ".fingerprint" / "serde-abc" / name,
            )
        for name in [
            "serde-abc/serde-abc.d",
            "helper-bbb/lib-helper",
            "unmatched-ccc/lib-unmatched",
        ]:
            if (dest_target / "debug" / ".fingerprint" / name).exists():
                raise AssertionError(f"unexpected fingerprint artifact linked: {name}")


def test_replace_semantics_are_explicit() -> None:
    with tempfile.TemporaryDirectory() as temp:
        manifest, source_target, dest_target = write_fixture(Path(temp))
        dest_deps = dest_target / "debug" / "deps"
        dest_deps.mkdir(parents=True)
        existing = dest_deps / "libserde-abc.rlib"
        existing.write_text("old file")

        no_replace = run_helper(
            "--source-target",
            str(source_target),
            "--dest-target",
            str(dest_target),
            "--manifest-path",
            str(manifest),
        )
        if no_replace["skipped_existing"] < 1:
            raise AssertionError(no_replace)
        if existing.read_text() != "old file":
            raise AssertionError("existing destination changed without --replace")

        replaced = run_helper(
            "--source-target",
            str(source_target),
            "--dest-target",
            str(dest_target),
            "--manifest-path",
            str(manifest),
            "--replace",
        )
        if replaced["status"] != "ok":
            raise AssertionError(replaced)
        assert_same_inode(source_target / "debug" / "deps" / "libserde-abc.rlib", existing)


def main() -> int:
    test_links_only_external_dependency_deps()
    test_replace_semantics_are_explicit()
    print("PASS test_warm_rust_dependency_cache")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
