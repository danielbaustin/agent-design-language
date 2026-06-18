#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Check reviewer-facing repo and milestone surfaces for obvious "
            "current-milestone staleness and tracked junk."
        )
    )
    parser.add_argument(
        "--repo-root",
        default=Path(__file__).resolve().parents[2],
        type=Path,
        help="Repository root to inspect.",
    )
    parser.add_argument(
        "--milestone",
        required=True,
        help="Current reviewer-facing milestone, for example v0.91.6.",
    )
    return parser.parse_args()


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def add_result(ok: bool, message: str, failures: list[str]) -> None:
    prefix = "PASS" if ok else "FAIL"
    print(f"{prefix} repo-quality-staleness {message}")
    if not ok:
        failures.append(message)


def expect_contains(path: Path, needle: str, label: str, failures: list[str]) -> None:
    add_result(needle in read_text(path), f"{label} in {path}: {needle}", failures)


def expect_exists(path: Path, label: str, failures: list[str]) -> None:
    add_result(path.exists(), f"{label} exists: {path}", failures)


def extract_markdown_links(text: str) -> list[str]:
    return re.findall(r"\[[^\]]+\]\(([^)]+)\)", text)


def section_body(text: str, title: str) -> str:
    pattern = re.compile(rf"^## {re.escape(title)}\n(.*?)(?=^## |\Z)", re.M | re.S)
    match = pattern.search(text)
    return match.group(1) if match else ""


def feature_links_from_index(text: str) -> list[str]:
    return re.findall(r"\(\s*(features/[^)]+\.md)\s*\)", text)


def tracked_junk(repo_root: Path) -> list[str]:
    proc = subprocess.run(
        ["git", "-C", str(repo_root), "ls-files", "-z"],
        check=True,
        capture_output=True,
    )
    tracked = [Path(item) for item in proc.stdout.decode("utf-8").split("\0") if item]
    return [
        str(path)
        for path in tracked
        if any(part == "__pycache__" for part in path.parts)
        or path.name.endswith((".pyc", ".pyo"))
        or path.name == ".DS_Store"
    ]


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    milestone = args.milestone
    failures: list[str] = []

    root_readme = repo_root / "README.md"
    changelog = repo_root / "CHANGELOG.md"
    milestone_dir = repo_root / "docs" / "milestones" / milestone
    milestone_readme = milestone_dir / "README.md"
    release_plan = milestone_dir / f"RELEASE_PLAN_{milestone}.md"
    release_notes = milestone_dir / f"RELEASE_NOTES_{milestone}.md"
    checklist = milestone_dir / f"MILESTONE_CHECKLIST_{milestone}.md"
    feature_index = milestone_dir / f"FEATURE_DOCS_{milestone}.md"
    feature_dir_index = milestone_dir / "features" / "README.md"

    expect_exists(root_readme, "root README", failures)
    expect_exists(changelog, "CHANGELOG", failures)
    expect_exists(milestone_dir, "milestone directory", failures)
    for path, label in [
        (milestone_readme, "milestone README"),
        (release_plan, "milestone release plan"),
        (release_notes, "milestone release notes"),
        (checklist, "milestone checklist"),
        (feature_index, "milestone feature index"),
        (feature_dir_index, "milestone feature directory index"),
    ]:
        expect_exists(path, label, failures)

    if failures:
        return 1

    expect_contains(
        root_readme,
        f"- Active milestone: {milestone}",
        "root README active milestone line",
        failures,
    )
    expect_contains(
        root_readme,
        f"docs/milestones/{milestone}/README.md",
        "root README current milestone link",
        failures,
    )
    expect_contains(
        changelog,
        f"## {milestone} (",
        "CHANGELOG current milestone heading",
        failures,
    )

    for path in [milestone_readme, release_plan, release_notes, checklist, feature_index]:
        expect_contains(path, f"`{milestone}`", f"milestone marker", failures)

    readme_text = read_text(milestone_readme)
    document_map = section_body(readme_text, "Document Map")
    required_doc_map_links = [
        f"RELEASE_PLAN_{milestone}.md",
        f"RELEASE_NOTES_{milestone}.md",
        f"MILESTONE_CHECKLIST_{milestone}.md",
        f"FEATURE_DOCS_{milestone}.md",
        "features/README.md",
    ]
    for rel in required_doc_map_links:
        add_result(rel in document_map, f"document-map link present: {rel}", failures)

    for link in extract_markdown_links(document_map):
        target = (milestone_dir / link).resolve()
        add_result(target.exists(), f"document-map target exists: {link}", failures)

    feature_index_text = read_text(feature_index)
    feature_dir_text = read_text(feature_dir_index)
    feature_index_links = sorted(set(feature_links_from_index(feature_index_text)))
    feature_dir_links = sorted(set(extract_markdown_links(feature_dir_text)))
    feature_dir_md_links = sorted(link for link in feature_dir_links if link.endswith(".md"))
    feature_index_names = sorted(Path(link).name for link in feature_index_links)
    feature_dir_names = sorted(Path(link).name for link in feature_dir_md_links)

    add_result(
        feature_index_names == feature_dir_names,
        "feature index and feature directory links match",
        failures,
    )
    for rel in feature_index_links:
        add_result((milestone_dir / rel).exists(), f"feature doc exists: {rel}", failures)

    junk = tracked_junk(repo_root)
    add_result(not junk, "tracked junk absent (__pycache__/.pyc/.pyo/.DS_Store)", failures)
    if junk:
        for path in junk:
            print(f"DETAIL tracked-junk {path}")

    if failures:
        print("repo-quality-staleness summary: FAIL", file=sys.stderr)
        return 1

    print(f"repo-quality-staleness summary: PASS milestone={milestone}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
