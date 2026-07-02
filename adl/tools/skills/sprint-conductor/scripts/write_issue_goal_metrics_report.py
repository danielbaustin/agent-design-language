#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


METRIC_FIELDS = (
    ("elapsed_seconds", "elapsed_availability"),
    ("active_work_seconds", "active_work_availability"),
    ("validation_seconds", "validation_availability"),
    ("pr_wait_seconds", "pr_wait_availability"),
    ("ci_wait_seconds", "ci_wait_availability"),
)

FULL_PREDICTION_FIELDS = (
    "elapsed_seconds",
    "total_tokens",
    "validation_seconds",
    "pr_wait_seconds",
    "ci_wait_seconds",
)


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def availability(summary: dict[str, Any], field: str, availability_field: str) -> str:
    explicit = summary.get(availability_field)
    if explicit:
        return str(explicit)
    return "known" if summary.get(field) is not None else "unknown"


def prediction_features(summary: dict[str, Any]) -> dict[str, Any]:
    token_usage = summary.get("token_usage") or {}
    feature_packet: dict[str, Any] = {
        "schema_version": "adl.issue_goal_metrics.reporting_prediction.v1",
        "status": summary.get("status") or "unknown",
        "issue_goal_ref": summary.get("issue_goal_ref"),
        "sprint_goal_ref": summary.get("sprint_goal_ref"),
        "selected_stage": summary.get("selected_stage"),
        "data_source": summary.get("data_source") or "unknown",
        "metrics_confidence": summary.get("metrics_confidence") or "unknown",
        "features": {},
        "feature_availability": {},
        "missing_prediction_features": [],
        "known_prediction_feature_count": 0,
        "unknown_values_policy": "unknown_is_not_zero",
    }

    for field, availability_field in METRIC_FIELDS:
        field_availability = availability(summary, field, availability_field)
        feature_packet["features"][field] = summary.get(field)
        feature_packet["feature_availability"][field] = field_availability
        if field_availability == "known":
            feature_packet["known_prediction_feature_count"] += 1
        else:
            feature_packet["missing_prediction_features"].append(field)

    total_availability = token_usage.get("total_availability") or token_usage.get("availability") or "unknown"
    feature_packet["features"]["total_tokens"] = token_usage.get("total_tokens")
    feature_packet["feature_availability"]["total_tokens"] = total_availability
    if total_availability == "known":
        feature_packet["known_prediction_feature_count"] += 1
    else:
        feature_packet["missing_prediction_features"].append("total_tokens")

    feature_packet["reporting_ready"] = (
        feature_packet["status"] == "recorded"
        and feature_packet["feature_availability"].get("elapsed_seconds") == "known"
        and feature_packet["feature_availability"].get("total_tokens") == "known"
    )
    feature_packet["minimal_prediction_ready"] = feature_packet["reporting_ready"]
    feature_packet["full_prediction_ready"] = all(
        feature_packet["feature_availability"].get(field) == "known" for field in FULL_PREDICTION_FIELDS
    )
    feature_packet["prediction_readiness"] = (
        "full" if feature_packet["full_prediction_ready"]
        else "minimal" if feature_packet["minimal_prediction_ready"]
        else "not_ready"
    )
    # Backward-compatible alias for existing consumers. New callers should use
    # minimal_prediction_ready/full_prediction_ready to avoid overstating proof.
    feature_packet["prediction_ready"] = feature_packet["minimal_prediction_ready"]
    return feature_packet


def render_markdown(summary: dict[str, Any], packet: dict[str, Any]) -> str:
    lines = [
        "# Issue Goal Metrics Reporting Sample",
        "",
        f"- status: `{packet['status']}`",
        f"- issue goal ref: `{packet.get('issue_goal_ref') or 'unknown'}`",
        f"- sprint goal ref: `{packet.get('sprint_goal_ref') or 'unknown'}`",
        f"- selected stage: `{packet.get('selected_stage') or 'unknown'}`",
        f"- data source: `{packet.get('data_source') or 'unknown'}`",
        f"- metrics confidence: `{packet.get('metrics_confidence') or 'unknown'}`",
        f"- reporting ready: `{packet['reporting_ready']}`",
        f"- minimal prediction ready: `{packet['minimal_prediction_ready']}`",
        f"- full prediction ready: `{packet['full_prediction_ready']}`",
        f"- prediction readiness: `{packet['prediction_readiness']}`",
        f"- unknown values policy: `{packet['unknown_values_policy']}`",
        "",
        "## Features",
        "",
    ]
    for field in [
        "elapsed_seconds",
        "active_work_seconds",
        "validation_seconds",
        "pr_wait_seconds",
        "ci_wait_seconds",
        "total_tokens",
    ]:
        value = packet["features"].get(field)
        availability_value = packet["feature_availability"].get(field, "unknown")
        rendered_value = value if value is not None else availability_value
        lines.append(f"- {field}: `{rendered_value}` availability=`{availability_value}`")

    missing = packet["missing_prediction_features"]
    if missing:
        lines.extend(["", "## Missing Prediction Features", ""])
        lines.extend(f"- `{field}`" for field in missing)
    else:
        lines.extend(["", "## Missing Prediction Features", "", "- none"])

    cumulative = summary.get("cumulative_metrics") or {}
    token_cumulative = cumulative.get("token_usage") or {}
    lines.extend(
        [
            "",
            "## Cumulative Rollup",
            "",
            f"- goal instance count: `{cumulative.get('goal_instance_count', 'unknown')}`",
            f"- elapsed known sum: `{cumulative.get('elapsed_seconds_known_sum', 'unknown')}`",
            f"- active work known sum: `{cumulative.get('active_work_seconds_known_sum', 'unknown')}`",
            f"- validation known sum: `{cumulative.get('validation_seconds_known_sum', 'unknown')}`",
            f"- pr wait known sum: `{cumulative.get('pr_wait_seconds_known_sum', 'unknown')}`",
            f"- ci wait known sum: `{cumulative.get('ci_wait_seconds_known_sum', 'unknown')}`",
            f"- total tokens known sum: `{token_cumulative.get('total_tokens_known_sum', 'unknown')}`",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Render a redacted issue goal-metrics report and prediction-feature packet from a summary artifact."
    )
    parser.add_argument("--summary", required=True)
    parser.add_argument("--report-out", required=True)
    parser.add_argument("--prediction-out", required=True)
    parser.add_argument("--print-json", action="store_true")
    args = parser.parse_args()

    summary_path = Path(args.summary)
    report_path = Path(args.report_out)
    prediction_path = Path(args.prediction_out)
    summary = load_json(summary_path)
    packet = prediction_features(summary)

    report_path.parent.mkdir(parents=True, exist_ok=True)
    prediction_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(render_markdown(summary, packet))
    prediction_path.write_text(json.dumps(packet, indent=2, sort_keys=True) + "\n")

    result = {
        "status": "written",
        "report_path": str(report_path),
        "prediction_path": str(prediction_path),
        "reporting_ready": packet["reporting_ready"],
        "prediction_ready": packet["prediction_ready"],
        "minimal_prediction_ready": packet["minimal_prediction_ready"],
        "full_prediction_ready": packet["full_prediction_ready"],
        "prediction_readiness": packet["prediction_readiness"],
        "known_prediction_feature_count": packet["known_prediction_feature_count"],
        "missing_prediction_features": packet["missing_prediction_features"],
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(str(report_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
