#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from statistics import median
from typing import Any


INPUT_SCHEMA = "adl.issue_goal_metrics.reporting_prediction.v1"
OUTPUT_SCHEMA = "adl.issue_goal_metrics.execution_prediction.v1"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def number_or_none(value: Any) -> float | None:
    if isinstance(value, bool) or value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        try:
            return float(value)
        except ValueError:
            return None
    return None


def availability(packet: dict[str, Any], field: str) -> str:
    feature_availability = packet.get("feature_availability")
    if isinstance(feature_availability, dict):
        value = feature_availability.get(field)
        if value:
            return str(value)
    features = packet.get("features")
    if isinstance(features, dict) and features.get(field) is not None:
        return "known"
    return "unknown"


def feature(packet: dict[str, Any], field: str) -> float | None:
    features = packet.get("features")
    if not isinstance(features, dict):
        return None
    if availability(packet, field) != "known":
        return None
    return number_or_none(features.get(field))


def risk_from_seconds(seconds: float | None, *, medium: float, high: float, missing: str) -> str:
    if seconds is None:
        return missing
    if seconds >= high:
        return "high"
    if seconds >= medium:
        return "medium"
    return "low"


def risk_rank(value: str) -> int:
    return {"low": 1, "medium": 2, "high": 3}.get(value, 0)


def max_risk(values: list[str]) -> str:
    ranked = sorted(values, key=risk_rank)
    return ranked[-1] if ranked else "unknown"


def aggregate_known(packets: list[dict[str, Any]], field: str) -> float | None:
    values = [value for packet in packets if (value := feature(packet, field)) is not None]
    if not values:
        return None
    return float(median(values))


def display_path(path: Path, *, root: Path | None = None) -> str:
    base = (root or Path.cwd()).resolve()
    resolved = path.expanduser().resolve()
    try:
        return str(resolved.relative_to(base))
    except ValueError:
        if not path.is_absolute():
            return str(path)
        return f"<external:{path.name}>"


def source_refs(paths: list[Path], *, root: Path | None = None) -> list[str]:
    return [display_path(path, root=root) for path in paths]


def require_consistent_identity(packets: list[dict[str, Any]], field: str) -> str:
    values = {
        str(packet.get(field) or "unknown")
        for packet in packets
        if str(packet.get(field) or "unknown") != "unknown"
    }
    if len(values) > 1:
        joined = ", ".join(sorted(values))
        raise ValueError(f"mixed {field} values are not safe to aggregate: {joined}")
    return next(iter(values)) if values else "unknown"


def actual_numeric(summary: dict[str, Any], field: str) -> float | None:
    if field == "total_tokens":
        token_usage = summary.get("token_usage")
        if isinstance(token_usage, dict):
            return number_or_none(token_usage.get("total_tokens"))
        return None
    return number_or_none(summary.get(field))


def compare_actuals(predictions: dict[str, Any], actual_summary: dict[str, Any] | None) -> dict[str, Any]:
    if actual_summary is None:
        return {
            "status": "not_available",
            "note": "No closeout actuals were supplied to this prediction run.",
        }
    comparisons: dict[str, Any] = {}
    known_actual_count = 0
    for field in ["elapsed_seconds", "total_tokens", "validation_seconds"]:
        actual = actual_numeric(actual_summary, field)
        predicted = number_or_none(predictions.get(field))
        if actual is None or predicted is None:
            comparisons[field] = {
                "status": "actual_unknown",
                "predicted": predicted,
                "actual": actual,
                "absolute_error": None,
                "percent_error": None,
            }
            continue
        known_actual_count += 1
        absolute_error = predicted - actual
        percent_error = None if actual == 0 else abs(absolute_error) / actual * 100.0
        comparisons[field] = {
            "status": "compared",
            "predicted": round(predicted),
            "actual": round(actual),
            "absolute_error": round(absolute_error),
            "percent_error": None if percent_error is None else round(percent_error, 2),
        }
    return {
        "status": "compared" if known_actual_count else "actuals_unavailable",
        "known_actual_count": known_actual_count,
        "comparisons": comparisons,
    }


def predict(
    packets: list[dict[str, Any]],
    paths: list[Path],
    *,
    actual_summary: dict[str, Any] | None = None,
    root: Path | None = None,
) -> dict[str, Any]:
    if not packets:
        raise ValueError("at least one prediction packet is required")
    for packet in packets:
        schema = packet.get("schema_version")
        if schema != INPUT_SCHEMA:
            raise ValueError(f"unsupported prediction packet schema: {schema}")
    issue_goal_ref = require_consistent_identity(packets, "issue_goal_ref")
    sprint_goal_ref = require_consistent_identity(packets, "sprint_goal_ref")

    elapsed = aggregate_known(packets, "elapsed_seconds")
    active_work = aggregate_known(packets, "active_work_seconds")
    validation = aggregate_known(packets, "validation_seconds")
    pr_wait = aggregate_known(packets, "pr_wait_seconds")
    ci_wait = aggregate_known(packets, "ci_wait_seconds")
    total_tokens = aggregate_known(packets, "total_tokens")

    elapsed_prediction = elapsed if elapsed is not None else active_work
    elapsed_basis = "known_elapsed_seconds" if elapsed is not None else "known_active_work_seconds"
    if elapsed_prediction is None:
        elapsed_prediction = 1800.0
        elapsed_basis = "heuristic_default"

    token_prediction = total_tokens
    token_basis = "known_total_tokens"
    if token_prediction is None:
        token_prediction = 100000.0
        token_basis = "heuristic_default"

    validation_prediction = validation
    validation_basis = "known_validation_seconds"
    if validation_prediction is None:
        validation_prediction = max(30.0, min(600.0, round(elapsed_prediction * 0.10)))
        validation_basis = "heuristic_from_elapsed_seconds"

    pr_wait_risk = risk_from_seconds(pr_wait, medium=300.0, high=900.0, missing="medium")
    ci_wait_risk = risk_from_seconds(ci_wait, medium=300.0, high=900.0, missing="medium")
    validation_risk = risk_from_seconds(validation_prediction, medium=180.0, high=600.0, missing="medium")
    scale_risk = max_risk(
        [
            risk_from_seconds(elapsed_prediction, medium=1800.0, high=3600.0, missing="medium"),
            "high" if token_prediction >= 200000 else "medium" if token_prediction >= 100000 else "low",
            validation_risk,
        ]
    )
    outlier_risk = max_risk([scale_risk, pr_wait_risk, ci_wait_risk])

    known_core = sum(value is not None for value in [elapsed, total_tokens, validation, pr_wait, ci_wait])
    if known_core >= 4:
        confidence = "high"
    elif known_core >= 2:
        confidence = "medium"
    else:
        confidence = "low"

    feature_availability = {
        field: [availability(packet, field) for packet in packets]
        for field in [
            "elapsed_seconds",
            "active_work_seconds",
            "validation_seconds",
            "pr_wait_seconds",
            "ci_wait_seconds",
            "total_tokens",
        ]
    }
    known_inputs = sorted(
        field for field, values in feature_availability.items() if any(value == "known" for value in values)
    )
    missing_inputs = sorted(
        field for field, values in feature_availability.items() if not any(value == "known" for value in values)
    )

    return {
        "schema_version": OUTPUT_SCHEMA,
        "status": "predicted",
        "input_schema_version": INPUT_SCHEMA,
        "input_packet_count": len(packets),
        "source_prediction_packets": source_refs(paths, root=root),
        "issue_goal_ref": issue_goal_ref,
        "sprint_goal_ref": sprint_goal_ref,
        "unknown_values_policy": "unknown_is_not_zero",
        "known_inputs": known_inputs,
        "missing_inputs": missing_inputs,
        "input_feature_availability": feature_availability,
        "predictions": {
            "elapsed_seconds": round(elapsed_prediction),
            "total_tokens": round(token_prediction),
            "validation_seconds": round(validation_prediction),
            "pr_wait_risk": pr_wait_risk,
            "ci_wait_risk": ci_wait_risk,
            "outlier_risk": outlier_risk,
        },
        "prediction_basis": {
            "elapsed_seconds": elapsed_basis,
            "total_tokens": token_basis,
            "validation_seconds": validation_basis,
            "pr_wait_risk": "known_pr_wait_seconds" if pr_wait is not None else "heuristic_missing_wait_input",
            "ci_wait_risk": "known_ci_wait_seconds" if ci_wait is not None else "heuristic_missing_wait_input",
            "outlier_risk": "max(scale_risk, pr_wait_risk, ci_wait_risk)",
        },
        "confidence": {
            "overall": confidence,
            "known_core_feature_count": known_core,
            "note": "Deterministic heuristic baseline; accuracy must be compared against closeout actuals as data accumulates.",
        },
        "actual_comparison": compare_actuals(
            {
                "elapsed_seconds": round(elapsed_prediction),
                "total_tokens": round(token_prediction),
                "validation_seconds": round(validation_prediction),
            },
            actual_summary,
        ),
    }


def render_markdown(prediction: dict[str, Any]) -> str:
    values = prediction["predictions"]
    basis = prediction["prediction_basis"]
    lines = [
        "# Issue Execution Metrics Prediction",
        "",
        f"- status: `{prediction['status']}`",
        f"- input packet count: `{prediction['input_packet_count']}`",
        f"- issue goal ref: `{prediction.get('issue_goal_ref', 'unknown')}`",
        f"- sprint goal ref: `{prediction.get('sprint_goal_ref', 'unknown')}`",
        f"- confidence: `{prediction['confidence']['overall']}`",
        f"- unknown values policy: `{prediction['unknown_values_policy']}`",
        "",
        "## Predictions",
        "",
    ]
    for field in [
        "elapsed_seconds",
        "total_tokens",
        "validation_seconds",
        "pr_wait_risk",
        "ci_wait_risk",
        "outlier_risk",
    ]:
        lines.append(f"- {field}: `{values[field]}` basis=`{basis[field]}`")

    lines.extend(["", "## Input Availability", ""])
    for field in [
        "elapsed_seconds",
        "active_work_seconds",
        "validation_seconds",
        "pr_wait_seconds",
        "ci_wait_seconds",
        "total_tokens",
    ]:
        availability_values = ", ".join(prediction["input_feature_availability"][field])
        lines.append(f"- {field}: `{availability_values}`")

    lines.extend(["", "## Missing Inputs", ""])
    missing = prediction["missing_inputs"]
    lines.extend(f"- `{field}`" for field in missing) if missing else lines.append("- none")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Predict issue execution metrics from goal-metrics reporting/prediction packets."
    )
    parser.add_argument("--packet", action="append", required=True, help="Input reporting_prediction packet JSON")
    parser.add_argument("--actual-summary", help="Optional goal-metrics summary JSON containing closeout actuals")
    parser.add_argument("--prediction-out", required=True)
    parser.add_argument("--report-out")
    parser.add_argument("--print-json", action="store_true")
    args = parser.parse_args()

    packet_paths = [Path(raw) for raw in args.packet]
    packets = [load_json(path) for path in packet_paths]
    actual_summary = load_json(Path(args.actual_summary)) if args.actual_summary else None
    prediction = predict(packets, packet_paths, actual_summary=actual_summary, root=Path.cwd())

    prediction_out = Path(args.prediction_out)
    prediction_out.parent.mkdir(parents=True, exist_ok=True)
    prediction_out.write_text(json.dumps(prediction, indent=2, sort_keys=True) + "\n")

    report_path = None
    if args.report_out:
        report_path = Path(args.report_out)
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(render_markdown(prediction))

    result = {
        "status": "written",
        "prediction_path": display_path(prediction_out),
        "report_path": display_path(report_path) if report_path else None,
        "confidence": prediction["confidence"]["overall"],
        "outlier_risk": prediction["predictions"]["outlier_risk"],
        "missing_inputs": prediction["missing_inputs"],
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(str(prediction_out))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
