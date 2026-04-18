#!/usr/bin/env python3
"""Create a bounded gap-analysis report from explicit baseline/evidence files."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from pathlib import Path
from typing import Any

SCHEMA = "adl.gap_analysis_report.v1"
SEVERITY_ORDER = {"P0": 0, "P1": 1, "P2": 2, "P3": 3}
GAP_TYPES = {
    "missing_evidence",
    "implementation_gap",
    "docs_drift",
    "test_gap",
    "closeout_drift",
    "scope_ambiguity",
}


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rel(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


def extract_bullets_after(text: str, heading: str) -> list[str]:
    pattern = re.compile(rf"^##+\s+{re.escape(heading)}\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    if not match:
        return []
    next_heading = re.search(r"^##+\s+", text[match.end() :], re.MULTILINE)
    end = match.end() + next_heading.start() if next_heading else len(text)
    section = text[match.end() : end]
    return [line.strip()[2:].strip() for line in section.splitlines() if line.strip().startswith("- ")][:80]


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def normalize(text: str) -> str:
    return re.sub(r"[^a-z0-9]+", " ", text.lower()).strip()


def keyword_hit(expected: str, observed_text: str) -> bool:
    words = [word for word in normalize(expected).split() if len(word) > 4]
    if not words:
        return False
    observed = normalize(observed_text)
    hits = sum(1 for word in set(words) if word in observed)
    return hits >= max(1, min(3, len(set(words)) // 2))


def line_value(text: str, key: str) -> str:
    pattern = re.compile(rf"^\s*-\s*{re.escape(key)}:\s*(.+?)\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    return match.group(1).strip() if match else ""


def metadata(root: Path) -> dict[str, str]:
    manifest = load_json(root / "gap_manifest.json")
    if not isinstance(manifest, dict):
        manifest = {}
    return {
        "run_id": str(manifest.get("run_id") or root.name),
        "scope": str(manifest.get("scope") or line_value(read_text(root / "expected_baseline.md"), "Scope") or root.name),
        "mode": str(manifest.get("mode") or "compare_issue_to_implementation"),
    }


def expected_items(text: str) -> list[dict[str, str]]:
    bullets = (
        extract_bullets_after(text, "Expected")
        or extract_bullets_after(text, "Acceptance Criteria")
        or extract_bullets_after(text, "Required Outcome")
        or extract_bullets_after(text, "Baseline")
    )
    items = []
    for index, bullet in enumerate(bullets, start=1):
        gap_type = infer_gap_type(bullet)
        items.append(
            {
                "id": f"E-{index:03d}",
                "expected": bullet,
                "gap_type": gap_type,
                "severity": infer_severity(bullet, gap_type),
            }
        )
    return items


def infer_gap_type(text: str) -> str:
    lowered = text.lower()
    if any(word in lowered for word in ["test", "validation", "coverage", "proof"]):
        return "test_gap"
    if any(word in lowered for word in ["doc", "readme", "changelog", "guide"]):
        return "docs_drift"
    if any(word in lowered for word in ["closeout", "sor", "sip", "stp", "pr body", "issue state"]):
        return "closeout_drift"
    if any(word in lowered for word in ["evidence", "artifact", "report"]):
        return "missing_evidence"
    return "implementation_gap"


def infer_severity(text: str, gap_type: str) -> str:
    lowered = text.lower()
    if any(word in lowered for word in ["security", "privacy", "release-blocking", "block release", "customer-facing"]):
        return "P1"
    if gap_type in {"closeout_drift", "test_gap"}:
        return "P2"
    return "P3"


def observed_text(root: Path, explicit_observed: Path | None) -> tuple[str, list[str]]:
    paths: list[Path] = []
    if explicit_observed and explicit_observed.is_file():
        paths.append(explicit_observed)
    for candidate in [
        root / "observed_evidence.md",
        root / "implementation_evidence.md",
        root / "validation_evidence.md",
        root / "closeout_evidence.md",
    ]:
        if candidate.is_file() and candidate not in paths:
            paths.append(candidate)
    text_parts = []
    for path in paths:
        text_parts.append(f"\n<!-- {rel(root, path)} -->\n{read_text(path)}")
    return "\n".join(text_parts), [rel(root, path) for path in paths]


def explicit_gaps(root: Path) -> list[dict[str, str]]:
    text = read_text(root / "known_gaps.md")
    if not text:
        return []
    bullets = extract_bullets_after(text, "Gaps") or [line.strip()[2:] for line in text.splitlines() if line.strip().startswith("- ")]
    findings = []
    for index, bullet in enumerate(bullets, start=1):
        gap_type = infer_gap_type(bullet)
        findings.append(
            {
                "id": f"G-{index:03d}",
                "gap_type": gap_type,
                "severity": infer_severity(bullet, gap_type),
                "title": bullet[:96],
                "expected": "Recorded baseline expectation or review finding.",
                "observed": bullet,
                "evidence": "known_gaps.md",
                "uncertainty": "low",
                "recommended_follow_up": recommended_follow_up(gap_type),
                "source_artifact": "known_gaps.md",
            }
        )
    return findings


def analyze(root: Path, baseline: Path | None, observed: Path | None) -> dict[str, object]:
    baseline_path = baseline if baseline else root / "expected_baseline.md"
    baseline_text = read_text(baseline_path)
    meta = metadata(root)
    observed_blob, observed_sources = observed_text(root, observed)
    if not baseline_text:
        return {
            "schema": SCHEMA,
            "created_at": now_utc(),
            "run_id": meta["run_id"],
            "status": "not_run",
            "scope": meta["scope"],
            "expected_baseline": {"path": rel(root, baseline_path), "items": []},
            "observed_evidence": {"sources": observed_sources},
            "findings": [],
            "missing_evidence": ["Expected baseline missing or unreadable."],
            "uncertainty": ["No baseline was available; no gap claims were made."],
            "recommended_follow_up": ["Provide an explicit expected baseline."],
            "stop_boundary": stop_boundary(),
        }
    items = expected_items(baseline_text)
    findings = explicit_gaps(root)
    missing: list[str] = []
    uncertainty: list[str] = []
    for item in items:
        if keyword_hit(item["expected"], observed_blob):
            continue
        finding = {
            "id": item["id"],
            "gap_type": item["gap_type"],
            "severity": item["severity"],
            "title": item["expected"][:96],
            "expected": item["expected"],
            "observed": "No matching observed evidence was found.",
            "evidence": "Observed evidence did not contain enough matching terms for this expectation.",
            "uncertainty": "medium" if observed_blob else "high",
            "recommended_follow_up": recommended_follow_up(item["gap_type"]),
            "source_artifact": rel(root, baseline_path),
        }
        findings.append(finding)
        missing.append(item["expected"])
        if not observed_blob:
            uncertainty.append(f"No observed evidence file was available for {item['id']}.")
    findings = sorted(findings, key=lambda item: (SEVERITY_ORDER.get(str(item["severity"]), 9), str(item["id"])))
    status = "pass" if not findings and items else "partial"
    if any(item["severity"] in {"P0", "P1"} for item in findings):
        status = "fail"
    return {
        "schema": SCHEMA,
        "created_at": now_utc(),
        "run_id": meta["run_id"],
        "status": status,
        "scope": meta["scope"],
        "expected_baseline": {"path": rel(root, baseline_path), "items": items},
        "observed_evidence": {"sources": observed_sources},
        "findings": findings,
        "missing_evidence": missing,
        "uncertainty": uncertainty or ["Uncertainty is limited to the supplied evidence bundle."],
        "recommended_follow_up": dedupe([str(item["recommended_follow_up"]) for item in findings]) or ["No follow-up required from this comparison."],
        "stop_boundary": stop_boundary(),
    }


def recommended_follow_up(gap_type: str) -> str:
    return {
        "missing_evidence": "Attach or produce the missing proof artifact before closeout.",
        "implementation_gap": "Open a bounded implementation follow-up after human review.",
        "docs_drift": "Route to documentation update or docs review after confirming intended truth.",
        "test_gap": "Route to review-to-test-planner or add focused validation evidence.",
        "closeout_drift": "Normalize the output card, PR body, or closeout note before closure.",
        "scope_ambiguity": "Clarify scope and rerun gap analysis with stronger evidence.",
    }.get(gap_type, "Review manually before follow-up.")


def dedupe(items: list[str]) -> list[str]:
    seen = set()
    out = []
    for item in items:
        if item not in seen:
            seen.add(item)
            out.append(item)
    return out


def stop_boundary() -> dict[str, bool]:
    return {
        "fixed_gaps": False,
        "created_issues": False,
        "created_prs": False,
        "approved_closeout": False,
        "approved_release": False,
        "mutated_repository": False,
    }


def bullet_lines(items: list[Any]) -> str:
    return "\n".join(f"- {item}" for item in items) if items else "- None."


def finding_lines(findings: list[dict[str, str]]) -> str:
    if not findings:
        return "- No gaps found for the supplied baseline and evidence."
    parts = []
    for finding in findings:
        parts.append(
            f"""### {finding['id']}: [{finding['severity']}] {finding['title']}

- Gap type: {finding['gap_type']}
- Expected: {finding['expected']}
- Observed: {finding['observed']}
- Evidence: {finding['evidence']}
- Uncertainty: {finding['uncertainty']}
- Recommended follow-up: {finding['recommended_follow_up']}
- Source artifact: {finding['source_artifact']}
"""
        )
    return "\n".join(parts)


def write_markdown(path: Path, report: dict[str, object]) -> None:
    boundary = report["stop_boundary"]
    content = f"""# Gap Analysis Report: {report['scope']}

## Gap Analysis Summary

- Status: {report['status']}
- Run id: {report['run_id']}
- Findings: {len(report['findings'])}

## Scope

- Scope: {report['scope']}

## Expected Baseline

- Baseline path: {report['expected_baseline']['path']}
- Expected items: {len(report['expected_baseline']['items'])}

## Observed Evidence

{bullet_lines(report['observed_evidence']['sources'])}

## Findings

{finding_lines(report['findings'])}

## Missing Evidence

{bullet_lines(report['missing_evidence'])}

## Uncertainty

{bullet_lines(report['uncertainty'])}

## Recommended Follow-up

{bullet_lines(report['recommended_follow_up'])}

## Stop Boundary

- Fixed gaps: {str(boundary['fixed_gaps']).lower()}.
- Created issues: {str(boundary['created_issues']).lower()}.
- Created PRs: {str(boundary['created_prs']).lower()}.
- Approved closeout: {str(boundary['approved_closeout']).lower()}.
- Approved release: {str(boundary['approved_release']).lower()}.
- Mutated repository: {str(boundary['mutated_repository']).lower()}.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("gap_root", help="Directory containing expected_baseline.md and observed evidence")
    parser.add_argument("--out", default=None, help="Gap analysis output root")
    parser.add_argument("--run-id", default=None, help="Run id override")
    parser.add_argument("--baseline", default=None, help="Explicit baseline path")
    parser.add_argument("--observed", default=None, help="Explicit observed evidence path")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path(args.gap_root)
    if not root.is_dir():
        raise SystemExit(f"gap root does not exist: {root}")
    baseline = Path(args.baseline) if args.baseline else None
    observed = Path(args.observed) if args.observed else None
    out_root = Path(args.out) if args.out else root / "gap-analysis"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)
    report = analyze(root, baseline, observed)
    if args.run_id:
        report["run_id"] = args.run_id
    write_json(out_root / "gap_analysis_report.json", report)
    write_markdown(out_root / "gap_analysis_report.md", report)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
