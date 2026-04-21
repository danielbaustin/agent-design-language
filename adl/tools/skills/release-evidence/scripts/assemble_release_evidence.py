#!/usr/bin/env python3
"""Assemble a bounded milestone release-evidence packet."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


EVIDENCE_FAMILIES = {
    "issue_pr_evidence": (
        "issue",
        "issues",
        "pr",
        "pull request",
        "wbs",
        "wave",
        "work package",
    ),
    "demo_proof_evidence": ("demo", "proof", "feature proof", "coverage"),
    "review_evidence": ("review", "gap", "internal", "external"),
    "remediation_evidence": ("remediation", "finding", "follow-up", "follow up", "closeout"),
    "validation_evidence": ("validation", "checklist", "test", "ci", "command"),
}

BLOCKER_RE = re.compile(r"\b(blocked|release-blocking|p0|p1|must fix before release)\b", re.I)
OPEN_CHECKBOX_RE = re.compile(r"^\s*[-*]\s+\[\s\]", re.M)


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="utf-8", errors="replace")


def collect_documents(root: Path) -> list[dict[str, str]]:
    docs: list[dict[str, str]] = []
    for path in sorted(root.rglob("*")):
        if not path.is_file() or path.suffix.lower() not in {".md", ".txt", ".yaml", ".yml", ".json"}:
            continue
        rel = path.relative_to(root).as_posix()
        docs.append({"path": rel, "text": read_text(path)})
    return docs


def summarize_matches(docs: list[dict[str, str]]) -> dict[str, dict[str, Any]]:
    families: dict[str, dict[str, Any]] = {}
    for family, terms in EVIDENCE_FAMILIES.items():
        paths: list[str] = []
        snippets: list[str] = []
        for doc in docs:
            haystack = f"{doc['path']}\n{doc['text']}".lower()
            if any(term in haystack for term in terms):
                paths.append(doc["path"])
                snippets.extend(extract_snippets(doc["text"], terms, limit=2))
        families[family] = {
            "status": "present" if paths else "missing",
            "paths": paths[:20],
            "signals": snippets[:6],
        }
    return families


def extract_snippets(text: str, terms: tuple[str, ...], limit: int) -> list[str]:
    snippets: list[str] = []
    for line in text.splitlines():
        compact = " ".join(line.strip().split())
        if not compact:
            continue
        lower = compact.lower()
        if any(term in lower for term in terms):
            snippets.append(compact[:180])
        if len(snippets) >= limit:
            break
    return snippets


def classify(docs: list[dict[str, str]], families: dict[str, dict[str, Any]]) -> tuple[str, list[str]]:
    if not docs:
        return "not_run", ["No readable milestone evidence documents were found."]

    combined = "\n".join(doc["text"] for doc in docs)
    missing = [name for name, data in families.items() if data["status"] == "missing"]
    reasons: list[str] = []

    if BLOCKER_RE.search(combined):
        reasons.append("Explicit blocker or high-priority finding marker found in evidence.")
        return "blocked", reasons

    if "release_readiness" in combined.lower() and "missing" in combined.lower():
        reasons.append("Release-readiness evidence appears to identify missing proof.")
        return "blocked", reasons

    if missing:
        reasons.append("Missing evidence families: " + ", ".join(missing) + ".")

    if OPEN_CHECKBOX_RE.search(combined):
        reasons.append("Open checklist items are visible in evidence.")

    if reasons:
        return "partial", reasons

    return "ready", ["All required evidence families are present and no open checklist or blocker markers were found."]


def markdown_report(report: dict[str, Any]) -> str:
    lines = [
        "# Release Evidence Summary",
        "",
        f"- Milestone: `{report['milestone']}`",
        f"- Run id: `{report['run_id']}`",
        f"- Status: `{report['status']}`",
        f"- Summary: {report['summary']}",
        "",
        "## Evidence Families",
        "",
    ]
    for family, data in report["evidence_families"].items():
        lines.append(f"### {family}")
        lines.append("")
        lines.append(f"- Status: `{data['status']}`")
        paths = data.get("paths") or []
        if paths:
            lines.append("- Paths: " + ", ".join(f"`{path}`" for path in paths[:8]))
        else:
            lines.append("- Paths: none recorded")
        signals = data.get("signals") or []
        if signals:
            lines.append("- Signals:")
            for signal in signals[:4]:
                lines.append(f"  - {signal}")
        lines.append("")

    lines.extend(
        [
            "## Blocking Or Partial Evidence",
            "",
        ]
    )
    for item in report["blocking_or_partial_evidence"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Non-Claims", ""])
    for item in report["non_claims"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Residual Risks", ""])
    for item in report["residual_risks"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Validation Commands", ""])
    for command in report["validation_commands"]:
        lines.append(f"- `{command}`")

    lines.extend(["", "## Safety Flags", ""])
    for key, value in report["safety_flags"].items():
        lines.append(f"- {key}: {str(value).lower()}")
    lines.append("")
    return "\n".join(lines)


def build_report(args: argparse.Namespace) -> dict[str, Any]:
    milestone_root = Path(args.milestone_root)
    if not milestone_root.exists() or not milestone_root.is_dir():
        families = {
            name: {"status": "missing", "paths": [], "signals": []}
            for name in EVIDENCE_FAMILIES
        }
        status = "not_run"
        reasons = ["Milestone evidence root is missing or unreadable."]
        docs: list[dict[str, str]] = []
    else:
        docs = collect_documents(milestone_root)
        families = summarize_matches(docs)
        status, reasons = classify(docs, families)

    residual_risks = list(reasons)
    if status == "ready":
        residual_risks = [
            "This packet is evidence-ready only; release approval still requires the normal release ceremony.",
        ]

    return {
        "schema": "adl.release_evidence_report.v1",
        "run_id": args.run_id,
        "milestone": args.milestone,
        "status": status,
        "summary": (
            "Release evidence assembled for review."
            if status != "not_run"
            else "Release evidence was not assembled because the milestone evidence root was unavailable or empty."
        ),
        "evidence_families": families,
        "blocking_or_partial_evidence": reasons,
        "non_claims": [
            "This report does not approve the release.",
            "This report does not publish release notes.",
            "This report does not create tags, merge PRs, or close issues.",
            "This report does not prove absent evidence is failed implementation.",
        ],
        "residual_risks": residual_risks,
        "validation_commands": [
            "python3 adl/tools/skills/release-evidence/scripts/assemble_release_evidence.py --milestone <version> --milestone-root docs/milestones/<version> --out <artifact-root> --run-id <run-id>",
        ],
        "safety_flags": {
            "release_approved": False,
            "published_release_notes": False,
            "created_tags": False,
            "merged_prs": False,
            "closed_issues": False,
            "mutated_repository": False,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--milestone", required=True)
    parser.add_argument("--milestone-root", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--run-id", default="release-evidence")
    args = parser.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)

    report = build_report(args)
    (out / "release_evidence_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (out / "release_evidence_report.md").write_text(
        markdown_report(report),
        encoding="utf-8",
    )
    print(f"WROTE {out / 'release_evidence_report.json'}")
    print(f"WROTE {out / 'release_evidence_report.md'}")
    print(f"STATUS {report['status']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

