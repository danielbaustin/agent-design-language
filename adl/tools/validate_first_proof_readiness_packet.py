#!/usr/bin/env python3
"""
Validate the tracked WP-08 first-proof-readiness packet.

This validator is stronger than a section-presence check:
- required packet files must exist
- the readiness record must contain required sections
- required upstream tracked proof surfaces must exist
- the canonical WP-02 manifest fixture must point at the expected WP-05/WP-06/WP-07 paths

It intentionally does not run the WP-09 proof demo or claim live GitHub gate
enforcement. WP-08 proves readiness, not execution.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


REQUIRED_FILES = [
    "README.md",
    "FIRST_PROOF_READINESS_PACKET_v0.91.3.md",
    "ct_demo_001_first_proof_readiness.md",
]

REQUIRED_SECTIONS = [
    "## Readiness Identity",
    "## Upstream Proof Inputs",
    "## Combined-Lane Readiness Checks",
    "## Closeout-Truth Lessons",
    "## Readiness Decision",
    "## Deferred / Non-Claims",
]

REQUIRED_PROOF_PATHS = [
    "docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/README.md",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/spp.md",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/srp.md",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sor.md",
    "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md",
    "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md",
    "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_evidence_bundle.md",
    "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_review_synthesis.md",
    "docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md",
    "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json",
    "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md",
]

REQUIRED_SNIPPETS = [
    "[#3203]",
    "[#3204]",
    "[#3205]",
    "[#3243]",
    "[#3244]",
    "[#3247]",
    "combined-lane validation",
    "closeout-truth",
    "ready_for_wp09",
]

EXPECTED_MANIFEST_PATHS = {
    "evidence_bundle_rel_path": "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_evidence_bundle.md",
    "merge_readiness_gate_rel_path": "docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md",
    "obsmem_handoff_rel_path": "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json",
}


def fail(message: str) -> int:
    print(f"first_proof_readiness_packet: FAIL {message}", file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_first_proof_readiness_packet.py <packet_root>")

    packet_root = Path(sys.argv[1]).resolve()
    if not packet_root.is_dir():
        return fail(f"packet root is not a directory: {packet_root}")

    repo_root = Path(__file__).resolve().parents[2]

    missing_files = [name for name in REQUIRED_FILES if not (packet_root / name).is_file()]
    if missing_files:
        return fail(f"missing required files: {', '.join(missing_files)}")

    readiness_record = (packet_root / "ct_demo_001_first_proof_readiness.md").read_text(
        encoding="utf-8"
    )
    missing_sections = [section for section in REQUIRED_SECTIONS if section not in readiness_record]
    if missing_sections:
        return fail(
            "readiness record missing required sections: " + ", ".join(missing_sections)
        )

    missing_snippets = [snippet for snippet in REQUIRED_SNIPPETS if snippet not in readiness_record]
    if missing_snippets:
        return fail(
            "readiness record missing required snippets: " + ", ".join(missing_snippets)
        )

    if ".adl/docs/TBD" in readiness_record:
        return fail("readiness record must not rely on local-only .adl/docs/TBD notes")

    missing_paths = [
        rel_path
        for rel_path in REQUIRED_PROOF_PATHS
        if not (repo_root / rel_path).is_file()
    ]
    if missing_paths:
        return fail("required upstream proof paths missing: " + ", ".join(missing_paths))

    manifest_path = repo_root / EXPECTED_MANIFEST_PATHS["evidence_bundle_rel_path"]
    del manifest_path  # silence linter intent; manifest is loaded from canonical fixture below
    manifest = json.loads(
        (
            repo_root
            / "docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json"
        ).read_text(encoding="utf-8")
    )
    for field, expected in EXPECTED_MANIFEST_PATHS.items():
        actual = manifest.get(field)
        if actual != expected:
            return fail(f"manifest field {field} mismatch: expected {expected}, got {actual}")

    print(f"first_proof_readiness_packet: PASS root={packet_root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
