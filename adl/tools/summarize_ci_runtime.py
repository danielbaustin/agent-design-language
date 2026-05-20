#!/usr/bin/env python3
"""Summarize GitHub Actions job timing JSON into CI runtime budget reports."""

from __future__ import annotations

import argparse
import json
from dataclasses import asdict, dataclass
from datetime import datetime
from pathlib import Path
from typing import Any


DEFAULT_JOB_BUDGETS = {
    "adl-ci": 600.0,
    "adl-coverage": 900.0,
}

DEFAULT_CATEGORY_BUDGETS = {
    "lane-selection": 60.0,
    "setup-install-cache": 240.0,
    "tooling-contracts": 240.0,
    "rust-test-execution": 600.0,
    "coverage-execution": 900.0,
    "reporting-upload": 120.0,
    "skipped-policy": 60.0,
    "other": 180.0,
}


@dataclass(frozen=True)
class StepTiming:
    job: str
    name: str
    category: str
    seconds: float
    conclusion: str
    over_budget: bool
    budget_seconds: float


@dataclass(frozen=True)
class JobTiming:
    name: str
    seconds: float
    conclusion: str
    over_budget: bool
    budget_seconds: float
    top_category: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Summarize GitHub Actions jobs JSON into runtime budget and hotspot buckets."
    )
    parser.add_argument(
        "jobs_json",
        type=Path,
        help="Path to JSON from `gh run view <run> --json jobs` or an equivalent jobs array.",
    )
    parser.add_argument(
        "--job-budget",
        action="append",
        default=[],
        metavar="JOB=SECONDS",
        help="Override a job budget, for example `adl-ci=600`.",
    )
    parser.add_argument(
        "--category-budget",
        action="append",
        default=[],
        metavar="CATEGORY=SECONDS",
        help="Override a category budget, for example `coverage-execution=900`.",
    )
    parser.add_argument(
        "--format",
        choices=("markdown", "json"),
        default="markdown",
        help="Output format.",
    )
    return parser.parse_args()


def parse_budget_overrides(values: list[str]) -> dict[str, float]:
    overrides: dict[str, float] = {}
    for value in values:
        if "=" not in value:
            raise SystemExit(f"invalid budget override {value!r}; expected NAME=SECONDS")
        name, raw_seconds = value.split("=", 1)
        name = name.strip()
        if not name:
            raise SystemExit(f"invalid budget override {value!r}; missing name")
        try:
            seconds = float(raw_seconds)
        except ValueError as exc:
            raise SystemExit(f"invalid budget override {value!r}; seconds must be numeric") from exc
        if seconds < 0:
            raise SystemExit(f"invalid budget override {value!r}; seconds must be non-negative")
        overrides[name] = seconds
    return overrides


def parse_time(value: str | None) -> datetime | None:
    if not value:
        return None
    normalized = value.replace("Z", "+00:00")
    return datetime.fromisoformat(normalized)


def elapsed_seconds(started_at: str | None, completed_at: str | None) -> float:
    start = parse_time(started_at)
    end = parse_time(completed_at)
    if start is None or end is None:
        return 0.0
    return max((end - start).total_seconds(), 0.0)


def load_jobs(path: Path) -> list[dict[str, Any]]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if isinstance(data, list):
        return [job for job in data if isinstance(job, dict)]
    if isinstance(data, dict) and isinstance(data.get("jobs"), list):
        return [job for job in data["jobs"] if isinstance(job, dict)]
    raise SystemExit("input must be a jobs array or an object with a jobs array")


def category_for_step(name: str) -> str:
    lower = name.lower()
    if "classify changed paths" in lower or "path-policy" in lower or "coverage-impact" in lower:
        return "lane-selection"
    if "install" in lower or "cache" in lower or "configure rust acceleration" in lower:
        return "setup-install-cache"
    if "tooling sanity" in lower or "contract" in lower or "guardrail" in lower or "docs command" in lower:
        return "tooling-contracts"
    if lower in {"fmt", "clippy", "test", "doc test"} or "nextest" in lower:
        return "rust-test-execution"
    if "skipped" in lower or "deferred" in lower or "covered by" in lower or "replacement" in lower:
        return "skipped-policy"
    if "coverage" in lower or "llvm-cov" in lower or "lcov" in lower or "codecov" in lower:
        return "coverage-execution"
    if "upload" in lower or "summary" in lower or "stats" in lower or "verify generated" in lower:
        return "reporting-upload"
    return "other"


def budget_for(name: str, budgets: dict[str, float], default: float) -> float:
    return budgets.get(name, default)


def summarize(
    jobs: list[dict[str, Any]],
    job_budgets: dict[str, float],
    category_budgets: dict[str, float],
) -> dict[str, Any]:
    job_rows: list[JobTiming] = []
    step_rows: list[StepTiming] = []
    category_totals: dict[str, float] = {}

    for job in jobs:
        job_name = str(job.get("name") or "unknown")
        job_seconds = elapsed_seconds(job.get("startedAt"), job.get("completedAt"))
        job_conclusion = str(job.get("conclusion") or job.get("status") or "unknown")
        job_category_totals: dict[str, float] = {}

        for step in job.get("steps") or []:
            if not isinstance(step, dict):
                continue
            step_name = str(step.get("name") or "unknown")
            seconds = elapsed_seconds(step.get("startedAt"), step.get("completedAt"))
            category = category_for_step(step_name)
            category_totals[category] = category_totals.get(category, 0.0) + seconds
            job_category_totals[category] = job_category_totals.get(category, 0.0) + seconds
            budget = budget_for(category, category_budgets, DEFAULT_CATEGORY_BUDGETS["other"])
            step_rows.append(
                StepTiming(
                    job=job_name,
                    name=step_name,
                    category=category,
                    seconds=seconds,
                    conclusion=str(step.get("conclusion") or step.get("status") or "unknown"),
                    over_budget=seconds > budget,
                    budget_seconds=budget,
                )
            )

        top_category = "none"
        if job_category_totals:
            top_category = max(job_category_totals.items(), key=lambda item: item[1])[0]
        job_budget = budget_for(job_name, job_budgets, 600.0)
        job_rows.append(
            JobTiming(
                name=job_name,
                seconds=job_seconds,
                conclusion=job_conclusion,
                over_budget=job_seconds > job_budget,
                budget_seconds=job_budget,
                top_category=top_category,
            )
        )

    return {
        "jobs": [asdict(row) for row in sorted(job_rows, key=lambda row: row.seconds, reverse=True)],
        "steps": [asdict(row) for row in sorted(step_rows, key=lambda row: row.seconds, reverse=True)],
        "categories": [
            {"category": category, "seconds": seconds, "budget_seconds": category_budgets.get(category)}
            for category, seconds in sorted(category_totals.items(), key=lambda item: item[1], reverse=True)
        ],
    }


def markdown_report(summary: dict[str, Any]) -> str:
    lines = [
        "# CI Runtime Budget Report",
        "",
        "## Job Budgets",
        "",
        "| Job | Seconds | Budget | Status | Dominant Bucket | Conclusion |",
        "| --- | ---: | ---: | --- | --- | --- |",
    ]
    for row in summary["jobs"]:
        status = "over_budget" if row["over_budget"] else "within_budget"
        lines.append(
            f"| `{row['name']}` | {row['seconds']:.1f} | {row['budget_seconds']:.1f} | "
            f"{status} | `{row['top_category']}` | `{row['conclusion']}` |"
        )

    lines.extend(["", "## Runtime Buckets", "", "| Bucket | Seconds |", "| --- | ---: |"])
    for row in summary["categories"]:
        lines.append(f"| `{row['category']}` | {row['seconds']:.1f} |")

    lines.extend(["", "## Slowest Steps", "", "| Job | Step | Bucket | Seconds | Budget | Status |", "| --- | --- | --- | ---: | ---: | --- |"])
    for row in summary["steps"][:12]:
        status = "over_budget" if row["over_budget"] else "within_budget"
        lines.append(
            f"| `{row['job']}` | `{row['name']}` | `{row['category']}` | "
            f"{row['seconds']:.1f} | {row['budget_seconds']:.1f} | {status} |"
        )

    lines.extend(
        [
            "",
            "## Routing Guidance",
            "",
            "- `lane-selection`: inspect `ci_path_policy` and coverage-impact classification.",
            "- `setup-install-cache`: inspect toolchain, cache, linker, or runner setup.",
            "- `rust-test-execution`: inspect test selection, slow-proof placement, or nextest hotspots.",
            "- `coverage-execution`: inspect coverage lane policy and changed-source coverage scope.",
            "- `tooling-contracts`: inspect shell/Python contract tests or guardrail scripts.",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    job_budgets = {**DEFAULT_JOB_BUDGETS, **parse_budget_overrides(args.job_budget)}
    category_budgets = {**DEFAULT_CATEGORY_BUDGETS, **parse_budget_overrides(args.category_budget)}
    summary = summarize(load_jobs(args.jobs_json), job_budgets, category_budgets)
    if args.format == "json":
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        print(markdown_report(summary), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
