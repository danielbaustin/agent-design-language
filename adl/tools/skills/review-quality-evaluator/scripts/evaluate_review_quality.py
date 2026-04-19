#!/usr/bin/env python3
"""Evaluate CodeBuddy review packet quality without publishing or mutating."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from pathlib import Path
from typing import Any

SCHEMA = "codebuddy.review_quality_evaluation.v1"
SEVERITY_ORDER = {"P0": 0, "P1": 1, "P2": 2, "P3": 3}
DEFAULT_REQUIRED_ROLES = ["code", "security", "tests", "docs", "architecture"]
REQUIRED_REPORT_SECTIONS = [
    "Executive Summary",
    "Review Scope",
    "Top Findings",
    "Architecture Summary",
    "Security And Privacy Notes",
    "Test Recommendations",
    "Remediation Sequence",
    "Residual Risks",
]
UNSUPPORTED_CLAIM_RE = re.compile(
    r"\b(approved|approval|compliant|compliance|merge[- ]?ready|production[- ]?ready|"
    r"remediation (?:complete|completed)|publication (?:approved|ready)|safe to publish)\b",
    re.IGNORECASE,
)
FINDING_RE = re.compile(
    r"^#{2,4}[ \t]+(?:Finding[ \t]+)?(?P<id>[A-Za-z0-9_.:-]+)?[ \t]*:?[ \t]*(?:\[(?P<severity>P[0-3])\])?[ \t]*(?P<title>[^\n#].*?)\s*$",
    re.MULTILINE,
)


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


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


def section_present(text: str, heading: str) -> bool:
    normalized = re.sub(r"\s+", " ", heading.strip()).lower()
    for line in text.splitlines():
        candidate = line.lstrip("# ").strip()
        candidate = re.sub(r"\s+", " ", candidate).lower()
        if candidate == normalized:
            return True
    return False


def extract_bullets_after(text: str, heading: str) -> list[str]:
    pattern = re.compile(rf"^##+\s+{re.escape(heading)}\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    if not match:
        return []
    next_heading = re.search(r"^##+\s+", text[match.end() :], re.MULTILINE)
    end = match.end() + next_heading.start() if next_heading else len(text)
    section = text[match.end() : end]
    return [line.strip()[2:].strip() for line in section.splitlines() if line.strip().startswith("- ")][:20]


def read_manifest(root: Path) -> dict[str, object]:
    manifest = load_json(root / "run_manifest.json")
    if not isinstance(manifest, dict):
        manifest = {}
    return {
        "run_id": str(manifest.get("run_id") or root.name),
        "repo_name": str(manifest.get("repo_name") or root.name),
        "repo_ref": str(manifest.get("repo_ref") or "not recorded"),
        "review_mode": str(manifest.get("review_mode") or "not recorded"),
        "privacy_mode": str(manifest.get("privacy_mode") or "not recorded"),
        "publication_allowed": bool(manifest.get("publication_allowed", False)),
    }


def preferred_review_sources(root: Path) -> list[Path]:
    for final_reports in (
        [root / "final_report.md"],
        [root / "product-report" / "codebuddy_product_report.md"],
        [root / "synthesis.md", root / "specialist_reviews" / "synthesis.md"],
    ):
        found_final = [path for path in final_reports if path.is_file()]
        if found_final:
            return found_final

    specialist = [
        root / "specialist_reviews" / "code.md",
        root / "specialist_reviews" / "security.md",
        root / "specialist_reviews" / "tests.md",
        root / "specialist_reviews" / "docs.md",
        root / "specialist_reviews" / "architecture.md",
        root / "specialist_reviews" / "dependencies.md",
        root / "specialist_reviews" / "dependency.md",
        root / "redaction_report.md",
        root / "redaction-audit" / "redaction_report.md",
    ]
    found = [path for path in specialist if path.is_file()]
    if found:
        return found
    return sorted(path for path in root.rglob("*.md") if "quality-evaluation" not in path.parts)[:50]


def role_artifact_sources(root: Path) -> list[Path]:
    preferred = [
        root / "specialist_reviews" / "code.md",
        root / "specialist_reviews" / "security.md",
        root / "specialist_reviews" / "tests.md",
        root / "specialist_reviews" / "docs.md",
        root / "specialist_reviews" / "architecture.md",
        root / "specialist_reviews" / "dependencies.md",
        root / "specialist_reviews" / "dependency.md",
        root / "architecture-review" / "architecture_review_scaffold.md",
        root / "dependency-review" / "dependency_review_scaffold.md",
    ]
    found = [path for path in preferred if path.is_file()]
    return found or preferred_review_sources(root)


def all_review_text(root: Path) -> str:
    chunks = []
    for path in preferred_review_sources(root):
        chunks.append(f"\n<!-- {packet_relative(root, path)} -->\n{read_text(path)}")
    return "\n".join(chunks)


def clean_title(title: str) -> str:
    title = re.sub(r"^\[[Pp][0-3]\]\s*", "", title).strip()
    return re.sub(r"\s+", " ", title).rstrip("# ").strip()


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


def role_from_path(path: Path, block: str) -> str:
    value = line_value(block, "Role") or line_value(block, "Source role")
    if value:
        return value.lower()
    lowered = f"{path.as_posix()} {block}".lower()
    for role in ("security", "tests", "docs", "architecture", "dependency", "diagram", "code"):
        if role in lowered:
            return "dependencies" if role == "dependency" else role
    return "review"


def split_findings(root: Path) -> list[dict[str, object]]:
    findings: list[dict[str, object]] = []
    for path in preferred_review_sources(root):
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
            findings.append(
                {
                    "finding_id": str(match.group("id") or f"{path.stem}-{index + 1}").strip(": "),
                    "title": clean_title(match.group("title")),
                    "severity": severity_from_block(match.group("severity"), block),
                    "role": role_from_path(path, block),
                    "source_artifact": packet_relative(root, path),
                    "evidence": line_value(block, "Evidence"),
                    "impact": line_value(block, "Impact") or line_value(block, "User/customer impact"),
                    "recommended_action": line_value(block, "Recommended action"),
                    "validation_gap": line_value(block, "Validation gap")
                    or line_value(block, "Validation or proof gap"),
                    "related_findings": line_value(block, "Related findings"),
                    "raw_block": block,
                }
            )
    return sorted(findings, key=lambda item: (SEVERITY_ORDER.get(str(item["severity"]), 9), str(item["finding_id"])))


def source_scope(root: Path) -> dict[str, object]:
    text = read_text(root / "repo_scope.md")
    if not text:
        return {
            "present": False,
            "included_paths": [],
            "excluded_paths": [],
            "non_reviewed_surfaces": [],
            "assumptions": [],
        }
    return {
        "present": True,
        "included_paths": extract_bullets_after(text, "Included paths"),
        "excluded_paths": extract_bullets_after(text, "Excluded paths"),
        "non_reviewed_surfaces": extract_bullets_after(text, "Non-reviewed surfaces"),
        "assumptions": extract_bullets_after(text, "Assumptions"),
    }


def role_coverage(root: Path, required_roles: list[str]) -> dict[str, object]:
    sources = role_artifact_sources(root)
    source_names = {packet_relative(root, path).lower() for path in sources}
    present: dict[str, bool] = {}
    for role in required_roles:
        role_l = role.lower()
        aliases = {
            "dependency": ["dependency", "dependencies"],
            "dependencies": ["dependency", "dependencies"],
        }.get(role_l, [role_l])
        present[role_l] = any(
            any(f"/{alias}.md" in f"/{name}" or alias in name for alias in aliases)
            or any(alias in "\n".join(read_text(path).lower().splitlines()[0:8]) for alias in aliases)
            for name, path in [(packet_relative(root, item).lower(), item) for item in sources]
        )
    missing = [role for role, exists in present.items() if not exists]
    return {
        "required_roles": required_roles,
        "present_roles": [role for role, exists in present.items() if exists],
        "missing_roles": missing,
        "source_artifacts": sorted(source_names),
    }


def template_compliance(text: str) -> dict[str, object]:
    missing = [heading for heading in REQUIRED_REPORT_SECTIONS if not section_present(text, heading)]
    return {
        "required_sections": REQUIRED_REPORT_SECTIONS,
        "missing_sections": missing,
        "status": "pass" if not missing else "fail",
    }


def unsupported_claims(root: Path) -> list[dict[str, str]]:
    claims: list[dict[str, str]] = []
    for path in preferred_review_sources(root):
        for line_no, line in enumerate(read_text(path).splitlines(), start=1):
            match = UNSUPPORTED_CLAIM_RE.search(line)
            if match:
                lowered = line.lower()
                if "claimed: false" in lowered or "published by this skill: false" in lowered:
                    continue
                claims.append(
                    {
                        "source_artifact": packet_relative(root, path),
                        "line": str(line_no),
                        "claim": match.group(0),
                        "sample": line.strip()[:180],
                    }
                )
    return claims


def redaction_present(root: Path) -> bool:
    return any(path.is_file() for path in [root / "redaction_report.md", root / "redaction-audit" / "redaction_report.md"])


def duplicate_titles(findings: list[dict[str, object]]) -> list[str]:
    seen: dict[str, int] = {}
    for finding in findings:
        title = str(finding.get("title", "")).lower()
        normalized = re.sub(r"[^a-z0-9]+", " ", title).strip()
        if normalized:
            seen[normalized] = seen.get(normalized, 0) + 1
    return [title for title, count in seen.items() if count > 1]


def evaluate(root: Path, publication_intent: str, required_roles: list[str]) -> dict[str, object]:
    manifest = read_manifest(root)
    text = all_review_text(root)
    findings = split_findings(root)
    scope = source_scope(root)
    coverage = role_coverage(root, required_roles)
    template = template_compliance(text)
    claims = unsupported_claims(root)
    blockers: list[dict[str, str]] = []
    warnings: list[dict[str, str]] = []

    if not preferred_review_sources(root):
        return {
            "schema": SCHEMA,
            "created_at": now_utc(),
            "run_id": manifest["run_id"],
            "repo_name": manifest["repo_name"],
            "repo_ref": manifest["repo_ref"],
            "status": "not_run",
            "publication_intent": publication_intent,
            "score": 0,
            "scorecard": {},
            "blocking_issues": [{"check": "source", "reason": "No readable review source was found."}],
            "warnings": [],
            "specialist_coverage": coverage,
            "template_compliance": template,
            "unsupported_claims": [],
            "residual_risk_clarity": {"status": "not_run"},
            "publication_boundary": publication_boundary(manifest, publication_intent),
            "recommended_handoffs": ["repo-packet-builder"],
        }

    if not scope["present"] or not scope["non_reviewed_surfaces"]:
        blockers.append({"check": "scope", "reason": "Scope or non-reviewed surfaces are missing."})
    if not findings:
        blockers.append({"check": "findings", "reason": "No findings were extracted from the review source."})
    for finding in findings:
        missing = [
            key
            for key in ("evidence", "impact", "recommended_action", "validation_gap")
            if not finding.get(key)
        ]
        if "evidence" in missing:
            blockers.append(
                {
                    "check": "finding_evidence",
                    "reason": f"Finding {finding['finding_id']} is missing evidence.",
                    "source_artifact": str(finding["source_artifact"]),
                }
            )
        if "impact" in missing:
            blockers.append(
                {
                    "check": "severity_accuracy",
                    "reason": f"Finding {finding['finding_id']} has no impact justification for severity.",
                    "source_artifact": str(finding["source_artifact"]),
                }
            )
        if "recommended_action" in missing or "validation_gap" in missing:
            warnings.append(
                {
                    "check": "actionability",
                    "reason": f"Finding {finding['finding_id']} has incomplete action or validation guidance.",
                    "source_artifact": str(finding["source_artifact"]),
                }
            )
    if publication_intent in {"customer_private", "public_candidate"} and not redaction_present(root):
        blockers.append({"check": "redaction", "reason": "Customer-facing publication intent requires redaction status."})
    if template["missing_sections"]:
        blockers.append(
            {
                "check": "template_compliance",
                "reason": "Required report sections are missing: " + ", ".join(template["missing_sections"]),
            }
        )
    for claim in claims:
        blockers.append(
            {
                "check": "unsupported_claim",
                "reason": f"Unsupported claim appears in {claim['source_artifact']} line {claim['line']}: {claim['claim']}",
            }
        )
    for role in coverage["missing_roles"]:
        warnings.append({"check": "specialist_coverage", "reason": f"Required specialist role missing or not visible: {role}"})
    for title in duplicate_titles(findings):
        warnings.append({"check": "duplication", "reason": f"Potential duplicate finding title needs synthesis review: {title}"})

    residual_ok = section_present(text, "Residual Risk") or section_present(text, "Residual Risks") or bool(scope["non_reviewed_surfaces"])
    if not residual_ok:
        blockers.append({"check": "residual_risk", "reason": "Residual risk or non-reviewed surfaces are not visible."})

    scorecard = {
        "evidence_quality": "fail" if any(item["check"] == "finding_evidence" for item in blockers) else "pass",
        "severity_accuracy": "fail" if any(item["check"] == "severity_accuracy" for item in blockers) else "pass",
        "actionability": "partial" if any(item["check"] == "actionability" for item in warnings) else "pass",
        "duplication": "partial" if any(item["check"] == "duplication" for item in warnings) else "pass",
        "unsupported_claims": "fail" if claims else "pass",
        "specialist_coverage": "partial" if coverage["missing_roles"] else "pass",
        "template_compliance": template["status"],
        "residual_risk_clarity": "pass" if residual_ok else "fail",
        "redaction_status": "pass" if redaction_present(root) else "partial",
    }
    status = "fail" if blockers else "partial" if warnings else "pass"
    score = quality_score(scorecard, blockers, warnings)
    return {
        "schema": SCHEMA,
        "created_at": now_utc(),
        "run_id": manifest["run_id"],
        "repo_name": manifest["repo_name"],
        "repo_ref": manifest["repo_ref"],
        "status": status,
        "publication_intent": publication_intent,
        "score": score,
        "scorecard": scorecard,
        "blocking_issues": blockers,
        "warnings": warnings,
        "specialist_coverage": coverage,
        "template_compliance": template,
        "unsupported_claims": claims,
        "residual_risk_clarity": {
            "status": "pass" if residual_ok else "fail",
            "non_reviewed_surfaces": scope["non_reviewed_surfaces"],
        },
        "publication_boundary": publication_boundary(manifest, publication_intent),
        "recommended_handoffs": recommended_handoffs(blockers, warnings),
    }


def quality_score(scorecard: dict[str, str], blockers: list[dict[str, str]], warnings: list[dict[str, str]]) -> int:
    score = 100
    score -= 15 * len(blockers)
    score -= 5 * len(warnings)
    score -= 10 * sum(1 for value in scorecard.values() if value == "partial")
    score -= 20 * sum(1 for value in scorecard.values() if value == "fail")
    return max(score, 0)


def publication_boundary(manifest: dict[str, object], publication_intent: str) -> dict[str, object]:
    return {
        "publication_intent": publication_intent,
        "privacy_mode": manifest["privacy_mode"],
        "publication_allowed_by_source": manifest["publication_allowed"],
        "published_by_skill": False,
        "approval_claimed": False,
        "compliance_claimed": False,
        "remediation_complete_claimed": False,
    }


def recommended_handoffs(blockers: list[dict[str, str]], warnings: list[dict[str, str]]) -> list[str]:
    checks = {item["check"] for item in blockers + warnings}
    handoffs: list[str] = []
    if "redaction" in checks or "unsupported_claim" in checks:
        handoffs.append("redaction-and-evidence-auditor")
    if "template_compliance" in checks or "actionability" in checks or "residual_risk" in checks:
        handoffs.append("product-report-writer")
    if "duplication" in checks or "specialist_coverage" in checks:
        handoffs.append("repo-review-synthesis")
    if "finding_evidence" in checks or "severity_accuracy" in checks:
        handoffs.append("owning specialist reviewer")
    return handoffs or ["human review"]


def bullet_lines(items: list[object]) -> str:
    if not items:
        return "- None."
    return "\n".join(f"- {item}" for item in items)


def issue_lines(items: list[dict[str, str]]) -> str:
    if not items:
        return "- None."
    lines = []
    for item in items:
        reason = item.get("reason", "No reason recorded.")
        source = item.get("source_artifact")
        suffix = f" Source: {source}." if source else ""
        lines.append(f"- {item.get('check', 'check')}: {reason}{suffix}")
    return "\n".join(lines)


def scorecard_lines(scorecard: dict[str, str]) -> str:
    return "\n".join(f"- {key.replace('_', ' ').title()}: {value}" for key, value in sorted(scorecard.items()))


def write_markdown(path: Path, evaluation: dict[str, object]) -> None:
    coverage = evaluation["specialist_coverage"]
    template = evaluation["template_compliance"]
    boundary = evaluation["publication_boundary"]
    content = f"""# CodeBuddy Review Quality Evaluation: {evaluation['repo_name']}

## Quality Gate Summary

- Status: {evaluation['status']}
- Score: {evaluation['score']}
- Publication intent: {evaluation['publication_intent']}
- Repo ref: {evaluation['repo_ref']}

## Scope And Source

- Run id: {evaluation['run_id']}
- Source artifacts: {', '.join(coverage.get('source_artifacts') or ['none'])}

## Scorecard

{scorecard_lines(evaluation['scorecard'])}

## Blocking Issues

{issue_lines(evaluation['blocking_issues'])}

## Warnings

{issue_lines(evaluation['warnings'])}

## Specialist Coverage

- Present roles: {', '.join(coverage.get('present_roles') or ['none'])}
- Missing roles: {', '.join(coverage.get('missing_roles') or ['none'])}

## Template Compliance

- Status: {template['status']}
- Missing sections: {', '.join(template.get('missing_sections') or ['none'])}

## Unsupported Claims Check

{issue_lines([{'check': 'unsupported_claim', 'reason': item.get('sample', ''), 'source_artifact': item.get('source_artifact', '')} for item in evaluation['unsupported_claims']])}

## Residual Risk Clarity

- Status: {evaluation['residual_risk_clarity']['status']}
- Non-reviewed surfaces: {', '.join(evaluation['residual_risk_clarity'].get('non_reviewed_surfaces') or ['none extracted'])}

## Publication Boundary

- Published by this skill: false.
- Approval claimed: false.
- Compliance claimed: false.
- Remediation complete claimed: false.
- Publication allowed by source manifest: {str(boundary['publication_allowed_by_source']).lower()}.

## Recommended Handoffs

{bullet_lines(evaluation['recommended_handoffs'])}
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_root", help="CodeBuddy packet or report artifact root")
    parser.add_argument("--out", default=None, help="Quality evaluation artifact root")
    parser.add_argument("--publication-intent", default="internal_review", help="Publication intent")
    parser.add_argument("--required-role", action="append", default=[], help="Required specialist role")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path(args.packet_root)
    if not root.is_dir():
        raise SystemExit(f"packet root does not exist: {root}")
    out_root = Path(args.out) if args.out else root / "quality-evaluation"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)
    required_roles = args.required_role or DEFAULT_REQUIRED_ROLES
    evaluation = evaluate(root, args.publication_intent, required_roles)
    write_json(out_root / "review_quality_evaluation.json", evaluation)
    write_markdown(out_root / "review_quality_evaluation.md", evaluation)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
