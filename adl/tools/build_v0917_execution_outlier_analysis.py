#!/usr/bin/env python3
"""Build the v0.91.7 WP-04 execution outlier analysis from metrics backfill."""

from __future__ import annotations

import argparse
import csv
import json
import statistics
from pathlib import Path
from typing import Iterable


DEFAULT_INPUT = Path("docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_INVENTORY_4441.csv")
DEFAULT_JSON = Path("docs/milestones/v0.91.7/review/V0917_WP04_EXECUTION_OUTLIER_ANALYSIS_4670.json")
DEFAULT_MD = Path("docs/milestones/v0.91.7/review/V0917_WP04_EXECUTION_OUTLIER_ANALYSIS_4670.md")
SCHEMA = "adl.v0917.execution_outlier_analysis.v1"


def parse_int(value: str) -> tuple[int | None, str]:
    if not value or value in {"unknown", "not_collected", "not_applicable"}:
        return None, "unknown"
    try:
        parsed = int(value)
    except ValueError:
        return None, "invalid"
    if parsed < 0:
        return None, "invalid"
    return parsed, "known"


def percentile_nearest_rank(values: list[int], percentile: float) -> int | None:
    if not values:
        return None
    ordered = sorted(values)
    rank = max(1, int((percentile / 100.0) * len(ordered) + 0.999999))
    return ordered[min(rank, len(ordered)) - 1]


def metric_summary(rows: list[dict[str, str]], field: str, status_field: str) -> dict[str, object]:
    parsed = [parse_int(row[field]) for row in rows]
    values = [value for value, _ in parsed]
    known = [value for value in values if value is not None]
    invalid_rows = []
    for row, (value, parse_status) in zip(rows, parsed, strict=True):
        if parse_status == "invalid":
            invalid_rows.append(
                {
                    "issue_number": int(row["issue_number"]),
                    "title": row["title"],
                    "raw_value": row[field],
                    "status": row[status_field],
                    "metric_availability_class": row["metric_availability_class"],
                    "row_confidence": row["row_confidence"],
                }
            )
    threshold = percentile_nearest_rank(known, 95)
    outlier_rows = []
    for row, value in zip(rows, values, strict=True):
        if value is not None and threshold is not None and value >= threshold:
            outlier_rows.append(
                {
                    "issue_number": int(row["issue_number"]),
                    "title": row["title"],
                    "value": value,
                    "status": row[status_field],
                    "metric_availability_class": row["metric_availability_class"],
                    "row_confidence": row["row_confidence"],
                }
            )
    outlier_rows.sort(key=lambda row: (-int(row["value"]), int(row["issue_number"])))
    return {
        "known_count": len(known),
        "unknown_count": sum(1 for _, parse_status in parsed if parse_status == "unknown"),
        "invalid_count": len(invalid_rows),
        "min": min(known) if known else None,
        "median": int(statistics.median(known)) if known else None,
        "p90": percentile_nearest_rank(known, 90),
        "p95": threshold,
        "max": max(known) if known else None,
        "outlier_count": len(outlier_rows),
        "top_outliers": outlier_rows[:15],
        "invalid_rows": invalid_rows,
    }


def count_by(rows: Iterable[dict[str, str]], field: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in rows:
        counts[row[field]] = counts.get(row[field], 0) + 1
    return dict(sorted(counts.items()))


def build_payload(rows: list[dict[str, str]], input_path: Path) -> dict[str, object]:
    return {
        "schema": SCHEMA,
        "issue": 4670,
        "source": {
            "input_csv": str(input_path),
            "backfill_issue": 4441,
            "consumption_issue": 4669,
        },
        "summary": {
            "surveyed_issue_count": len(rows),
            "closed_issue_count": sum(1 for row in rows if row["issue_state"].lower() == "closed"),
            "row_contract_counts": count_by(rows, "row_contract_completeness"),
            "metric_availability_counts": count_by(rows, "metric_availability_class"),
            "row_confidence_counts": count_by(rows, "row_confidence"),
        },
        "metrics": {
            "actual_session_elapsed_seconds": metric_summary(
                rows,
                "actual_session_elapsed_seconds",
                "actual_session_elapsed_status",
            ),
            "github_cycle_time_seconds": metric_summary(
                rows,
                "github_cycle_time_seconds",
                "github_cycle_time_status",
            ),
            "actual_total_tokens": metric_summary(
                rows,
                "actual_total_tokens",
                "actual_total_tokens_status",
            ),
        },
        "non_claims": [
            "Unknown values are excluded from percentile and outlier thresholds, not treated as zero.",
            "Invalid numeric values are excluded from thresholds and counted separately from unknown values.",
            "Historical token totals remain sparse; token outliers cover only rows with explicit token evidence.",
            "This analysis is descriptive over the retained backfill artifact and is not a predictive model.",
            "The report reflects the input CSV at runtime.",
        ],
    }


def write_markdown(path: Path, payload: dict[str, object]) -> None:
    summary = payload["summary"]  # type: ignore[index]
    metrics = payload["metrics"]  # type: ignore[index]
    source = payload["source"]  # type: ignore[index]
    lines = [
        "# V0.91.7 WP-04 Execution Outlier Analysis",
        "",
        "Issue: `#4670`",
        "",
        "## Summary",
        "",
        "This packet is the deterministic execution-outlier analysis for WP-04.",
        "It consumes the bounded v0.91.6 workflow metric backfill and keeps",
        "`unknown` values out of numeric thresholds instead of treating them as zero.",
        "",
        "## Source",
        "",
        f"- Input CSV: `{source['input_csv']}`",
        f"- Historical backfill issue: `#{source['backfill_issue']}`",
        f"- v0.91.7 consumption issue: `#{source['consumption_issue']}`",
        "- Baseline freshness: this report reflects the input CSV at runtime.",
        "",
        "## Coverage",
        "",
        f"- Surveyed issues: `{summary['surveyed_issue_count']}`",
        f"- Closed issues: `{summary['closed_issue_count']}`",
        f"- Row contract counts: `{json.dumps(summary['row_contract_counts'], sort_keys=True)}`",
        f"- Metric availability counts: `{json.dumps(summary['metric_availability_counts'], sort_keys=True)}`",
        f"- Row confidence counts: `{json.dumps(summary['row_confidence_counts'], sort_keys=True)}`",
        "",
        "## Metric Thresholds",
        "",
        "| Metric | Known | Unknown | Invalid | Median | P90 | P95 | Max | Outliers |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for metric_name, metric in metrics.items():  # type: ignore[union-attr]
        lines.append(
            f"| `{metric_name}` | {metric['known_count']} | {metric['unknown_count']} | {metric['invalid_count']} | "
            f"{metric['median']} | {metric['p90']} | {metric['p95']} | {metric['max']} | {metric['outlier_count']} |"
        )
    lines.extend(["", "## Top Outliers", ""])
    for metric_name, metric in metrics.items():  # type: ignore[union-attr]
        lines.append(f"### `{metric_name}`")
        lines.append("")
        lines.append("| Issue | Value | Status | Confidence | Title |")
        lines.append("| --- | ---: | --- | --- | --- |")
        for row in metric["top_outliers"]:
            title = str(row["title"]).replace("|", "\\|")
            lines.append(
                f"| `#{row['issue_number']}` | {row['value']} | {row['status']} | {row['row_confidence']} | {title} |"
            )
        if not metric["top_outliers"]:
            lines.append("| none | 0 | not_applicable | not_applicable | no known values |")
        lines.append("")
    lines.extend(
        [
            "## Non-Claims",
            "",
            "- Unknown values are excluded from percentile and outlier thresholds, not treated as zero.",
            "- Invalid numeric values are excluded from thresholds and counted separately from unknown values.",
            "- Historical token totals remain sparse; token outliers cover only rows with explicit token evidence.",
            "- This analysis is descriptive over the retained backfill artifact and is not a predictive model.",
            "- The report reflects the input CSV at runtime.",
            "",
            "## v0.91.7 Consumption",
            "",
            "- WP-04 closeout should report timing and token outliers separately.",
            "- Future validation-manager work should compare forward v0.91.7 issue metrics against these descriptive baselines.",
            "- Token outlier analysis should remain explicitly incomplete until forward capture substantially reduces the historical token gap.",
            "",
        ]
    )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, default=DEFAULT_INPUT)
    parser.add_argument("--json-out", type=Path, default=DEFAULT_JSON)
    parser.add_argument("--md-out", type=Path, default=DEFAULT_MD)
    args = parser.parse_args()

    rows = list(csv.DictReader(args.input.open(encoding="utf-8", newline="")))
    if not rows:
        raise SystemExit(f"No rows found in {args.input}")
    payload = build_payload(rows, args.input)
    args.json_out.parent.mkdir(parents=True, exist_ok=True)
    args.json_out.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    write_markdown(args.md_out, payload)
    print(
        json.dumps(
            {
                "json": str(args.json_out),
                "markdown": str(args.md_out),
                "summary": payload["summary"],
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
