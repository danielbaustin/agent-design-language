#!/usr/bin/env python3
"""Deterministic WP-16 quality gate for v0.91.7 retained evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


REQUIRED_PATHS = [
    "docs/milestones/v0.91.7/README.md",
    "docs/milestones/v0.91.7/WBS_v0.91.7.md",
    "docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml",
    "docs/milestones/v0.91.7/MILESTONE_CHECKLIST_v0.91.7.md",
    "docs/milestones/v0.91.7/REVIEW_AND_VALIDATION_CHECKLIST_v0.91.7.md",
    "docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md",
    "docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md",
    "docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md",
    "docs/milestones/v0.91.7/review/wp14_launch_birthday_4641/ledger.yaml",
    "docs/milestones/v0.91.7/review/V0917_WP15_DEMO_CONVERGENCE_4642.md",
    "docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json",
]

REQUIRED_FEATURE_SURFACES = {
    "demo_matrix": {
        "classification": "proven",
        "issues": {4691},
        "evidence": {"docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md"},
    },
    "html_observatory": {
        "classification": "proven",
        "issues": {4690},
        "evidence": {"demos/html-observatory/README.md"},
    },
    "runtime_v2_observatory_packet": {
        "classification": "proven_retained",
        "issues": {4682},
        "evidence": {
            "docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json"
        },
    },
    "runtime_v3_observatory_consumption": {
        "classification": "proved_explicit_opt_in",
        "issues": {5286},
        "evidence": {"docs/architecture/runtime_v3_observatory_consumption_5286.v1.json"},
    },
    "unity_observatory": {
        "classification": "proven_limited",
        "issues": {4652, 4689, 4702, 4703, 4704, 4745},
        "evidence": {
            "docs/milestones/v0.91.7/review/unity_observatory_4689/4689-unity-observatory-integrated-proof.md"
        },
    },
    "launch_birthday_handoff": {
        "classification": "retained_handoff",
        "issues": {4641},
        "evidence": {"docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md"},
    },
    "curiosity_engine_discovery_substrate": {
        "classification": "proof_backed_no_demo",
        "issues": {4692},
        "evidence": {
            "docs/milestones/v0.91.7/features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md"
        },
    },
    "constructability_anchor_validator": {
        "classification": "proof_backed_no_demo",
        "issues": {4693},
        "evidence": {"docs/milestones/v0.91.7/features/CONSTRUCTABILITY_GATE_v0.91.7.md"},
    },
    "reasoning_graph_loop_skill_aee_obsmem_pvf": {
        "classification": "proof_backed_partial_demo",
        "issues": {4694, 4695, 4696, 4697, 4912, 5096, 5136},
        "evidence": {
            "docs/milestones/v0.91.7/features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md"
        },
    },
}

REQUIRED_OPEN_GATES = {
    "WP-16": 4643,
    "WP-17": 4644,
    "WP-18": 4645,
    "WP-19": 4646,
    "WP-20": 4647,
    "WP-23": 4650,
}

REQUIRED_NON_CLAIMS = [
    "does not claim v0.91.7 release readiness",
    "does not claim v0.92 activation readiness",
    "does not claim Runtime v3 as the default runtime",
]


def read_text(root: Path, path: str) -> str:
    return (root / path).read_text(encoding="utf-8")


def check_required_paths(root: Path) -> list[dict[str, object]]:
    checks = []
    for path in REQUIRED_PATHS:
        checks.append({"path": path, "exists": (root / path).exists()})
    return checks


def check_feature_coverage(root: Path) -> dict[str, object]:
    path = root / "docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    entries = {entry.get("surface"): entry for entry in data.get("coverage", [])}
    missing_surfaces = sorted(set(REQUIRED_FEATURE_SURFACES) - set(entries))
    surface_checks = []
    for surface, required in REQUIRED_FEATURE_SURFACES.items():
        entry = entries.get(surface, {})
        evidence = set(entry.get("evidence", []))
        issue_truth = {
            int(item.get("issue"))
            for item in entry.get("current_issue_truth", [])
            if str(item.get("issue", "")).isdigit() and item.get("state") == "closed"
        }
        required_evidence = required["evidence"]
        missing_evidence = sorted(required_evidence - evidence)
        missing_existing_evidence = [
            item for item in sorted(evidence) if not (root / item).exists()
        ]
        missing_issues = sorted(required["issues"] - issue_truth)
        has_non_claims = bool(entry.get("non_claims"))
        surface_checks.append(
            {
                "surface": surface,
                "classification": entry.get("classification"),
                "expected_classification": required["classification"],
                "classification_ok": entry.get("classification") == required["classification"],
                "missing_required_evidence": missing_evidence,
                "missing_existing_evidence_paths": missing_existing_evidence,
                "missing_closed_issue_truth": missing_issues,
                "non_claims_present": has_non_claims,
                "pass": (
                    entry.get("classification") == required["classification"]
                    and not missing_evidence
                    and not missing_existing_evidence
                    and not missing_issues
                    and has_non_claims
                ),
            }
        )
    readiness_flags = {
        "demo_for_every_new_feature": data.get("demo_for_every_new_feature"),
        "release_readiness_claimed": data.get("release_readiness_claimed"),
        "v092_activation_readiness_claimed": data.get("v092_activation_readiness_claimed"),
        "runtime_v3_policy": data.get("runtime_v3_policy"),
    }
    return {
        "path": str(path.relative_to(root)),
        "valid_json": True,
        "missing_required_surfaces": missing_surfaces,
        "surface_checks": surface_checks,
        "readiness_flags": readiness_flags,
        "pass": (
            not missing_surfaces
            and all(check["pass"] for check in surface_checks)
            and data.get("demo_for_every_new_feature") is False
            and data.get("release_readiness_claimed") is False
            and data.get("v092_activation_readiness_claimed") is False
            and data.get("runtime_v3_policy") == "explicit_opt_in_only"
        ),
    }


def check_non_claims(root: Path) -> list[dict[str, object]]:
    wp15 = read_text(root, "docs/milestones/v0.91.7/review/V0917_WP15_DEMO_CONVERGENCE_4642.md")
    return [
        {"needle": needle, "present": needle in wp15}
        for needle in REQUIRED_NON_CLAIMS
    ]


def check_open_gate_table(root: Path) -> list[dict[str, object]]:
    coverage = read_text(root, "docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md")
    checks = []
    for gate, issue in REQUIRED_OPEN_GATES.items():
        row = f"| {gate} | #{issue} |"
        checks.append(
            {
                "gate": gate,
                "issue": issue,
                "expected_row_prefix": row,
                "present": row in coverage,
            }
        )
    return checks


def check_launch_handoff(root: Path) -> dict[str, object]:
    packet = read_text(root, "docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md")
    ledger = read_text(root, "docs/milestones/v0.91.7/review/wp14_launch_birthday_4641/ledger.yaml")
    child_issues = ["#4758", "#4759", "#4760", "#4761", "#4762", "#4763"]
    return {
        "packet_present": "routed_with_evidence" in packet,
        "child_issue_routes_present": all(issue in packet for issue in child_issues),
        "ledger_present": "routed_with_evidence" in ledger,
        "pass": (
            "routed_with_evidence" in packet
            and all(issue in packet for issue in child_issues)
            and "routed_with_evidence" in ledger
        ),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".", help="Repository root")
    parser.add_argument("--output", required=True, help="JSON output path")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    path_checks = check_required_paths(root)
    feature_coverage = check_feature_coverage(root)
    non_claims = check_non_claims(root)
    open_gates = check_open_gate_table(root)
    launch_handoff = check_launch_handoff(root)

    blocker_register = [
        {
            "id": "QG-4643-1",
            "status": "open_next_gate",
            "owner": "#4644",
            "summary": "Documentation alignment remains a required downstream gate.",
        },
        {
            "id": "QG-4643-2",
            "status": "open_next_gate",
            "owner": "#4645",
            "summary": "Internal review remains a required downstream gate.",
        },
        {
            "id": "QG-4643-3",
            "status": "open_next_gate",
            "owner": "#4646",
            "summary": "External review remains a required downstream gate.",
        },
        {
            "id": "QG-4643-4",
            "status": "open_next_gate",
            "owner": "#4647",
            "summary": "Review remediation/preflight remains a required downstream gate.",
        },
        {
            "id": "QG-4643-5",
            "status": "open_next_gate",
            "owner": "#4650",
            "summary": "Release ceremony remains a required downstream gate.",
        },
    ]

    passed = (
        all(check["exists"] for check in path_checks)
        and feature_coverage["pass"]
        and all(check["present"] for check in non_claims)
        and all(check["present"] for check in open_gates)
        and launch_handoff["pass"]
    )

    result = {
        "schema": "adl.v0917.wp16.quality_gate_4643.v1",
        "issue": 4643,
        "wp": "WP-16",
        "milestone": "v0.91.7",
        "status": "passed_with_open_downstream_gates" if passed else "failed",
        "release_readiness_claimed": False,
        "v092_activation_readiness_claimed": False,
        "aws_used": False,
        "checks": {
            "required_paths": path_checks,
            "feature_coverage": feature_coverage,
            "non_claims": non_claims,
            "open_gate_table": open_gates,
            "launch_handoff": launch_handoff,
        },
        "blocker_register": blocker_register,
    }
    output = root / args.output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
