#!/usr/bin/env python3
"""Write a CodeBuddy product report from existing review packet artifacts."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from pathlib import Path
from typing import Any

SCHEMA = "codebuddy.product_report.v1"
FINDING_RE = re.compile(
    r"^#{2,4}[ \t]+(?:Finding[ \t]+)?(?P<id>[A-Za-z0-9_.:-]+)?[ \t]*:?[ \t]*(?:\[(?P<severity>P[0-3])\])?[ \t]*(?P<title>[^\n#].*?)\s*$",
    re.MULTILINE,
)
SEVERITY_ORDER = {"P0": 0, "P1": 1, "P2": 2, "P3": 3}


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def packet_relative(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


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
    for key in ("Severity", "Priority"):
        value = line_value(block, key)
        match = re.search(r"\bP[0-3]\b", value.upper())
        if match:
            return match.group(0)
    match = re.search(r"\bP[0-3]\b", block.upper())
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


def role_from_path(path: Path, block: str) -> str:
    value = line_value(block, "Role") or line_value(block, "Source role")
    if value:
        return value
    lowered = f"{path.as_posix()} {block}".lower()
    for role in ("security", "tests", "docs", "architecture", "dependencies", "diagram", "code"):
        if role in lowered:
            return role
    return "review"


def finding_sources(root: Path) -> list[Path]:
    preferred = [
        root / "final_report.md",
        root / "synthesis.md",
        root / "specialist_reviews" / "synthesis.md",
        root / "specialist_reviews" / "code.md",
        root / "specialist_reviews" / "security.md",
        root / "specialist_reviews" / "tests.md",
        root / "specialist_reviews" / "docs.md",
        root / "specialist_reviews" / "architecture.md",
        root / "specialist_reviews" / "dependencies.md",
    ]
    found = [path for path in preferred if path.is_file()]
    if found:
        return found
    return sorted(root.rglob("*.md"))[:40]


def split_findings(root: Path) -> list[dict[str, object]]:
    findings: list[dict[str, object]] = []
    for path in finding_sources(root):
        text = read_text(path)
        matches = [
            match
            for match in FINDING_RE.finditer(text)
            if re.match(r"^#{2,4}\s+Finding\s+", match.group(0), re.IGNORECASE)
        ]
        for index, match in enumerate(matches):
            start = match.end()
            end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
            block = text[start:end].strip()
            severity = severity_from_block(match.group("severity"), block)
            finding_id = str(match.group("id") or f"{path.stem}-{index + 1}").strip(": ")
            findings.append(
                {
                    "finding_id": finding_id,
                    "title": clean_title(match.group("title")),
                    "severity": severity,
                    "confidence": confidence_from_block(block),
                    "source_role": role_from_path(path, block),
                    "source_artifact": packet_relative(root, path),
                    "affected_path": line_value(block, "Affected path or artifact")
                    or line_value(block, "File")
                    or line_value(block, "Affected path"),
                    "evidence": line_value(block, "Evidence"),
                    "impact": line_value(block, "Impact") or line_value(block, "User/customer impact"),
                    "recommended_action": line_value(block, "Recommended action"),
                    "validation_gap": line_value(block, "Validation gap")
                    or line_value(block, "Validation or proof gap"),
                    "related_findings": line_value(block, "Related findings"),
                }
            )
    return sorted(findings, key=lambda item: (SEVERITY_ORDER.get(str(item["severity"]), 9), str(item["finding_id"])))


def scope_from_repo_scope(root: Path) -> dict[str, object]:
    text = read_text(root / "repo_scope.md")
    if not text:
        return {
            "included_paths": [],
            "excluded_paths": [],
            "non_reviewed_surfaces": ["repo_scope.md missing from packet"],
            "assumptions": [],
        }
    return {
        "included_paths": extract_bullets_after(text, "Included paths"),
        "excluded_paths": extract_bullets_after(text, "Excluded paths"),
        "non_reviewed_surfaces": extract_bullets_after(text, "Non-reviewed surfaces"),
        "assumptions": extract_bullets_after(text, "Assumptions"),
    }


def extract_bullets_after(text: str, heading: str) -> list[str]:
    pattern = re.compile(rf"^##+\s+{re.escape(heading)}\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    if not match:
        return []
    next_heading = re.search(r"^##+\s+", text[match.end() :], re.MULTILINE)
    end = match.end() + next_heading.start() if next_heading else len(text)
    section = text[match.end() : end]
    bullets = [line.strip()[2:].strip() for line in section.splitlines() if line.strip().startswith("- ")]
    return bullets[:20]


def read_manifest(root: Path, repo_name_override: str | None) -> dict[str, object]:
    manifest = load_json(root / "run_manifest.json")
    if not isinstance(manifest, dict):
        manifest = {}
    repo_name = repo_name_override or str(manifest.get("repo_name") or root.name)
    return {
        "run_id": str(manifest.get("run_id") or root.name),
        "repo_name": repo_name,
        "repo_ref": str(manifest.get("repo_ref") or "not recorded"),
        "review_mode": str(manifest.get("review_mode") or "not recorded"),
        "privacy_mode": str(manifest.get("privacy_mode") or "not recorded"),
        "publication_allowed": bool(manifest.get("publication_allowed", False)),
    }


def artifact_status(root: Path) -> dict[str, object]:
    return {
        "redaction_report": (root / "redaction_report.md").is_file(),
        "quality_evaluation": any(path.is_file() for path in [root / "quality_evaluation.md", root / "review_quality.md"]),
        "diagram_manifest": any(path.is_file() for path in [root / "diagrams" / "diagram_manifest.md", root / "diagram_manifest.md"]),
        "test_plan": any(path.is_file() for path in [root / "test_recommendations" / "test_gap_report.md", root / "review-to-test-plan" / "review_to_test_plan.md"]),
        "issue_plan": any(path.is_file() for path in [root / "issue-planning" / "issue_candidates.md"]),
    }


def diagram_links(root: Path) -> list[str]:
    links: list[str] = []
    for rel in ("diagrams/diagram_manifest.md", "diagram_manifest.md"):
        path = root / rel
        if path.is_file():
            links.append(rel)
    return links or ["No diagram manifest found in packet."]


def test_recommendations(root: Path) -> list[str]:
    paths = [
        root / "test_recommendations" / "test_gap_report.md",
        root / "review-to-test-plan" / "review_to_test_plan.md",
    ]
    found = [packet_relative(root, path) for path in paths if path.is_file()]
    return found or ["No test recommendation artifact found in packet."]


def remediation_sequence(findings: list[dict[str, object]]) -> list[str]:
    if not findings:
        return ["No finding-backed remediation sequence could be generated."]
    sequence = []
    for finding in findings[:5]:
        action = str(finding.get("recommended_action") or "Address the finding with a bounded fix.")
        sequence.append(f"[{finding['severity']}] {finding['title']}: {action}")
    return sequence


def residual_risks(scope: dict[str, object], artifacts: dict[str, object], findings: list[dict[str, object]]) -> list[str]:
    risks: list[str] = []
    if scope.get("non_reviewed_surfaces"):
        risks.append("Non-reviewed surfaces remain and should be checked before broad release decisions.")
    for key, present in artifacts.items():
        if not present:
            risks.append(f"{key.replace('_', ' ')} was not found in the packet.")
    if any(not finding.get("evidence") for finding in findings):
        risks.append("At least one finding is missing extracted evidence and needs manual review.")
    return risks or ["No additional residual risk beyond the reviewed findings was extracted."]


def build_report(root: Path, audience: str, repo_name_override: str | None) -> dict[str, object]:
    manifest = read_manifest(root, repo_name_override)
    findings = split_findings(root)
    scope = scope_from_repo_scope(root)
    artifacts = artifact_status(root)
    residual = residual_risks(scope, artifacts, findings)
    status = "pass" if artifacts["redaction_report"] and findings else "partial"
    return {
        "schema": SCHEMA,
        "created_at": now_utc(),
        "repo_name": manifest["repo_name"],
        "repo_ref": manifest["repo_ref"],
        "run_id": manifest["run_id"],
        "status": status,
        "audience": audience,
        "publication_boundary": {
            "privacy_mode": manifest["privacy_mode"],
            "publication_allowed_by_source": manifest["publication_allowed"],
            "published_by_skill": False,
            "approval_claimed": False,
            "compliance_claimed": False,
            "remediation_complete_claimed": False,
        },
        "scope": scope,
        "top_findings": findings[:8],
        "architecture_summary": architecture_summary(findings, artifacts),
        "security_privacy_notes": security_privacy_notes(findings, artifacts, manifest),
        "diagram_links": diagram_links(root),
        "test_recommendations": test_recommendations(root),
        "documentation_onboarding_notes": docs_notes(findings),
        "remediation_sequence": remediation_sequence(findings),
        "residual_risks": residual,
        "appendix": appendix(root),
    }


def architecture_summary(findings: list[dict[str, object]], artifacts: dict[str, object]) -> list[str]:
    arch = [finding for finding in findings if "architecture" in str(finding.get("source_role", "")).lower()]
    notes = [f"[{item['severity']}] {item['title']}" for item in arch[:5]]
    if artifacts["diagram_manifest"]:
        notes.append("Diagram artifacts are present; verify diagram truth boundaries before publication.")
    return notes or ["No architecture-specific finding was extracted from the packet."]


def security_privacy_notes(findings: list[dict[str, object]], artifacts: dict[str, object], manifest: dict[str, object]) -> list[str]:
    notes = [f"[{item['severity']}] {item['title']}" for item in findings if "security" in str(item.get("source_role", "")).lower()]
    notes.append(f"Privacy mode: {manifest['privacy_mode']}.")
    notes.append("Redaction report present." if artifacts["redaction_report"] else "Redaction report missing; do not publish until reviewed.")
    return notes


def docs_notes(findings: list[dict[str, object]]) -> list[str]:
    docs = [finding for finding in findings if "docs" in str(finding.get("source_role", "")).lower()]
    return [f"[{item['severity']}] {item['title']}" for item in docs[:5]] or ["No documentation-specific finding was extracted from the packet."]


def appendix(root: Path) -> list[str]:
    names = []
    for rel in (
        "run_manifest.json",
        "repo_scope.md",
        "final_report.md",
        "synthesis.md",
        "redaction_report.md",
        "quality_evaluation.md",
        "diagrams/diagram_manifest.md",
        "test_recommendations/test_gap_report.md",
        "issue-planning/issue_candidates.md",
    ):
        if (root / rel).is_file():
            names.append(rel)
    return names


def bullet_lines(items: list[object]) -> str:
    return "\n".join(f"- {item}" for item in items) if items else "- None extracted."


def write_markdown(path: Path, report: dict[str, object]) -> None:
    findings = report["top_findings"]
    finding_lines = []
    for finding in findings:
        finding_lines.append(
            f"""### Finding {finding['finding_id']}: [{finding['severity']}] {finding['title']}

- Source role: {finding['source_role']}
- Confidence: {finding['confidence']}
- Affected path or artifact: {finding.get('affected_path') or 'not extracted'}
- Evidence: {finding.get('evidence') or 'not extracted; manual review required'}
- Impact: {finding.get('impact') or 'not extracted'}
- Recommended action: {finding.get('recommended_action') or 'not extracted'}
- Validation gap: {finding.get('validation_gap') or 'not extracted'}
- Source artifact: {finding['source_artifact']}
"""
        )
    scope = report["scope"]
    boundary = report["publication_boundary"]
    content = f"""# CodeBuddy Review Report: {report['repo_name']}

## Executive Summary

- Overall risk: {overall_risk(findings)}
- Top risks: {', '.join(str(item['title']) for item in findings[:3]) if findings else 'No findings extracted.'}
- Recommended remediation sequence: see Remediation Sequence.
- Publication/privacy status: privacy mode {boundary['privacy_mode']}; published by this skill: false.

## Review Scope

- Repository: {report['repo_name']}
- Ref / branch / diff: {report['repo_ref']}
- Audience: {report['audience']}
- Included paths: {', '.join(scope.get('included_paths') or ['not extracted'])}
- Excluded paths: {', '.join(scope.get('excluded_paths') or ['not extracted'])}
- Non-reviewed surfaces: {', '.join(scope.get('non_reviewed_surfaces') or ['not extracted'])}
- Assumptions: {', '.join(scope.get('assumptions') or ['not extracted'])}

## Top Findings

{chr(10).join(finding_lines) if finding_lines else '- No top findings extracted from the supplied packet.'}

## Architecture Summary

{bullet_lines(report['architecture_summary'])}

## Security And Privacy Notes

{bullet_lines(report['security_privacy_notes'])}

## Diagram Links

{bullet_lines(report['diagram_links'])}

## Test Recommendations

{bullet_lines(report['test_recommendations'])}

## Documentation And Onboarding Notes

{bullet_lines(report['documentation_onboarding_notes'])}

## Remediation Sequence

{numbered_lines(report['remediation_sequence'])}

## Caveats And Residual Risks

{bullet_lines(report['residual_risks'])}

## Appendix

{bullet_lines(report['appendix'])}

## Publication Boundary

- This report was generated for review, not published.
- Approval claimed: false.
- Compliance claimed: false.
- Remediation complete claimed: false.
- Publication allowed by source manifest: {str(boundary['publication_allowed_by_source']).lower()}.
"""
    path.write_text(content, encoding="utf-8")


def overall_risk(findings: list[dict[str, object]]) -> str:
    if not findings:
        return "unknown"
    severity = str(findings[0]["severity"])
    return {"P0": "critical", "P1": "high", "P2": "medium", "P3": "low"}.get(severity, "unknown")


def numbered_lines(items: list[object]) -> str:
    return "\n".join(f"{index}. {item}" for index, item in enumerate(items, start=1)) if items else "1. None extracted."


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_root", help="CodeBuddy review packet root")
    parser.add_argument("--out", default=None, help="Product report artifact root")
    parser.add_argument("--audience", default="customer_private", help="Report audience")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path(args.packet_root)
    if not root.is_dir():
        raise SystemExit(f"packet root does not exist: {root}")
    out_root = Path(args.out) if args.out else root / "product-report"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    report = build_report(root, args.audience, args.repo_name)
    write_json(out_root / "codebuddy_product_report.json", report)
    write_markdown(out_root / "codebuddy_product_report.md", report)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
