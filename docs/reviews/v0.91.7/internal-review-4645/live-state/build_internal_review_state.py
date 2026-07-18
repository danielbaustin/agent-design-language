#!/usr/bin/env python3
"""Build bounded live/local state for the v0.91.7 #4645 internal review."""

from __future__ import annotations

import json
import os
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[5]
OUT = ROOT / "docs/reviews/v0.91.7/internal-review-4645/live-state"


def run(cmd: list[str]) -> tuple[int, str, str]:
    env = os.environ.copy()
    env.setdefault("GH_PAGER", "cat")
    proc = subprocess.run(cmd, cwd=ROOT, env=env, text=True, capture_output=True)
    return proc.returncode, proc.stdout, proc.stderr


def gh_json(cmd: list[str]) -> dict | list | None:
    code, out, err = run(cmd)
    record = {"cmd": cmd, "exit_code": code, "stderr": err.strip()}
    if code != 0:
        return {"_error": record}
    try:
        value = json.loads(out)
    except json.JSONDecodeError as exc:
        return {"_error": record | {"json_error": str(exc), "stdout_prefix": out[:1000]}}
    return value


def write_json(name: str, value: object) -> None:
    (OUT / name).write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def issue_numbers_from_local_cards() -> list[int]:
    nums: list[int] = []
    issues_dir = ROOT / ".csdlc/issues"
    for path in issues_dir.glob("*/index.json"):
        try:
            data = json.loads(path.read_text())
        except Exception:
            continue
        issue = data.get("issue")
        if isinstance(issue, int):
            nums.append(issue)
    return sorted(set(nums))


def classify_wp(text: str) -> str | None:
    match = re.search(r"\bWP[-_ ]?(\d{1,2}[A-Z]?)\b", text, flags=re.IGNORECASE)
    if not match:
        return None
    suffix = match.group(1).upper()
    return f"WP-{suffix.zfill(2) if suffix.isdigit() else suffix}"


def local_issue_card_summary() -> list[dict]:
    rows: list[dict] = []
    for issue in issue_numbers_from_local_cards():
        root = ROOT / ".csdlc/issues" / str(issue)
        index_path = root / "index.json"
        try:
            index = json.loads(index_path.read_text())
        except Exception as exc:
            rows.append({"issue": issue, "error": str(exc)})
            continue
        cards = {}
        for card in ("sip", "stp", "spp", "vpp", "srp", "sor"):
            card_path = root / "cards" / f"{card}.md"
            values_path = root / "cards" / f"{card}.values.json"
            cards[card] = {
                "md": card_path.exists(),
                "values": values_path.exists(),
            }
        text = json.dumps(index, sort_keys=True)
        claim = index.get("claim") if isinstance(index.get("claim"), dict) else {}
        rows.append(
            {
                "issue": issue,
                "branch": claim.get("branch") or index.get("branch"),
                "worktree": claim.get("worktree") or index.get("worktree"),
                "status": index.get("status") or index.get("state"),
                "wp": classify_wp(text),
                "cards": cards,
            }
        )
    return rows


def summarize_pr(pr: dict) -> dict:
    checks = pr.get("statusCheckRollup") or []
    conclusions: dict[str, int] = {}
    names: list[dict] = []
    for check in checks:
        conclusion = check.get("conclusion") or check.get("status") or "UNKNOWN"
        conclusions[conclusion] = conclusions.get(conclusion, 0) + 1
        names.append(
            {
                "name": check.get("name"),
                "status": check.get("status"),
                "conclusion": check.get("conclusion"),
                "workflow": check.get("workflowName"),
                "url": check.get("detailsUrl"),
            }
        )
    return {
        "number": pr.get("number"),
        "title": pr.get("title"),
        "state": pr.get("state"),
        "isDraft": pr.get("isDraft"),
        "mergeable": pr.get("mergeable"),
        "mergedAt": pr.get("mergedAt"),
        "headRefName": pr.get("headRefName"),
        "baseRefName": pr.get("baseRefName"),
        "url": pr.get("url"),
        "check_conclusions": conclusions,
        "checks": names,
    }


def label_names(issue: dict) -> list[str]:
    return sorted(label.get("name", "") for label in issue.get("labels", []) if label.get("name"))


def wp_from_labels(labels: list[str]) -> str | None:
    for label in labels:
        if label.startswith("wp:"):
            return label.removeprefix("wp:")
    return None


def summarize_issue(issue: dict) -> dict:
    labels = label_names(issue)
    return {
        "number": issue.get("number"),
        "title": issue.get("title"),
        "state": issue.get("state"),
        "closedAt": issue.get("closedAt"),
        "wp": wp_from_labels(labels) or classify_wp(issue.get("title") or ""),
        "labels": labels,
        "url": issue.get("url"),
    }


def summarize_issue_set(issues: list[dict]) -> dict:
    rows = [summarize_issue(issue) for issue in issues]
    open_rows = [row for row in rows if row["state"] == "OPEN"]
    by_wp: dict[str, dict[str, int]] = {}
    for row in rows:
        wp = row.get("wp") or "unmapped"
        by_wp.setdefault(wp, {"OPEN": 0, "CLOSED": 0})
        state = row.get("state") or "UNKNOWN"
        by_wp[wp][state] = by_wp[wp].get(state, 0) + 1
    return {
        "count": len(rows),
        "open_count": len(open_rows),
        "by_wp": dict(sorted(by_wp.items())),
        "open_issues": sorted(open_rows, key=lambda row: row["number"] or 0),
    }


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)

    local_cards = local_issue_card_summary()
    write_json("local_csdlc_issue_cards.json", local_cards)

    issue_queries = {
        "version_label": [
            "gh",
            "issue",
            "list",
            "--state",
            "all",
            "--limit",
            "1000",
            "--search",
            "label:version:v0.91.7",
            "--json",
            "number,title,state,labels,closedAt,url,assignees",
        ],
        "title_v0917": [
            "gh",
            "issue",
            "list",
            "--state",
            "all",
            "--limit",
            "1000",
            "--search",
            "v0.91.7",
            "--json",
            "number,title,state,labels,closedAt,url,assignees",
        ],
    }
    issues = {name: gh_json(cmd) for name, cmd in issue_queries.items()}
    issue_summaries = {
        name: summarize_issue_set(value) if isinstance(value, list) else value
        for name, value in issues.items()
    }
    write_json("github_issue_summary.json", issue_summaries)

    pr_query = [
        "gh",
        "pr",
        "list",
        "--state",
        "all",
        "--limit",
        "600",
        "--search",
        "v0.91.7",
        "--json",
        "number,title,state,isDraft,mergeable,mergedAt,headRefName,baseRefName,url,statusCheckRollup",
    ]
    prs_raw = gh_json(pr_query)
    if isinstance(prs_raw, list):
        prs_summary = [summarize_pr(pr) for pr in prs_raw]
    else:
        prs_summary = prs_raw
    if isinstance(prs_summary, list):
        open_prs = [pr for pr in prs_summary if pr.get("state") == "OPEN"]
        failed_or_pending_prs = [
            pr
            for pr in open_prs
            if any(
                conclusion in {"FAILURE", "CANCELLED", "TIMED_OUT", "ACTION_REQUIRED", "IN_PROGRESS", "QUEUED", ""}
                for conclusion in pr.get("check_conclusions", {})
            )
        ]
        write_json(
            "github_open_pr_summary.json",
            {
                "open_count": len(open_prs),
                "failed_or_pending_count": len(failed_or_pending_prs),
                "open_prs": open_prs,
                "failed_or_pending_prs": failed_or_pending_prs,
            },
        )

    watch_issue = gh_json(
        [
            "gh",
            "issue",
            "view",
            "5408",
            "--json",
            "number,title,state,labels,closedAt,url,assignees,comments",
        ]
    )
    watch_pr = gh_json(
        [
            "gh",
            "pr",
            "view",
            "5419",
            "--json",
            "number,title,state,isDraft,mergeable,mergedAt,headRefName,baseRefName,url,statusCheckRollup",
        ]
    )
    if isinstance(watch_pr, dict) and "_error" not in watch_pr:
        watch_pr = summarize_pr(watch_pr)
    write_json("dependency_5408_5419.json", {"issue_5408": watch_issue, "pr_5419": watch_pr})

    summary = {
        "local_csdlc_issue_count": len(local_cards),
        "local_cards_missing": [
            row
            for row in local_cards
            if any(not state["md"] or not state["values"] for state in row.get("cards", {}).values())
        ],
        "github_issue_query_counts": {
            name: len(value) if isinstance(value, list) else None for name, value in issues.items()
        },
        "github_issue_open_counts": {
            name: value.get("open_count") if isinstance(value, dict) else None
            for name, value in issue_summaries.items()
        },
        "github_pr_count": len(prs_summary) if isinstance(prs_summary, list) else None,
        "dependency_5408_state": {
            "issue_state": watch_issue.get("state") if isinstance(watch_issue, dict) else None,
            "pr_state": watch_pr.get("state") if isinstance(watch_pr, dict) else None,
            "pr_is_draft": watch_pr.get("isDraft") if isinstance(watch_pr, dict) else None,
            "pr_check_conclusions": watch_pr.get("check_conclusions") if isinstance(watch_pr, dict) else None,
        },
    }
    write_json("summary.json", summary)
    print(json.dumps(summary, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
