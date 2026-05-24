#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> int:
    print(f"v0913_quality_gate_review_surfaces: FAIL {message}", file=sys.stderr)
    return 1


def require_text(path: Path, snippets: list[str]) -> str | None:
    text = path.read_text(encoding="utf-8")
    missing = [snippet for snippet in snippets if snippet not in text]
    if missing:
        return f"{path} missing required snippets: {', '.join(missing)}"
    return None


def require_demo_map_routes(path: Path) -> str | None:
    text = path.read_text(encoding="utf-8")
    required_routes = {
        "Cognitive Transition Manifest": "cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture",
        "Card Lifecycle Integration": "adl/tools/pr.sh doctor 3201 --version v0.91.3 --json",
        "Transition DAG And Shard Coordination": "python3 adl/tools/validate_transition_dag_packet.py docs/milestones/v0.91.3/review/transition_dag",
        "Evidence Bundle And Review Synthesis": "python3 adl/tools/validate_evidence_bundle_packet.py docs/milestones/v0.91.3/review/evidence_bundle",
        "Governed Merge-Readiness Gate": "python3 adl/tools/validate_merge_readiness_packet.py docs/milestones/v0.91.3/review/merge_readiness",
        "SRP/SOR ObsMem Handoff": "python3 adl/tools/validate_obsmem_handoff_packet.py docs/milestones/v0.91.3/review/obsmem_handoff",
        "Integrated Process Lessons And Proof Readiness": "python3 adl/tools/validate_first_proof_readiness_packet.py docs/milestones/v0.91.3/review/first_proof_readiness",
        "Five-Minute Sprint First Proof": "python3 adl/tools/demo_v0913_first_proof_demo.py",
        "C-SDLC Demo Proof Contract": "python3 adl/tools/validate_csdlc_demo_proof_contract_packet.py docs/milestones/v0.91.3/review/csdlc_demo_proof_contract",
    }
    missing: list[str] = []
    for feature, route in required_routes.items():
        if feature not in text or route not in text:
            missing.append(f"{feature} -> {route}")
    if missing:
        return "demo coverage map missing executable proof routes: " + "; ".join(missing)
    if "Missing feature with no truthful demo/proof route: none found" not in text:
        return "demo coverage map must record the missing-feature verdict"
    return None


def main() -> int:
    if len(sys.argv) != 3:
        return fail(
            "usage: validate_v0913_quality_gate_review_surfaces.py <repo_root> <surface>"
        )

    repo_root = Path(sys.argv[1]).resolve()
    surface = sys.argv[2]
    docs_root = repo_root / "docs/milestones/v0.91.3"

    if surface == "quality_gate_doc":
        path = docs_root / "QUALITY_GATE_v0.91.3.md"
        error = require_text(
            path,
            [
                "## Primary Run Path",
                "bash adl/tools/demo_v0913_quality_gate.sh",
                "## Current Gate Dimensions",
                "## Review Gate",
                "## Blockers",
                "## Non-Claims",
                "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md",
            ],
        )
        if error:
            return fail(error)
        linked = docs_root / "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md"
        if not linked.is_file():
            return fail(f"missing linked packet surface: {linked}")
    elif surface == "quality_gate_packet":
        path = docs_root / "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md"
        error = require_text(
            path,
            [
                "## Scope",
                "## Packet Contents",
                "## Demo Command",
                "## Focused Validation",
                "## Current Gate Dimensions",
                "## Boundaries",
                "README.md",
                "bash adl/tools/demo_v0913_quality_gate.sh",
                "bash adl/tools/test_demo_v0913_quality_gate.sh",
            ],
        )
        if error:
            return fail(error)
        linked = docs_root / "review/quality_gate/README.md"
        if not linked.is_file():
            return fail(f"missing linked packet README: {linked}")
    elif surface == "demo_coverage":
        path = docs_root / "review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md"
        error = require_text(
            path,
            [
                "## Claim Boundary",
                "## Primary Artifact",
                "## Review Use",
                "## Validation",
                "every current `v0.91.3` feature has a bounded reviewer-facing demo or proof path",
                "ct_demo_006_feature_demo_map.md",
            ],
        )
        if error:
            return fail(error)
        linked = docs_root / "review/demo_coverage/ct_demo_006_feature_demo_map.md"
        if not linked.is_file():
            return fail(f"missing linked demo-coverage map: {linked}")
        error = require_demo_map_routes(linked)
        if error:
            return fail(error)
    else:
        return fail(f"unknown surface: {surface}")

    print(f"v0913_quality_gate_review_surfaces: PASS surface={surface}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
