#!/usr/bin/env python3
import json
import sys
from pathlib import Path
import hashlib


def fail(message: str) -> None:
    print(f"FAIL validate_v0914_obsmem_transition_memory: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except Exception as exc:  # pragma: no cover - fail closed
        fail(f"could not parse {path}: {exc}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def sha256_text(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def repo_relative(path: str) -> bool:
    return bool(path) and not path.startswith("/") and ".." not in Path(path).parts


def find_repo_root(start: Path) -> Path:
    for candidate in [start, *start.parents]:
        if (candidate / "adl").is_dir() and (candidate / "docs").is_dir():
            return candidate
    fail(f"could not determine repo root from {start}")


def require_repo_relative_path(repo_root: Path, rel: str, key: str) -> Path:
    require(repo_relative(rel), f"{key} must be repo-relative without traversal")
    require(not rel.startswith(".adl/"), f"{key} must not point into local-only .adl state")
    target = repo_root / rel
    require(target.exists(), f"{key} target missing: {rel}")
    return target


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0914_obsmem_transition_memory.py <packet_dir>")

    packet_dir = Path(sys.argv[1]).resolve()
    repo_root = find_repo_root(packet_dir)

    required_files = [
        packet_dir / "README.md",
        packet_dir / "OBSMEM_TRANSITION_MEMORY_PACKET_v0.91.4.md",
        packet_dir / "ct_demo_001_transition_outcome_truth.json",
        packet_dir / "ct_demo_001_obsmem_transition_memory_handoff.json",
    ]
    for path in required_files:
        require(path.exists(), f"missing required packet file: {path}")

    handoff = load_json(packet_dir / "ct_demo_001_obsmem_transition_memory_handoff.json")
    outcome = load_json(packet_dir / "ct_demo_001_transition_outcome_truth.json")
    require(handoff.get("schema_version") == 1, "handoff schema_version must be 1")
    require(outcome.get("schema_version") == 1, "outcome schema_version must be 1")
    require(handoff.get("workflow_id") == "v0914_csdlc_transition_memory", "unexpected workflow_id")
    require(len(handoff.get("follow_ons", [])) >= 2, "handoff must preserve visible follow-ons")

    outcome_truth_path = require_repo_relative_path(repo_root, handoff.get("outcome_truth_path", ""), "outcome_truth_path")
    evidence_bundle_path = require_repo_relative_path(repo_root, handoff.get("evidence_bundle_path", ""), "evidence_bundle_path")
    review_synthesis_path = require_repo_relative_path(repo_root, handoff.get("review_synthesis_path", ""), "review_synthesis_path")
    signed_trace_path = require_repo_relative_path(repo_root, handoff.get("signed_trace_path", ""), "signed_trace_path")
    signed_trace_public_key_path = require_repo_relative_path(
        repo_root,
        handoff.get("signed_trace_public_key_path", ""),
        "signed_trace_public_key_path",
    )

    require(
        outcome_truth_path.resolve() == (packet_dir / "ct_demo_001_transition_outcome_truth.json").resolve(),
        "outcome_truth_path must point at the tracked packet-local outcome truth file",
    )

    review = load_json(review_synthesis_path)
    evidence = load_json(evidence_bundle_path)

    signed_trace = evidence.get("signed_trace", {})
    require(
        handoff.get("signed_trace_path") == signed_trace.get("signed_path"),
        "handoff signed_trace_path must match evidence bundle signed_trace.signed_path",
    )
    require(
        handoff.get("signed_trace_public_key_path") == signed_trace.get("public_key_path"),
        "handoff signed_trace_public_key_path must match evidence bundle signed_trace.public_key_path",
    )
    require(
        signed_trace.get("verification_mode") == "explicit_key",
        "signed trace verification_mode must be explicit_key",
    )
    require(
        signed_trace_path == (repo_root / signed_trace.get("signed_path", "")),
        "signed trace path must align with the evidence bundle signed trace path",
    )
    require(
        signed_trace_public_key_path == (repo_root / signed_trace.get("public_key_path", "")),
        "signed trace public key path must align with the evidence bundle public key path",
    )

    evidence_inputs = evidence.get("evidence_inputs", [])
    require(evidence_inputs, "evidence bundle must include tracked evidence inputs")
    for entry in evidence_inputs:
        rel = entry.get("path", "")
        require(repo_relative(rel), f"evidence input path must stay repo-relative: {rel}")
        target = repo_root / rel
        require(target.exists(), f"evidence input target missing: {rel}")
        expected = entry.get("sha256", "")
        require(len(expected) == 64, f"evidence input sha256 must be 64 hex chars: {rel}")
        require(sha256_text(target) == expected, f"evidence input sha256 mismatch: {rel}")

    require(
        outcome.get("issue_number") == review.get("source_issue_number") == evidence.get("issue_number"),
        "issue_number must align across outcome truth, review synthesis, and evidence bundle",
    )
    require(
        outcome.get("pr_number") == review.get("source_pr_number"),
        "pr_number must align across outcome truth and review synthesis",
    )
    require(
        outcome.get("lifecycle_state") == "merged",
        "outcome truth must record merged lifecycle state",
    )

    packet_text = (packet_dir / "OBSMEM_TRANSITION_MEMORY_PACKET_v0.91.4.md").read_text()
    for required_snippet in [
        "tracked handoff inputs only",
        "review findings and residual risks remain",
        "local ignored `.adl` state",
    ]:
        require(required_snippet in packet_text, f"packet missing required phrase: {required_snippet}")

    print("PASS validate_v0914_obsmem_transition_memory")


if __name__ == "__main__":
    main()
