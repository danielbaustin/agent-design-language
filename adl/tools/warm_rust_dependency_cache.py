#!/usr/bin/env python3
"""Warm a Rust target directory by hardlinking dependency artifacts.

This helper is intentionally conservative. It uses Cargo.lock to identify dependency package names, then links only
artifacts that look like dependency outputs. Workspace package outputs are not
selected intentionally.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import tomllib
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Iterable


@dataclass
class Summary:
    source_target: str
    dest_target: str
    manifest_path: str
    profile: str
    dry_run: bool
    replace: bool
    external_packages: int = 0
    external_target_prefixes: int = 0
    candidate_files: int = 0
    linked_files: int = 0
    skipped_dep_info: int = 0
    skipped_existing: int = 0
    skipped_unmatched: int = 0
    errors: int = 0


def normalize_target_name(name: str) -> str:
    return name.replace("-", "_")


def read_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def read_workspace_package_names(manifest_path: Path) -> set[str]:
    manifest = read_toml(manifest_path)
    names: set[str] = set()

    package_name = manifest.get("package", {}).get("name")
    if package_name:
        names.add(package_name)

    for member in manifest.get("workspace", {}).get("members", []):
        if "*" in member:
            # Keep wildcard handling conservative; the current ADL manifest does
            # not need it, and guessing can accidentally exclude dependencies.
            continue
        member_manifest = (manifest_path.parent / member / "Cargo.toml").resolve()
        if not member_manifest.exists():
            continue
        member_name = read_toml(member_manifest).get("package", {}).get("name")
        if member_name:
            names.add(member_name)

    return names


def read_lockfile_package_prefixes(manifest_path: Path) -> set[str]:
    lockfile = manifest_path.parent / "Cargo.lock"
    if not lockfile.exists():
        lockfile = manifest_path.parent.parent / "Cargo.lock"
    if not lockfile.exists():
        raise RuntimeError(f"Cargo.lock not found near manifest: {manifest_path}")

    workspace_names = read_workspace_package_names(manifest_path)
    prefixes: set[str] = set()
    for line in lockfile.read_text().splitlines():
        stripped = line.strip()
        if not stripped.startswith("name ="):
            continue
        _, value = stripped.split("=", 1)
        package_name = value.strip().strip('"')
        if package_name in workspace_names:
            continue
        prefixes.add(normalize_target_name(package_name))
    return prefixes


def load_external_target_prefixes(manifest_path: Path) -> set[str]:
    # Cargo metadata can have registry side effects in constrained environments.
    # Cargo.lock gives us a deterministic, local, conservative dependency-name
    # set and keeps this warmup helper usable before any network/cache access.
    return read_lockfile_package_prefixes(manifest_path)

def artifact_matches(path: Path, prefixes: set[str]) -> bool:
    name = path.name
    stem = name.split(".", 1)[0]
    stem = stem.removeprefix("lib")
    for prefix in prefixes:
        if stem == prefix or stem.startswith(prefix + "-"):
            return True
    return False


def iter_dependency_artifacts(profile_dir: Path, prefixes: set[str], summary: Summary) -> Iterable[tuple[Path, Path]]:
    deps_dir = profile_dir / "deps"
    if not deps_dir.exists():
        return
    for source in deps_dir.iterdir():
        if not source.is_file():
            continue
        if source.suffix == ".d":
            # Cargo dep-info files can contain source-target/build paths from
            # the donor target. Do not hardlink them unless path rewriting is
            # implemented and proven.
            summary.skipped_dep_info += 1
            continue
        if artifact_matches(source, prefixes):
            yield source, Path("deps") / source.name
        else:
            summary.skipped_unmatched += 1

def same_inode(a: Path, b: Path) -> bool:
    try:
        return a.stat().st_ino == b.stat().st_ino and a.stat().st_dev == b.stat().st_dev
    except FileNotFoundError:
        return False


def hardlink_file(source: Path, dest: Path, summary: Summary) -> None:
    summary.candidate_files += 1
    if dest.exists() or dest.is_symlink():
        if same_inode(source, dest):
            summary.skipped_existing += 1
            return
        if not summary.replace:
            summary.skipped_existing += 1
            return
        if summary.dry_run:
            summary.linked_files += 1
            return
        dest.unlink()

    if summary.dry_run:
        summary.linked_files += 1
        return

    dest.parent.mkdir(parents=True, exist_ok=True)
    try:
        os.link(source, dest)
        summary.linked_files += 1
    except OSError as exc:
        summary.errors += 1
        print(f"error: failed to hardlink {source} -> {dest}: {exc}", file=sys.stderr)


def warm_cache(args: argparse.Namespace) -> Summary:
    manifest_path = Path(args.manifest_path).resolve()
    source_target = Path(args.source_target).resolve()
    dest_target = Path(args.dest_target).resolve()
    source_profile = source_target / args.profile
    dest_profile = dest_target / args.profile

    if not manifest_path.exists():
        raise RuntimeError(f"manifest path does not exist: {manifest_path}")
    if not source_profile.exists():
        raise RuntimeError(f"source profile directory does not exist: {source_profile}")

    prefixes = load_external_target_prefixes(manifest_path)
    summary = Summary(
        source_target=str(source_target),
        dest_target=str(dest_target),
        manifest_path=str(manifest_path),
        profile=args.profile,
        dry_run=args.dry_run,
        replace=args.replace,
        external_packages=len(prefixes),
        external_target_prefixes=len(prefixes),
    )

    for source, relative in iter_dependency_artifacts(source_profile, prefixes, summary):
        hardlink_file(source, dest_profile / relative, summary)

    return summary


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Warm a Rust target directory by hardlinking external dependency artifacts.")
    parser.add_argument("--source-target", required=True, help="Existing warm Cargo target directory.")
    parser.add_argument("--dest-target", required=True, help="Destination Cargo target directory to warm.")
    parser.add_argument("--manifest-path", default="adl/Cargo.toml", help="Cargo manifest used for dependency classification.")
    parser.add_argument("--profile", default="debug", help="Cargo profile directory under target, usually debug or release.")
    parser.add_argument("--dry-run", action="store_true", help="Report what would be linked without modifying the destination.")
    parser.add_argument("--replace", action="store_true", help="Replace existing destination files before linking.")
    parser.add_argument("--json", action="store_true", help="Emit a JSON summary.")
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    try:
        summary = warm_cache(args)
    except Exception as exc:  # noqa: BLE001 - CLI fail-closed with clear message.
        if args.json:
            print(json.dumps({"status": "error", "error": str(exc)}, indent=2, sort_keys=True))
        else:
            print(f"error: {exc}", file=sys.stderr)
        return 1

    payload = asdict(summary)
    payload["status"] = "ok" if summary.errors == 0 else "completed_with_errors"
    if args.json:
        print(json.dumps(payload, indent=2, sort_keys=True))
    else:
        print(f"status={payload['status']} candidate_files={summary.candidate_files} linked_files={summary.linked_files} skipped_existing={summary.skipped_existing} errors={summary.errors}")
    return 0 if summary.errors == 0 else 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
