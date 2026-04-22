#!/usr/bin/env python3
"""Compute Cognitive Compression Cost v0 from fixture counters."""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any

SCHEMA_VERSION = "adl.ccc.v0.fixture.v1"
REPORT_SCHEMA_VERSION = "adl.ccc.v0.report.v1"

REQUIRED_COUNTERS = [
    "num_reframes",
    "num_low_adequacy_events",
    "num_iterations",
    "num_retries",
    "num_steps",
    "num_tool_calls",
    "num_model_calls",
    "num_validation_failures",
    "num_contradictions",
]
TERMINATION_REASONS = {
    "success": 0,
    "bounded_failure": 5,
    "no_progress": 5,
}
WEIGHTS = {
    "framing": {"num_reframes": 3, "num_low_adequacy_events": 1},
    "exploration": {"num_iterations": 1, "num_retries": 2},
    "execution": {"num_steps": 0.5, "num_tool_calls": 1, "num_model_calls": 1},
    "residual_error": {
        "num_validation_failures": 2,
        "num_contradictions": 2,
        "termination_penalty": 1,
    },
}
HOST_PATH_PREFIXES = ["/" + "Users/", "/" + "private/", "/" + "tmp/"]
ABSOLUTE_PATH_PATTERNS = [
    re.compile(re.escape(prefix)) for prefix in HOST_PATH_PREFIXES
] + [re.compile(r"[A-Za-z]:\\\\")]


class CccInputError(ValueError):
    pass


def load_fixture(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise CccInputError(f"{path.name}: invalid JSON: {exc}") from exc
    if not isinstance(payload, dict):
        raise CccInputError(f"{path.name}: fixture must be a JSON object")
    return payload


def validate_fixture(payload: dict[str, Any], path: Path) -> None:
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise CccInputError(
            f"{path.name}: schema_version must be {SCHEMA_VERSION!r}"
        )
    for key in ["run_id", "task_label", "trace_family", "counters", "source_note"]:
        if key not in payload:
            raise CccInputError(f"{path.name}: missing required field {key!r}")
    if not isinstance(payload["counters"], dict):
        raise CccInputError(f"{path.name}: counters must be an object")
    counters = payload["counters"]
    for key in REQUIRED_COUNTERS:
        if key not in counters:
            raise CccInputError(f"{path.name}: missing required counter {key!r}")
        value = counters[key]
        if not isinstance(value, int) or value < 0:
            raise CccInputError(
                f"{path.name}: counter {key!r} must be a non-negative integer"
            )
    termination_reason = payload.get("termination_reason")
    if termination_reason not in TERMINATION_REASONS:
        allowed = ", ".join(sorted(TERMINATION_REASONS))
        raise CccInputError(
            f"{path.name}: termination_reason must be one of: {allowed}"
        )
    encoded = json.dumps(payload, sort_keys=True)
    for pattern in ABSOLUTE_PATH_PATTERNS:
        if pattern.search(encoded):
            raise CccInputError(
                f"{path.name}: fixture contains a host-local absolute path"
            )


def stable_number(value: float) -> int | float:
    if value == int(value):
        return int(value)
    return round(value, 3)


def dominant_driver(component_costs: dict[str, int | float]) -> str:
    order = ["framing", "exploration", "execution", "residual_error"]
    return max(order, key=lambda key: (component_costs[key], -order.index(key)))


def interpret(driver: str) -> str:
    return {
        "framing": "Framing cost dominates, so the run spent most effort finding a workable representation.",
        "exploration": "Exploration cost dominates, so search, retries, or branching drove the cost.",
        "execution": "Execution cost dominates, so the selected frame required many steps or invocations.",
        "residual_error": "Residual-error cost dominates, so validation churn or contradictions drove the cost.",
    }[driver]


def compute_fixture(payload: dict[str, Any]) -> dict[str, Any]:
    counters = payload["counters"]
    termination_penalty = TERMINATION_REASONS[payload["termination_reason"]]
    framing = (
        WEIGHTS["framing"]["num_reframes"] * counters["num_reframes"]
        + WEIGHTS["framing"]["num_low_adequacy_events"]
        * counters["num_low_adequacy_events"]
    )
    exploration = (
        WEIGHTS["exploration"]["num_iterations"] * counters["num_iterations"]
        + WEIGHTS["exploration"]["num_retries"] * counters["num_retries"]
    )
    execution = (
        WEIGHTS["execution"]["num_steps"] * counters["num_steps"]
        + WEIGHTS["execution"]["num_tool_calls"] * counters["num_tool_calls"]
        + WEIGHTS["execution"]["num_model_calls"] * counters["num_model_calls"]
    )
    residual_error = (
        WEIGHTS["residual_error"]["num_validation_failures"]
        * counters["num_validation_failures"]
        + WEIGHTS["residual_error"]["num_contradictions"]
        * counters["num_contradictions"]
        + WEIGHTS["residual_error"]["termination_penalty"] * termination_penalty
    )
    component_costs = {
        "framing": stable_number(framing),
        "exploration": stable_number(exploration),
        "execution": stable_number(execution),
        "residual_error": stable_number(residual_error),
    }
    total = stable_number(sum(float(value) for value in component_costs.values()))
    driver = dominant_driver(component_costs)
    return {
        "run_id": payload["run_id"],
        "task_label": payload["task_label"],
        "trace_family": payload["trace_family"],
        "source_note": payload["source_note"],
        "ccc_metric": {
            "version": "v0",
            "weights": WEIGHTS,
            "component_costs": component_costs,
            "total": total,
            "dominant_cost_driver": driver,
            "interpretation": interpret(driver),
            "breakdown": {
                **{key: counters[key] for key in REQUIRED_COUNTERS},
                "termination_reason": payload["termination_reason"],
                "termination_penalty": termination_penalty,
            },
        },
        "claim_boundary": {
            "not_pricing": True,
            "not_moral_worth": True,
            "not_absolute_intelligence": True,
            "not_productivity_ranking": True,
            "not_cross_agent_normalized": True,
        },
    }


def summarize(results: list[dict[str, Any]]) -> dict[str, Any]:
    sorted_by_total = sorted(
        results, key=lambda item: (item["ccc_metric"]["total"], item["run_id"])
    )
    drivers = Counter(
        item["ccc_metric"]["dominant_cost_driver"] for item in sorted_by_total
    )
    lowest = sorted_by_total[0]
    highest = sorted_by_total[-1]
    return {
        "run_count": len(results),
        "lowest_ccc_run": {
            "run_id": lowest["run_id"],
            "total": lowest["ccc_metric"]["total"],
            "dominant_cost_driver": lowest["ccc_metric"]["dominant_cost_driver"],
        },
        "highest_ccc_run": {
            "run_id": highest["run_id"],
            "total": highest["ccc_metric"]["total"],
            "dominant_cost_driver": highest["ccc_metric"]["dominant_cost_driver"],
        },
        "dominant_driver_distribution": dict(sorted(drivers.items())),
        "comparison_note": comparison_note(lowest, highest),
        "caveats": [
            "CCC v0 is a deterministic fixture-derived effort signal.",
            "CCC v0 is not pricing, moral worth, absolute intelligence, or productivity ranking.",
            "CCC v0 is not cross-agent normalized and should not be used as a leaderboard.",
            "Lower CCC can indicate easier task shape, better framing, or less validation churn; it is not a virtue score.",
        ],
    }


def comparison_note(lowest: dict[str, Any], highest: dict[str, Any]) -> str:
    low = lowest["ccc_metric"]
    high = highest["ccc_metric"]
    delta = stable_number(float(high["total"]) - float(low["total"]))
    return (
        f"{highest['run_id']} is {delta} CCC points higher than {lowest['run_id']}. "
        f"The comparison is explained by {high['dominant_cost_driver']} dominance, "
        "not by an absolute capability claim."
    )


def markdown_report(report: dict[str, Any]) -> str:
    lines: list[str] = [
        "# CCC v0 Fixture Report",
        "",
        "Generated by `adl/tools/compute_ccc_v0.py` from fixture counters.",
        "",
        "## Claim Boundary",
        "",
        "- CCC v0 is a trace-derived effort signal, not an absolute score.",
        "- CCC v0 is not pricing, moral worth, intelligence ranking, or productivity ranking.",
        "- CCC v0 is not cross-agent normalized.",
        "- This report compares fixture runs only.",
        "",
        "## Source Links",
        "",
        "- Local plan: `.adl/docs/TBD/ccc/CCC_FIRST_PASS_PLAN.md`",
        "- Local metric draft: `.adl/docs/TBD/CCC_METRIC_v0.md`",
        "- Milestone compression plan: `.adl/docs/TBD/MILESTONE_COMPRESSION_PLAN.md`",
        "- Feature-list row: `docs/planning/ADL_FEATURE_LIST.md`",
        "",
        "## Weights",
        "",
        "- Framing: `3 * num_reframes + 1 * num_low_adequacy_events`",
        "- Exploration: `1 * num_iterations + 2 * num_retries`",
        "- Execution: `0.5 * num_steps + 1 * num_tool_calls + 1 * num_model_calls`",
        "- Residual error: `2 * num_validation_failures + 2 * num_contradictions + termination_penalty`",
        "",
        "## Runs",
        "",
        "| Run | Label | Framing | Exploration | Execution | Residual Error | Total | Dominant Driver |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |",
    ]
    for result in report["runs"]:
        metric = result["ccc_metric"]
        costs = metric["component_costs"]
        lines.append(
            "| {run_id} | {label} | {framing} | {exploration} | {execution} | {residual} | {total} | {driver} |".format(
                run_id=result["run_id"],
                label=result["task_label"],
                framing=costs["framing"],
                exploration=costs["exploration"],
                execution=costs["execution"],
                residual=costs["residual_error"],
                total=metric["total"],
                driver=metric["dominant_cost_driver"],
            )
        )
    summary = report["summary"]
    lines.extend(
        [
            "",
            "## Comparison",
            "",
            f"- Lowest CCC fixture: `{summary['lowest_ccc_run']['run_id']}` with total `{summary['lowest_ccc_run']['total']}`.",
            f"- Highest CCC fixture: `{summary['highest_ccc_run']['run_id']}` with total `{summary['highest_ccc_run']['total']}`.",
            f"- Dominant driver distribution: `{json.dumps(summary['dominant_driver_distribution'], sort_keys=True)}`.",
            f"- Interpretation: {summary['comparison_note']}",
            "",
            "## Operator Interpretation",
            "",
        ]
    )
    for result in report["runs"]:
        metric = result["ccc_metric"]
        lines.append(
            f"- `{result['run_id']}`: {metric['interpretation']} Total CCC: `{metric['total']}`."
        )
    lines.extend(
        [
            "",
            "## Caveats",
            "",
            *[f"- {caveat}" for caveat in summary["caveats"]],
            "",
            "## Validation Notes",
            "",
            "- Missing required counters are rejected before report generation.",
            "- Fixtures and generated tracked reports must not contain host-local absolute paths.",
            "- The validation command checks deterministic output by generating the report twice and comparing bytes.",
        ]
    )
    return "\n".join(lines) + "\n"


def load_results(fixtures_dir: Path) -> list[dict[str, Any]]:
    fixture_paths = sorted(fixtures_dir.glob("*.json"))
    if not fixture_paths:
        raise CccInputError(f"no fixture JSON files found in {fixtures_dir}")
    results: list[dict[str, Any]] = []
    for path in fixture_paths:
        payload = load_fixture(path)
        validate_fixture(payload, path)
        results.append(compute_fixture(payload))
    return sorted(results, key=lambda item: item["run_id"])


def assert_no_absolute_paths(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    for pattern in ABSOLUTE_PATH_PATTERNS:
        if pattern.search(text):
            raise CccInputError(
                f"generated artifact {path.name} contains a host-local absolute path"
            )


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--fixtures-dir",
        default="demos/fixtures/ccc_v0",
        help="Directory containing CCC v0 fixture JSON files.",
    )
    parser.add_argument(
        "--out-dir",
        default="demos/v0.90.3/ccc_v0",
        help="Directory for generated report artifacts.",
    )
    args = parser.parse_args(argv)

    fixtures_dir = Path(args.fixtures_dir)
    out_dir = Path(args.out_dir)
    results = load_results(fixtures_dir)
    report = {
        "schema_version": REPORT_SCHEMA_VERSION,
        "metric_version": "v0",
        "source_fixtures": [
            f"demos/fixtures/ccc_v0/{path.name}"
            for path in sorted(fixtures_dir.glob("*.json"))
        ],
        "source_planning": [
            ".adl/docs/TBD/CCC_METRIC_v0.md",
            ".adl/docs/TBD/ccc/CCC_FIRST_PASS_PLAN.md",
            ".adl/docs/TBD/MILESTONE_COMPRESSION_PLAN.md",
            ".adl/docs/TBD/capability_testing/",
            ".adl/docs/TBD/economics/REVIEW_SUMMARY_SHAPE_v0.md",
            ".adl/docs/TBD/economics/EVALUATION_MODEL_v0.md",
            "docs/planning/ADL_FEATURE_LIST.md",
        ],
        "runs": results,
        "summary": summarize(results),
    }

    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / "ccc_v0_report.json"
    md_path = out_dir / "ccc_v0_report.md"
    json_path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    md_path.write_text(markdown_report(report), encoding="utf-8")
    assert_no_absolute_paths(json_path)
    assert_no_absolute_paths(md_path)
    print(f"wrote {json_path}")
    print(f"wrote {md_path}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except CccInputError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        raise SystemExit(2)
