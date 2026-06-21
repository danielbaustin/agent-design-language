#!/usr/bin/env python3
"""Select the smallest ADL validation lane plan for a changed-file set."""

from __future__ import annotations

import argparse
import fnmatch
import json
import shlex
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = ROOT / "adl/config/validation_lane_selector.v0.91.6.json"


def fail(message: str) -> None:
    print(f"select_validation_lanes: {message}", file=sys.stderr)
    raise SystemExit(2)


def normalize_row(row: str) -> str | None:
    row = row.strip()
    if not row:
        return None
    parts = row.split("\t")
    if len(parts) == 1:
        return parts[0]
    if parts[0].startswith("R") and len(parts) >= 3:
        return parts[2]
    if len(parts) >= 2:
        return parts[1]
    return None


def changed_paths_from_file(path: Path) -> list[str]:
    return [
        normalized
        for line in path.read_text().splitlines()
        if (normalized := normalize_row(line))
    ]


def changed_paths_from_git(base: str, head: str, include_worktree: bool) -> list[str]:
    if include_worktree:
        cmd = ["git", "-C", str(ROOT), "diff", "--name-status", "--diff-filter=ACMR", base, "--"]
        result = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        untracked = subprocess.run(
            ["git", "-C", str(ROOT), "ls-files", "--others", "--exclude-standard"],
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
        )
        rows = result.stdout.splitlines() + untracked.stdout.splitlines()
        return [
            normalized
            for line in rows
            if (normalized := normalize_row(line))
        ]
    else:
        cmd = [
            "git",
            "-C",
            str(ROOT),
            "diff",
            "--name-status",
            "--diff-filter=ACMR",
            f"{base}...{head}",
        ]
    result = subprocess.run(cmd, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    if result.returncode != 0 and not include_worktree:
        result = subprocess.run(
            ["git", "-C", str(ROOT), "diff", "--name-status", "--diff-filter=ACMR", base, head],
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
        )
    return [
        normalized
        for line in result.stdout.splitlines()
        if (normalized := normalize_row(line))
    ]


def matches(path: str, hints: list[str]) -> bool:
    return any(path == hint or fnmatch.fnmatch(path, hint) for hint in hints)


def selectors_for(entry: dict[str, Any]) -> list[str]:
    selectors = entry.get("path_selectors") or entry.get("path_hints")
    if not isinstance(selectors, list) or not selectors:
        fail(f"manifest entry {entry.get('id', '<unknown>')} selectors must be a non-empty array")
    return selectors


def validate_vpp_record(entry: dict[str, Any], *, entry_id: str, manifest_path: Path) -> None:
    record = entry.get("vpp_record")
    if record is None:
        return
    if not isinstance(record, dict):
        fail(f"{manifest_path}: entry {entry_id} vpp_record must be an object")
    for key in (
        "contract_version",
        "artifacts",
        "expected_runtime_class",
        "parallel_group",
        "cache_equivalence_group",
        "failure_semantics",
    ):
        if key not in record:
            fail(f"{manifest_path}: entry {entry_id} vpp_record missing required key: {key}")
    if not isinstance(record["contract_version"], str) or not record["contract_version"]:
        fail(f"{manifest_path}: entry {entry_id} vpp_record.contract_version must be a non-empty string")
    if not isinstance(record["artifacts"], list) or not record["artifacts"] or not all(
        isinstance(item, str) and item for item in record["artifacts"]
    ):
        fail(f"{manifest_path}: entry {entry_id} vpp_record.artifacts must be a non-empty array of strings")
    for key in ("expected_runtime_class", "parallel_group", "cache_equivalence_group", "failure_semantics"):
        if not isinstance(record[key], str) or not record[key]:
            fail(f"{manifest_path}: entry {entry_id} vpp_record.{key} must be a non-empty string")


REQUIRED_SURFACE_METADATA = (
    "owner",
    "resource_class",
    "determinism_posture",
    "proof_role",
    "risk_class",
    "escalation_rule",
)


def merge_surface_defaults(
    manifest: dict[str, Any], entry: dict[str, Any], *, entry_id: str, manifest_path: Path = DEFAULT_MANIFEST
) -> dict[str, Any]:
    default_surface = entry.get("default_surface")
    if not isinstance(default_surface, str) or not default_surface:
        fail(f"{manifest_path}: entry {entry_id} missing required key: default_surface")
    surface_defaults = manifest["surface_defaults"].get(default_surface)
    if not isinstance(surface_defaults, dict):
        fail(f"{manifest_path}: entry {entry_id} references unknown default_surface: {default_surface}")
    merged = dict(surface_defaults)
    merged.update(entry)
    merged["path_selectors"] = selectors_for(entry)
    for key in REQUIRED_SURFACE_METADATA:
        if not isinstance(merged.get(key), str) or not merged[key]:
            fail(f"{manifest_path}: entry {entry_id} missing required surface metadata: {key}")
    validate_vpp_record(merged, entry_id=entry_id, manifest_path=manifest_path)
    return merged


def plan_entry(
    manifest: dict[str, Any],
    entry: dict[str, Any],
    *,
    manifest_rule: str,
    status: str,
    matched_paths: list[str],
    command: str | None = None,
    run_command: str | None = None,
    lane_class: str | None = None,
    extra: dict[str, Any] | None = None,
) -> dict[str, Any]:
    normalized = merge_surface_defaults(manifest, entry, entry_id=entry["id"])
    result = {
        "manifest_rule": manifest_rule,
        "lane_class": lane_class or normalized["lane_class"],
        "status": status,
        "reason": normalized["reason"],
        "command": command if command is not None else normalized["command"],
        "run_command": run_command if run_command is not None else normalized.get("run_command", normalized["command"]),
        "matched_paths": matched_paths,
        "owner": normalized["owner"],
        "resource_class": normalized["resource_class"],
        "determinism_posture": normalized["determinism_posture"],
        "proof_role": normalized["proof_role"],
        "risk_class": normalized["risk_class"],
        "escalation_rule": normalized["escalation_rule"],
        "default_surface": normalized["default_surface"],
        "path_selectors": normalized["path_selectors"],
        "requirement_ids": normalized.get("requirement_ids", []),
    }
    broad_lane_reason = normalized.get("broad_lane_reason")
    if broad_lane_reason:
        result["broad_lane_reason"] = broad_lane_reason
    if "vpp_record" in normalized:
        result["vpp_record"] = normalized["vpp_record"]
    if extra:
        result.update(extra)
    return result


def manifest_entry_for(
    manifest: dict[str, Any], surface_id: str, fallback: dict[str, Any], *, manifest_path: Path = DEFAULT_MANIFEST
) -> dict[str, Any]:
    special_surfaces = manifest.get("special_surfaces", {})
    entry = special_surfaces.get(surface_id, fallback)
    if not isinstance(entry, dict):
        fail(f"{manifest_path}: special_surfaces.{surface_id} must be an object")
    return entry


def load_manifest(path: Path) -> dict[str, Any]:
    manifest = json.loads(path.read_text())
    if manifest.get("schema_version") != "adl.validation_lane_selector.v1":
        fail(f"{path}: unsupported schema_version")
    for key in ("surface_defaults", "lanes", "release_gate_hints", "rust_path_hints"):
        if key not in manifest:
            fail(f"{path}: missing required key: {key}")
    if not isinstance(manifest["surface_defaults"], dict) or not manifest["surface_defaults"]:
        fail(f"{path}: surface_defaults must be a non-empty object")
    lane_ids: set[str] = set()
    for index, lane in enumerate(manifest["lanes"]):
        if not isinstance(lane, dict):
            fail(f"{path}: lanes[{index}] must be an object")
        for key in ("id", "lane_class", "command", "reason", "default_surface"):
            if key not in lane:
                fail(f"{path}: lanes[{index}] missing required key: {key}")
        if lane["id"] in lane_ids:
            fail(f"{path}: duplicate lane id: {lane['id']}")
        lane_ids.add(lane["id"])
        selectors_for(lane)
        if "run_command" in lane and not isinstance(lane["run_command"], str):
            fail(f"{path}: lane {lane['id']} run_command must be a string")
        merge_surface_defaults(manifest, lane, entry_id=lane["id"], manifest_path=path)
    special_surfaces = manifest.get("special_surfaces", {})
    if special_surfaces:
        if not isinstance(special_surfaces, dict):
            fail(f"{path}: special_surfaces must be an object")
        for surface_id, surface in special_surfaces.items():
            if not isinstance(surface, dict):
                fail(f"{path}: special_surfaces.{surface_id} must be an object")
            for key in ("id", "lane_class", "command", "reason", "default_surface"):
                if key not in surface:
                    fail(f"{path}: special_surfaces.{surface_id} missing required key: {key}")
            selectors_for(surface)
            merge_surface_defaults(manifest, surface, entry_id=surface_id, manifest_path=path)
    for key in ("release_gate_hints", "rust_path_hints"):
        if not isinstance(manifest[key], list):
            fail(f"{path}: {key} must be an array")
    return manifest


def pr_fast_plan(changed_files: Path | None, base: str, head: str) -> dict[str, str]:
    cmd = ["bash", "adl/tools/run_pr_fast_test_lane.sh", "--print-plan", "--json"]
    if changed_files is not None:
        cmd.extend(["--changed-files", str(changed_files)])
    else:
        cmd.extend(["--base", base, "--head", head])
    result = subprocess.run(cmd, cwd=ROOT, check=False, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode != 0:
        fail(f"run_pr_fast_test_lane failed while planning Rust validation: {result.stderr.strip()}")
    try:
        plan = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        fail(f"run_pr_fast_test_lane returned invalid JSON: {exc}")
    if plan.get("schema_version") != "adl.pr_fast_lane_plan.v1":
        fail("run_pr_fast_test_lane returned unsupported JSON schema")
    return plan


def select_lanes(
    manifest_path: Path,
    manifest: dict[str, Any],
    paths: list[str],
    changed_files: Path | None,
    base: str,
    head: str,
) -> dict[str, Any]:
    selected: dict[str, dict[str, Any]] = {}
    release_gate_entry = manifest_entry_for(
        manifest,
        "release_gate_review",
        {
            "id": "release_gate_review",
            "lane_class": "release_gate",
            "default_surface": "release_gate",
            "owner": "tools",
            "path_selectors": manifest["release_gate_hints"],
            "command": "record release-gate disposition; do not treat focused PR validation as release proof",
            "run_command": "",
            "reason": "changed_surface_requires_release_or_ci_policy_review",
        },
        manifest_path=manifest_path,
    )
    rust_entry = manifest_entry_for(
        manifest,
        "rust_pr_fast",
        {
            "id": "rust_pr_fast",
            "lane_class": "fast_unit",
            "escalated_lane_class": "release_gate",
            "default_surface": "shared_rust",
            "owner": "shared",
            "path_selectors": manifest["rust_path_hints"],
            "command": "bash adl/tools/run_pr_fast_test_lane.sh",
            "run_command": "bash adl/tools/run_pr_fast_test_lane.sh",
            "reason": "delegate_rust_changed_surface_to_pr_fast_lane_selector",
        },
        manifest_path=manifest_path,
    )
    slow_proof_entry = manifest_entry_for(
        manifest,
        "slow_proof_review",
        {
            "id": "slow_proof_review",
            "lane_class": "slow_proof",
            "default_surface": "slow_proof",
            "owner": "runtime",
            "path_selectors": [
                "adl/tools/test_slow_proof_lane_contract.sh",
                "adl/config/slow_proof_families.v0.91.6.json",
                "docs/milestones/**/features/PVF_INITIAL_LANE_INVENTORY*.md",
                "docs/milestones/**/features/PVF_CI_RELEASE_POLICY*.md",
            ],
            "command": "record slow-proof disposition; do not treat ordinary PR-fast proof as slow-proof coverage",
            "run_command": "",
            "reason": "changed_surface_requires_slow_proof_review",
        },
        manifest_path=manifest_path,
    )

    release_gate_paths = [
        path for path in paths if matches(path, selectors_for(release_gate_entry))
    ]
    slow_proof_paths = [
        path for path in paths if matches(path, selectors_for(slow_proof_entry))
    ]

    for path in paths:
        for lane in manifest["lanes"]:
            if matches(path, selectors_for(lane)):
                lane_id = lane["id"]
                entry = selected.setdefault(
                    lane_id,
                    plan_entry(
                        manifest,
                        lane,
                        manifest_rule=f"lanes.{lane_id}",
                        status="selected",
                        matched_paths=[],
                    ),
                )
                entry["matched_paths"].append(path)
                break

    rust_paths = [
        path
        for path in paths
        if matches(path, selectors_for(rust_entry))
        and not path.endswith("/README.md")
        and not path.endswith(".md")
    ]
    if rust_paths:
        fast_plan = pr_fast_plan(changed_files, base, head)
        mode = fast_plan.get("mode", "full")
        reason = fast_plan.get("reason", "missing_pr_fast_reason")
        status = "selected" if mode in {"focused", "family"} else "escalated"
        command = "bash adl/tools/run_pr_fast_test_lane.sh"
        if changed_files is not None:
            command += f" --changed-files {shlex.quote(str(changed_files))}"
        else:
            command += f" --base {shlex.quote(base)} --head {shlex.quote(head)}"
        selected["rust_pr_fast"] = plan_entry(
            manifest,
            rust_entry,
            manifest_rule="special_surfaces.rust_pr_fast",
            status=status,
            matched_paths=rust_paths,
            command=command,
            run_command=command,
            lane_class=(
                rust_entry.get("contract_only_lane_class")
                if mode == "contract_only"
                else rust_entry.get("escalated_lane_class")
                if status == "escalated"
                else rust_entry["lane_class"]
            ),
            extra={
                "reason": reason,
                "mode": mode,
                "filter_tokens": fast_plan.get("filter_tokens", ""),
                "filter_expression": fast_plan.get("filter_expression", ""),
                "rust_surface_count": fast_plan.get("rust_surface_count", 0),
                "structural_surface_count": fast_plan.get("structural_surface_count", 0),
                "slow_proof_inventory_surface_count": fast_plan.get("slow_proof_inventory_surface_count", 0),
            },
        )
        selected["rust_pr_fast"]["reason"] = reason

    if release_gate_paths:
        selected["release_gate_review"] = plan_entry(
            manifest,
            release_gate_entry,
            manifest_rule="special_surfaces.release_gate_review",
            status="release_gate_required",
            matched_paths=release_gate_paths,
        )
    if slow_proof_paths:
        selected["slow_proof_review"] = plan_entry(
            manifest,
            slow_proof_entry,
            manifest_rule="special_surfaces.slow_proof_review",
            status="escalated",
            matched_paths=slow_proof_paths,
        )

    aggregate = "skipped"
    for lane in selected.values():
        if lane["status"] == "escalated":
            aggregate = "escalated"
            break
        if lane["status"] == "release_gate_required":
            aggregate = "release_gate_required"
        elif aggregate == "skipped" and lane["status"] == "selected":
            aggregate = "selected"

    sufficient = aggregate == "selected"
    return {
        "schema_version": "adl.validation_lane_plan.v1",
        "manifest_path": str(manifest_path.relative_to(ROOT)) if manifest_path.is_relative_to(ROOT) else str(manifest_path),
        "changed_paths": paths,
        "aggregate_status": aggregate,
        "pr_publication_sufficient": sufficient,
        "lanes": selected,
    }


def print_text(plan: dict[str, Any]) -> None:
    print("Validation lane plan")
    print(f"  aggregate_status={plan['aggregate_status']}")
    print(f"  pr_publication_sufficient={str(plan['pr_publication_sufficient']).lower()}")
    if not plan["lanes"]:
        print("  - no lanes selected")
        return
    for lane_id, lane in plan["lanes"].items():
        print(
            f"  - {lane_id} status={lane['status']} class={lane['lane_class']} reason={lane['reason']}"
        )
        if lane.get("mode"):
            print(f"    mode={lane['mode']}")
        if lane.get("filter_expression"):
            print(f"    filter_expression={lane['filter_expression']}")
        print(f"    command={lane['command']}")
        if lane.get("run_command") and lane.get("run_command") != lane.get("command"):
            print(f"    run_command={lane['run_command']}")
        if lane.get("vpp_record"):
            record = lane["vpp_record"]
            print(
                "    vpp_record="
                f"contract={record['contract_version']} "
                f"runtime={record['expected_runtime_class']} "
                f"parallel={record['parallel_group']} "
                f"cache={record['cache_equivalence_group']} "
                f"failure={record['failure_semantics']}"
            )
            print(f"    artifacts={','.join(record['artifacts'])}")
        for path in lane["matched_paths"]:
            print(f"    path={path}")


def write_report(path: Path, plan: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(plan, indent=2, sort_keys=True) + "\n")


def run_selected_lanes(plan: dict[str, Any]) -> int:
    if plan["aggregate_status"] != "selected":
        print_text(plan)
        print(
            "select_validation_lanes: refusing --run because the plan is not fully selected",
            file=sys.stderr,
        )
        return 1

    failed = False
    for lane_id, lane in plan["lanes"].items():
        command = lane.get("run_command") or lane.get("command")
        if not command:
            lane["run_status"] = "skipped"
            lane["run_reason"] = "no_run_command"
            continue
        print(f"==> {lane_id}: {command}", file=sys.stderr)
        result = subprocess.run(command, cwd=ROOT, shell=True)
        if result.returncode == 0:
            lane["run_status"] = "passed"
            lane["run_reason"] = "command_succeeded"
        else:
            lane["run_status"] = "failed"
            lane["run_reason"] = f"exit_code_{result.returncode}"
            failed = True

    plan["run_status"] = "failed" if failed else "passed"
    return 1 if failed else 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", default=str(DEFAULT_MANIFEST))
    parser.add_argument("--changed-files")
    parser.add_argument("--base", default="origin/main")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--include-working-tree", action="store_true")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--report-out")
    args = parser.parse_args(argv)

    manifest_path = Path(args.manifest)
    if not manifest_path.is_absolute():
        manifest_path = ROOT / manifest_path
    manifest = load_manifest(manifest_path)

    changed_file_path = Path(args.changed_files).resolve() if args.changed_files else None
    if changed_file_path is not None:
        paths = changed_paths_from_file(changed_file_path)
    else:
        paths = changed_paths_from_git(args.base, args.head, args.include_working_tree)

    plan = select_lanes(manifest_path, manifest, paths, changed_file_path, args.base, args.head)
    exit_code = 0
    if args.run:
        exit_code = run_selected_lanes(plan)
    if args.report_out:
        write_report(Path(args.report_out), plan)
    if args.json:
        print(json.dumps(plan, indent=2, sort_keys=True))
    else:
        print_text(plan)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
