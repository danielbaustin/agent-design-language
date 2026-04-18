#!/usr/bin/env python3
"""Plan human-approved issue candidates from CodeBuddy review findings."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import re
from pathlib import Path

SCHEMA = "codebuddy.finding_to_issue_planner.v1"
FINDING_HEADING_RE = re.compile(
    r"^#{2,4}[ \t]+(?:Finding[ \t]+)?(?P<id>[A-Za-z0-9_.:-]+)?[ \t]*:?[ \t]*(?:\[(?P<severity>P[0-3])\])?[ \t]*(?P<title>[^\n#].*?)\s*$",
    re.MULTILINE,
)
SEVERITY_ORDER = {"P0": 0, "P1": 1, "P2": 2, "P3": 3}
DEFAULT_NON_GOALS = [
    "Do not broaden into unrelated remediation.",
    "Do not claim the issue is fixed until validation evidence exists.",
    "Do not create tracker items without explicit operator approval.",
]


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def source_files(source: Path) -> list[Path]:
    if source.is_file():
        return [source]
    if not source.is_dir():
        return []
    preferred = [
        source / "final_report.md",
        source / "synthesis.md",
        source / "specialist_reviews" / "code.md",
        source / "specialist_reviews" / "security.md",
        source / "specialist_reviews" / "tests.md",
        source / "specialist_reviews" / "docs.md",
        source / "specialist_reviews" / "architecture.md",
        source / "specialist_reviews" / "dependencies.md",
    ]
    files = [path for path in preferred if path.is_file()]
    if files:
        return files
    return sorted(source.rglob("*.md"))[:30]


def line_value(block: str, key: str) -> str:
    pattern = re.compile(rf"^\s*-\s*{re.escape(key)}:\s*(.+?)\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(block)
    return match.group(1).strip() if match else ""


def clean_title(title: str) -> str:
    title = re.sub(r"^\[[Pp][0-3]\]\s*", "", title).strip()
    title = re.sub(r"\s+", " ", title)
    return title.rstrip("# ").strip()


def severity_from_block(heading_severity: str | None, block: str) -> str:
    if heading_severity:
        return heading_severity.upper()
    value = line_value(block, "Severity")
    match = re.search(r"\bP[0-3]\b", value.upper())
    return match.group(0) if match else "P2"


def confidence_from_block(block: str) -> str:
    value = line_value(block, "Confidence")
    if value:
        return value
    lowered = block.lower()
    if "confidence: high" in lowered:
        return "high"
    if "confidence: low" in lowered:
        return "low"
    return "medium"


def split_findings(path: Path) -> list[dict[str, object]]:
    text = read_text(path)
    matches = [
        match
        for match in FINDING_HEADING_RE.finditer(text)
        if re.match(r"^#{2,4}\s+Finding\s+", match.group(0), re.IGNORECASE)
    ]
    findings: list[dict[str, object]] = []
    for index, match in enumerate(matches):
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        block = text[start:end].strip()
        raw_title = clean_title(match.group("title"))
        severity = severity_from_block(match.group("severity"), block)
        finding_id = match.group("id") or f"{path.stem}-{index + 1}"
        role = line_value(block, "Role") or line_value(block, "Source role") or infer_role(path, block)
        evidence = line_value(block, "Evidence") or line_value(block, "Validation or proof gap")
        action = line_value(block, "Recommended action") or line_value(block, "Recommended follow-up owner")
        impact = line_value(block, "User/customer impact") or line_value(block, "Impact")
        affected = line_value(block, "Affected path or artifact") or line_value(block, "Affected path")
        findings.append(
            {
                "finding_id": str(finding_id).strip(": "),
                "title": raw_title,
                "severity": severity,
                "confidence": confidence_from_block(block),
                "role": role,
                "source_file": path.name,
                "affected_path": affected,
                "evidence": evidence,
                "recommended_action": action,
                "impact": impact,
                "body": block,
            }
        )
    return findings


def infer_role(path: Path, block: str) -> str:
    lowered = f"{path.as_posix()} {block}".lower()
    for role in ("security", "tests", "docs", "architecture", "dependencies", "diagram", "code"):
        if role in lowered:
            return role
    return "review"


def normalize_group_key(finding: dict[str, object]) -> str:
    title = str(finding["title"]).lower()
    title = re.sub(r"[^a-z0-9]+", " ", title).strip()
    affected = str(finding.get("affected_path", "")).lower().strip()
    return f"{title}|{affected}"


def highest_severity(findings: list[dict[str, object]]) -> str:
    return min((str(item["severity"]) for item in findings), key=lambda item: SEVERITY_ORDER.get(item, 9))


def candidate_id(group_key: str) -> str:
    digest = hashlib.sha1(group_key.encode("utf-8")).hexdigest()[:8]
    return f"issue-candidate-{digest}"


def candidate_from_group(group_key: str, findings: list[dict[str, object]], tracker: str) -> dict[str, object]:
    primary = findings[0]
    severity = highest_severity(findings)
    roles = sorted({str(item.get("role") or "review") for item in findings})
    affected_paths = sorted({str(item.get("affected_path") or "") for item in findings if item.get("affected_path")})
    evidence_items = [str(item.get("evidence") or "").strip() for item in findings if item.get("evidence")]
    action = str(primary.get("recommended_action") or "Address the review finding with a bounded fix.")
    problem = str(primary.get("impact") or primary.get("title") or "Review finding requires follow-up.")
    title = f"[{severity}] {primary['title']}"
    return {
        "candidate_id": candidate_id(group_key),
        "title": title,
        "severity": severity,
        "source_finding_ids": [str(item["finding_id"]) for item in findings],
        "source_roles": roles,
        "confidence": str(primary.get("confidence") or "medium"),
        "affected_paths": affected_paths,
        "evidence": evidence_items or ["Evidence was not extracted cleanly; human review required before tracker creation."],
        "problem": problem,
        "recommended_action": action,
        "acceptance_criteria": [
            "The issue states the source finding evidence and affected surface.",
            "The fix or follow-up remains bounded to the finding scope.",
            "Validation evidence is recorded before closeout.",
        ],
        "validation_plan": [
            "Run the smallest targeted test, lint, review, or demo command that proves the finding is addressed.",
            "Record any commands not run and why.",
        ],
        "non_goals": DEFAULT_NON_GOALS,
        "dependencies": dependency_notes(findings),
        "approval_status": "candidate_only",
        "tracker": tracker,
    }


def dependency_notes(findings: list[dict[str, object]]) -> list[str]:
    notes: list[str] = []
    for item in findings:
        related = line_value(str(item.get("body", "")), "Related findings")
        if related:
            notes.append(f"Related findings: {related}")
    return notes or ["No dependency links extracted."]


def severity_allowed(severity: str, floor: str) -> bool:
    return SEVERITY_ORDER.get(severity, 9) <= SEVERITY_ORDER.get(floor, 3)


def build_plan(source: Path, tracker: str, severity_floor: str) -> dict[str, object]:
    findings: list[dict[str, object]] = []
    for path in source_files(source):
        findings.extend(split_findings(path))

    deferred: list[dict[str, object]] = []
    groups: dict[str, list[dict[str, object]]] = {}
    for finding in findings:
        if not severity_allowed(str(finding["severity"]), severity_floor):
            deferred.append({**finding, "deferred_reason": f"below severity floor {severity_floor}"})
            continue
        if not str(finding.get("evidence") or "").strip():
            deferred.append({**finding, "deferred_reason": "missing extracted evidence"})
            continue
        groups.setdefault(normalize_group_key(finding), []).append(finding)

    candidates = [candidate_from_group(key, items, tracker) for key, items in sorted(groups.items())]
    status = "not_run"
    if candidates and deferred:
        status = "partial"
    elif candidates:
        status = "pass"
    elif deferred:
        status = "partial"

    return {
        "schema": SCHEMA,
        "source": source.name,
        "created_at": now_utc(),
        "tracker": tracker,
        "status": status,
        "candidate_count": len(candidates),
        "deferred_count": len(deferred),
        "candidates": candidates,
        "deferred_findings": deferred,
        "approval_boundary": {
            "tracker_creation_allowed": False,
            "approval_required": True,
            "mutation_performed": False,
        },
    }


def write_markdown(path: Path, plan: dict[str, object]) -> None:
    candidates = plan["candidates"]
    deferred = plan["deferred_findings"]
    candidate_lines = []
    for candidate in candidates:
        evidence = "\n".join(f"  - {item}" for item in candidate["evidence"])
        criteria = "\n".join(f"  - {item}" for item in candidate["acceptance_criteria"])
        validation = "\n".join(f"  - {item}" for item in candidate["validation_plan"])
        candidate_lines.append(
            f"""### {candidate['candidate_id']}: {candidate['title']}

- Severity: {candidate['severity']}
- Source findings: {', '.join(candidate['source_finding_ids'])}
- Source roles: {', '.join(candidate['source_roles'])}
- Confidence: {candidate['confidence']}
- Affected paths: {', '.join(candidate['affected_paths']) or 'not extracted'}
- Approval status: {candidate['approval_status']}
- Problem: {candidate['problem']}
- Recommended action: {candidate['recommended_action']}
- Evidence:
{evidence}
- Acceptance criteria:
{criteria}
- Validation plan:
{validation}
- Non-goals:
  - {'; '.join(candidate['non_goals'])}
- Dependencies: {'; '.join(candidate['dependencies'])}
"""
        )
    deferred_lines = []
    for finding in deferred:
        deferred_lines.append(
            f"- {finding.get('finding_id')}: [{finding.get('severity')}] {finding.get('title')} ({finding.get('deferred_reason')})"
        )

    content = f"""# Finding To Issue Plan

## Metadata

- Skill: finding-to-issue-planner
- Source: {plan['source']}
- Tracker: {plan['tracker']}
- Status: {plan['status']}
- Date: {plan['created_at']}

## Scope

- Candidate count: {plan['candidate_count']}
- Deferred finding count: {plan['deferred_count']}
- Tracker creation allowed: false

## Issue Candidates

{chr(10).join(candidate_lines) if candidate_lines else '- No issue candidates generated.'}

## Deferred Findings

{chr(10).join(deferred_lines) if deferred_lines else '- None.'}

## Approval Boundary

- Human approval is required before tracker mutation.
- No issues, PRs, tests, or remediation branches were created by this skill.

## Validation Notes

- This artifact is a planning surface. It does not prove remediation.
- Each approved candidate needs its own execution validation.

## Residual Risk

- Markdown extraction may miss custom finding formats; manually review deferred findings and grouped candidates before issue creation.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("finding_source", help="Review markdown file or CodeBuddy packet root")
    parser.add_argument("--out", default=None, help="Issue-planning artifact root")
    parser.add_argument("--tracker", default="github", help="Target tracker name for candidate wording")
    parser.add_argument("--severity-floor", default="P3", choices=["P0", "P1", "P2", "P3"])
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    source = Path(args.finding_source)
    if not source.exists():
        raise SystemExit(f"finding source does not exist: {source}")

    out_root = Path(args.out) if args.out else source.parent / "issue-planning"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    plan = build_plan(source, args.tracker, args.severity_floor)
    write_json(out_root / "issue_candidates.json", plan)
    write_markdown(out_root / "issue_candidates.md", plan)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
