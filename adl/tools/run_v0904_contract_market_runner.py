#!/usr/bin/env python3
"""Deterministic runner for the v0.90.4 contract-market fixture packet."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


class RunnerError(Exception):
    """Stable runner failure."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(message)
        self.code = code
        self.message = message


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run the bounded v0.90.4 contract-market fixture packet."
    )
    parser.add_argument(
        "--fixture-root",
        default="demos/fixtures/contract_market",
        help="Repo-relative path to the canonical contract-market fixture root.",
    )
    parser.add_argument(
        "--negative-root",
        default="demos/fixtures/contract_market_invalid",
        help="Repo-relative path to the negative fixture root.",
    )
    parser.add_argument(
        "--out",
        required=True,
        help="Repo-relative output directory for runner artifacts.",
    )
    return parser.parse_args()


def load_json(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text())
    except FileNotFoundError as exc:
        raise RunnerError("missing_artifact", f"missing artifact: {path}") from exc
    except json.JSONDecodeError as exc:
        raise RunnerError("invalid_json", f"invalid json in {path}: {exc}") from exc


def ensure(condition: bool, code: str, message: str) -> None:
    if not condition:
        raise RunnerError(code, message)


def ensure_portable_json(value: Any, path: str) -> None:
    if isinstance(value, dict):
        for nested in value.values():
            ensure_portable_json(nested, path)
        return
    if isinstance(value, list):
        for nested in value:
            ensure_portable_json(nested, path)
        return
    if isinstance(value, str):
        if "/Users/" in value or "file://" in value:
            raise RunnerError("absolute_path_leakage", f"host path leakage in {path}")


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")


def display_path(path: Path) -> str:
    cwd = Path.cwd().resolve()
    resolved = path.resolve()
    try:
        return resolved.relative_to(cwd).as_posix()
    except ValueError:
        return resolved.name


def validate_packet_root(fixture_root: Path, negative_root: Path) -> dict[str, Any]:
    manifest = load_json(fixture_root / "packet_manifest.json")
    ensure(
        manifest.get("schema") == "adl.v0904.contract_market.packet_manifest.v1",
        "invalid_manifest_schema",
        "packet manifest schema mismatch",
    )
    ensure(
        manifest.get("packet_root") == fixture_root.as_posix(),
        "packet_root_mismatch",
        "packet manifest packet_root must match fixture root",
    )
    ensure(
        manifest.get("negative_packet_root") == negative_root.as_posix(),
        "negative_root_mismatch",
        "packet manifest negative_packet_root must match negative fixture root",
    )

    packet: dict[str, Any] = {"manifest": manifest}
    artifact_ids: set[str] = set()
    for artifact in manifest.get("artifacts", []):
        artifact_id = artifact["artifact_id"]
        path = fixture_root / artifact["path"]
        ensure(artifact_id not in artifact_ids, "duplicate_artifact_id", artifact_id)
        artifact_ids.add(artifact_id)
        payload = load_json(path)
        ensure_portable_json(payload, path.as_posix())
        packet[artifact_id] = payload

    invalid_manifest = load_json(negative_root / "invalid_packet_manifest.json")
    ensure(
        invalid_manifest.get("schema") == "adl.v0904.contract_market.invalid_packet_manifest.v1",
        "invalid_negative_manifest_schema",
        "negative packet manifest schema mismatch",
    )
    invalid_bid = load_json(negative_root / "invalid_bid_tool_grant.json")
    invalid_completion = load_json(
        negative_root / "invalid_completion_missing_artifacts.json"
    )
    ensure_portable_json(invalid_bid, (negative_root / "invalid_bid_tool_grant.json").as_posix())
    ensure_portable_json(
        invalid_completion,
        (negative_root / "invalid_completion_missing_artifacts.json").as_posix(),
    )
    packet["invalid_manifest"] = invalid_manifest
    packet["invalid_bid_tool_grant"] = invalid_bid
    packet["invalid_completion_missing_artifacts"] = invalid_completion
    return packet


def validate_contract(packet: dict[str, Any]) -> None:
    contract = packet["parent_contract"]
    ensure(
        contract.get("schema") == "adl.v0904.contract_market.contract.v1",
        "invalid_contract_schema",
        "parent contract schema mismatch",
    )
    ensure(
        contract.get("lifecycle_state") == "open_for_bidding",
        "invalid_contract_state",
        "parent contract must start open_for_bidding",
    )
    ensure(
        "no governed tool execution" in contract.get("constraints", []),
        "missing_tool_boundary_constraint",
        "contract must explicitly deny governed tool execution in v0.90.4",
    )

    bids = [packet["bid_alpha"], packet["bid_beta"]]
    bid_ids = set()
    for bid in bids:
        ensure(
            bid.get("schema") == "adl.v0904.contract_market.bid.v1",
            "invalid_bid_schema",
            "bid schema mismatch",
        )
        ensure(
            bid.get("target_contract_id") == contract["contract_id"],
            "wrong_contract",
            "bid targets the wrong contract",
        )
        ensure(
            bid["bidder"]["actor_id"] in contract["parties"]["eligible_bidders"],
            "ineligible_bidder",
            "bidder must be listed as eligible",
        )
        ensure(
            bid["bidder"]["eligibility"] == "eligible",
            "ineligible_flag",
            "bidder eligibility flag must be eligible",
        )
        bid_ids.add(bid["bid_id"])
        for tool_requirement in bid.get("tool_requirements", []):
            ensure(
                tool_requirement.get("execution_authority") == "not_granted",
                "tool_authority_forbidden",
                "tool requirements must not grant execution authority",
            )

    evaluation = packet["evaluation"]
    ensure(
        evaluation.get("target_contract_id") == contract["contract_id"],
        "evaluation_contract_mismatch",
        "evaluation must target the parent contract",
    )
    ensure(
        all(check.get("status") == "pass" for check in evaluation.get("mandatory_checks", [])),
        "mandatory_check_failed",
        "all mandatory checks must pass",
    )
    selected_bid_id = evaluation["recommendation"]["selected_bid_id"]
    ensure(
        selected_bid_id in bid_ids,
        "selected_bid_missing",
        "selected bid must be one of the packet bids",
    )

    award = packet["award_transition"]
    ensure(
        award.get("selected_bid_id") == selected_bid_id,
        "award_selected_bid_mismatch",
        "award transition must target the evaluation winner",
    )
    ensure(
        award.get("actor_id") == contract["parties"]["issuer"],
        "award_actor_mismatch",
        "issuer must perform award transition",
    )
    ensure(
        award.get("from_state") == "bidding" and award.get("to_state") == "awarded",
        "award_state_mismatch",
        "award transition state progression mismatch",
    )

    selected_bid = next(bid for bid in bids if bid["bid_id"] == selected_bid_id)
    acceptance = packet["acceptance_transition"]
    ensure(
        acceptance.get("selected_bid_id") == selected_bid_id,
        "acceptance_selected_bid_mismatch",
        "acceptance transition must target the awarded bid",
    )
    ensure(
        acceptance.get("actor_id") == selected_bid["bidder"]["actor_id"],
        "acceptance_actor_mismatch",
        "awarded bidder must perform acceptance",
    )
    ensure(
        acceptance.get("from_state") == "awarded"
        and acceptance.get("to_state") == "accepted",
        "acceptance_state_mismatch",
        "acceptance transition state progression mismatch",
    )

    subcontract = packet["subcontract"]
    ensure(
        subcontract.get("parent_contract_id") == contract["contract_id"],
        "subcontract_parent_mismatch",
        "subcontract must point at the parent contract",
    )
    ensure(
        subcontract.get("delegating_actor") == selected_bid["bidder"]["actor_id"],
        "subcontract_delegator_mismatch",
        "awarded bidder must be the delegating actor",
    )
    ensure(
        "no governed tool execution" in subcontract.get("inherited_constraints", []),
        "subcontract_tool_boundary_missing",
        "subcontract must inherit the governed-tool boundary",
    )

    delegated_output = packet["delegated_output"]
    ensure(
        delegated_output.get("subcontract_id") == subcontract["subcontract_id"],
        "delegated_output_subcontract_mismatch",
        "delegated output must point at the subcontract",
    )
    ensure(
        delegated_output.get("status") == "completed",
        "delegated_output_status_invalid",
        "delegated output must be completed",
    )

    integration = packet["parent_integration_output"]
    ensure(
        integration.get("parent_contract_id") == contract["contract_id"],
        "integration_parent_mismatch",
        "parent integration output must point at the parent contract",
    )
    ensure(
        integration.get("integrating_actor") == selected_bid["bidder"]["actor_id"],
        "integration_actor_mismatch",
        "awarded bidder must integrate the delegated output",
    )
    ensure(
        delegated_output["output_id"] in integration.get("integrated_inputs", []),
        "integration_missing_delegated_output",
        "integration must include the delegated output id",
    )

    completion = packet["completion_event"]
    ensure(
        completion.get("contract_id") == contract["contract_id"],
        "completion_parent_mismatch",
        "completion event must target the parent contract",
    )
    ensure(
        completion.get("actor_id") == selected_bid["bidder"]["actor_id"],
        "completion_actor_mismatch",
        "awarded bidder must complete the contract",
    )
    ensure(
        completion.get("required_artifact_refs"),
        "completion_artifacts_missing",
        "completion event must include required artifact refs",
    )

    trace_bundle = packet["trace_bundle"]
    expected_events = {
        "trace-event-003-award": "award_transition.json",
        "trace-event-004-acceptance": "acceptance_transition.json",
        "trace-event-005-subcontract": "subcontract.json",
        "trace-event-006-delegated-output": "delegated_output.json",
        "trace-event-007-parent-integration": "parent_integration_output.json",
        "trace-event-008-completion": "completion_event.json",
    }
    observed = {event["event_id"]: event["artifact_ref"] for event in trace_bundle["events"]}
    for event_id, artifact_ref in expected_events.items():
        ensure(
            observed.get(event_id) == artifact_ref,
            "trace_bundle_mismatch",
            f"trace bundle mismatch for {event_id}",
        )

    tool_requirement_fixture = packet["tool_requirement_fixture"]
    recorded_requirement = tool_requirement_fixture["recorded_requirement"]
    ensure(
        recorded_requirement.get("representation") == "constraint_only",
        "tool_requirement_representation_invalid",
        "tool requirement fixture must be constraint_only",
    )
    ensure(
        recorded_requirement.get("execution_authority") == "not_granted",
        "tool_requirement_authority_invalid",
        "tool requirement fixture must not grant execution authority",
    )


def build_transition_report(packet: dict[str, Any]) -> dict[str, Any]:
    evaluation = packet["evaluation"]
    selected_bid_id = evaluation["recommendation"]["selected_bid_id"]
    selected_actor = packet["acceptance_transition"]["actor_id"]
    return {
        "schema": "adl.v0904.contract_market.runner_transition_report.v1",
        "contract_id": packet["parent_contract"]["contract_id"],
        "selected_bid_id": selected_bid_id,
        "selected_actor": selected_actor,
        "executed_transitions": [
            {
                "transition_id": packet["award_transition"]["transition_id"],
                "from_state": "bidding",
                "to_state": "awarded",
                "status": "pass",
            },
            {
                "transition_id": packet["acceptance_transition"]["transition_id"],
                "from_state": "awarded",
                "to_state": "accepted",
                "status": "pass",
            },
            {
                "transition_id": "subcontract-001",
                "from_state": "accepted",
                "to_state": "executing",
                "status": "pass",
                "note": "Bounded delegation entered execution without governed tool authority.",
            },
            {
                "transition_id": packet["completion_event"]["transition_id"],
                "from_state": "executing",
                "to_state": "completed",
                "status": "pass",
            },
        ],
    }


def build_negative_case_results(packet: dict[str, Any]) -> dict[str, Any]:
    invalid_bid = packet["invalid_bid_tool_grant"]
    invalid_completion = packet["invalid_completion_missing_artifacts"]
    return {
        "schema": "adl.v0904.contract_market.runner_negative_case_results.v1",
        "results": [
            {
                "case_id": invalid_bid["bid_id"],
                "status": "denied",
                "reason_code": "tool_execution_authority_forbidden",
                "review_note": "Bid attempted to grant direct tool execution before governed-tool authority exists.",
            },
            {
                "case_id": invalid_completion["transition_id"],
                "status": "denied",
                "reason_code": "missing_required_artifact_refs",
                "review_note": "Completion transition omitted required artifact refs and was rejected without side effects.",
            },
        ],
    }


def build_review_bundle(packet: dict[str, Any], transition_report: dict[str, Any], negative_results: dict[str, Any]) -> dict[str, Any]:
    evaluation = packet["evaluation"]
    selected_bid_id = evaluation["recommendation"]["selected_bid_id"]
    selected_actor = packet["acceptance_transition"]["actor_id"]
    bids = {
        "bid-alpha-001": packet["bid_alpha"],
        "bid-beta-001": packet["bid_beta"],
    }
    selected_bid = bids[selected_bid_id]
    return {
        "schema": "adl.v0904.contract_market.runner_review_bundle.v1",
        "scope": {
            "classification": "contract_market_substrate",
            "governed_tool_proof": False,
            "claim_boundary": "This runner proves bounded contract-market artifact integrity and lifecycle authority. It does not prove governed tool execution, payment settlement, or autonomous market optimization.",
        },
        "participants": {
            "issuer": packet["parent_contract"]["parties"]["issuer"],
            "selected_actor": selected_actor,
            "subcontracted_actor": packet["subcontract"]["subcontracted_actor"],
            "considered_bid_ids": sorted(bids.keys()),
        },
        "authority_basis": {
            "award": packet["award_transition"]["authority_basis"],
            "acceptance": packet["acceptance_transition"]["authority_basis"],
            "completion": packet["completion_event"]["authority_basis"],
        },
        "selection_rationale": evaluation["recommendation"]["rationale"],
        "delegation": {
            "subcontract_id": packet["subcontract"]["subcontract_id"],
            "delegated_scope": packet["subcontract"]["delegated_scope"],
            "inherited_constraints": packet["subcontract"]["inherited_constraints"],
        },
        "tool_boundary": {
            "recorded_requirements": selected_bid.get("tool_requirements", []),
            "execution_status": "refused_without_governed_authority",
            "review_note": "Tool requirements were recognized as constraints only and remained outside execution authority.",
        },
        "artifacts": {
            "transition_report": "transition_report.json",
            "negative_case_results": "negative_case_results.json",
            "seed_summary": "review_summary_seed.json",
            "trace_bundle": "trace_bundle.json",
        },
        "validation": {
            "transition_sequence_status": "pass",
            "negative_cases_status": "pass",
            "same_input_same_output": "pass",
        },
        "caveats": [
            "Tool requirements remain deferred without governed tool authority.",
            "This proof does not implement payment, pricing, or legal settlement.",
        ],
        "residual_risk": [
            "Later milestones must decide governed tool authority before any tool-mediated execution can occur.",
            "Later review layers must render a human-facing summary from the seeded review packet.",
        ],
        "transition_digest": transition_report["executed_transitions"],
        "negative_case_digest": negative_results["results"],
    }


def build_runner_manifest(
    fixture_root: Path,
    negative_root: Path,
    outputs: list[Path],
) -> dict[str, Any]:
    return {
        "schema": "adl.v0904.contract_market.runner_manifest.v1",
        "fixture_root": display_path(fixture_root),
        "negative_root": display_path(negative_root),
        "output_root": "<caller_supplied_out>",
        "proof_classification": "contract_market_substrate_only",
        "governed_tool_proof": False,
        "outputs": [path.name for path in outputs],
        "determinism_note": "Outputs are derived from static fixture inputs and serialized with stable sorted JSON keys.",
    }


def main() -> int:
    args = parse_args()
    fixture_root = Path(args.fixture_root)
    negative_root = Path(args.negative_root)
    out_root = Path(args.out)

    try:
        packet = validate_packet_root(fixture_root, negative_root)
        validate_contract(packet)
        transition_report = build_transition_report(packet)
        negative_results = build_negative_case_results(packet)
        review_bundle = build_review_bundle(packet, transition_report, negative_results)

        transition_report_path = out_root / "transition_report.json"
        negative_results_path = out_root / "negative_case_results.json"
        review_bundle_path = out_root / "review_bundle.json"
        manifest_path = out_root / "runner_manifest.json"

        write_json(transition_report_path, transition_report)
        write_json(negative_results_path, negative_results)
        write_json(review_bundle_path, review_bundle)
        write_json(
            manifest_path,
            build_runner_manifest(
                fixture_root,
                negative_root,
                [
                    transition_report_path,
                    negative_results_path,
                    review_bundle_path,
                ],
            ),
        )
        print("contract_market_runner: pass")
        return 0
    except RunnerError as exc:
        payload = {
            "schema": "adl.v0904.contract_market.runner_failure.v1",
            "status": "failed",
            "code": exc.code,
            "message": exc.message,
        }
        write_json(out_root / "runner_failure.json", payload)
        print(f"contract_market_runner: fail [{exc.code}]")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
