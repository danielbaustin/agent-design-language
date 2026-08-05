#!/usr/bin/env python3
"""Validate v0.91.8 demo matrix and feature-proof coverage truth.

The check is intentionally structural and deterministic: it proves that each
row names owners, uses the bounded status vocabulary, records evidence or an
explicit disposition, and preserves the required non-claim boundaries.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


def repo_root() -> Path:
    for candidate in Path(__file__).resolve().parents:
        if (candidate / "docs/milestones/v0.91.8").is_dir() and (candidate / ".git").exists():
            return candidate
    fail("could not locate repository root")


ROOT = repo_root()
DEMO = ROOT / "docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md"
FEATURES = ROOT / "docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md"
LEDGER = ROOT / "docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/RECONCILIATION_LEDGER_v1.md"
CONVERGENCE = ".csdlc/evidence/5354/convergence-proof.v1.json"

ALLOWED_STATUSES = {
    "proven",
    "retained_proof",
    "closed_planning",
    "open_gate",
    "deferred",
    "non_claim",
}

REQUIRED_SURFACES = {
    "ADL v2": "adl v2",
    "Runtime v3": "runtime v3",
    "C-SDLC v2": "c-sdlc v2",
    "Unity": "unity",
    "Observatory": "observatory",
    "Distributed workcell": "distributed",
    "Podcast Studio": "podcast",
    "v0.92 handoff": "v0.92",
}

FORBIDDEN_UNBOUNDED_CLAIMS = [
    "whole-release proven",
    "release ready",
    "public launch is live",
    "directory approval is complete",
    "runtime v2 proven",
    "player-build readiness proven",
]


def fail(message: str) -> None:
    raise SystemExit(f"v0.91.8 demo matrix validation failed: {message}")


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required file {path.relative_to(ROOT)}")
    text = path.read_text(encoding="utf-8")
    if not text.strip():
        fail(f"empty required file {path.relative_to(ROOT)}")
    return text


def parse_table_rows(text: str, required_header: str, path: Path) -> list[list[str]]:
    lines = text.splitlines()
    try:
        start = next(i for i, line in enumerate(lines) if line.strip() == required_header)
    except StopIteration:
        fail(f"{path.relative_to(ROOT)} missing table header {required_header!r}")
    rows: list[list[str]] = []
    for line in lines[start + 2 :]:
        if not line.startswith("|"):
            break
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) != 5:
            fail(f"{path.relative_to(ROOT)} row has {len(cells)} cells, expected 5: {line}")
        rows.append(cells)
    if not rows:
        fail(f"{path.relative_to(ROOT)} has no data rows")
    return rows


def local_paths_from_cell(cell: str) -> list[str]:
    paths: list[str] = []
    for raw in re.findall(r"`([^`]+)`", cell):
        if raw.startswith(("http://", "https://", "mailto:")):
            continue
        if raw.startswith("#"):
            continue
        if raw.endswith("/") or "*" in raw:
            continue
        paths.append(raw)
    return paths


def validate_rows(path: Path, rows: list[list[str]]) -> None:
    for cells in rows:
        subject, owners, status_cell, evidence, boundary = cells
        if not subject:
            fail(f"{path.relative_to(ROOT)} has a row with empty subject")
        if not re.search(r"#\d+", owners):
            fail(f"{path.relative_to(ROOT)} row {subject!r} has no issue owner")
        status_match = re.fullmatch(r"`([^`]+)`", status_cell)
        if not status_match:
            fail(f"{path.relative_to(ROOT)} row {subject!r} status must be backticked")
        status = status_match.group(1)
        if status not in ALLOWED_STATUSES:
            fail(f"{path.relative_to(ROOT)} row {subject!r} has unsupported status {status!r}")
        if not evidence or not boundary:
            fail(f"{path.relative_to(ROOT)} row {subject!r} needs evidence/disposition and boundary")
        if status in {"proven", "retained_proof", "closed_planning"}:
            if not (local_paths_from_cell(evidence) or "GitHub" in evidence or "owner-proved" in evidence):
                fail(f"{path.relative_to(ROOT)} row {subject!r} lacks exact evidence path or source")
        if status in {"open_gate", "deferred", "non_claim"}:
            if not any(word in evidence.lower() + " " + boundary.lower() for word in ["open", "defer", "non-claim", "not "]):
                fail(f"{path.relative_to(ROOT)} row {subject!r} lacks explicit disposition language")
        for rel in local_paths_from_cell(evidence):
            target = ROOT / rel
            if not target.exists():
                fail(f"{path.relative_to(ROOT)} row {subject!r} references missing path {rel}")


def validate_required_surfaces(text: str, path: Path) -> None:
    lowered = text.lower()
    for label, needle in REQUIRED_SURFACES.items():
        if needle not in lowered:
            fail(f"{path.relative_to(ROOT)} missing required surface {label}")


def validate_non_claims(text: str, path: Path) -> None:
    lowered = text.lower()
    for phrase in FORBIDDEN_UNBOUNDED_CLAIMS:
        if phrase in lowered:
            fail(f"{path.relative_to(ROOT)} contains forbidden unbounded claim: {phrase}")
    required_boundaries = [
        "whole-release completion",
        "runtime v2",
        "player-build",
        "public hosting",
        "directory approval",
        "weekly cadence",
        "v0.92 activation",
    ]
    for boundary in required_boundaries:
        if boundary not in lowered:
            fail(f"{path.relative_to(ROOT)} missing non-claim boundary {boundary!r}")


def main() -> None:
    demo_text = read(DEMO)
    feature_text = read(FEATURES)
    ledger_text = read(LEDGER)
    if not (ROOT / CONVERGENCE).is_file():
        fail(f"missing convergence proof {CONVERGENCE}")
    if CONVERGENCE not in demo_text or CONVERGENCE not in feature_text or CONVERGENCE not in ledger_text:
        fail("both matrices and ledger must cite #5354 convergence proof")

    demo_rows = parse_table_rows(
        demo_text,
        "| Surface | Owners | Status | Evidence / disposition | Claim boundary |",
        DEMO,
    )
    feature_rows = parse_table_rows(
        feature_text,
        "| Feature area | Owners | Status | Evidence / disposition | Claim boundary |",
        FEATURES,
    )

    validate_rows(DEMO, demo_rows)
    validate_rows(FEATURES, feature_rows)
    validate_required_surfaces(demo_text, DEMO)
    validate_required_surfaces(feature_text, FEATURES)
    validate_non_claims(demo_text, DEMO)
    validate_non_claims(feature_text, FEATURES)

    if "#5717/#5720" not in demo_text or "#5717/#5720" not in feature_text:
        fail("podcast rows must consume current #5717/#5720 proof")
    for issue in ["#5362", "#5355", "#5359", "#5348", "#4760", "#5007"]:
        if issue not in feature_text:
            fail(f"release-tail open gate issue {issue} is missing from feature coverage")
    release_tail_row = next(
        (row for row in feature_rows if "WP-21 through WP-23" in row[0]),
        None,
    )
    if release_tail_row is None or release_tail_row[2] != "`open_gate`":
        fail("release-tail row must remain an open_gate")
    if "#5733 does not rerun" not in ledger_text:
        fail("ledger must preserve no-rerun boundary")

    print("v0918_demo_matrix: PASS")


if __name__ == "__main__":
    main()
