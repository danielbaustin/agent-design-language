#!/usr/bin/env python3
"""Scan and optionally normalize machine-local contract assumptions."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any


TEXT_SUFFIXES = {
    ".bash",
    ".json",
    ".md",
    ".py",
    ".sh",
    ".toml",
    ".txt",
    ".yaml",
    ".yml",
}

RULES = [
    (
        "brittle_worktree_name",
        re.compile(r"\.worktrees/adl-wp-\d+"),
        ".worktrees/adl-wp-<issue>",
        "hard-coded issue worktree name should be generalized",
        True,
    ),
    (
        "absolute_host_path",
        re.compile(r"/Users/[A-Za-z0-9._-]+/[^\s\"')]*?(?=/.worktrees|[\s\"')]|\Z)"),
        "<host-path>",
        "absolute user path should be parameterized",
        True,
    ),
    (
        "machine_local_temp_path",
        re.compile(r"/(?:private/)?var/folders/[^\s\"')]+|/tmp/[^\s\"')]+"),
        "<temp-path>",
        "machine-local temp path should not be embedded in portable artifacts",
        True,
    ),
    (
        "environment_specific_assertion",
        re.compile(r"\b(?:USER|HOME|HOSTNAME|SHELL)=['\"]?[A-Za-z0-9_./-]+"),
        "<env-var>=<value>",
        "environment-specific assertion should be parameterized",
        False,
    ),
    (
        "stale_contract_reference",
        re.compile(r"hard-?coded (?:skill )?(?:list|inventory|contract)|stale contract", re.I),
        "<contract-reference>",
        "hard-coded contract reference may need a derived source of truth",
        False,
    ),
]


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="utf-8", errors="replace")


def iter_files(root: Path) -> list[Path]:
    if root.is_file():
        return [root] if root.suffix.lower() in TEXT_SUFFIXES else []
    if not root.is_dir():
        return []
    files: list[Path] = []
    for path in sorted(root.rglob("*")):
        if path.is_file() and path.suffix.lower() in TEXT_SUFFIXES:
            files.append(path)
    return files


def line_for_offset(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def scan_file(path: Path, scan_root: Path) -> tuple[list[dict[str, Any]], str]:
    text = read_text(path)
    rel = path.relative_to(scan_root).as_posix()
    findings: list[dict[str, Any]] = []
    for category, pattern, replacement, reason, safe_fix in RULES:
        for match in pattern.finditer(text):
            findings.append(
                {
                    "category": category,
                    "path": rel,
                    "line": line_for_offset(text, match.start()),
                    "evidence": match.group(0),
                    "reason": reason,
                    "safe_fix_available": safe_fix,
                    "replacement": replacement if safe_fix else None,
                }
            )
    return findings, text


def apply_safe_fixes(path: Path) -> tuple[bool, list[str]]:
    original = read_text(path)
    updated = original
    applied: list[str] = []
    for category, pattern, replacement, _reason, safe_fix in RULES:
        if not safe_fix:
            continue
        updated, count = pattern.subn(replacement, updated)
        if count:
            applied.append(f"{category}:{count}")
    if updated != original:
        path.write_text(updated, encoding="utf-8")
        return True, applied
    return False, applied


def build_report(args: argparse.Namespace) -> dict[str, Any]:
    scan_root = Path(args.root)
    if not scan_root.exists():
        findings = [
            {
                "category": "not_run",
                "path": "scan_root",
                "line": 0,
                "evidence": "missing root",
                "reason": "scan root is missing",
                "safe_fix_available": False,
                "replacement": None,
            }
        ]
        return report_for(args, "not_run", findings, [], mutated=False)

    files = iter_files(scan_root)
    if not files:
        findings = [
            {
                "category": "not_run",
                "path": "scan_root",
                "line": 0,
                "evidence": "no supported text files",
                "reason": "scan root contains no supported contract surfaces",
                "safe_fix_available": False,
                "replacement": None,
            }
        ]
        return report_for(args, "not_run", findings, [], mutated=False)

    all_findings: list[dict[str, Any]] = []
    applied: list[dict[str, Any]] = []
    for path in files:
        findings, _text = scan_file(path, scan_root)
        all_findings.extend(findings)
        if args.apply:
            changed, applied_rules = apply_safe_fixes(path)
            if changed:
                applied.append(
                    {
                        "path": path.relative_to(scan_root).as_posix(),
                        "rules": applied_rules,
                    }
                )

    status = "clean"
    if all_findings:
        status = "fixed" if args.apply and applied else "findings"
    if any(not finding.get("safe_fix_available") for finding in all_findings):
        status = "blocked" if args.apply else status
    return report_for(args, status, all_findings, applied, mutated=bool(applied))


def report_for(
    args: argparse.Namespace,
    status: str,
    findings: list[dict[str, Any]],
    applied: list[dict[str, Any]],
    mutated: bool,
) -> dict[str, Any]:
    counts = Counter(finding["category"] for finding in findings)
    return {
        "schema": "adl.portable_contract_normalizer_report.v1",
        "run_id": args.run_id,
        "status": status,
        "mode": "scan_and_apply_safe_fixes" if args.apply else "scan_contracts",
        "counts": dict(sorted(counts.items())),
        "findings": findings,
        "applied_fixes": applied,
        "non_claims": [
            "This report covers only the bounded scan root.",
            "This report does not resolve design-decision findings.",
            "This report does not prove the full repository is portable.",
            "This report does not redact legitimate evidence automatically.",
        ],
        "safety_flags": {
            "mutated_repository": mutated,
            "applied_safe_fixes_only": mutated,
            "design_decisions_resolved": False,
            "legitimate_evidence_redacted": False,
            "unbounded_scan": False,
        },
    }


def markdown_report(report: dict[str, Any]) -> str:
    lines = [
        "# Portable Contract Normalizer Summary",
        "",
        f"- Run id: `{report['run_id']}`",
        f"- Status: `{report['status']}`",
        f"- Mode: `{report['mode']}`",
        "",
        "## Finding Counts",
        "",
    ]
    if report["counts"]:
        for category, count in report["counts"].items():
            lines.append(f"- {category}: {count}")
    else:
        lines.append("- none")

    lines.extend(["", "## Findings", ""])
    if report["findings"]:
        for finding in report["findings"]:
            lines.append(
                f"- `{finding['category']}` in `{finding['path']}` line {finding['line']}: "
                f"{finding['reason']} Evidence: `{finding['evidence']}`"
            )
    else:
        lines.append("- none")

    lines.extend(["", "## Safe Mechanical Normalization", ""])
    safe = [finding for finding in report["findings"] if finding.get("safe_fix_available")]
    if safe:
        for finding in safe:
            lines.append(f"- `{finding['path']}` line {finding['line']}: replace with `{finding['replacement']}`")
    else:
        lines.append("- none")

    lines.extend(["", "## Design Decisions Required", ""])
    design = [finding for finding in report["findings"] if not finding.get("safe_fix_available")]
    if design:
        for finding in design:
            lines.append(f"- `{finding['path']}` line {finding['line']}: {finding['reason']}")
    else:
        lines.append("- none")

    lines.extend(["", "## Applied Fixes", ""])
    if report["applied_fixes"]:
        for fix in report["applied_fixes"]:
            lines.append(f"- `{fix['path']}`: {', '.join(fix['rules'])}")
    else:
        lines.append("- none")

    lines.extend(["", "## Non-Claims", ""])
    for item in report["non_claims"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Safety Flags", ""])
    for key, value in report["safety_flags"].items():
        lines.append(f"- {key}: {str(value).lower()}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--run-id", default="portable-contract-normalizer")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    report = build_report(args)
    (out / "portable_contract_normalizer_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (out / "portable_contract_normalizer_report.md").write_text(
        markdown_report(report),
        encoding="utf-8",
    )
    print(f"WROTE {out / 'portable_contract_normalizer_report.json'}")
    print(f"WROTE {out / 'portable_contract_normalizer_report.md'}")
    print(f"STATUS {report['status']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
