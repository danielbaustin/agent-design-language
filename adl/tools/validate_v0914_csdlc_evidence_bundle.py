#!/usr/bin/env python3
import hashlib
import json
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def sha256_text(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def ensure_repo_relative(path: str) -> None:
    if path.startswith("/") or ".." in Path(path).parts:
        fail(f"path must stay repo-relative and non-traversing: {path}")


def extract_signed_public_key(signed_trace_path: Path) -> str:
    for line in signed_trace_path.read_text().splitlines():
        stripped = line.strip()
        if stripped.startswith("public_key_b64:"):
            return stripped.split(":", 1)[1].strip()
    fail("signed trace fixture is missing embedded public_key_b64")


def find_repo_root(start: Path) -> Path:
    for candidate in [start, *start.parents]:
        if (candidate / "adl").is_dir() and (candidate / "docs").is_dir():
            return candidate
    fail(f"could not determine repo root from {start}")


def resolve_existing_ancestor_path(start: Path, rel_path: str) -> Path:
    for candidate in [start, *start.parents]:
        probe = candidate / rel_path
        if probe.exists():
            return probe
    fail(f"required tracked/retained artifact missing across ancestor roots: {rel_path}")


def ensure_sor_links(repo_root: Path) -> None:
    sor_path = resolve_existing_ancestor_path(
        repo_root,
        ".adl/v0.91.4/tasks/issue-3354__v0-91-4-wp-06-evidence-convergence-review-synthesis-and-signed-trace-proof/sor.md",
    )
    sor_text = sor_path.read_text()
    required_refs = [
        "docs/milestones/v0.91.4/review/evidence/csdlc/C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md",
        "docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json",
        "docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_review_synthesis.json",
        "docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_signed.adl.yaml",
    ]
    missing = [ref for ref in required_refs if ref not in sor_text]
    if missing:
        fail(f"WP-06 SOR missing required evidence linkage refs: {missing}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0914_csdlc_evidence_bundle.py <packet_dir>")
    packet_dir = Path(sys.argv[1]).resolve()
    repo_root = find_repo_root(packet_dir)

    required = [
        packet_dir / "README.md",
        packet_dir / "C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md",
        packet_dir / "ct_demo_001_transition_evidence_bundle.json",
        packet_dir / "ct_demo_001_review_synthesis.json",
        packet_dir / "fixtures" / "minimal_transition_trace_unsigned.adl.yaml",
        packet_dir / "fixtures" / "minimal_transition_trace_signed.adl.yaml",
        packet_dir / "fixtures" / "minimal_transition_trace_public_key.b64",
    ]
    for path in required:
        if not path.exists():
            fail(f"missing required artifact: {path}")

    bundle = json.loads((packet_dir / "ct_demo_001_transition_evidence_bundle.json").read_text())
    if bundle.get("bundle_scope") != "csdlc_transition_proof":
        fail("bundle_scope must be csdlc_transition_proof")
    if bundle.get("issue_number") != 3354 or bundle.get("depends_on_issue_number") != 3353:
        fail("issue dependency truth drift in evidence bundle")
    if bundle.get("depends_on_pr_number") != 3387:
        fail("expected evidence bundle to reference PR #3387")

    evidence_inputs = bundle.get("evidence_inputs", [])
    if len(evidence_inputs) < 4:
        fail("expected at least four tracked evidence inputs")
    for entry in evidence_inputs:
        path = entry.get("path", "")
        ensure_repo_relative(path)
        target = repo_root / path
        if not target.exists():
            fail(f"tracked evidence input missing: {path}")
        expected = entry.get("sha256", "")
        if len(expected) != 64:
            fail(f"invalid sha256 length for {path}")
        actual = sha256_text(target)
        if actual != expected:
            fail(f"digest mismatch for {path}")

    review_synthesis_path = bundle.get("review_synthesis_path", "")
    ensure_repo_relative(review_synthesis_path)
    if not (repo_root / review_synthesis_path).exists():
        fail("review synthesis path missing")

    synthesis = json.loads((packet_dir / "ct_demo_001_review_synthesis.json").read_text())
    if synthesis.get("source_issue_number") != bundle.get("issue_number"):
        fail("review synthesis source issue must align with bundle issue_number")
    if not isinstance(synthesis.get("source_pr_number"), int) or synthesis.get("source_pr_number", 0) <= 0:
        fail("review synthesis source_pr_number must be a positive integer")
    findings = synthesis.get("findings", [])
    if len(findings) != 4:
        fail("expected four preserved review findings")
    if any(item.get("disposition") != "fixed" for item in findings):
        fail("all bounded review findings must be fixed in synthesis")
    if not synthesis.get("review_truth", {}).get("residual_risks_preserved", False):
        fail("review synthesis must preserve residual risk truth")

    signed_trace = bundle.get("signed_trace", {})
    for key in ["unsigned_path", "signed_path", "public_key_path"]:
        value = signed_trace.get(key, "")
        ensure_repo_relative(value)
        if not (repo_root / value).exists():
            fail(f"signed trace artifact missing: {value}")
    if signed_trace.get("verification_mode") != "explicit_key":
        fail("signed trace verification_mode must be explicit_key")
    signed_path = repo_root / signed_trace["signed_path"]
    public_key_path = repo_root / signed_trace["public_key_path"]
    embedded_key = extract_signed_public_key(signed_path)
    tracked_public_key = public_key_path.read_text().strip()
    if embedded_key != tracked_public_key:
        fail("tracked public key does not match embedded signed trace public_key_b64")

    ensure_sor_links(repo_root)

    for path in required:
        text = path.read_text()
        if "/Users/" in text:
            fail(f"absolute path leakage in {path}")

    print(f"PASS: C-SDLC evidence bundle valid at {packet_dir.relative_to(repo_root)}")


if __name__ == "__main__":
    main()
