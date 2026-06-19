#!/usr/bin/env python3
"""Build a deterministic local inventory of ADL validation surfaces."""

from __future__ import annotations

import argparse
import fnmatch
import json
import re
import subprocess
import sys
import tomllib
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = ROOT / "adl/config/validation_lane_selector.v0.91.6.json"
DEFAULT_CARGO = ROOT / "adl/Cargo.toml"

TEST_ATTRIBUTE_RE = re.compile(
    r"#\[\s*(?:tokio::test|test|rstest|quickcheck|proptest)\b"
)
DOC_FENCE_RE = re.compile(
    r"^\s*(?:(?://[/!])|(?:/\*\*)|(?:\*))?.*```(?:rust|ignore|no_run|should_panic|compile_fail|edition\d{4})\b",
    re.MULTILINE,
)

SLOW_PROOF_PATTERNS = (
    "adl/Cargo.toml",
    "adl/tools/test_slow_proof_lane_contract.sh",
    "adl/src/runtime_v2/tests/**",
    ".github/workflows/ci.yaml",
)
RELEASE_GATE_EXTRA_PATTERNS = (
    "adl/tools/run_authoritative_coverage_lane.sh",
    "adl/tools/run_local_authoritative_coverage_gate.sh",
    "adl/tools/test_slow_proof_lane_contract.sh",
)
COVERAGE_ONLY_PATTERNS = (
    ".github/workflows/nightly-coverage-ratchet.yaml",
    "adl/tools/check_coverage_impact.sh",
    "adl/tools/test_check_coverage_impact.sh",
    "adl/tools/run_authoritative_coverage_lane.sh",
    "adl/tools/run_local_authoritative_coverage_gate.sh",
)


def fail(message: str) -> None:
    print(f"validation_inventory: {message}", file=sys.stderr)
    raise SystemExit(2)


def git_files(repo_root: Path) -> list[str]:
    result = subprocess.run(
        ["git", "-C", str(repo_root), "ls-files"],
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        fail(f"git ls-files failed: {result.stderr.strip()}")
    return sorted(line for line in result.stdout.splitlines() if line.strip())


def load_manifest(path: Path) -> dict[str, Any]:
    manifest = json.loads(path.read_text())
    if manifest.get("schema_version") != "adl.validation_lane_selector.v1":
        fail(f"{path}: unsupported schema_version")
    return manifest


def load_features(path: Path) -> dict[str, list[str]]:
    cargo = tomllib.loads(path.read_text())
    features = cargo.get("features", {})
    return {
        name: sorted(value) if isinstance(value, list) else []
        for name, value in sorted(features.items())
    }


def matches(path: str, patterns: tuple[str, ...] | list[str]) -> bool:
    return any(path == pattern or fnmatch.fnmatch(path, pattern) for pattern in patterns)


def classify_owner(path: str) -> str | None:
    if path in {"adl/Cargo.toml", "adl/Cargo.lock"}:
        return "runtime"
    if path.startswith("docs/") or path.endswith(".md"):
        return "docs"
    if path.startswith(".github/workflows/"):
        return "github"
    if path.startswith("adl/src/bin/adl_issue"):
        return "github"
    if path.startswith("adl/src/bin/adl_pr_"):
        return "csdlc"
    if path in {
        "adl/src/bin/adl_csdlc.rs",
        "adl/src/bin/adl_prompt_template.rs",
        "adl/src/bin/adl_validate_structured_prompt.rs",
        "adl/src/bin/adl_lint_prompt_spec.rs",
    }:
        return "csdlc"
    if path.startswith("adl/src/runtime_v2/") or path.startswith("adl/src/bin/adl_runtime"):
        return "runtime"
    if path.startswith("adl/src/provider") or path.startswith("adl/src/bin/adl_provider"):
        return "provider"
    if path.startswith("adl/src/review") or path.startswith("adl/src/bin/adl_review"):
        return "review"
    if path.startswith("adl/src/bin/"):
        return "tools"
    if path.startswith("adl/src/cli/pr_cmd") or path == "adl/tools/pr.sh":
        return "csdlc"
    if path.startswith("adl/src/cli/"):
        return "tools"
    if path.startswith("adl/tools/"):
        return "tools"
    if path.startswith("adl/src/"):
        return "runtime"
    if path.startswith("adl/tests/"):
        return "runtime"
    return None


def resource_class_for_lane(lane_class: str | None) -> str | None:
    if lane_class == "docs":
        return "tiny"
    if lane_class in {"cli_workflow", "contract_schema_card", "fast_unit"}:
        return "normal"
    if lane_class == "integration_worktree":
        return "expensive"
    if lane_class in {"release_gate", "provider_live"}:
        return "external" if lane_class == "provider_live" else "expensive"
    return None


def doc_test_block_count(text: str) -> int:
    return len(DOC_FENCE_RE.findall(text))


def rust_category_for_path(path: str) -> str | None:
    if not path.endswith(".rs"):
        return None
    if path.startswith("adl/src/bin/"):
        return "rust_bin_tests"
    if path.startswith("adl/tests/"):
        return "rust_integration_tests"
    if path.startswith("adl/src/"):
        return "rust_lib_unit_tests"
    return None


def build_rust_inventory(files: list[str]) -> dict[str, Any]:
    categories: dict[str, dict[str, Any]] = {
        "rust_lib_unit_tests": {"files": [], "test_count": 0, "doc_test_blocks": 0},
        "rust_bin_tests": {"files": [], "test_count": 0, "doc_test_blocks": 0},
        "rust_integration_tests": {"files": [], "test_count": 0, "doc_test_blocks": 0},
    }
    doc_test_paths: list[str] = []
    owner_counts: Counter[str] = Counter()

    for rel in files:
        category = rust_category_for_path(rel)
        if category is None:
            continue
        abs_path = ROOT / rel
        text = abs_path.read_text(encoding="utf-8")
        test_count = len(TEST_ATTRIBUTE_RE.findall(text))
        doc_blocks = doc_test_block_count(text)
        if test_count == 0 and doc_blocks == 0:
            continue
        categories[category]["files"].append(rel)
        categories[category]["test_count"] += test_count
        categories[category]["doc_test_blocks"] += doc_blocks
        owner = classify_owner(rel)
        if owner:
            owner_counts[owner] += 1
        if doc_blocks:
            doc_test_paths.append(rel)

    rust = {
        key: {
            "file_count": len(value["files"]),
            "test_entry_count": value["test_count"],
            "doc_test_block_count": value["doc_test_blocks"],
            "sample_paths": value["files"][:12],
        }
        for key, value in categories.items()
    }
    rust["rust_doc_tests"] = {
        "file_count": len(doc_test_paths),
        "doc_test_block_count": sum(categories[key]["doc_test_blocks"] for key in categories),
        "sample_paths": sorted(doc_test_paths)[:12],
    }
    rust["owner_summary"] = dict(sorted(owner_counts.items()))
    rust["candidate_paths"] = sorted(
        {
            *categories["rust_lib_unit_tests"]["files"],
            *categories["rust_bin_tests"]["files"],
            *categories["rust_integration_tests"]["files"],
            *doc_test_paths,
        }
    )
    return rust


def add_surface_record(
    records: list[dict[str, Any]],
    path: str,
    *,
    surface_kind: str,
    lane_class: str | None,
    owner: str | None,
    resource_class: str | None,
    proof_role: str,
    feature_requirements: list[str] | None = None,
) -> None:
    records.append(
        {
            "path": path,
            "surface_kind": surface_kind,
            "lane_class": lane_class,
            "owner": owner,
            "resource_class": resource_class,
            "proof_role": proof_role,
            "feature_requirements": sorted(feature_requirements or []),
        }
    )


def build_surface_inventory(
    files: list[str],
    manifest: dict[str, Any],
    rust_candidate_paths: list[str],
) -> dict[str, Any]:
    records: list[dict[str, Any]] = []
    known_paths: set[str] = set()

    for lane in manifest.get("lanes", []):
        lane_class = lane["lane_class"]
        resource_class = resource_class_for_lane(lane_class)
        for path in files:
            if matches(path, lane["path_hints"]):
                add_surface_record(
                    records,
                    path,
                    surface_kind="manifest_surface",
                    lane_class=lane_class,
                    owner=classify_owner(path),
                    resource_class=resource_class,
                    proof_role="ordinary_pr",
                )
                known_paths.add(path)

    rust_hints = tuple(manifest.get("rust_path_hints", []))
    for path in rust_candidate_paths:
        if matches(path, rust_hints):
            add_surface_record(
                records,
                path,
                surface_kind="rust_validation_surface",
                lane_class="fast_unit",
                owner=classify_owner(path),
                resource_class="normal",
                proof_role="ordinary_pr",
            )
            known_paths.add(path)

    for path in files:
        if path.startswith("adl/tools/") and path not in known_paths:
            name = Path(path).name
            if name.startswith("demo_") and (path.endswith(".sh") or path.endswith(".py")):
                add_surface_record(
                    records,
                    path,
                    surface_kind="demo_proof_surface",
                    lane_class="integration_worktree",
                    owner=classify_owner(path),
                    resource_class="expensive",
                    proof_role="integration_demo",
                )
            if path.endswith(".sh") and (
                name.startswith("test_")
                or name.startswith("check_")
                or name.startswith("validate_")
                or (
                    name.startswith("run_")
                    and any(
                        token in name
                        for token in ("validation", "coverage", "proof", "lane", "benchmark")
                    )
                )
            ):
                add_surface_record(
                    records,
                    path,
                    surface_kind="shell_validator",
                    lane_class=None,
                    owner=classify_owner(path),
                    resource_class="normal",
                    proof_role="ordinary_pr",
                )
            if path.endswith(".py") and (
                name.startswith("test_")
                or
                name.startswith("validate_")
                or name.startswith("check_")
                or (
                    name.startswith("run_")
                    and any(
                        token in name
                        for token in ("validation", "coverage", "proof", "benchmark")
                    )
                )
            ):
                add_surface_record(
                    records,
                    path,
                    surface_kind="python_validator",
                    lane_class=None,
                    owner=classify_owner(path),
                    resource_class="normal",
                    proof_role="ordinary_pr",
                )

    for path in files:
        if matches(path, SLOW_PROOF_PATTERNS):
            add_surface_record(
                records,
                path,
                surface_kind="slow_proof_surface",
                lane_class="release_gate",
                owner=classify_owner(path),
                resource_class="expensive",
                proof_role="slow_proof",
                feature_requirements=["slow-proof-tests"],
            )
        if matches(path, COVERAGE_ONLY_PATTERNS):
            add_surface_record(
                records,
                path,
                surface_kind="coverage_only_surface",
                lane_class="release_gate",
                owner=classify_owner(path),
                resource_class="expensive",
                proof_role="coverage_only",
            )
        if matches(path, tuple(manifest.get("release_gate_hints", [])) + RELEASE_GATE_EXTRA_PATTERNS):
            add_surface_record(
                records,
                path,
                surface_kind="release_gate_surface",
                lane_class="release_gate",
                owner=classify_owner(path),
                resource_class="expensive",
                proof_role="release_gate",
                feature_requirements=["slow-proof-tests"] if path in {"adl/Cargo.toml"} else [],
            )

    unique_records = {
        (
            record["path"],
            record["surface_kind"],
            record["lane_class"],
            record["proof_role"],
        ): record
        for record in records
    }
    deduped = [unique_records[key] for key in sorted(unique_records)]
    return {
        "records": deduped,
        "known_paths": sorted({record["path"] for record in deduped}),
    }


def candidate_validation_paths(files: list[str], rust_candidate_paths: list[str]) -> list[str]:
    candidates = set(rust_candidate_paths)
    for path in files:
        if path.startswith("adl/tools/"):
            name = Path(path).name
            if path.endswith((".sh", ".py")) and (
                name.startswith("test_")
                or name.startswith("check_")
                or name.startswith("validate_")
                or name.startswith("demo_")
                or (
                    name.startswith("run_")
                    and any(
                        token in name
                        for token in ("validation", "coverage", "proof", "lane", "benchmark")
                    )
                )
            ):
                candidates.add(path)
        if path.startswith(".github/workflows/"):
            candidates.add(path)
    return sorted(candidates)


def feature_inventory(features: dict[str, list[str]]) -> dict[str, Any]:
    interesting = {}
    for name, members in features.items():
        if "slow-proof" in name or "proof" in name or "coverage" in name:
            interesting[name] = {
                "member_count": len(members),
                "members": members,
            }
    return interesting


def build_inventory(files: list[str], manifest: dict[str, Any], features: dict[str, list[str]]) -> dict[str, Any]:
    rust = build_rust_inventory(files)
    surfaces = build_surface_inventory(files, manifest, rust["candidate_paths"])
    candidates = candidate_validation_paths(files, rust["candidate_paths"])
    unmatched_paths = sorted(set(candidates) - set(surfaces["known_paths"]))

    partial_records = []
    for record in surfaces["records"]:
        missing_fields = [
            field
            for field in ("lane_class", "owner", "resource_class")
            if record[field] is None
        ]
        if missing_fields:
            partial_records.append(
                {
                    "path": record["path"],
                    "classification_status": "partial",
                    "missing_fields": missing_fields,
                    "surface_kind": record["surface_kind"],
                }
            )

    unknown_records = [
        {
            "path": path,
            "classification_status": "unmatched",
            "missing_fields": ["lane_class", "owner", "resource_class"],
            "surface_kind": "unmatched_candidate",
        }
        for path in unmatched_paths
    ]
    remediation_index = {
        (record["path"], record["classification_status"]): record
        for record in [*unknown_records, *partial_records]
    }
    remediation_records = [
        remediation_index[key]
        for key in sorted(remediation_index, key=lambda item: (item[0], item[1]))
    ]

    lane_summary: Counter[str] = Counter()
    owner_summary: Counter[str] = Counter()
    resource_summary: Counter[str] = Counter()
    proof_role_summary: Counter[str] = Counter()
    kind_summary: Counter[str] = Counter()
    for record in surfaces["records"]:
        if record["lane_class"]:
            lane_summary[record["lane_class"]] += 1
        if record["owner"]:
            owner_summary[record["owner"]] += 1
        if record["resource_class"]:
            resource_summary[record["resource_class"]] += 1
        proof_role_summary[record["proof_role"]] += 1
        kind_summary[record["surface_kind"]] += 1

    return {
        "schema_version": "adl.validation_inventory.v1",
        "repo_root": ".",
        "repo_name": ROOT.name,
        "manifest_path": str(DEFAULT_MANIFEST.relative_to(ROOT)),
        "tracked_file_count": len(files),
        "rust": {
            "rust_lib_unit_tests": rust["rust_lib_unit_tests"],
            "rust_bin_tests": rust["rust_bin_tests"],
            "rust_integration_tests": rust["rust_integration_tests"],
            "rust_doc_tests": rust["rust_doc_tests"],
            "owner_summary": rust["owner_summary"],
        },
        "surface_summary": {
            "by_surface_kind": dict(sorted(kind_summary.items())),
            "by_lane_class": dict(sorted(lane_summary.items())),
            "by_owner": dict(sorted(owner_summary.items())),
            "by_resource_class": dict(sorted(resource_summary.items())),
            "by_proof_role": dict(sorted(proof_role_summary.items())),
        },
        "feature_inventory": feature_inventory(features),
        "all_surface_records": surfaces["records"],
        "shell_validators": [
            record for record in surfaces["records"] if record["surface_kind"] == "shell_validator"
        ],
        "python_validators": [
            record for record in surfaces["records"] if record["surface_kind"] == "python_validator"
        ],
        "demo_proof_surfaces": [
            record for record in surfaces["records"] if record["surface_kind"] == "demo_proof_surface"
        ],
        "slow_proof_surfaces": [
            record for record in surfaces["records"] if record["surface_kind"] == "slow_proof_surface"
        ],
        "coverage_only_surfaces": [
            record for record in surfaces["records"] if record["surface_kind"] == "coverage_only_surface"
        ],
        "release_gate_surfaces": [
            record for record in surfaces["records"] if record["surface_kind"] == "release_gate_surface"
        ],
        "manifest_backed_surfaces": [
            record for record in surfaces["records"] if record["surface_kind"] == "manifest_surface"
        ],
        "unknown_or_unclassified_surfaces": remediation_records,
    }


def markdown_table_row(label: str, values: dict[str, Any]) -> str:
    return (
        f"| {label} | {values['file_count']} | {values.get('test_entry_count', 0)} | "
        f"{values.get('doc_test_block_count', 0)} |"
    )


def render_markdown(inventory: dict[str, Any]) -> str:
    rust = inventory["rust"]
    lines = [
        "# Validation Inventory",
        "",
        "## Rust test-bearing surfaces",
        "",
        "| Category | Files | Test entries | Doc-test blocks |",
        "| --- | ---: | ---: | ---: |",
        markdown_table_row("Rust lib unit tests", rust["rust_lib_unit_tests"]),
        markdown_table_row("Rust bin tests", rust["rust_bin_tests"]),
        markdown_table_row("Rust integration tests", rust["rust_integration_tests"]),
        markdown_table_row("Rust doc tests", rust["rust_doc_tests"]),
        "",
        "## Surface summaries",
        "",
    ]

    for heading, values in (
        ("Lane class", inventory["surface_summary"]["by_lane_class"]),
        ("Owner", inventory["surface_summary"]["by_owner"]),
        ("Resource class", inventory["surface_summary"]["by_resource_class"]),
        ("Proof role", inventory["surface_summary"]["by_proof_role"]),
        ("Surface kind", inventory["surface_summary"]["by_surface_kind"]),
    ):
        lines.extend([f"### {heading}", ""])
        if not values:
            lines.append("- none")
        else:
            for key, count in values.items():
                lines.append(f"- `{key}`: {count}")
        lines.append("")

    lines.extend(
        [
            "## Feature expansions",
            "",
        ]
    )
    if not inventory["feature_inventory"]:
        lines.append("- none")
    else:
        for name, details in inventory["feature_inventory"].items():
            lines.append(
                f"- `{name}`: members={details['member_count']} "
                f"({', '.join(details['members']) if details['members'] else 'none'})"
            )
    lines.append("")

    for heading, records in (
        ("Shell validators", inventory["shell_validators"]),
        ("Python validators", inventory["python_validators"]),
        ("Demo proof surfaces", inventory["demo_proof_surfaces"]),
    ):
        lines.extend([f"## {heading}", ""])
        if not records:
            lines.append("- none")
        else:
            lines.append(f"- count: {len(records)}")
            for record in records[:20]:
                lines.append(
                    f"- `{record['path']}` owner={record['owner'] or 'unknown'} "
                    f"lane={record['lane_class'] or 'unknown'} "
                    f"resource={record['resource_class'] or 'unknown'}"
                )
        lines.append("")

    for heading, records in (
        ("Slow-proof surfaces", inventory["slow_proof_surfaces"]),
        ("Coverage-only surfaces", inventory["coverage_only_surfaces"]),
        ("Release-gate surfaces", inventory["release_gate_surfaces"]),
    ):
        lines.extend([f"## {heading}", ""])
        if not records:
            lines.append("- none")
        else:
            for record in records[:20]:
                lines.append(
                    f"- `{record['path']}` owner={record['owner'] or 'unknown'} "
                    f"lane={record['lane_class'] or 'unknown'} "
                    f"resource={record['resource_class'] or 'unknown'}"
                )
        lines.append("")

    lines.extend(["## Unknown or unclassified surfaces", ""])
    if not inventory["unknown_or_unclassified_surfaces"]:
        lines.append("- none")
    else:
        for record in inventory["unknown_or_unclassified_surfaces"][:40]:
            lines.append(
                f"- `{record['path']}` status={record['classification_status']} "
                f"missing={','.join(record['missing_fields'])}"
            )
    lines.append("")
    return "\n".join(lines)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--format",
        choices=("json", "markdown", "both"),
        default="both",
    )
    parser.add_argument("--json-out", type=Path)
    parser.add_argument("--markdown-out", type=Path)
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    files = git_files(ROOT)
    manifest = load_manifest(DEFAULT_MANIFEST)
    features = load_features(DEFAULT_CARGO)
    inventory = build_inventory(files, manifest, features)
    markdown = render_markdown(inventory)

    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")
    if args.markdown_out:
        args.markdown_out.parent.mkdir(parents=True, exist_ok=True)
        args.markdown_out.write_text(markdown + "\n")

    if args.format == "json":
        print(json.dumps(inventory, indent=2, sort_keys=True))
    elif args.format == "markdown":
        print(markdown)
    else:
        print(json.dumps(inventory, indent=2, sort_keys=True))
        print()
        print(markdown)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
