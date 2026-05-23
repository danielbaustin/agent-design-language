#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path


REQUIRED_FILES = {
    "README.md": [
        "# ObsMem Handoff Review Packet",
        "## Primary Proof Surfaces",
    ],
    "OBSMEM_HANDOFF_PROOF_PACKET_v0.91.3.md": [
        "# v0.91.3 ObsMem Handoff Proof Packet",
        "## Focused Validation",
    ],
    "ct_demo_001_obsmem_handoff.md": [
        "# CT Demo 001 ObsMem Handoff",
        "## Source Truth Boundary",
        "## SRP Review Learning Memory",
        "## SOR Outcome Truth Memory",
        "## Deferred / Outside Memory",
    ],
}


def fail(message: str) -> int:
    print(f"obsmem_handoff_packet: FAIL {message}", file=sys.stderr)
    return 1


def validate_repo_relative(label: str, value: str) -> None:
    if not value.strip():
        raise ValueError(f"{label} must be non-empty")
    if value.startswith("/") or ":" in value or "\\" in value or ".." in value:
        raise ValueError(f"{label} must be repo-relative without traversal")
    if value.startswith(".adl/"):
        raise ValueError(f"{label} must cite tracked packet artifacts, not local-only .adl paths")


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        return fail("usage: validate_obsmem_handoff_packet.py <packet-root>")

    root = Path(argv[1])
    if not root.is_dir():
        return fail(f"packet root does not exist: {root}")
    try:
        repo_root = root.resolve().parents[4]
    except IndexError:
        return fail(f"packet root does not have the expected repository depth: {root}")

    for rel_path, required_snippets in REQUIRED_FILES.items():
        path = root / rel_path
        if not path.is_file():
            return fail(f"missing required packet file: {path}")
        text = path.read_text(encoding="utf-8")
        for snippet in required_snippets:
            if snippet not in text:
                return fail(f"missing required snippet `{snippet}` in {path}")

    json_path = root / "ct_demo_001_obsmem_handoff.json"
    if not json_path.is_file():
        return fail(f"missing required packet file: {json_path}")

    data = json.loads(json_path.read_text(encoding="utf-8"))
    if data.get("schema_version") != "srp_sor_obsmem_handoff.v1":
        return fail("unexpected schema_version in handoff json")
    if data.get("transition_id") != "cts.v0_91_3.issue_3200.ct_demo_001":
        return fail("unexpected transition_id in handoff json")

    tracked = data.get("tracked_supporting_artifacts", {})
    for key in [
        "evidence_bundle_rel_path",
        "review_synthesis_rel_path",
        "merge_readiness_gate_rel_path",
    ]:
        value = tracked.get(key)
        if not isinstance(value, str):
            return fail(f"missing tracked supporting artifact `{key}`")
        try:
            validate_repo_relative(key, value)
        except ValueError as err:
            return fail(str(err))

    for entry_name, expected_kind, expected_truth in [
        ("srp_memory_entry", "srp_review_learning", "derived_from_final_srp"),
        ("sor_memory_entry", "sor_outcome_truth", "derived_from_final_sor"),
    ]:
        entry = data.get(entry_name)
        if not isinstance(entry, dict):
            return fail(f"missing required entry `{entry_name}`")
        if entry.get("entry_kind") != expected_kind:
            return fail(f"{entry_name} has unexpected entry_kind")
        if entry.get("source_truth") != expected_truth:
            return fail(f"{entry_name} has unexpected source_truth")
        source_record_rel_path = entry.get("source_record_rel_path")
        if not isinstance(source_record_rel_path, str):
            return fail(f"{entry_name} must declare source_record_rel_path")
        try:
            validate_repo_relative(f"{entry_name}.source_record_rel_path", source_record_rel_path)
        except ValueError as err:
            return fail(str(err))
        expected_suffix = "cards/srp.md" if entry_name == "srp_memory_entry" else "cards/sor.md"
        if not source_record_rel_path.endswith(expected_suffix):
            return fail(f"{entry_name}.source_record_rel_path must end with {expected_suffix}")
        if not (repo_root / source_record_rel_path).is_file():
            return fail(f"{entry_name}.source_record_rel_path does not exist in the repo")
        citations = entry.get("citations")
        if not isinstance(citations, list) or not citations:
            return fail(f"{entry_name} must have citations")
        if source_record_rel_path not in citations:
            return fail(f"{entry_name} citations must include source_record_rel_path")
        for idx, citation in enumerate(citations):
            if not isinstance(citation, str):
                return fail(f"{entry_name} citation {idx} must be a string")
            try:
                validate_repo_relative(f"{entry_name}.citations[{idx}]", citation)
            except ValueError as err:
                return fail(str(err))

    outside_memory = data.get("outside_memory")
    if not isinstance(outside_memory, list) or not outside_memory:
        return fail("outside_memory must be a non-empty list")

    print(f"obsmem_handoff_packet: PASS root={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
