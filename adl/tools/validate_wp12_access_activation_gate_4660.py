#!/usr/bin/env python3
"""Validate WP-12 #4660 access and activation gate evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ACCESS_SCHEMA = "adl.wp12.access_activation_gate.v1"
PARENT_GATE_SCHEMA = "adl.wp12.security_cav_gate.v1"
ACCESS_ROW_ID = "external_agent_access_rules"


def fail(message: str) -> None:
    raise SystemExit(f"validate_wp12_access_activation_gate_4660: {message}")


def load_json(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"{path} is not valid JSON: {exc}")
    if not isinstance(data, dict):
        fail(f"{path} must contain a JSON object")
    return data


def require_refs_exist(repo_root: Path, refs: list[Any], *, label: str) -> None:
    for ref in refs:
        if not isinstance(ref, str) or not ref:
            fail(f"{label} contains an empty or non-string ref")
        if ref.startswith("https://"):
            continue
        if ref.startswith("/"):
            fail(f"{label} must use repository-relative refs, got {ref}")
        if not (repo_root / ref).exists():
            fail(f"{label} ref does not exist: {ref}")


def validate_access_gate(repo_root: Path, data: dict[str, Any]) -> None:
    if data.get("schema") != ACCESS_SCHEMA:
        fail("access gate has unexpected schema")
    if data.get("issue") != 4660:
        fail("access gate must be issue 4660")
    if data.get("parent_issue") != 4639:
        fail("access gate must point at parent issue 4639")
    if data.get("status") != "access_gate_recorded":
        fail("access gate status must reflect the closed WP-12 owner-issue set")

    required_consumers = {
        "docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md",
        "docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md",
        "docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md",
    }
    consumers = data.get("v092_consumers")
    if set(consumers or []) != required_consumers:
        fail("v0.92 consumers must match the readiness packet, bridge ledger, and checklist")
    require_refs_exist(repo_root, list(required_consumers), label="v092_consumers")

    rules = data.get("access_rules")
    if not isinstance(rules, list) or len(rules) < 7:
        fail("access_rules must list the fail-closed activation rules")
    rule_ids = {rule.get("id") for rule in rules if isinstance(rule, dict)}
    for rule_id in (
        "schema_access_not_content_access",
        "external_agent_trust_requires_access_decision",
        "websocket_carrier_not_runtime_api_activation",
        "ssm_operations_not_secret_authority",
        "custody_signatures_require_trusted_anchor",
        "credential_break_glass_requires_approval_and_rebind",
        "cav_red_blue_claims_require_retained_runtime_scenario",
    ):
        if rule_id not in rule_ids:
            fail(f"missing access rule {rule_id}")
    for rule in rules:
        if not isinstance(rule, dict):
            fail("access_rules entries must be objects")
        if rule.get("decision") not in {"fail_closed", "bounded_allow"}:
            fail(f"rule {rule.get('id')} has invalid decision")
        require_refs_exist(repo_root, rule.get("required_evidence", []), label=f"rule {rule.get('id')} evidence")

    checklist = data.get("activation_checklist")
    if not isinstance(checklist, list):
        fail("activation_checklist must be a list")
    rows = {row.get("owner_issue"): row for row in checklist if isinstance(row, dict)}
    expected_states = {
        4656: "gate_recorded_child_blockers_remaining",
        4657: "integrated_proven",
        4658: "integrated_proven",
        4659: "boundary_proven",
        4660: "access_gate_recorded",
        4914: "boundary_proven",
        4917: "integrated_proven",
        4920: "integrated_proven",
    }
    if set(rows) != set(expected_states):
        fail("activation_checklist owner issues must match WP-12 activation owners")
    for issue, expected_state in expected_states.items():
        row = rows[issue]
        if row.get("state") != expected_state:
            fail(f"owner issue {issue} state must be {expected_state}")
        require_refs_exist(repo_root, row.get("evidence", []), label=f"checklist issue {issue} evidence")

    blockers = data.get("current_blockers")
    if blockers != []:
        fail("current_blockers must be empty after #4659 and PR #5146 closed")

    non_claims = data.get("non_claims")
    if not isinstance(non_claims, list) or len(non_claims) < 5:
        fail("non_claims must preserve transport, x402, and readiness non-claims")


def validate_credential_policy_artifacts(repo_root: Path) -> None:
    summary_path = repo_root / "docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_policy_summary.json"
    event_log_path = repo_root / "docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_lifecycle_events.jsonl"
    summary = load_json(summary_path)
    observability = summary.get("observability")
    if not isinstance(observability, dict):
        fail("credential policy summary must retain observability classification")
    if observability.get("event_origin") != "synthetic_proof_fixture":
        fail("credential policy observability must classify events as synthetic proof fixtures")
    if observability.get("proof_classification") != "synthetic_negative_case":
        fail("credential policy observability must classify negative cases as synthetic")
    if observability.get("operational_audit_stream") is not False:
        fail("credential policy observability must exclude synthetic events from operational audit streams")

    event_lines = event_log_path.read_text(encoding="utf-8").splitlines()
    if not event_lines:
        fail("credential policy event log must retain synthetic events")
    for index, line in enumerate(event_lines, start=1):
        try:
            event = json.loads(line)
        except json.JSONDecodeError as exc:
            fail(f"credential policy event log line {index} is not valid JSON: {exc}")
        if event.get("event_origin") != "synthetic_proof_fixture":
            fail(f"credential policy event log line {index} must classify event_origin")
        if event.get("proof_classification") != "synthetic_negative_case":
            fail(f"credential policy event log line {index} must classify proof_classification")
        if event.get("operational_audit_stream") is not False:
            fail(f"credential policy event log line {index} must set operational_audit_stream=false")


def validate_parent_gate(parent: dict[str, Any]) -> None:
    if parent.get("schema") != PARENT_GATE_SCHEMA:
        fail("parent gate has unexpected schema")
    rows = parent.get("requirements")
    if not isinstance(rows, list):
        fail("parent gate requirements must be a list")
    matches = [row for row in rows if isinstance(row, dict) and row.get("id") == ACCESS_ROW_ID]
    if len(matches) != 1:
        fail("parent gate must contain exactly one external_agent_access_rules row")
    row = matches[0]
    if row.get("owner_issue") != 4660:
        fail("external_agent_access_rules row must be owned by #4660")
    if row.get("state") != "access_gate_recorded":
        fail("external_agent_access_rules row must be access_gate_recorded")
    if row.get("v092_disposition") != "defines_fail_closed_activation_checklist":
        fail("external_agent_access_rules row must define the fail-closed activation checklist")
    required_evidence = "docs/milestones/v0.91.7/review/security/wp12_access_activation_gate_4660.json"
    if required_evidence not in row.get("evidence", []):
        fail("external_agent_access_rules row must cite the #4660 JSON gate")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--access-gate", required=True, type=Path)
    parser.add_argument("--parent-gate", required=True, type=Path)
    args = parser.parse_args()

    repo_root = Path.cwd()
    access_gate = load_json(args.access_gate)
    parent_gate = load_json(args.parent_gate)
    validate_access_gate(repo_root, access_gate)
    validate_credential_policy_artifacts(repo_root)
    validate_parent_gate(parent_gate)
    print("PASS validate_wp12_access_activation_gate_4660")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
