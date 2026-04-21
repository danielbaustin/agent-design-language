#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

ALLOWED_INTEGRATION_STATES = {"worktree_only", "pr_open", "merged", "closed_no_pr"}
PLACEHOLDER_RE = re.compile(r"\b(?:TODO|TBD)\b|<[^>\n]+>|\bN/A\b")
STATUS_RE = re.compile(r"^Status:\s*(?P<status>[A-Z_]+)\s*$", re.MULTILINE)
INTEGRATION_RE = re.compile(r"^- Integration state:\s*(?P<state>\S+)\s*$", re.MULTILINE)


def repo_relative(path: Path, root: Path | None) -> str:
    if root:
        try:
            return path.resolve().relative_to(root.resolve()).as_posix()
        except ValueError:
            pass
    return path.as_posix()


def iter_markdown(paths: list[Path]) -> list[Path]:
    discovered: list[Path] = []
    for path in paths:
        if path.is_file():
            discovered.append(path)
        elif path.is_dir():
            discovered.extend(sorted(path.rglob("*.md")))
    return sorted(dict.fromkeys(discovered))


def finding(severity: str, area: str, message: str, evidence: str, path: str) -> dict:
    return {
        "severity": severity,
        "area": area,
        "message": message,
        "evidence": evidence,
        "files": [path],
        "can_auto_fix": False,
    }


def analyze_file(path: Path, root: Path | None) -> list[dict]:
    text = path.read_text(encoding="utf-8")
    rel = repo_relative(path, root)
    findings: list[dict] = []

    for match in INTEGRATION_RE.finditer(text):
        state = match.group("state")
        if state not in ALLOWED_INTEGRATION_STATES:
            findings.append(
                finding(
                    "blocking",
                    "integration_truth",
                    f"Invalid integration state {state!r}",
                    f"Allowed values: {', '.join(sorted(ALLOWED_INTEGRATION_STATES))}",
                    rel,
                )
            )

    status_match = STATUS_RE.search(text)
    integration_match = INTEGRATION_RE.search(text)
    if status_match and integration_match:
        status = status_match.group("status")
        integration = integration_match.group("state")
        if status in {"NOT_STARTED", "IN_PROGRESS"} and integration in {"pr_open", "merged", "closed_no_pr"}:
            findings.append(
                finding(
                    "warning",
                    "status_drift",
                    f"Lifecycle status {status!r} conflicts with integration state {integration!r}",
                    "A post-run or integrated record should not retain a pre-run status without explanation.",
                    rel,
                )
            )

    for line_no, line in enumerate(text.splitlines(), start=1):
        if PLACEHOLDER_RE.search(line):
            findings.append(
                finding(
                    "warning",
                    "placeholder_drift",
                    "Placeholder-like text remains in a lifecycle record",
                    f"line {line_no}: {line.strip()}",
                    rel,
                )
            )

    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description="Analyze ADL lifecycle records for bounded truth drift.")
    parser.add_argument("paths", nargs="+", help="Record files or directories to scan.")
    parser.add_argument("--repo-root", help="Optional repository root for relative paths.")
    parser.add_argument("--out", help="Optional JSON output path.")
    args = parser.parse_args()

    root = Path(args.repo_root).resolve() if args.repo_root else None
    paths = [Path(item) for item in args.paths]
    files = iter_markdown(paths)

    all_findings: list[dict] = []
    for path in files:
        all_findings.extend(analyze_file(path, root))

    blocking = any(item["severity"] == "blocking" for item in all_findings)
    output = {
        "schema_version": "records_hygiene.analysis.v1",
        "status": "blocked" if blocking else ("findings" if all_findings else "clean"),
        "scanned_files": [repo_relative(path, root) for path in files],
        "counts": {
            "files": len(files),
            "findings": len(all_findings),
            "blocking": sum(1 for item in all_findings if item["severity"] == "blocking"),
            "warning": sum(1 for item in all_findings if item["severity"] == "warning"),
        },
        "findings": all_findings,
        "safe_repairs_applied": [],
        "validation_performed": ["analyze_records_hygiene.py"],
        "handoff_state": {
            "ready_for_editor": bool(all_findings),
            "ready_for_execution": not blocking,
            "ready_for_follow_on_implementation": blocking,
        },
    }

    rendered = json.dumps(output, indent=2, sort_keys=True) + "\n"
    if args.out:
        out = Path(args.out)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(rendered, encoding="utf-8")
    else:
        print(rendered, end="")
    return 0 if not blocking else 1


if __name__ == "__main__":
    raise SystemExit(main())
