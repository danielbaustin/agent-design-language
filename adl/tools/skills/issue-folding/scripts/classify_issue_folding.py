#!/usr/bin/env python3
"""Classify whether a bounded issue should fold or stay actionable."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


PATTERNS: dict[str, re.Pattern[str]] = {
    "duplicate": re.compile(r"\b(duplicate|dup(?:licate)? of|same as)\b", re.I),
    "superseded": re.compile(r"\b(superseded by|replaced by|use issue)\b", re.I),
    "absorbed": re.compile(r"\b(absorbed by|folded into|covered by|handled by)\b", re.I),
    "already_satisfied": re.compile(
        r"\b(already satisfied|already implemented|already complete|no code change needed)\b",
        re.I,
    ),
    "obsolete": re.compile(r"\b(obsolete|no longer needed|no longer relevant|withdrawn)\b", re.I),
}
ISSUE_REF_RE = re.compile(r"#(\d+)")
PR_REF_RE = re.compile(r"pull/(\d+)")


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="utf-8", errors="replace")


def compact(line: str) -> str:
    return " ".join(line.strip().split())[:180]


def evidence_line(text: str, pattern: re.Pattern[str]) -> str:
    for line in text.splitlines():
        if pattern.search(line):
            return compact(line)
    return "marker present"


def collect_texts(task_bundle: Path | None, source_prompt: Path | None) -> list[dict[str, str]]:
    texts: list[dict[str, str]] = []
    if source_prompt and source_prompt.is_file():
        texts.append({"source": "source_issue_prompt", "text": read_text(source_prompt)})
    if task_bundle and task_bundle.is_dir():
        for name in ("stp.md", "sip.md", "sor.md"):
            path = task_bundle / name
            if path.is_file():
                texts.append({"source": name, "text": read_text(path)})
    return texts


def classify(texts: list[dict[str, str]]) -> tuple[str, list[dict[str, str]], list[str]]:
    matches: dict[str, list[dict[str, str]]] = {key: [] for key in PATTERNS}
    refs: set[str] = set()
    pr_refs: set[str] = set()

    for entry in texts:
        text = entry["text"]
        refs.update(f"#{num}" for num in ISSUE_REF_RE.findall(text))
        pr_refs.update(f"PR:{num}" for num in PR_REF_RE.findall(text))
        for label, pattern in PATTERNS.items():
            if pattern.search(text):
                matches[label].append(
                    {
                        "source": entry["source"],
                        "reason": f"{label.replace('_', ' ')} marker found",
                        "evidence": evidence_line(text, pattern),
                    }
                )

    active = [label for label, items in matches.items() if items]
    references = sorted(refs | pr_refs)

    linked_required = {"duplicate", "superseded", "absorbed"}
    for label in active:
        if label in linked_required and not references:
            matches[label].append(
                {
                    "source": "issue_packet",
                    "reason": "linked issue or PR evidence missing",
                    "evidence": "linked dispositions require issue or PR references",
                }
            )
            return "blocked", matches[label], references

    if len(active) > 1:
        conflict_evidence: list[dict[str, str]] = []
        for label in active:
            conflict_evidence.extend(matches[label][:1])
        conflict_evidence.append(
            {
                "source": "issue_packet",
                "reason": "conflicting disposition markers found",
                "evidence": ", ".join(active),
            }
        )
        return "blocked", conflict_evidence, references

    if not active:
        return "actionable", [], references

    label = active[0]
    return label, matches[label], references


def closeout_outcome(classification: str) -> str | None:
    if classification == "duplicate":
        return "duplicate"
    if classification in {"superseded", "absorbed"}:
        return "superseded"
    if classification in {"already_satisfied", "obsolete"}:
        return "closed_no_pr"
    return None


def status_for(classification: str) -> str:
    if classification == "actionable":
        return "actionable"
    if classification == "blocked":
        return "blocked"
    return "foldable"


def handoff_for(classification: str) -> str:
    if classification == "actionable":
        return "workflow-conductor"
    if classification == "blocked":
        return "operator-review"
    return "pr-closeout"


def worktree_action_for(classification: str) -> str:
    if classification in {"duplicate", "superseded", "absorbed", "already_satisfied", "obsolete"}:
        return "retire_bound_worktree_if_present"
    if classification == "blocked":
        return "preserve_until_operator_confirms"
    return "preserve_for_execution"


def build_report(args: argparse.Namespace) -> dict[str, Any]:
    task_bundle = Path(args.task_bundle) if args.task_bundle else None
    source_prompt = Path(args.source_prompt) if args.source_prompt else None
    texts = collect_texts(task_bundle, source_prompt)

    if not texts:
        classification = "blocked"
        evidence = [
            {
                "source": "issue_packet",
                "reason": "no readable issue packet surfaces found",
                "evidence": "expected source issue prompt or task bundle files missing",
            }
        ]
        references: list[str] = []
    else:
        classification, evidence, references = classify(texts)

    closure_outcome = closeout_outcome(classification)
    return {
        "schema": "adl.issue_folding_report.v1",
        "run_id": args.run_id,
        "status": status_for(classification),
        "classification": classification,
        "summary": f"Issue packet classified as {classification.replace('_', ' ')}.",
        "evidence": evidence,
        "closure_outcome": closure_outcome,
        "closure_references": references,
        "worktree_action": worktree_action_for(classification),
        "recommended_handoff": handoff_for(classification),
        "non_claims": [
            "This report does not close the issue.",
            "This report does not merge or prune worktrees.",
            "This report does not claim the folded work was revalidated here.",
        ],
        "safety_flags": {
            "issue_closed": False,
            "github_mutated": False,
            "merged_hidden": False,
            "worktree_pruned": False,
            "implementation_claimed": False,
        },
    }


def markdown_report(report: dict[str, Any]) -> str:
    lines = [
        "# Issue Folding Summary",
        "",
        f"- Run id: `{report['run_id']}`",
        f"- Status: `{report['status']}`",
        f"- Summary: {report['summary']}",
        "",
        "## Classification",
        "",
        f"- classification: `{report['classification']}`",
        "",
        "## Evidence",
        "",
    ]
    if report["evidence"]:
        for item in report["evidence"]:
            lines.append(
                f"- `{item['source']}`: {item['reason']} Evidence: {item['evidence']}"
            )
    else:
        lines.append("- no folding markers found")

    lines.extend(
        [
            "",
            "## Closure Outcome",
            "",
            f"- closure_outcome: `{report['closure_outcome'] or 'none'}`",
            f"- closure_references: {', '.join(report['closure_references']) if report['closure_references'] else 'none'}",
            "",
            "## Worktree Action",
            "",
            f"- worktree_action: `{report['worktree_action']}`",
            "",
            "## Recommended Handoff",
            "",
            f"- recommended_handoff: `{report['recommended_handoff']}`",
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in report["non_claims"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Safety Flags", ""])
    for key, value in report["safety_flags"].items():
        lines.append(f"- {key}: {str(value).lower()}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--task-bundle")
    parser.add_argument("--source-prompt")
    parser.add_argument("--out", required=True)
    parser.add_argument("--run-id", default="issue-folding")
    args = parser.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    report = build_report(args)
    (out / "issue_folding_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (out / "issue_folding_report.md").write_text(
        markdown_report(report),
        encoding="utf-8",
    )
    print(f"WROTE {out / 'issue_folding_report.json'}")
    print(f"WROTE {out / 'issue_folding_report.md'}")
    print(f"STATUS {report['status']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
