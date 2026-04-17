#!/usr/bin/env python3
"""Audit a CodeBuddy review packet for redaction and evidence-boundary risks."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from dataclasses import asdict, dataclass
from pathlib import Path

TEXT_SUFFIXES = {".md", ".txt", ".json", ".yaml", ".yml", ".toml", ".csv", ".log"}
DEFAULT_MAX_EXCERPT_LINES = 80

SECRET_PATTERNS: list[tuple[str, re.Pattern[str]]] = [
    ("openai_api_key", re.compile(r"\bsk-[A-Za-z0-9_-]{12,}\b")),
    ("anthropic_api_key", re.compile(r"\bsk-ant-[A-Za-z0-9_-]{12,}\b")),
    ("github_token", re.compile(r"\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9_]{20,}\b")),
    ("github_pat", re.compile(r"\bgithub_pat_[A-Za-z0-9_]{20,}\b")),
    ("slack_token", re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{12,}\b")),
    ("aws_access_key", re.compile(r"\bAKIA[0-9A-Z]{16}\b")),
    ("private_key", re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----")),
    (
        "credential_assignment",
        re.compile(
            r"\b(?:API_KEY|TOKEN|SECRET|PASSWORD|OPENAI_API_KEY|ANTHROPIC_API_KEY)\s*=\s*[^\s]+",
            re.IGNORECASE,
        ),
    ),
]

PRIVATE_PATH_PATTERN = re.compile(r"(?<![\w.-])(?:/Users/|/home/|/private/var/|/var/folders/|[A-Za-z]:\\)")
INTERNAL_URL_PATTERN = re.compile(
    r"\bhttps?://(?:localhost|127\.0\.0\.1|10\.\d{1,3}\.\d{1,3}\.\d{1,3}|"
    r"192\.168\.\d{1,3}\.\d{1,3}|172\.(?:1[6-9]|2\d|3[0-1])\.\d{1,3}\.\d{1,3}|"
    r"[^/\s]*\.local)(?::\d+)?[^\s)]*",
    re.IGNORECASE,
)
PROMPT_LEAK_PATTERN = re.compile(r"\b(?:system prompt|developer message|tool arguments|raw prompt)\b", re.IGNORECASE)


@dataclass(frozen=True)
class Finding:
    severity: str
    category: str
    path: str
    line: int
    message: str
    sample: str
    recommendation: str


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def mask_sample(sample: str) -> str:
    text = sample.strip()
    for _, pattern in SECRET_PATTERNS:
        text = pattern.sub(lambda match: mask_secret(match.group(0)), text)
    text = re.sub(r"\s+", " ", text)
    return text[:180]


def mask_secret(value: str) -> str:
    if len(value) <= 8:
        return "<masked>"
    return f"{value[:4]}...{value[-4:]}"


def is_text_file(path: Path) -> bool:
    return path.suffix.lower() in TEXT_SUFFIXES or path.name in {"Dockerfile", "Makefile"}


def iter_files(root: Path) -> list[Path]:
    return sorted(path for path in root.rglob("*") if path.is_file() and is_text_file(path))


def relative_path(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


def read_manifest(root: Path) -> dict[str, object]:
    manifest_path = root / "run_manifest.json"
    if not manifest_path.is_file():
        return {}
    try:
        data = json.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}
    return data if isinstance(data, dict) else {}


def scan_lines(root: Path, path: Path, max_excerpt_lines: int) -> list[Finding]:
    findings: list[Finding] = []
    rel = relative_path(root, path)
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError as exc:
        return [
            Finding(
                "warning",
                "unreadable_file",
                rel,
                0,
                f"Could not read file: {exc.__class__.__name__}",
                "",
                "Confirm whether this artifact should be part of the audit surface.",
            )
        ]

    fence_start = 0
    in_fence = False
    for index, line in enumerate(lines, start=1):
        for category, pattern in SECRET_PATTERNS:
            if pattern.search(line):
                findings.append(
                    Finding(
                        "blocker",
                        category,
                        rel,
                        index,
                        "Secret-like value appears in an audit surface.",
                        mask_sample(line),
                        "Remove the value from the artifact and rotate it if it was real.",
                    )
                )
        if PRIVATE_PATH_PATTERN.search(line):
            findings.append(
                Finding(
                    "blocker",
                    "private_host_path",
                    rel,
                    index,
                    "Private host path appears in an artifact intended to be portable.",
                    mask_sample(line),
                    "Replace host-specific paths with repo-relative paths or placeholders.",
                )
            )
        if INTERNAL_URL_PATTERN.search(line):
            findings.append(
                Finding(
                    "warning",
                    "internal_url",
                    rel,
                    index,
                    "Internal URL or private network address appears in the artifact.",
                    mask_sample(line),
                    "Confirm audience suitability or replace with a generic endpoint label.",
                )
            )
        if PROMPT_LEAK_PATTERN.search(line):
            findings.append(
                Finding(
                    "warning",
                    "prompt_or_tool_leak",
                    rel,
                    index,
                    "Prompt or tool execution details may be exposed.",
                    mask_sample(line),
                    "Summarize the evidence without exposing hidden prompts or raw tool arguments.",
                )
            )

        if line.strip().startswith("```"):
            if not in_fence:
                fence_start = index
                in_fence = True
            else:
                fence_len = index - fence_start - 1
                if fence_len > max_excerpt_lines:
                    findings.append(
                        Finding(
                            "warning",
                            "excessive_source_excerpt",
                            rel,
                            fence_start,
                            "Fenced source excerpt exceeds the configured line limit.",
                            f"fenced block length: {fence_len} lines",
                            "Replace long source excerpts with path and line references.",
                        )
                    )
                in_fence = False

    return findings


def status_from_findings(findings: list[Finding]) -> str:
    if any(finding.severity == "blocker" for finding in findings):
        return "fail"
    if any(finding.severity == "warning" for finding in findings):
        return "partial"
    return "pass"


def recommendation(status: str, manifest: dict[str, object], audience: str) -> str:
    if status == "not_run":
        return "not_run"
    if manifest.get("publication_allowed") is False and audience in {"customer_private", "public_candidate"}:
        return "block_publication"
    if status == "fail":
        return "block_publication"
    if status == "partial" or audience == "public_candidate":
        return "hold_for_review"
    return "allow_internal"


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_markdown(path: Path, report: dict[str, object]) -> None:
    findings = report["findings"]
    if findings:
        findings_md = "\n".join(
            "- [{severity}] {category} in {path}:{line} - {message} Sample: `{sample}`".format(**finding)
            for finding in findings
        )
    else:
        findings_md = "- No blocker or warning findings."

    content = f"""# Redaction And Evidence Audit

## Verdict

- Status: {report["status"]}
- Files scanned: {report["files_scanned"]}
- Blockers: {report["counts"]["blocker"]}
- Warnings: {report["counts"]["warning"]}
- Info: {report["counts"]["info"]}

## Publication Recommendation

- Recommendation: {report["publication_recommendation"]}
- Audience: {report["audience"]}

## Scope

- Artifact root: {report["artifact_root"]}
- Started at: {report["started_at"]}
- Completed at: {report["completed_at"]}

## Findings

{findings_md}

## Evidence Boundary Notes

- Secret-like samples are masked.
- Paths are relative to the audited artifact root.
- This audit does not mutate source artifacts.
- This audit does not replace specialist code, security, docs, or test review.

## Required Follow-Up

{required_follow_up(str(report["publication_recommendation"]))}
"""
    path.write_text(content, encoding="utf-8")


def required_follow_up(publication_recommendation: str) -> str:
    if publication_recommendation == "block_publication":
        return "- Block sharing until the owning artifact producer removes or replaces unsafe evidence."
    if publication_recommendation == "hold_for_review":
        return "- Review warnings manually before customer-facing or public sharing."
    if publication_recommendation == "allow_internal":
        return "- Keep the report internal unless a later pre-publication gate explicitly approves broader sharing."
    return "- Re-run the audit with a readable artifact root."


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_root", help="Packet, review bundle, or report root to audit")
    parser.add_argument("--out", default=None, help="Audit artifact root")
    parser.add_argument("--audience", default="local_only", choices=["local_only", "customer_private", "public_candidate"])
    parser.add_argument("--max-excerpt-lines", type=int, default=DEFAULT_MAX_EXCERPT_LINES)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    started_at = now_utc()
    root = Path(args.artifact_root).resolve()
    out_root = Path(args.out) if args.out else root / "redaction-audit"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    if not root.exists():
        report = {
            "schema": "codebuddy.redaction_audit.v1",
            "status": "not_run",
            "publication_recommendation": "not_run",
            "audience": args.audience,
            "artifact_root": root.name,
            "files_scanned": 0,
            "findings": [],
            "counts": {"blocker": 0, "warning": 0, "info": 0},
            "started_at": started_at,
            "completed_at": now_utc(),
            "notes": ["artifact root does not exist"],
        }
        write_json(out_root / "redaction_report.json", report)
        write_markdown(out_root / "redaction_report.md", report)
        print(out_root)
        return 1

    manifest = read_manifest(root)
    files = iter_files(root)
    findings: list[Finding] = []
    for path in files:
        if out_root in path.parents:
            continue
        findings.extend(scan_lines(root, path, args.max_excerpt_lines))

    if not manifest:
        findings.append(
            Finding(
                "warning",
                "missing_manifest",
                "run_manifest.json",
                0,
                "No CodeBuddy run manifest was found.",
                "",
                "Add or repair the packet manifest before publication gating.",
            )
        )
    elif manifest.get("publication_allowed") is False and args.audience in {"customer_private", "public_candidate"}:
        findings.append(
            Finding(
                "blocker",
                "publication_forbidden_by_manifest",
                "run_manifest.json",
                0,
                "Packet manifest forbids publication for the requested audience.",
                '"publication_allowed": false',
                "Keep the artifact internal or run an explicit remediation and approval workflow.",
            )
        )

    status = status_from_findings(findings)
    counts = {
        "blocker": sum(1 for finding in findings if finding.severity == "blocker"),
        "warning": sum(1 for finding in findings if finding.severity == "warning"),
        "info": sum(1 for finding in findings if finding.severity == "info"),
    }
    report = {
        "schema": "codebuddy.redaction_audit.v1",
        "status": status,
        "publication_recommendation": recommendation(status, manifest, args.audience),
        "audience": args.audience,
        "artifact_root": root.name,
        "files_scanned": len(files),
        "findings": [asdict(finding) for finding in findings],
        "counts": counts,
        "started_at": started_at,
        "completed_at": now_utc(),
        "notes": [
            "Audit is deterministic and read-only.",
            "Findings are bounded to text-like artifacts under the audited root.",
        ],
    }

    write_json(out_root / "redaction_report.json", report)
    write_markdown(out_root / "redaction_report.md", report)
    if report["publication_recommendation"] == "block_publication":
        (out_root / "blocked_publication_note.md").write_text(
            "# Blocked Publication\n\nDo not share this packet until blocker findings are resolved.\n",
            encoding="utf-8",
        )
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

