#!/usr/bin/env python3
"""Render the v0.90.4 contract-market review summary example."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


class RenderError(Exception):
    """Stable renderer failure."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(message)
        self.code = code
        self.message = message


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Render the deterministic v0.90.4 contract-market review summary."
    )
    parser.add_argument(
        "--seed",
        default="demos/fixtures/contract_market/review_summary_seed.json",
        help="Repo-relative path to the review summary seed.",
    )
    parser.add_argument(
        "--review-bundle",
        required=True,
        help="Repo-relative path to the WP-12 review bundle artifact.",
    )
    parser.add_argument(
        "--schema",
        default="demos/fixtures/contract_market/review_summary_schema.json",
        help="Repo-relative path to the review summary schema.",
    )
    parser.add_argument(
        "--out",
        required=True,
        help="Repo-relative output path for the rendered Markdown summary.",
    )
    return parser.parse_args()


def load_json(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text())
    except FileNotFoundError as exc:
        raise RenderError("missing_input", f"missing input: {path}") from exc
    except json.JSONDecodeError as exc:
        raise RenderError("invalid_json", f"invalid json in {path}: {exc}") from exc


def ensure(condition: bool, code: str, message: str) -> None:
    if not condition:
        raise RenderError(code, message)


def bullet_lines(items: list[str]) -> list[str]:
    return [f"- {item}" for item in items]


def format_recorded_requirements(review_bundle: dict[str, Any]) -> list[str]:
    requirements = review_bundle["tool_boundary"]["recorded_requirements"]
    if not requirements:
        return ["- none recorded"]
    return [
        "- "
        + requirement["description"]
        + f" (`{requirement['mode']}`, authority `{requirement['execution_authority']}`)"
        for requirement in requirements
    ]


def render_summary(schema: dict[str, Any], seed: dict[str, Any], review_bundle: dict[str, Any]) -> str:
    ensure(
        schema.get("schema") == "adl.v0904.contract_market.review_summary_schema.v1",
        "schema_mismatch",
        "review summary schema mismatch",
    )
    ensure(
        seed.get("schema") == "adl.v0904.contract_market.review_summary_seed.v1",
        "seed_mismatch",
        "review summary seed schema mismatch",
    )
    ensure(
        review_bundle.get("schema") == "adl.v0904.contract_market.runner_review_bundle.v1",
        "review_bundle_mismatch",
        "review bundle schema mismatch",
    )

    labels = schema["labels"]
    tool_language = schema["tool_language"]

    artifact_items = list(seed["artifacts"]) + [
        review_bundle["artifacts"]["transition_report"],
        review_bundle["artifacts"]["negative_case_results"],
    ]
    participants = review_bundle["participants"]
    considered_bids = ", ".join(participants["considered_bid_ids"])

    lines = [
        "# Contract-Market Review Summary",
        "",
        f"Schema: `{schema['schema']}`",
        f"Summary ID: `{seed['summary_id']}`",
        f"Claim boundary: {schema['claim_boundary']}",
        "",
        "## Scope",
        f"{labels['proof']}: {seed['scope']}",
        f"{labels['judgment']}: This is a bounded contract-market substrate proof, not a live market run or governed-tool execution proof.",
        "",
        "## Participants",
        f"{labels['proof']}:",
        f"- Issuer: `{participants['issuer']}`",
        f"- Selected actor: `{participants['selected_actor']}`",
        f"- Considered bids: {considered_bids}",
        f"- Subcontracted actor: `{participants['subcontracted_actor']}`",
        "",
        "## Authority Basis",
        f"{labels['proof']}: {seed['authority_basis']}",
        f"{labels['judgment']}: Award, acceptance, and completion remain tied to explicit authority bases in the runner review bundle.",
        "",
        "## Bid Comparison",
        f"{labels['proof']}: {seed['bid_comparison']}",
        f"{labels['judgment']}: The runner confirms the selected path because stronger trace and delegation posture beat lower complexity alone while tool needs remain deferred.",
        "",
        "## Selection Rationale",
        f"{labels['judgment']}: {seed['selection_rationale']}",
        "",
        "## Delegation",
        f"{labels['proof']}: {seed['delegation']}",
        f"{labels['judgment']}: Delegation stays bounded because inherited subcontract constraints preserve portable artifacts and no governed tool execution.",
        "",
        "## Artifacts",
        f"{labels['proof']}:",
        *bullet_lines([f"`{item}`" for item in artifact_items]),
        "",
        "## Trace",
        f"{labels['proof']}: {seed['trace']}",
        f"{labels['judgment']}: The review surface relies on explicit trace-linked lifecycle events rather than hidden state or model confidence.",
        "",
        "## Validation",
        f"{labels['proof']}: {seed['validation']}",
        f"{labels['non_claims']}:",
        "- This summary does not claim payment settlement, pricing, tax handling, or legal enforcement.",
        "- This summary does not claim governed tool execution.",
        "",
        "## Tool Requirements",
        f"{labels['recorded']}:",
        *format_recorded_requirements(review_bundle),
        f"{labels['deferred']}:",
        f"- {tool_language['recorded']}",
        f"- {tool_language['denied']}",
        f"- {tool_language['deferred']}",
        "",
        "## Caveats",
        *bullet_lines(seed["caveats"]),
        "",
        "## Residual Risk",
        f"{labels['residual_risk']}:",
        *bullet_lines(seed["residual_risk"] + review_bundle["residual_risk"]),
        "",
    ]
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    args = parse_args()
    seed = load_json(Path(args.seed))
    review_bundle = load_json(Path(args.review_bundle))
    schema = load_json(Path(args.schema))
    out_path = Path(args.out)
    try:
        rendered = render_summary(schema, seed, review_bundle)
    except RenderError as exc:
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(f"render_failure: {exc.code}: {exc.message}\n")
        print(f"contract_market_summary: fail [{exc.code}]")
        return 1
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(rendered)
    print("contract_market_summary: pass")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
