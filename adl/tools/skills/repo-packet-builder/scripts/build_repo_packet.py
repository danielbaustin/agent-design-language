#!/usr/bin/env python3
"""Build a deterministic CodeBuddy repository review packet."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
from collections import Counter, defaultdict
from pathlib import Path
from urllib.parse import urlparse

SCHEMA_PREFIX = "codebuddy.repo_packet"
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
MANIFEST_NAMES = {
    "Cargo.toml",
    "Cargo.lock",
    "package.json",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "pyproject.toml",
    "requirements.txt",
    "go.mod",
    "go.sum",
    "pom.xml",
    "build.gradle",
    "settings.gradle",
    "Dockerfile",
    "docker-compose.yml",
    "Makefile",
}
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
DOC_SUFFIXES = {".md", ".mdx", ".rst", ".adoc", ".txt"}
MAX_LIST = 40


def run_git(repo_root: Path, args: list[str]) -> tuple[int, str]:
    result = subprocess.run(
        ["git", "-C", str(repo_root), *args],
        check=False,
        capture_output=True,
        text=True,
    )
    return result.returncode, result.stdout.strip()


def tracked_files(repo_root: Path) -> list[str]:
    code, stdout = run_git(repo_root, ["ls-files"])
    if code == 0 and stdout:
        return [line for line in stdout.splitlines() if line.strip()]
    files: list[str] = []
    for path in sorted(repo_root.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(repo_root)
        if any(part in IGNORED_DIR_NAMES for part in rel.parts):
            continue
        files.append(rel.as_posix())
    return files


def repo_ref(repo_root: Path) -> str:
    code, stdout = run_git(repo_root, ["rev-parse", "--short", "HEAD"])
    if code == 0 and stdout:
        return stdout
    return "unknown"


def canonical_repo_name_from_remote(url: str) -> str | None:
    stripped = url.strip().rstrip("/")
    if not stripped:
        return None

    path = ""
    if "://" in stripped:
        parsed = urlparse(stripped)
        path = parsed.path.lstrip("/")
    else:
        # SSH-style git@github.com:org/repo.git or git@host:repo.git.
        if ":" in stripped:
            path = stripped.split(":", 1)[1]
        elif "/" in stripped:
            path = stripped
    if not path:
        return None
    candidate = Path(path).name.removesuffix(".git")
    return candidate or None


def canonical_repo_name(repo_root: Path) -> str:
    for remote_name in ("origin", "upstream"):
        code, stdout = run_git(repo_root, ["remote", "get-url", remote_name])
        if code != 0:
            continue
        candidate = canonical_repo_name_from_remote(stdout)
        if candidate:
            return candidate
    return repo_root.name


def derive_repo_identity(repo_root: Path) -> tuple[str, str]:
    return canonical_repo_name(repo_root), repo_root.name


def count_lines(path: Path) -> int:
    try:
        with path.open("rb") as handle:
            return sum(1 for _ in handle)
    except OSError:
        return -1


def is_test_path(path: Path) -> bool:
    lowered = path.as_posix().lower()
    parts = {part.lower() for part in path.parts}
    stem = path.stem.lower()
    return (
        bool(parts & {"test", "tests", "__tests__", "spec", "specs"})
        or stem.endswith("_test")
        or stem.endswith("_spec")
        or ".test." in lowered
        or ".spec." in lowered
    )


def is_doc_path(path: Path) -> bool:
    parts = {part.lower() for part in path.parts}
    return path.suffix.lower() in DOC_SUFFIXES or bool(parts & {"doc", "docs", "documentation"})


def is_ci_path(path: Path) -> bool:
    lowered = path.as_posix().lower()
    return lowered.startswith(".github/workflows/") or lowered.startswith(".gitlab-ci") or "ci" in path.parts


def is_code_path(path: Path) -> bool:
    parts = {part.lower() for part in path.parts}
    return path.suffix.lower() in CODE_SUFFIXES or bool(parts & {"src", "lib", "app", "server", "client", "bin"})


def is_manifest(path: Path) -> bool:
    return path.name in MANIFEST_NAMES or path.name.endswith(".lock")


def category_for(path: Path) -> str:
    if is_manifest(path):
        return "manifest"
    if is_ci_path(path):
        return "ci"
    if is_test_path(path):
        return "test"
    if is_doc_path(path):
        if "architecture" in {part.lower() for part in path.parts}:
            return "architecture_docs"
        return "docs"
    if is_code_path(path):
        return "code"
    if any(part in IGNORED_DIR_NAMES for part in path.parts):
        return "generated_or_vendor"
    return "other"


def lanes_for(category: str, path: Path) -> list[str]:
    lanes: set[str] = set()
    if category in {"code", "manifest", "ci"}:
        lanes.add("code")
    if category in {"manifest", "ci"} or path.suffix.lower() in {".sh", ".py", ".js", ".ts"}:
        lanes.add("security")
    if category == "test":
        lanes.add("tests")
    if category in {"docs", "architecture_docs"}:
        lanes.add("docs")
    if category in {"architecture_docs", "manifest", "code"}:
        lanes.add("architecture")
    if category in {"manifest", "ci"}:
        lanes.add("dependencies")
    if category in {"architecture_docs", "docs", "manifest", "code"}:
        lanes.add("diagrams")
    lanes.add("redaction")
    lanes.add("synthesis")
    return sorted(lanes)


def limited(items: list[str], limit: int = MAX_LIST) -> list[str]:
    return items[:limit]


def inventory(repo_root: Path, files: list[str], repo_name: str) -> dict[str, object]:
    ext_counts: Counter[str] = Counter()
    top_dirs: Counter[str] = Counter()
    code_roots: Counter[str] = Counter()
    manifests: list[str] = []
    docs: list[str] = []
    tests: list[str] = []
    ci: list[str] = []
    largest_files: list[tuple[int, str]] = []
    largest_code_files: list[tuple[int, str]] = []

    for rel in files:
        path = Path(rel)
        full_path = repo_root / rel
        ext_counts[path.suffix.lower() or "<no_ext>"] += 1
        top_dirs[path.parts[0] if len(path.parts) > 1 else "."] += 1
        line_count = count_lines(full_path)
        largest_files.append((line_count, rel))
        if is_code_path(path):
            code_roots[path.parts[0] if len(path.parts) > 1 else "."] += 1
            largest_code_files.append((line_count, rel))
        if is_manifest(path):
            manifests.append(rel)
        if is_doc_path(path):
            docs.append(rel)
        if is_test_path(path):
            tests.append(rel)
        if is_ci_path(path):
            ci.append(rel)

    largest_files.sort(key=lambda item: (-item[0], item[1]))
    largest_code_files.sort(key=lambda item: (-item[0], item[1]))

    return {
        "schema": f"{SCHEMA_PREFIX}.inventory.v1",
        "repo_name": repo_name,
        "worktree_name": repo_root.name,
        "is_worktree": ".worktrees" in repo_root.parts,
        "file_count": len(files),
        "extension_counts": dict(sorted(ext_counts.items())),
        "top_level_dirs": dict(sorted(top_dirs.items())),
        "manifests": sorted(manifests),
        "docs": limited(sorted(docs)),
        "tests": limited(sorted(tests)),
        "ci": sorted(ci),
        "likely_code_roots": [name for name, _ in code_roots.most_common(20)],
        "largest_files": [{"path": path, "line_count": lines} for lines, path in largest_files[:20]],
        "largest_code_files": [
            {"path": path, "line_count": lines} for lines, path in largest_code_files[:20]
        ],
    }


def build_evidence(repo_root: Path, files: list[str]) -> list[dict[str, object]]:
    scored: list[tuple[int, str, dict[str, object]]] = []
    for rel in files:
        path = Path(rel)
        category = category_for(path)
        line_count = count_lines(repo_root / rel)
        score = 0
        reasons: list[str] = []
        if category == "manifest":
            score += 50
            reasons.append("manifest or dependency/build surface")
        if category == "ci":
            score += 35
            reasons.append("CI or automation surface")
        if category == "architecture_docs":
            score += 35
            reasons.append("architecture documentation surface")
        if category == "test":
            score += 20
            reasons.append("test or validation surface")
        if category == "code":
            score += 20
            reasons.append("executable code surface")
        if line_count > 500:
            score += 10
            reasons.append("large file")
        if path.name.lower() in {"readme.md", "readme"}:
            score += 20
            reasons.append("top-level onboarding surface")
        if score == 0:
            continue
        scored.append(
            (
                -score,
                rel,
                {
                    "path": rel,
                    "category": category,
                    "line_count": line_count,
                    "reason": "; ".join(reasons),
                    "specialist_lanes": lanes_for(category, path),
                },
            )
        )
    return [entry for _, _, entry in sorted(scored)[:120]]


def assignments_from_evidence(evidence: list[dict[str, object]]) -> dict[str, list[str]]:
    assignments: dict[str, list[str]] = defaultdict(list)
    for entry in evidence:
        for lane in entry["specialist_lanes"]:
            assignments[lane].append(str(entry["path"]))
    for lane in ["code", "security", "tests", "docs", "architecture", "dependencies", "diagrams"]:
        assignments.setdefault(lane, [])
    assignments["redaction"] = ["run_manifest.json", "repo_scope.md", "repo_inventory.json", "evidence_index.json"]
    assignments["synthesis"] = ["all specialist artifacts after review lanes complete"]
    return {lane: limited(sorted(set(paths)), 30) for lane, paths in sorted(assignments.items())}


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_scope(
    path: Path,
    args: argparse.Namespace,
    canonical_repo_name: str,
    worktree_name: str,
    inv: dict[str, object],
) -> None:
    included = [
        "tracked repository files",
        "top-level manifests",
        "docs, tests, CI, and likely code roots",
    ]
    if args.target_path:
        included = [f"path scope: {args.target_path}"]
    excluded = sorted(f"{name}/**" for name in IGNORED_DIR_NAMES)
    content = f"""# Repo Scope

## Scope Reviewed

- Repository: {canonical_repo_name}
- Review mode: {args.mode}
- Target path: {args.target_path or "not specified"}
- Diff base: {args.diff_base or "not specified"}
- Privacy mode: {args.privacy_mode}

## Included Paths

{chr(10).join(f"- {item}" for item in included)}

## Excluded Paths

{chr(10).join(f"- {item}" for item in excluded)}

## Non-Reviewed Surfaces

- Runtime behavior was not executed.
- Specialist review lanes were not run by this packet builder.
- Generated/vendor/cache surfaces are excluded by default.

## Assumptions

- Git-tracked files represent the intended review surface when available.
- Repo-relative paths are sufficient evidence references for downstream lanes.

## Known Limits

- The packet contains path evidence and metadata, not source excerpts.
- Publication safety requires a separate redaction/evidence audit.
- Path or diff scope must be expanded explicitly if downstream reviewers need more context.

## Next Specialist Lanes

- code
- security
- tests
- docs
- architecture
- dependencies
- diagrams
- redaction
- synthesis

## Inventory Summary

- File count: {inv["file_count"]}
- Manifest count: {len(inv["manifests"])}
- Docs sampled: {len(inv["docs"])}
- Tests sampled: {len(inv["tests"])}
- CI files: {len(inv["ci"])}

## Review Context

- Worktree name: {worktree_name}
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("repo_root", help="Repository root to inventory")
    parser.add_argument("--out", default=None, help="Packet artifact root")
    parser.add_argument("--mode", default="build_repository_packet", help="Packet mode")
    parser.add_argument("--target-path", default=None, help="Optional path scope")
    parser.add_argument("--diff-base", default=None, help="Optional diff base")
    parser.add_argument("--privacy-mode", default="local_only", help="Privacy mode")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not repo_root.is_dir():
        raise SystemExit(f"repo root does not exist: {repo_root}")

    run_id = dt.datetime.now(dt.UTC).strftime("%Y%m%d-%H%M%S-repo-packet")
    artifact_root = Path(args.out) if args.out else repo_root / ".adl" / "reviews" / "codebuddy" / run_id
    if not artifact_root.is_absolute():
        artifact_root = repo_root / artifact_root
    artifact_root.mkdir(parents=True, exist_ok=True)

    files = tracked_files(repo_root)
    if args.target_path:
        prefix = Path(args.target_path).as_posix().rstrip("/") + "/"
        files = [path for path in files if path == args.target_path or path.startswith(prefix)]

    canonical_name, worktree_name = derive_repo_identity(repo_root)
    inv = inventory(repo_root, files, canonical_name)
    evidence = build_evidence(repo_root, files)
    assignments = assignments_from_evidence(evidence)
    now = dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")

    manifest = {
        "schema": f"{SCHEMA_PREFIX}.run_manifest.v1",
        "run_id": run_id,
        "repo_name": canonical_name,
        "worktree_name": worktree_name,
        "is_worktree": ".worktrees" in repo_root.parts,
        "repo_ref": repo_ref(repo_root),
        "review_mode": args.mode,
        "started_at": now,
        "completed_at": now,
        "skills_used": ["repo-packet-builder"],
        "artifact_root": artifact_root.relative_to(repo_root).as_posix()
        if artifact_root.is_relative_to(repo_root)
        else artifact_root.name,
        "privacy_mode": args.privacy_mode,
        "publication_allowed": False,
    }

    write_json(artifact_root / "run_manifest.json", manifest)
    write_scope(artifact_root / "repo_scope.md", args, canonical_name, worktree_name, inv)
    write_json(artifact_root / "repo_inventory.json", inv)
    write_json(artifact_root / "evidence_index.json", {"schema": f"{SCHEMA_PREFIX}.evidence.v1", "evidence": evidence})
    write_json(
        artifact_root / "specialist_assignments.json",
        {"schema": f"{SCHEMA_PREFIX}.assignments.v1", "assignments": assignments},
    )

    print(artifact_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
