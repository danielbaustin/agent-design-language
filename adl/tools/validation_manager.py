#!/usr/bin/env python3
"""Build and optionally run an ADL validation profile for a change set."""

from __future__ import annotations

import argparse
import json
import shlex
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SELECTOR = ROOT / "adl/tools/select_validation_lanes.sh"
SLOW_PROOF_FAMILIES = ROOT / "adl/config/slow_proof_families.v0.91.6.json"
DEFAULT_MANIFEST = ROOT / "adl/config/validation_lane_selector.v0.91.6.json"
NESSUS_REMOTE_RUNNER = "bash adl/tools/run_nessus_remote_validation.sh"


def fail(message: str) -> None:
    print(f"validation_manager: {message}", file=sys.stderr)
    raise SystemExit(2)


def load_slow_proof_families() -> dict[str, Any]:
    payload = json.loads(SLOW_PROOF_FAMILIES.read_text())
    if payload.get("schema_version") != "adl.slow_proof_families.v1":
        fail("slow-proof families config returned unsupported schema_version")
    if not isinstance(payload.get("families"), list):
        fail("slow-proof families config must expose a families array")
    return payload


def selector_plan(args: argparse.Namespace) -> dict[str, Any]:
    cmd = ["bash", str(SELECTOR), "--json"]
    if args.manifest:
        cmd.extend(["--manifest", str(args.manifest.resolve())])
    if args.changed_files:
        cmd.extend(["--changed-files", str(args.changed_files.resolve())])
    else:
        cmd.extend(["--base", args.base, "--head", args.head])
    if args.include_working_tree:
        cmd.append("--include-working-tree")
    result = subprocess.run(
        cmd,
        cwd=ROOT,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        fail(f"selector failed: {result.stderr.strip()}")
    try:
        plan = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        fail(f"selector returned invalid JSON: {exc}")
    if plan.get("schema_version") != "adl.validation_lane_plan.v1":
        fail("selector returned unsupported schema_version")
    return plan


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        manifest = json.loads(path.read_text())
    except FileNotFoundError as exc:
        fail(f"validation manifest not found: {exc.filename}")
    except json.JSONDecodeError as exc:
        fail(f"validation manifest is not valid JSON: {exc}")
    if manifest.get("schema_version") != "adl.validation_lane_selector.v1":
        fail("validation manifest returned unsupported schema_version")
    return manifest


def guardrail_int(value: Any, field: str, default: int) -> int:
    if value is None:
        return default
    try:
        return int(value)
    except (TypeError, ValueError):
        fail(f"manager guardrail {field} must be an integer")
    raise AssertionError("unreachable")


def manager_guardrails(manifest: dict[str, Any], max_selected_lanes: int) -> dict[str, Any]:
    configured = manifest.get("manager_guardrails", {})
    pr_fast = configured.get("pr_fast", {})
    return {
        "max_selected_lanes": max_selected_lanes,
        "docs_only_forbidden_lane_ids": list(
            configured.get("docs_only_forbidden_lane_ids", ["rust_pr_fast"])
        ),
        "pr_fast": {
            "max_rust_surface_count": guardrail_int(
                pr_fast.get("max_rust_surface_count"), "pr_fast.max_rust_surface_count", 4
            ),
            "max_filter_token_count": guardrail_int(
                pr_fast.get("max_filter_token_count"), "pr_fast.max_filter_token_count", 4
            ),
            "max_family_token_count": guardrail_int(
                pr_fast.get("max_family_token_count"), "pr_fast.max_family_token_count", 3
            ),
            "blocked_modes": list(pr_fast.get("blocked_modes", ["full", "contract_only"])),
        },
    }


def profile_id(plan: dict[str, Any]) -> str:
    aggregate = plan.get("aggregate_status", "unknown")
    lanes = sorted(plan.get("lanes", {}).keys())
    if not lanes:
        return "validation_none"
    if aggregate == "selected" and len(lanes) == 1:
        return f"{lanes[0]}_profile"
    return f"{aggregate}_{len(lanes)}_lane_profile"


def lane_requirement_ids(lane: dict[str, Any]) -> list[str]:
    requirement_ids = lane.get("requirement_ids", [])
    if not isinstance(requirement_ids, list):
        return []
    return [item for item in requirement_ids if isinstance(item, str) and item]


def split_csv(value: Any) -> list[str]:
    if not isinstance(value, str) or not value.strip():
        return []
    return [item for item in value.split(",") if item]


def lane_behavior_id(lane_id: str, lane: dict[str, Any]) -> str:
    proof_role = str(lane.get("proof_role", "")).strip()
    default_surface = str(lane.get("default_surface", "")).strip()
    mode = str(lane.get("mode", "")).strip()
    if lane_id == "rust_pr_fast":
        suffix = mode or "unknown"
        return f"rust_{suffix}_behavior"
    if proof_role:
        return f"{proof_role}_{lane_id}"
    if default_surface:
        return f"{default_surface}_behavior"
    return lane_id


def lane_behavior_surface(lane_id: str, lane: dict[str, Any]) -> dict[str, Any]:
    matched_paths = lane.get("matched_paths", [])
    requirement_ids = lane_requirement_ids(lane)
    if lane_id == "rust_pr_fast":
        requirement_ids.extend(split_csv(lane.get("filter_tokens", "")))
    return {
        "id": lane_behavior_id(lane_id, lane),
        "source": "validation_lane_selector",
        "lane_id": lane_id,
        "owner": lane.get("owner", "unknown"),
        "default_surface": lane.get("default_surface", "unknown"),
        "proof_role": lane.get("proof_role", "unknown"),
        "resource_class": lane.get("resource_class", "unknown"),
        "determinism_posture": lane.get("determinism_posture", "unknown"),
        "escalation_rule": lane.get("escalation_rule", "unknown"),
        "requirement_ids": requirement_ids or [lane_id],
        "matched_paths": matched_paths,
        "risk_class": lane.get("risk_class", "unknown"),
    }


def validation_dag_node(lane_id: str, lane: dict[str, Any], behavior_id: str) -> dict[str, Any]:
    status = lane.get("status", "unknown")
    if status == "selected":
        node_status = "runnable"
    elif status in {"escalated", "release_gate_required"}:
        node_status = "blocked_for_escalation"
    else:
        node_status = "not_selected"
    return {
        "id": f"node_{lane_id}",
        "lane_id": lane_id,
        "behavior_surface": behavior_id,
        "status": node_status,
        "proof_role": lane.get("proof_role", "unknown"),
        "owner": lane.get("owner", "unknown"),
        "resource_class": lane.get("resource_class", "unknown"),
        "determinism_posture": lane.get("determinism_posture", "unknown"),
        "command": lane.get("run_command") or lane.get("command", ""),
        "depends_on": [],
    }


def estimate_cost(selected: list[tuple[str, dict[str, Any]]], blocked: list[tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    if blocked:
        runtime_class = "escalated"
    elif not selected:
        runtime_class = "none"
    elif len(selected) == 1:
        lane_id = selected[0][0]
        runtime_class = "tiny" if "docs" in lane_id else "normal"
    else:
        runtime_class = "normal"
    return {
        "runtime_class": runtime_class,
        "selected_lane_count": len(selected),
        "blocked_lane_count": len(blocked),
        "expected_test_scope": "focused_or_family" if not blocked else "requires_human_or_release_gate_decision",
        "token_review_cost": "low" if runtime_class in {"none", "tiny"} else "medium",
    }


def manifest_rule_for(lane: dict[str, Any]) -> str:
    manifest_rule = lane.get("manifest_rule")
    if isinstance(manifest_rule, str) and manifest_rule:
        return manifest_rule
    return f"lane:{lane.get('lane_id', 'unknown')}"


def add_escalation_reason(
    escalation_reasons: list[dict[str, Any]],
    *,
    lane_id: str,
    status: str,
    reason: str,
    matched_paths: list[str],
    manifest_rule: str,
    remediation_hint: str,
    triggering_surface: str | None = None,
) -> None:
    item: dict[str, Any] = {
        "lane_id": lane_id,
        "status": status,
        "reason": reason,
        "matched_paths": matched_paths,
        "manifest_rule": manifest_rule,
        "remediation_hint": remediation_hint,
    }
    if triggering_surface:
        item["triggering_surface"] = triggering_surface
    escalation_reasons.append(item)


def add_diagnostic(
    diagnostics: list[dict[str, Any]],
    *,
    code: str,
    lane_id: str,
    message: str,
    matched_paths: list[str],
    manifest_rule: str,
    remediation_hint: str,
    triggering_surface: str | None = None,
) -> None:
    item: dict[str, Any] = {
        "code": code,
        "severity": "error",
        "lane_id": lane_id,
        "message": message,
        "matched_paths": matched_paths,
        "manifest_rule": manifest_rule,
        "remediation_hint": remediation_hint,
    }
    if triggering_surface:
        item["triggering_surface"] = triggering_surface
    diagnostics.append(item)


def docs_only_paths(paths: list[str]) -> bool:
    if not paths:
        return False
    return all(path.endswith(".md") or path.startswith("docs/") for path in paths)


def build_profile(plan: dict[str, Any], guardrails: dict[str, Any], manifest_path: Path) -> dict[str, Any]:
    slow_proof_config = load_slow_proof_families()
    slow_proof_families = slow_proof_config.get("families", [])
    lanes = plan.get("lanes", {})
    changed_paths = plan.get("changed_paths", [])
    covered_paths = {
        path
        for lane in lanes.values()
        for path in lane.get("matched_paths", [])
    }
    uncovered_paths = [path for path in changed_paths if path not in covered_paths]
    selected = [
        (lane_id, lane)
        for lane_id, lane in lanes.items()
        if lane.get("status") == "selected"
    ]
    blocked = [
        (lane_id, lane)
        for lane_id, lane in lanes.items()
        if lane.get("status") in {"escalated", "release_gate_required"}
    ]

    run = []
    behavior_surfaces = []
    dag_nodes = []
    diagnostics: list[dict[str, Any]] = []
    for lane_id, lane in selected:
        behavior = lane_behavior_surface(lane_id, lane)
        behavior_surfaces.append(behavior)
        dag_nodes.append(validation_dag_node(lane_id, lane, behavior["id"]))
        command = lane.get("run_command") or lane.get("command")
        if command:
            run.append(
                {
                    "lane_id": lane_id,
                    "command": command,
                    "reason": lane.get("reason", "selector_selected_lane"),
                    "matched_paths": lane.get("matched_paths", []),
                    "vpp_record": lane.get("vpp_record"),
                }
            )

    not_run = [
        {
            "surface": "full_workspace_nextest",
            "reason": "not selected by validation profile",
        },
        {
            "surface": "cargo_clippy_all_targets",
            "reason": "reserved for broad shared changes or release gates",
        },
        {
            "surface": "slow_proof",
            "reason": "reserved for explicit proof-family selection",
        },
        {
            "surface": "coverage_release_gate",
            "reason": "reserved for coverage or release policy selection",
        },
    ]
    not_run.extend(
        {
            "surface": f"slow_proof/{family['id']}",
            "reason": "reserved for explicit proof-family selection",
            "feature": family["feature"],
        }
        for family in slow_proof_families
    )

    unmapped_change_gap = bool(uncovered_paths)
    selected_lane_limit = int(guardrails["max_selected_lanes"])
    manifest_path_display = (
        str(manifest_path.relative_to(ROOT))
        if manifest_path.is_relative_to(ROOT)
        else str(manifest_path)
    )

    escalation_required = (
        bool(blocked)
        or len(selected) > selected_lane_limit
        or unmapped_change_gap
    )
    escalation_reasons = []
    for lane_id, lane in blocked:
        behavior = lane_behavior_surface(lane_id, lane)
        behavior_surfaces.append(behavior)
        dag_nodes.append(validation_dag_node(lane_id, lane, behavior["id"]))
        manifest_rule = manifest_rule_for(lane)
        matched_paths = lane.get("matched_paths", [])
        remediation_hint = (
            "Record a release-gate disposition before publication."
            if lane_id == "release_gate_review"
            else "Record a slow-proof disposition before treating the change as ordinary PR proof."
            if lane_id == "slow_proof_review"
            else "Split the Rust change further or route it to the appropriate broad proof lane."
            if lane_id == "rust_pr_fast"
            else "Adjust the validation manifest or route the work to the correct owner lane."
        )
        add_escalation_reason(
            escalation_reasons,
            lane_id=lane_id,
            status=str(lane.get("status", "escalated")),
            reason=str(lane.get("reason", "selector_requires_escalation")),
            matched_paths=matched_paths,
            manifest_rule=manifest_rule,
            remediation_hint=remediation_hint,
            triggering_surface=matched_paths[0] if matched_paths else None,
        )
        add_diagnostic(
            diagnostics,
            code=f"{lane_id}_requires_escalation",
            lane_id=lane_id,
            message=f"{lane_id} requires escalation because {lane.get('reason', 'selector_requires_escalation')}",
            matched_paths=matched_paths,
            manifest_rule=manifest_rule,
            remediation_hint=remediation_hint,
            triggering_surface=matched_paths[0] if matched_paths else None,
        )
    if len(selected) > selected_lane_limit:
        add_escalation_reason(
            escalation_reasons,
            lane_id="selected_lane_threshold",
            status="escalated",
            reason=f"selected lane count {len(selected)} exceeds limit {selected_lane_limit}",
            matched_paths=plan.get("changed_paths", []),
            manifest_rule="manager_guardrails.max_selected_lanes",
            remediation_hint="Split the change set or raise the threshold intentionally in the validation manager guardrails.",
        )
        add_diagnostic(
            diagnostics,
            code="selected_lane_threshold_exceeded",
            lane_id="selected_lane_threshold",
            message=f"selected lane count {len(selected)} exceeds configured limit {selected_lane_limit}",
            matched_paths=plan.get("changed_paths", []),
            manifest_rule="manager_guardrails.max_selected_lanes",
            remediation_hint="Split the change set or raise the threshold intentionally in the validation manager guardrails.",
        )
    if unmapped_change_gap:
        add_escalation_reason(
            escalation_reasons,
            lane_id="unmapped_change_surface",
            status="escalated",
            reason="selector left changed paths without validation-lane coverage",
            matched_paths=uncovered_paths,
            manifest_rule=manifest_path_display,
            remediation_hint="Add or refine a path selector in the validation manifest so the changed surface maps to a proving lane.",
        )
        add_diagnostic(
            diagnostics,
            code="unmapped_change_surface",
            lane_id="unmapped_change_surface",
            message="selector left changed paths without validation-lane coverage",
            matched_paths=uncovered_paths,
            manifest_rule=manifest_path_display,
            remediation_hint="Add or refine a path selector in the validation manifest so the changed surface maps to a proving lane.",
        )

    if docs_only_paths(changed_paths):
        forbidden_lane_ids = set(guardrails.get("docs_only_forbidden_lane_ids", []))
        rust_docs_lanes = [lane_id for lane_id, _lane in selected + blocked if lane_id in forbidden_lane_ids]
        if rust_docs_lanes:
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="docs_only_rust_guardrail",
                status="escalated",
                reason=f"docs-only change selected forbidden lanes: {', '.join(sorted(rust_docs_lanes))}",
                matched_paths=changed_paths,
                manifest_rule="manager_guardrails.docs_only_forbidden_lane_ids",
                remediation_hint="Keep docs-only profiles mapped to docs proof only; route Rust-affecting docs through a separate non-docs issue if needed.",
            )
            add_diagnostic(
                diagnostics,
                code="docs_only_rust_guardrail",
                lane_id="docs_only_rust_guardrail",
                message=f"docs-only change selected forbidden lanes: {', '.join(sorted(rust_docs_lanes))}",
                matched_paths=changed_paths,
                manifest_rule="manager_guardrails.docs_only_forbidden_lane_ids",
                remediation_hint="Keep docs-only profiles mapped to docs proof only; route Rust-affecting docs through a separate non-docs issue if needed.",
            )

    rust_lane = lanes.get("rust_pr_fast")
    if isinstance(rust_lane, dict):
        pr_fast_guardrails = guardrails["pr_fast"]
        rust_surface_count = int(rust_lane.get("rust_surface_count", 0))
        filter_tokens = split_csv(rust_lane.get("filter_tokens", ""))
        mode = str(rust_lane.get("mode", "")).strip()
        matched_paths = rust_lane.get("matched_paths", [])
        if mode in pr_fast_guardrails["blocked_modes"]:
            escalation_required = True
            add_diagnostic(
                diagnostics,
                code=f"pr_fast_mode_{mode}",
                lane_id="rust_pr_fast",
                message=(
                    "slow-proof contract-only planning cannot be run as an ordinary PR-fast profile"
                    if mode == "contract_only"
                    else "PR-fast planning expanded beyond the configured ordinary profile guardrails"
                ),
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.blocked_modes",
                remediation_hint="Use the named slow-proof or broad proof path instead of forcing ordinary PR-fast execution.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
        if mode == "focused" and len(filter_tokens) > int(pr_fast_guardrails["max_filter_token_count"]):
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="rust_pr_fast",
                status="escalated",
                reason=f"focused filter count {len(filter_tokens)} exceeds limit {pr_fast_guardrails['max_filter_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_filter_token_count",
                remediation_hint="Split the change or raise the focused threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
            add_diagnostic(
                diagnostics,
                code="pr_fast_filter_threshold_exceeded",
                lane_id="rust_pr_fast",
                message=f"focused filter count {len(filter_tokens)} exceeds configured limit {pr_fast_guardrails['max_filter_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_filter_token_count",
                remediation_hint="Split the change or raise the focused threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
        if mode == "family" and len(filter_tokens) > int(pr_fast_guardrails["max_family_token_count"]):
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="rust_pr_fast",
                status="escalated",
                reason=f"family filter count {len(filter_tokens)} exceeds limit {pr_fast_guardrails['max_family_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_family_token_count",
                remediation_hint="Split the change or raise the family threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
            add_diagnostic(
                diagnostics,
                code="pr_fast_family_threshold_exceeded",
                lane_id="rust_pr_fast",
                message=f"family filter count {len(filter_tokens)} exceeds configured limit {pr_fast_guardrails['max_family_token_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_family_token_count",
                remediation_hint="Split the change or raise the family threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
        if rust_surface_count > int(pr_fast_guardrails["max_rust_surface_count"]):
            escalation_required = True
            add_escalation_reason(
                escalation_reasons,
                lane_id="rust_pr_fast",
                status="escalated",
                reason=f"Rust surface count {rust_surface_count} exceeds limit {pr_fast_guardrails['max_rust_surface_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_rust_surface_count",
                remediation_hint="Split the change or raise the Rust-surface threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )
            add_diagnostic(
                diagnostics,
                code="pr_fast_rust_surface_threshold_exceeded",
                lane_id="rust_pr_fast",
                message=f"Rust surface count {rust_surface_count} exceeds configured limit {pr_fast_guardrails['max_rust_surface_count']}",
                matched_paths=matched_paths,
                manifest_rule="manager_guardrails.pr_fast.max_rust_surface_count",
                remediation_hint="Split the change or raise the Rust-surface threshold intentionally in manager guardrails.",
                triggering_surface=matched_paths[0] if matched_paths else None,
            )

    status = "ready_to_run"
    if not changed_paths:
        status = "no_validation_needed"
    elif escalation_required:
        status = "escalation_required"
    elif plan.get("aggregate_status") != "selected":
        status = "not_runnable"

    pr_publication_sufficient = (
        bool(plan.get("pr_publication_sufficient"))
        and not unmapped_change_gap
        and not escalation_required
        and status == "ready_to_run"
    )

    return {
        "schema_version": "adl.validation_profile.v1",
        "selected_profile": profile_id(plan),
        "status": status,
        "selector_aggregate_status": plan.get("aggregate_status"),
        "pr_publication_sufficient": pr_publication_sufficient,
        "changed_paths": changed_paths,
        "run": run,
        "not_run": not_run,
        "deferred": [],
        "behavior_surfaces": behavior_surfaces,
        "validation_dag": {
            "nodes": dag_nodes,
            "edges": [],
            "compression_note": "profile validates behavior surfaces rather than enumerating every test-bearing module",
        },
        "slow_proof_families": [
            {
                "id": family["id"],
                "feature": family["feature"],
                "proof_role": family.get("proof_role", "slow_proof"),
                "description": family.get("description", ""),
                "selection_mode": "manual_only",
                "command": f"bash adl/tools/run_slow_proof_family.sh --family {family['id']} --run",
                "sample_tests": family.get("sample_tests", []),
            }
            for family in slow_proof_families
        ],
        "estimated_cost": estimate_cost(selected, blocked),
        "escalation": {
            "required": escalation_required,
            "reasons": escalation_reasons,
        },
        "diagnostics": diagnostics,
        "selector_plan": plan,
    }


def shell_quote(value: str) -> str:
    return shlex.quote(value)


def remote_runner_decision(profile: dict[str, Any], args: argparse.Namespace) -> dict[str, Any] | None:
    if not args.remote_runner and not args.remote_command:
        return None
    if bool(args.remote_runner) != bool(args.remote_command):
        fail("remote runner selection requires both --remote-runner and --remote-command")
    if args.remote_runner != "nessus":
        fail(f"unsupported remote runner: {args.remote_runner}")

    selected_lane_count = int(profile["estimated_cost"]["selected_lane_count"])
    runtime_class = str(profile["estimated_cost"]["runtime_class"])
    behavior_surfaces = profile.get("behavior_surfaces", [])
    allowed_determinism = {"deterministic", "evidence_bound"}
    unsupported_determinism = [
        surface.get("determinism_posture", "unknown")
        for surface in behavior_surfaces
        if surface.get("determinism_posture", "unknown") not in allowed_determinism
    ]

    if profile["status"] != "ready_to_run":
        return {
            "requested": "nessus",
            "decision": "rejected",
            "reason": f"validation profile status {profile['status']} is not remote-runnable",
        }
    if profile["escalation"]["required"]:
        return {
            "requested": "nessus",
            "decision": "rejected",
            "reason": "validation profile already requires escalation",
        }
    if selected_lane_count != 1:
        return {
            "requested": "nessus",
            "decision": "rejected",
            "reason": f"remote runner requires exactly 1 selected lane, observed {selected_lane_count}",
        }
    if runtime_class in {"none", "tiny", "escalated"}:
        return {
            "requested": "nessus",
            "decision": "rejected",
            "reason": f"runtime_class {runtime_class} is not eligible for Nessus remote execution",
        }
    if unsupported_determinism:
        return {
            "requested": "nessus",
            "decision": "rejected",
            "reason": "remote runner supports deterministic or evidence-bound lanes only",
            "unsupported_determinism": unsupported_determinism,
        }

    remote_command = f"{NESSUS_REMOTE_RUNNER} --command {shell_quote(args.remote_command)}"
    if args.remote_artifact_dir:
        remote_command += f" --local-artifact-dir {shell_quote(str(args.remote_artifact_dir.resolve()))}"
    return {
        "requested": "nessus",
        "decision": "selected",
        "reason": "single-lane non-tiny deterministic profile is eligible for Nessus remote execution",
        "command": remote_command,
    }


def print_text(profile: dict[str, Any]) -> None:
    print("Validation profile")
    print(f"  selected_profile={profile['selected_profile']}")
    print(f"  status={profile['status']}")
    print(f"  selector_aggregate_status={profile['selector_aggregate_status']}")
    print(f"  pr_publication_sufficient={str(profile['pr_publication_sufficient']).lower()}")
    if profile["run"]:
        print("  run:")
        for item in profile["run"]:
            print(f"    - lane={item['lane_id']} reason={item['reason']}")
            print(f"      command={item['command']}")
    else:
        print("  run: []")
    if profile.get("remote_runner"):
        remote = profile["remote_runner"]
        print("  remote_runner:")
        print(f"    requested={remote['requested']}")
        print(f"    decision={remote['decision']}")
        print(f"    reason={remote['reason']}")
        if remote.get("command"):
            print(f"    command={remote['command']}")
    if profile["escalation"]["required"]:
        print("  escalation:")
        for reason in profile["escalation"]["reasons"]:
            print(f"    - lane={reason['lane_id']} status={reason['status']} reason={reason['reason']}")
            if reason.get("triggering_surface"):
                print(f"      triggering_surface={reason['triggering_surface']}")
            if reason.get("manifest_rule"):
                print(f"      manifest_rule={reason['manifest_rule']}")
            if reason.get("remediation_hint"):
                print(f"      remediation_hint={reason['remediation_hint']}")
    if profile["diagnostics"]:
        print("  diagnostics:")
        for item in profile["diagnostics"]:
            print(f"    - code={item['code']} lane={item['lane_id']}")
            print(f"      message={item['message']}")
            if item.get("triggering_surface"):
                print(f"      triggering_surface={item['triggering_surface']}")
            print(f"      manifest_rule={item['manifest_rule']}")
            print(f"      remediation_hint={item['remediation_hint']}")
    if profile["behavior_surfaces"]:
        print("  behavior_surfaces:")
        for behavior in profile["behavior_surfaces"]:
            print(f"    - id={behavior['id']} risk={behavior['risk_class']} source={behavior['source']}")
    if profile.get("slow_proof_families"):
        print("  slow_proof_families:")
        for family in profile["slow_proof_families"]:
            print(
                f"    - id={family['id']} feature={family['feature']} selection_mode={family['selection_mode']}"
            )
    print(
        "  estimated_cost="
        f"{profile['estimated_cost']['runtime_class']} "
        f"lanes={profile['estimated_cost']['selected_lane_count']}"
    )


def write_report(path: Path, profile: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(profile, indent=2, sort_keys=True) + "\n")


def run_profile(profile: dict[str, Any]) -> int:
    remote_runner = profile.get("remote_runner")
    if remote_runner and remote_runner.get("decision") != "selected":
        print_text(profile)
        print("validation_manager: refusing --run because the requested remote runner is not eligible", file=sys.stderr)
        return 1
    if profile["status"] not in {"ready_to_run", "no_validation_needed"}:
        print_text(profile)
        print("validation_manager: refusing --run for non-runnable profile", file=sys.stderr)
        return 1
    failed = False
    for item in profile["run"]:
        print(f"==> {item['lane_id']}: {item['command']}", file=sys.stderr)
        result = subprocess.run(item["command"], cwd=ROOT, shell=True)
        item["run_status"] = "passed" if result.returncode == 0 else "failed"
        if result.returncode != 0:
            failed = True
    profile["run_status"] = "failed" if failed else "passed"
    return 1 if failed else 0


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group()
    source.add_argument("--changed-files", type=Path)
    source.add_argument("--include-working-tree", action="store_true")
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--base", default="origin/main")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--max-selected-lanes", type=int, default=8)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--report-out", type=Path)
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--remote-runner", choices=["nessus"])
    parser.add_argument("--remote-command")
    parser.add_argument("--remote-artifact-dir", type=Path)
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    plan = selector_plan(args)
    manifest_path = args.manifest.resolve() if args.manifest else DEFAULT_MANIFEST
    guardrails = manager_guardrails(load_manifest(manifest_path), args.max_selected_lanes)
    profile = build_profile(plan, guardrails, manifest_path)
    remote = remote_runner_decision(profile, args)
    if remote:
        profile["remote_runner"] = remote
        if remote["decision"] == "selected":
            profile["run"] = [
                {
                    "lane_id": "nessus_remote_validation",
                    "command": remote["command"],
                    "reason": remote["reason"],
                    "matched_paths": profile["changed_paths"],
                    "vpp_record": None,
                    "local_run": profile["run"],
                }
            ]
    exit_code = 0
    if args.run:
        exit_code = run_profile(profile)
    if args.report_out:
        write_report(args.report_out, profile)
    if args.json:
        print(json.dumps(profile, indent=2, sort_keys=True))
    else:
        print_text(profile)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
