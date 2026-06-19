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


def load_manifest(path: Path) -> dict[str, Any]:
    manifest = json.loads(path.read_text())
    if manifest.get("schema_version") != "adl.validation_lane_selector.v1":
        fail(f"{path}: unsupported schema_version")
    for key in ("lanes", "release_gate_hints", "rust_path_hints"):
        if key not in manifest:
            fail(f"{path}: missing required key: {key}")
    lane_ids: set[str] = set()
    for index, lane in enumerate(manifest["lanes"]):
        if not isinstance(lane, dict):
            fail(f"{path}: lanes[{index}] must be an object")
        for key in ("id", "lane_class", "path_hints", "command", "reason"):
            if key not in lane:
                fail(f"{path}: lanes[{index}] missing required key: {key}")
        if lane["id"] in lane_ids:
            fail(f"{path}: duplicate lane id: {lane['id']}")
        lane_ids.add(lane["id"])
        if not isinstance(lane["path_hints"], list) or not lane["path_hints"]:
            fail(f"{path}: lane {lane['id']} path_hints must be a non-empty array")
        if "run_command" in lane and not isinstance(lane["run_command"], str):
            fail(f"{path}: lane {lane['id']} run_command must be a string")
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

    release_gate_paths = [
        path for path in paths if matches(path, manifest["release_gate_hints"])
    ]

    for path in paths:
        for lane in manifest["lanes"]:
            if matches(path, lane["path_hints"]):
                lane_id = lane["id"]
                entry = selected.setdefault(
                    lane_id,
                    {
                        "lane_class": lane["lane_class"],
                        "status": "selected",
                        "reason": lane["reason"],
                        "command": lane["command"],
                        "run_command": lane.get("run_command", lane["command"]),
                        "matched_paths": [],
                    },
                )
                entry["matched_paths"].append(path)
                break

    rust_paths = [
        path
        for path in paths
        if matches(path, manifest["rust_path_hints"])
        and not path.endswith("/README.md")
        and not path.endswith(".md")
    ]
    if rust_paths:
        fast_plan = pr_fast_plan(changed_files, base, head)
        mode = fast_plan.get("mode", "full")
        reason = fast_plan.get("reason", "missing_pr_fast_reason")
        status = "selected" if mode in {"focused", "family", "contract_only"} else "escalated"
        command = "bash adl/tools/run_pr_fast_test_lane.sh"
        if changed_files is not None:
            command += f" --changed-files {shlex.quote(str(changed_files))}"
        else:
            command += f" --base {shlex.quote(base)} --head {shlex.quote(head)}"
        selected["rust_pr_fast"] = {
            "lane_class": "fast_unit" if status == "selected" else "release_gate",
            "status": status,
            "reason": reason,
            "command": command,
            "run_command": command,
            "matched_paths": rust_paths,
            "mode": mode,
            "filter_tokens": fast_plan.get("filter_tokens", ""),
            "filter_expression": fast_plan.get("filter_expression", ""),
        }

    if release_gate_paths:
        selected["release_gate_review"] = {
            "lane_class": "release_gate",
            "status": "release_gate_required",
            "reason": "changed_surface_requires_release_or_ci_policy_review",
            "command": "record release-gate disposition; do not treat focused PR validation as release proof",
            "run_command": "",
            "matched_paths": release_gate_paths,
        }

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
