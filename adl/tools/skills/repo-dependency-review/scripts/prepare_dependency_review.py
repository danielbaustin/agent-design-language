#!/usr/bin/env python3
"""Prepare deterministic scaffolding for a CodeBuddy dependency review."""

from __future__ import annotations

import argparse
import datetime as dt
import json
from pathlib import Path

SCHEMA = "codebuddy.repo_dependency_review.scaffold.v1"
DEPENDENCY_CATEGORIES = {
    "ci",
    "code",
    "config",
    "dependency",
    "dependencies",
    "docker",
    "docs",
    "lockfile",
    "manifest",
    "package_manager",
    "test",
}
DEPENDENCY_TERMS = (
    "cargo",
    "ci",
    "compose",
    "container",
    "dependency",
    "dependencies",
    "devcontainer",
    "docker",
    "install",
    "license",
    "lock",
    "manifest",
    "notice",
    "npm",
    "package",
    "pip",
    "pnpm",
    "poetry",
    "requirements",
    "setup",
    "supply",
    "third_party",
    "toolchain",
    "vendor",
    "yarn",
)
MANIFEST_NAMES = {
    "cargo.toml",
    "go.mod",
    "package.json",
    "pom.xml",
    "pyproject.toml",
    "requirements.txt",
    "setup.cfg",
    "setup.py",
}
LOCKFILE_NAMES = {
    "cargo.lock",
    "go.sum",
    "gradle.lockfile",
    "package-lock.json",
    "pnpm-lock.yaml",
    "poetry.lock",
    "uv.lock",
    "yarn.lock",
}
LICENSE_TERMS = ("license", "notice", "copying", "third_party", "third-party", "attribution")
CI_TERMS = (".github/workflows", "gitlab-ci", "circleci", "buildkite", "azure-pipelines")
CONTAINER_TERMS = ("dockerfile", "docker-compose", ".devcontainer", "containerfile")
TEST_TERMS = ("test", "check", "validate", "smoke", "install")


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def evidence_entries(packet_root: Path) -> list[dict[str, object]]:
    data = load_json(packet_root / "evidence_index.json")
    if not isinstance(data, dict):
        return []
    evidence = data.get("evidence")
    if not isinstance(evidence, list):
        return []
    return [item for item in evidence if isinstance(item, dict)]


def classify_dependency_surface(entry: dict[str, object]) -> str:
    path = str(entry.get("path", ""))
    lowered = path.lower()
    name = Path(lowered).name
    category = str(entry.get("category", "")).lower()
    if name in LOCKFILE_NAMES or "lockfile" in category:
        return "lockfile"
    if name in MANIFEST_NAMES or "manifest" in category:
        return "manifest"
    if any(term in lowered for term in CI_TERMS) or category == "ci":
        return "ci_dependency_setup"
    if any(term in lowered for term in CONTAINER_TERMS) or category == "docker":
        return "container_dependency_setup"
    if any(term in lowered for term in LICENSE_TERMS):
        return "license_or_attribution"
    if "vendor" in lowered or "third_party" in lowered or "third-party" in lowered:
        return "vendored_or_generated_dependency"
    if any(term in lowered for term in TEST_TERMS) or category == "test":
        return "dependency_test_surface"
    if "package" in lowered or "dependency" in lowered or "toolchain" in lowered:
        return "package_manager_config"
    return "dependency_related"


def is_dependency_evidence(entry: dict[str, object]) -> bool:
    category = str(entry.get("category", "")).lower()
    path = str(entry.get("path", "")).lower()
    reason = str(entry.get("reason", "")).lower()
    lanes = entry.get("specialist_lanes")
    lane_match = isinstance(lanes, list) and "dependency" in lanes
    name = Path(path).name
    term_match = any(term in path or term in reason for term in DEPENDENCY_TERMS)
    known_file_match = name in MANIFEST_NAMES or name in LOCKFILE_NAMES
    return lane_match or known_file_match or term_match or category in DEPENDENCY_CATEGORIES


def build_surface_map(entries: list[dict[str, object]]) -> dict[str, list[str]]:
    surfaces: dict[str, list[str]] = {}
    for entry in entries:
        path = str(entry.get("path", ""))
        if not path:
            continue
        surface = classify_dependency_surface(entry)
        surfaces.setdefault(surface, [])
        if path not in surfaces[surface]:
            surfaces[surface].append(path)
    return {key: sorted(value) for key, value in sorted(surfaces.items())}


def build_candidate_supply_chain_findings(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    for entry in entries:
        path = str(entry.get("path", ""))
        if not path:
            continue
        surface = classify_dependency_surface(entry)
        lowered = path.lower()
        if surface in {"manifest", "lockfile", "ci_dependency_setup", "container_dependency_setup"}:
            candidates.append(
                {
                    "surface": surface,
                    "source": path,
                    "reason": "Review for pinning, lockfile consistency, floating versions, cache keys, and install determinism.",
                }
            )
        elif "vendor" in lowered or "third_party" in lowered or "third-party" in lowered:
            candidates.append(
                {
                    "surface": surface,
                    "source": path,
                    "reason": "Review whether vendored or copied dependency evidence is intentionally scoped and attributed.",
                }
            )
    return candidates[:16]


def build_candidate_dependency_test_gaps(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    has_manifest_or_lock = any(classify_dependency_surface(entry) in {"manifest", "lockfile"} for entry in entries)
    has_test_surface = any(classify_dependency_surface(entry) == "dependency_test_surface" for entry in entries)
    if has_manifest_or_lock and not has_test_surface:
        candidates.append(
            {
                "candidate": "Add bounded install or import smoke proof for dependency changes.",
                "source": "manifest_or_lockfile_surfaces",
                "reason": "Packet includes dependency declarations without an obvious dependency-focused test surface.",
            }
        )
    for entry in entries:
        path = str(entry.get("path", ""))
        lowered = path.lower()
        if path and any(term in lowered for term in ("dockerfile", "compose", ".github/workflows")):
            candidates.append(
                {
                    "candidate": f"Verify dependency bootstrap path represented by {path}",
                    "source": path,
                    "reason": "Runtime or CI dependency setup often needs a smoke check to catch drift.",
                }
            )
    return candidates[:12]


def build_candidate_license_notes(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    notes: list[dict[str, str]] = []
    for entry in entries:
        path = str(entry.get("path", ""))
        lowered = path.lower()
        if path and any(term in lowered for term in LICENSE_TERMS):
            notes.append(
                {
                    "source": path,
                    "reason": "License or attribution surface should be checked by a human reviewer; this scaffold does not make legal determinations.",
                }
            )
    return notes[:12]


def lines_for_surface_map(surface_map: dict[str, list[str]]) -> str:
    if not surface_map:
        return "- No dependency surfaces selected from packet."
    lines: list[str] = []
    for surface, paths in surface_map.items():
        lines.append(f"- {surface}:")
        for path in paths:
            lines.append(f"  - {path}")
    return "\n".join(lines)


def write_markdown(path: Path, scaffold: dict[str, object]) -> None:
    evidence = scaffold["dependency_evidence"]
    evidence_lines = "\n".join(
        f"- {item['path']} ({classify_dependency_surface(item)}): {item.get('reason', 'dependency evidence')}"
        for item in evidence
    ) or "- No dependency evidence selected from packet."
    supply_lines = "\n".join(
        f"- {item['source']} ({item['surface']}): {item['reason']}"
        for item in scaffold["candidate_supply_chain_findings"]
    ) or "- None identified by scaffold."
    test_lines = "\n".join(
        f"- {item['candidate']} Source: {item['source']}. Reason: {item['reason']}"
        for item in scaffold["candidate_dependency_test_gaps"]
    ) or "- None identified by scaffold."
    license_lines = "\n".join(
        f"- {item['source']}: {item['reason']}"
        for item in scaffold["candidate_license_review_notes"]
    ) or "- None identified by scaffold."

    content = f"""# Repo Dependency Review Scaffold

## Metadata

- Skill: repo-dependency-review
- Repo: {scaffold["repo_name"]}
- Packet: {scaffold["packet_root"]}
- Date: {scaffold["created_at"]}

## Findings

- No findings have been written yet. Replace this section with findings-first dependency review output after inspection.

## Dependency Surface Map

{lines_for_surface_map(scaffold["dependency_surface_map"])}

## Reviewed Surfaces

{evidence_lines}

## Candidate Supply-Chain Findings

{supply_lines}

## Candidate Dependency Test Gaps

{test_lines}

## Candidate License Review Notes

{license_lines}

## Validation Performed

- Scaffold generation only; no repository validation commands were run by this helper.

## Residual Risk

- This scaffold is not a review finding artifact. A reviewer must inspect the selected surfaces and record findings or an explicit no-material-findings result.
- This helper does not query external vulnerability databases or perform legal review.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_root", help="CodeBuddy review packet root")
    parser.add_argument("--out", default=None, help="Dependency review scaffold output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    packet_root = Path(args.packet_root).resolve()
    if not packet_root.is_dir():
        raise SystemExit(f"packet root does not exist: {packet_root}")

    out_root = Path(args.out) if args.out else packet_root / "dependency-review"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(packet_root / "run_manifest.json")
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or packet_root.name

    entries = [entry for entry in evidence_entries(packet_root) if is_dependency_evidence(entry)]
    entries = sorted(entries, key=lambda item: str(item.get("path", "")))[:80]
    scaffold = {
        "schema": SCHEMA,
        "repo_name": repo_name,
        "packet_root": packet_root.name,
        "created_at": now_utc(),
        "dependency_evidence": entries,
        "dependency_surface_map": build_surface_map(entries),
        "candidate_supply_chain_findings": build_candidate_supply_chain_findings(entries),
        "candidate_dependency_test_gaps": build_candidate_dependency_test_gaps(entries),
        "candidate_license_review_notes": build_candidate_license_notes(entries),
        "notes": [
            "Scaffold is deterministic except for created_at.",
            "Paths are packet evidence paths, not absolute host paths.",
            "Reviewer must replace scaffold findings with source-grounded dependency review findings.",
            "Helper does not install dependencies, mutate lockfiles, use network feeds, or make legal determinations.",
        ],
    }
    write_json(out_root / "dependency_review_scaffold.json", scaffold)
    write_markdown(out_root / "dependency_review_scaffold.md", scaffold)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

