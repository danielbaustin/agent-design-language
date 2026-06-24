#!/usr/bin/env python3
"""Generate the bounded #4441 v0.91.6 workflow metrics backfill inventory."""

from __future__ import annotations

import csv
import json
import os
import re
import subprocess
import sys
import argparse
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable


CSV_RELATIVE_PATH = Path(
    "docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_INVENTORY_4441.csv"
)
MARKDOWN_RELATIVE_PATH = Path(
    "docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.md"
)
JSON_RELATIVE_PATH = Path(
    "docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.json"
)
TASK_DIR_PATTERN = "issue-*__*"
SOR_FIELD_RE = re.compile(r"^- ([^:]+): `([^`]*)`$", re.MULTILINE)
EXEC_FIELD_RE = re.compile(r"^- (Start Time|End Time): (.+)$", re.MULTILINE)


@dataclass
class MetricValue:
    value: str
    status: str
    source: str
    confidence: str
    note: str


def run(cmd: list[str], cwd: Path) -> str:
    result = subprocess.run(
        cmd,
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
        env=os.environ.copy(),
    )
    return result.stdout


def parse_iso8601(value: str) -> datetime | None:
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00")).astimezone(timezone.utc)
    except ValueError:
        return None


def seconds_between(start: str | None, end: str | None) -> str:
    if not start or not end:
        return "unknown"
    start_dt = parse_iso8601(start)
    end_dt = parse_iso8601(end)
    if start_dt is None or end_dt is None:
        return "unknown"
    return str(int((end_dt - start_dt).total_seconds()))


def resolve_primary_checkout_root(repo_root: Path) -> Path:
    worktree_listing = run(["git", "worktree", "list", "--porcelain"], cwd=repo_root)
    current_path: Path | None = None
    current_branch: str | None = None
    for line in worktree_listing.splitlines():
        if line.startswith("worktree "):
            current_path = Path(line.removeprefix("worktree ").strip())
            current_branch = None
        elif line.startswith("branch "):
            current_branch = line.removeprefix("branch ").strip()
        elif line == "" and current_path is not None:
            if current_branch == "refs/heads/main":
                return current_path
            current_path = None
            current_branch = None
    if current_path is not None and current_branch == "refs/heads/main":
        return current_path
    return repo_root


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8") if path.exists() else ""


def parse_sor_fields(sor_text: str) -> tuple[dict[str, str], dict[str, str]]:
    metric_fields = {name: value for name, value in SOR_FIELD_RE.findall(sor_text)}
    exec_fields = {name: value.strip() for name, value in EXEC_FIELD_RE.findall(sor_text)}
    return metric_fields, exec_fields


def extract_sor_bullet_value(sor_text: str, field_name: str) -> str | None:
    pattern = re.compile(rf"^- {re.escape(field_name)}: (.+)$", re.MULTILINE)
    match = pattern.search(sor_text)
    return match.group(1).strip() if match else None


def metric_from_sor_value(raw_value: str | None, source: str, note: str) -> MetricValue:
    if raw_value is None:
        return MetricValue("unknown", "unknown", source, "low", note)
    normalized = raw_value.strip()
    if normalized in {"unknown", "not_collected", "not_applicable"}:
        confidence = "low" if normalized != "not_applicable" else "medium"
        return MetricValue(normalized, normalized, source, confidence, note)
    return MetricValue(normalized, "explicit", source, "high", note)


def actual_elapsed_metric(metric_fields: dict[str, str], exec_fields: dict[str, str]) -> MetricValue:
    actual = metric_fields.get("Actual elapsed seconds")
    explicit = metric_from_sor_value(
        actual,
        "sor_issue_metrics_truth",
        "Derived from the issue-local SOR Issue Metrics Truth section when present.",
    )
    if explicit.status == "explicit":
        return explicit
    derived_seconds = seconds_between(exec_fields.get("Start Time"), exec_fields.get("End Time"))
    if derived_seconds != "unknown":
        return MetricValue(
            derived_seconds,
            "derived",
            "sor_execution_window",
            "medium",
            "Derived from SOR execution start/end timestamps because no explicit actual elapsed seconds value was recorded.",
        )
    return explicit


def github_cycle_metric(issue_record: dict[str, object]) -> MetricValue:
    created_at = issue_record.get("createdAt")
    closed_at = issue_record.get("closedAt")
    state = str(issue_record.get("state", "unknown")).lower()
    if isinstance(created_at, str) and isinstance(closed_at, str):
        delta = seconds_between(created_at, closed_at)
        if delta != "unknown":
            return MetricValue(
                delta,
                "derived",
                "github_issue_created_closed",
                "high",
                "Derived from repo-native GitHub issue createdAt and closedAt timestamps.",
            )
    if state != "closed":
        return MetricValue(
            "unknown",
            "unknown",
            "github_issue_state_open",
            "low",
            "Issue is still open, so closed-cycle duration is not yet available.",
        )
    return MetricValue(
        "unknown",
        "unknown",
        "github_issue_timestamp_incomplete",
        "low",
        "Repo-native GitHub issue timestamps were incomplete for cycle-time reconstruction.",
    )


def token_metric(metric_fields: dict[str, str], sor_text: str) -> MetricValue:
    substrate_note = extract_sor_bullet_value(sor_text, "Goal-metrics substrate note")
    explicit = metric_from_sor_value(
        metric_fields.get("Actual total tokens"),
        "sor_issue_metrics_truth",
        "Derived from the issue-local SOR Issue Metrics Truth section when present.",
    )
    if explicit.status == "explicit":
        return explicit
    if substrate_note:
        return MetricValue(
            explicit.value,
            explicit.status,
            "sor_goal_metrics_substrate_note",
            explicit.confidence,
            substrate_note,
        )
    return explicit


def row_contract_completeness(
    issue_record: dict[str, object],
    actual_elapsed: MetricValue,
    cycle_metric: MetricValue,
    token_value: MetricValue,
    row_confidence: str,
) -> tuple[str, list[str]]:
    completeness_basis = {
        "issue_metadata": bool(issue_record.get("title")) and bool(issue_record.get("state")),
        "actual_session_elapsed_value": bool(actual_elapsed.value),
        "actual_session_elapsed_source": bool(actual_elapsed.source),
        "actual_session_elapsed_note": bool(actual_elapsed.note),
        "github_cycle_time_value": bool(cycle_metric.value),
        "github_cycle_time_source": bool(cycle_metric.source),
        "github_cycle_time_note": bool(cycle_metric.note),
        "actual_total_tokens_value": bool(token_value.value),
        "actual_total_tokens_source": bool(token_value.source),
        "actual_total_tokens_note": bool(token_value.note),
        "row_confidence": bool(row_confidence),
    }
    missing = [key for key, present in completeness_basis.items() if not present]
    if not missing:
        return "complete", missing
    if len(missing) <= 2:
        return "partial", missing
    return "incomplete", missing


def metric_availability_class(
    issue_state: str,
    actual_elapsed: MetricValue,
    cycle_metric: MetricValue,
    token_value: MetricValue,
) -> str:
    actual_known = actual_elapsed.status in {"explicit", "derived"}
    cycle_known = cycle_metric.status == "derived"
    token_known = token_value.status == "explicit"
    if actual_known and cycle_known and token_known:
        return "full_metrics_known"
    if actual_known and cycle_known and not token_known:
        return "timing_recovered_token_gap"
    if not actual_known and cycle_known and token_known:
        return "cycle_and_token_known_elapsed_gap"
    if not actual_known and cycle_known and not token_known:
        return "cycle_only_recovered"
    if issue_state.lower() != "closed" and actual_known and token_known:
        return "open_issue_with_local_metrics"
    if issue_state.lower() != "closed" and actual_known and not token_known:
        return "open_issue_local_timing_only"
    if issue_state.lower() != "closed" and not actual_known and token_known:
        return "open_issue_token_only"
    if issue_state.lower() != "closed":
        return "open_issue_sparse_metrics"
    return "other_gap_shape"


def title_from_body(body_path: Path) -> str:
    text = read_text(body_path)
    match = re.search(r'^title:\s*"([^"]+)"$', text, re.MULTILINE)
    if match:
        return match.group(1)
    for line in text.splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    return body_path.stem


def collect_rows(repo_root: Path, primary_root: Path, limit: int | None) -> list[dict[str, str]]:
    tasks_root = primary_root / ".adl" / "v0.91.6" / "tasks"
    rows: list[dict[str, str]] = []
    task_dirs = sorted(tasks_root.glob(TASK_DIR_PATTERN), key=lambda path: int(path.name.split("__", 1)[0].split("-")[1]))
    if limit is not None:
        task_dirs = task_dirs[:limit]
    total = len(task_dirs)
    for task_dir in task_dirs:
        if total >= 25 and len(rows) % 25 == 0:
            print(
                f"[build_v0916_workflow_metric_backfill_inventory] processing {len(rows)+1}/{total}",
                file=sys.stderr,
            )
        issue_number = task_dir.name.split("__", 1)[0].split("-", 1)[1]
        slug = task_dir.name.split("__", 1)[1]
        sor_path = task_dir / "sor.md"
        body_path = primary_root / ".adl" / "v0.91.6" / "bodies" / f"issue-{issue_number}-{slug}.md"
        issue_json = run(
            [
                "bash",
                "adl/tools/pr.sh",
                "issue",
                "view",
                issue_number,
                "--json",
            ],
            cwd=repo_root,
        )
        issue_record = json.loads(issue_json)
        sor_text = read_text(sor_path)
        metric_fields, exec_fields = parse_sor_fields(sor_text)
        actual_elapsed = actual_elapsed_metric(metric_fields, exec_fields)
        cycle_metric = github_cycle_metric(issue_record)
        token_value = token_metric(metric_fields, sor_text)
        row_confidence = min(
            [actual_elapsed.confidence, cycle_metric.confidence, token_value.confidence],
            key=lambda value: {"low": 0, "medium": 1, "high": 2}.get(value, 0),
        )
        completeness, missing_contract_fields = row_contract_completeness(
            issue_record,
            actual_elapsed,
            cycle_metric,
            token_value,
            row_confidence,
        )
        availability_class = metric_availability_class(
            str(issue_record.get("state", "unknown")),
            actual_elapsed,
            cycle_metric,
            token_value,
        )
        row = {
            "issue_number": issue_number,
            "slug": slug,
            "title": str(issue_record.get("title") or title_from_body(body_path)),
            "issue_state": str(issue_record.get("state", "unknown")),
            "issue_created_at": str(issue_record.get("createdAt") or "unknown"),
            "issue_closed_at": str(issue_record.get("closedAt") or "unknown"),
            "actual_session_elapsed_seconds": actual_elapsed.value,
            "actual_session_elapsed_status": actual_elapsed.status,
            "actual_session_elapsed_source": actual_elapsed.source,
            "github_cycle_time_seconds": cycle_metric.value,
            "github_cycle_time_status": cycle_metric.status,
            "github_cycle_time_source": cycle_metric.source,
            "actual_total_tokens": token_value.value,
            "actual_total_tokens_status": token_value.status,
            "actual_total_tokens_source": token_value.source,
            "row_contract_completeness": completeness,
            "row_contract_missing_fields": ",".join(missing_contract_fields) or "none",
            "metric_availability_class": availability_class,
            "full_metrics_known": "yes" if availability_class == "full_metrics_known" else "no",
            "row_confidence": row_confidence,
            "notes": " | ".join(
                [
                    f"actual_session_elapsed: {actual_elapsed.note}",
                    f"github_cycle_time: {cycle_metric.note}",
                    f"actual_total_tokens: {token_value.note}",
                ]
            ),
            "source_prompt_relpath": str(body_path.relative_to(primary_root)),
            "sor_relpath": str(sor_path.relative_to(primary_root)),
        }
        rows.append(row)
    return rows


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = list(rows[0].keys()) if rows else []
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def summarize(rows: Iterable[dict[str, str]]) -> dict[str, int]:
    rows = list(rows)
    return {
        "surveyed_issue_count": len(rows),
        "closed_issue_count": sum(1 for row in rows if row["issue_state"].lower() == "closed"),
        "open_issue_count": sum(1 for row in rows if row["issue_state"].lower() != "closed"),
        "actual_elapsed_explicit_count": sum(1 for row in rows if row["actual_session_elapsed_status"] == "explicit"),
        "actual_elapsed_derived_count": sum(1 for row in rows if row["actual_session_elapsed_status"] == "derived"),
        "actual_elapsed_unknown_count": sum(1 for row in rows if row["actual_session_elapsed_status"] == "unknown"),
        "github_cycle_known_count": sum(1 for row in rows if row["github_cycle_time_status"] == "derived"),
        "token_explicit_count": sum(1 for row in rows if row["actual_total_tokens_status"] == "explicit"),
        "token_unknown_count": sum(1 for row in rows if row["actual_total_tokens_status"] == "unknown"),
        "token_not_collected_count": sum(1 for row in rows if row["actual_total_tokens_status"] == "not_collected"),
        "row_contract_complete_count": sum(1 for row in rows if row["row_contract_completeness"] == "complete"),
        "row_contract_partial_count": sum(1 for row in rows if row["row_contract_completeness"] == "partial"),
        "row_contract_incomplete_count": sum(1 for row in rows if row["row_contract_completeness"] == "incomplete"),
        "full_metrics_known_count": sum(1 for row in rows if row["metric_availability_class"] == "full_metrics_known"),
        "timing_recovered_token_gap_count": sum(1 for row in rows if row["metric_availability_class"] == "timing_recovered_token_gap"),
        "cycle_only_recovered_count": sum(1 for row in rows if row["metric_availability_class"] == "cycle_only_recovered"),
        "open_issue_local_timing_only_count": sum(1 for row in rows if row["metric_availability_class"] == "open_issue_local_timing_only"),
        "open_issue_sparse_metrics_count": sum(1 for row in rows if row["metric_availability_class"] == "open_issue_sparse_metrics"),
    }


def write_json(path: Path, rows: list[dict[str, str]], summary: dict[str, int]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema": "adl.v0916.workflow_metric_backfill.v1",
        "survey_definition": {
            "scope": "primary checkout .adl/v0.91.6/tasks/issue-* task bundles only",
            "forward_capture_issue": 4431,
            "historical_backfill_issue": 4441,
        },
        "summary": summary,
        "row_contract_complete_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["row_contract_completeness"] == "complete"
        ],
        "row_contract_partial_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["row_contract_completeness"] == "partial"
        ],
        "row_contract_incomplete_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["row_contract_completeness"] == "incomplete"
        ],
        "full_metrics_known_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["metric_availability_class"] == "full_metrics_known"
        ],
        "timing_recovered_token_gap_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["metric_availability_class"] == "timing_recovered_token_gap"
        ],
        "cycle_only_recovered_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["metric_availability_class"] == "cycle_only_recovered"
        ],
        "open_issue_local_timing_only_issue_numbers": [
            int(row["issue_number"])
            for row in rows
            if row["metric_availability_class"] == "open_issue_local_timing_only"
        ],
    }
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def write_markdown(
    path: Path,
    rows: list[dict[str, str]],
    summary: dict[str, int],
    repo_root: Path,
    primary_root: Path,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    sample_rows = [row for row in rows if row["issue_number"] in {"4431", "4479", "4393"}]
    contract_complete_issues = [row["issue_number"] for row in rows if row["row_contract_completeness"] == "complete"]
    contract_partial_issues = [row["issue_number"] for row in rows if row["row_contract_completeness"] == "partial"]
    contract_incomplete_issues = [row["issue_number"] for row in rows if row["row_contract_completeness"] == "incomplete"]
    full_metrics_known_issues = [row["issue_number"] for row in rows if row["metric_availability_class"] == "full_metrics_known"]
    timing_recovered_token_gap_issues = [row["issue_number"] for row in rows if row["metric_availability_class"] == "timing_recovered_token_gap"]
    cycle_only_recovered_issues = [row["issue_number"] for row in rows if row["metric_availability_class"] == "cycle_only_recovered"]
    open_issue_local_timing_only_issues = [row["issue_number"] for row in rows if row["metric_availability_class"] == "open_issue_local_timing_only"]
    lines = [
        "# V0.91.6 Workflow Metric Backfill",
        "",
        "Issue: `#4441`",
        "",
        "## Summary",
        "",
        "This artifact is the bounded historical workflow-metrics backfill for `v0.91.6` only.",
        "It stays separate from `#4431`, which owns forward authoritative capture.",
        "",
        "## Survey Definition",
        "",
        "- Surveyed set: issue-local task bundles present under the primary checkout local corpus `/.adl/v0.91.6/tasks/issue-*`.",
        "- Exclusions: sprint umbrellas, sprint review packets, and non-issue artifacts outside the issue-task corpus.",
        "- Issue metadata source: repo-native `adl/tools/pr.sh issue view <issue> --json`.",
        "- Issue-local metrics source: `sor.md` Issue Metrics Truth section, with fallback to SOR execution start/end timestamps for elapsed-time derivation when explicit elapsed seconds are absent.",
        "- Missing-data rule: values remain `unknown` or `not_collected`; they are never inferred from diff size, elapsed chat time, or subjective effort.",
        "",
        "## Refresh Command",
        "",
        "```bash",
        "python3 adl/tools/build_v0916_workflow_metric_backfill_inventory.py",
        "```",
        "",
        "## Aggregate Counts",
        "",
        f"- Surveyed issues: `{summary['surveyed_issue_count']}`",
        f"- Closed issues: `{summary['closed_issue_count']}`",
        f"- Open issues: `{summary['open_issue_count']}`",
        f"- Actual session elapsed explicit: `{summary['actual_elapsed_explicit_count']}`",
        f"- Actual session elapsed derived from SOR execution window: `{summary['actual_elapsed_derived_count']}`",
        f"- Actual session elapsed unknown: `{summary['actual_elapsed_unknown_count']}`",
        f"- GitHub cycle time reconstructed from created/closed timestamps: `{summary['github_cycle_known_count']}`",
        f"- Actual total tokens explicit: `{summary['token_explicit_count']}`",
        f"- Actual total tokens unknown: `{summary['token_unknown_count']}`",
        f"- Actual total tokens not_collected: `{summary['token_not_collected_count']}`",
        f"- Row-contract complete rows: `{summary['row_contract_complete_count']}`",
        f"- Row-contract partial rows: `{summary['row_contract_partial_count']}`",
        f"- Row-contract incomplete rows: `{summary['row_contract_incomplete_count']}`",
        f"- Full metrics known rows: `{summary['full_metrics_known_count']}`",
        f"- Timing recovered but token gap rows: `{summary['timing_recovered_token_gap_count']}`",
        f"- Cycle-only recovered rows: `{summary['cycle_only_recovered_count']}`",
        f"- Open-issue local-timing-only rows: `{summary['open_issue_local_timing_only_count']}`",
        f"- Open-issue sparse-metrics rows: `{summary['open_issue_sparse_metrics_count']}`",
        "",
        "## Output Files",
        "",
        f"- CSV inventory: `{CSV_RELATIVE_PATH}`",
        f"- JSON issue-group summary: `{JSON_RELATIVE_PATH}`",
        f"- Review note: `{MARKDOWN_RELATIVE_PATH}`",
        "",
        "## Row-Contract Groups",
        "",
        f"- Row-contract complete rows: `{', '.join(f'#{issue}' for issue in contract_complete_issues) if contract_complete_issues else 'none'}`",
        f"- Row-contract partial rows: `{', '.join(f'#{issue}' for issue in contract_partial_issues) if contract_partial_issues else 'none'}`",
        f"- Row-contract incomplete rows: `{', '.join(f'#{issue}' for issue in contract_incomplete_issues) if contract_incomplete_issues else 'none'}`",
        "",
        "## Metric Availability Groups",
        "",
        f"- Full metrics known rows: `{', '.join(f'#{issue}' for issue in full_metrics_known_issues) if full_metrics_known_issues else 'none'}`",
        f"- Timing recovered but token gap rows: `{', '.join(f'#{issue}' for issue in timing_recovered_token_gap_issues) if timing_recovered_token_gap_issues else 'none'}`",
        f"- Cycle-only recovered rows: `{', '.join(f'#{issue}' for issue in cycle_only_recovered_issues) if cycle_only_recovered_issues else 'none'}`",
        f"- Open-issue local-timing-only rows: `{', '.join(f'#{issue}' for issue in open_issue_local_timing_only_issues) if open_issue_local_timing_only_issues else 'none'}`",
        "",
        "## Spot-Check Rows",
        "",
        "| Issue | Row Contract | Metrics Known | Actual Session Elapsed | GitHub Cycle Time | Actual Total Tokens | Confidence |",
        "| --- | --- | --- | --- | --- | --- | --- |",
    ]
    for row in sample_rows:
        lines.append(
            f"| #{row['issue_number']} | {row['row_contract_completeness']} | {row['metric_availability_class']} | {row['actual_session_elapsed_seconds']} ({row['actual_session_elapsed_status']}) | "
            f"{row['github_cycle_time_seconds']} ({row['github_cycle_time_status']}) | "
            f"{row['actual_total_tokens']} ({row['actual_total_tokens_status']}) | {row['row_confidence']} |"
        )
    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- `actual_session_elapsed_seconds` records issue-local execution time when explicit SOR metrics exist, otherwise a derived execution-window value when SOR start/end timestamps exist.",
            "- `github_cycle_time_seconds` records reconstructed GitHub issue calendar duration only when repo-native `createdAt` and `closedAt` are both available.",
            "- `actual_total_tokens` stays explicit only when issue-local evidence recorded it truthfully.",
            "",
            "_Generated from the bound issue worktree using the primary checkout local-state corpus as the survey root._",
            "",
        ]
    )
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=None)
    args = parser.parse_args()
    repo_root = Path(run(["git", "rev-parse", "--show-toplevel"], cwd=Path.cwd()).strip())
    primary_root = resolve_primary_checkout_root(repo_root)
    rows = collect_rows(repo_root, primary_root, args.limit)
    if not rows:
        raise SystemExit("No v0.91.6 issue task bundles found in the primary checkout local corpus.")
    csv_path = repo_root / CSV_RELATIVE_PATH
    json_path = repo_root / JSON_RELATIVE_PATH
    markdown_path = repo_root / MARKDOWN_RELATIVE_PATH
    write_csv(csv_path, rows)
    summary = summarize(rows)
    write_json(json_path, rows, summary)
    write_markdown(markdown_path, rows, summary, repo_root, primary_root)
    print(
        json.dumps(
            {
                "csv": str(CSV_RELATIVE_PATH),
                "json": str(JSON_RELATIVE_PATH),
                "markdown": str(MARKDOWN_RELATIVE_PATH),
                "summary": summary,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
