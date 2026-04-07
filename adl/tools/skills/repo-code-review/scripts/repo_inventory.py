#!/usr/bin/env python3
"""
Create a deterministic, read-only repository inventory for repo-wide reviews.
"""

from __future__ import annotations

import json
import subprocess
import sys
from collections import Counter
from pathlib import Path

IGNORED_DIR_NAMES = {
    ".git",
    "node_modules",
    "dist",
    "build",
    "target",
    "coverage",
    ".next",
    ".venv",
    "venv",
    "__pycache__",
}

MAX_SAMPLE_PATHS = 12
TOP_N_LARGEST = 20
CODE_SUFFIXES = {
    ".rs",
    ".py",
    ".js",
    ".jsx",
    ".ts",
    ".tsx",
    ".go",
    ".java",
    ".kt",
    ".rb",
    ".php",
    ".c",
    ".cc",
    ".cpp",
    ".cxx",
    ".h",
    ".hh",
    ".hpp",
    ".swift",
    ".scala",
    ".sh",
}


def git_files(repo_root: Path) -> list[str]:
    result = subprocess.run(
        ["git", "-C", str(repo_root), "ls-files"],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        return []
    return [line for line in result.stdout.splitlines() if line.strip()]


def walk_files(repo_root: Path) -> list[str]:
    files: list[str] = []
    for path in sorted(repo_root.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(repo_root)
        if any(part in IGNORED_DIR_NAMES for part in rel.parts):
            continue
        files.append(rel.as_posix())
    return files


def count_lines(path: Path) -> int:
    try:
        with path.open("rb") as handle:
            return sum(1 for _ in handle)
    except OSError:
        return -1


def is_code_path(path: Path) -> bool:
    suffix = path.suffix.lower()
    if suffix in CODE_SUFFIXES:
        return True
    parts = {part.lower() for part in path.parts}
    return bool(parts & {"src", "lib", "app", "server", "client", "bin"})


def classify(files: list[str]) -> dict[str, object]:
    ext_counts: Counter[str] = Counter()
    top_dirs: Counter[str] = Counter()
    tests: list[str] = []
    docs: list[str] = []
    code_roots: Counter[str] = Counter()
    largest_files: list[tuple[int, str]] = []
    largest_code_files: list[tuple[int, str]] = []

    for rel in files:
        path = Path(rel)
        suffix = path.suffix.lower() or "<no_ext>"
        ext_counts[suffix] += 1
        top_dirs[path.parts[0] if len(path.parts) > 1 else "."] += 1
        if is_code_path(path):
            code_roots[path.parts[0] if len(path.parts) > 1 else "."] += 1

        lowered = rel.lower()
        parts_lower = [part.lower() for part in path.parts]
        stem_lower = path.stem.lower()
        is_testish = (
            any(part in {"test", "tests", "__tests__", "spec", "specs"} for part in parts_lower)
            or stem_lower.endswith("_test")
            or stem_lower.endswith("_spec")
            or ".test." in lowered
            or ".spec." in lowered
        )
        if is_testish:
            if len(tests) < MAX_SAMPLE_PATHS:
                tests.append(rel)
        if lowered.endswith(".md") or lowered.startswith("docs/"):
            if len(docs) < MAX_SAMPLE_PATHS:
                docs.append(rel)

        abs_path = REPO_ROOT / path
        line_count = count_lines(abs_path)
        if line_count >= 0:
            largest_files.append((line_count, rel))
            if is_code_path(path):
                largest_code_files.append((line_count, rel))

    largest_files.sort(reverse=True)
    largest_code_files.sort(reverse=True)

    return {
        "file_count": len(files),
        "top_level_areas": top_dirs.most_common(15),
        "likely_code_roots": code_roots.most_common(15),
        "extensions": ext_counts.most_common(20),
        "sample_tests": tests,
        "sample_docs": docs,
        "sample_paths": files[:MAX_SAMPLE_PATHS],
        "largest_files": [
            {"lines": lines, "path": rel} for lines, rel in largest_files[:TOP_N_LARGEST]
        ],
        "largest_code_files": [
            {"lines": lines, "path": rel}
            for lines, rel in largest_code_files[:TOP_N_LARGEST]
        ],
    }


REPO_ROOT = Path(".")


def main() -> int:
    global REPO_ROOT
    if len(sys.argv) != 2:
        print("usage: repo_inventory.py <repo-root>", file=sys.stderr)
        return 2

    repo_root = Path(sys.argv[1]).resolve()
    if not repo_root.exists() or not repo_root.is_dir():
        print(json.dumps({"error": "repo root not found", "path": str(repo_root)}))
        return 1

    files = git_files(repo_root)
    source = "git"
    if not files:
        files = walk_files(repo_root)
        source = "filesystem"

    data = {
        "repo_root": str(repo_root),
        "source": source,
        "ignored_dir_names": sorted(IGNORED_DIR_NAMES),
        "inventory": classify(files),
    }
    print(json.dumps(data, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
    REPO_ROOT = repo_root
