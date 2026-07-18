#!/usr/bin/env python3
"""Validate WP-12 #4914 CSM CAV red-blue proof evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


PROOF_SCHEMA = "adl.wp12.csm_cav_red_blue_proof.v1"
PARENT_GATE_SCHEMA = "adl.wp12.security_cav_gate.v1"
COHERENCE_SCHEMA = "adl.v0917.final_csm_runtime_coherence_gate.v1"
CAV_ROW_ID = "cav_runtime_red_blue_proof"

REQUIRED_SCENARIOS = {
    "malformed_snapshot",
    "unauthorized_control_command",
    "telemetry_injection",
    "credential_path_leakage",
    "replay_tampering",
    "cloud_hook_denial",
}


def fail(message: str) -> None:
    raise SystemExit(f"validate_wp12_cav_red_blue_4914: {message}")


def load_json(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"{path} is not valid JSON: {exc}")
    if not isinstance(data, dict):
        fail(f"{path} must contain a JSON object")
    return data


def require_refs_exist(repo_root: Path, refs: list[Any], *, base: Path | None = None, label: str) -> None:
    for ref in refs:
        if not isinstance(ref, str) or not ref:
            fail(f"{label} contains an empty or non-string ref")
        if ref.startswith("https://"):
            continue
        if ref.startswith("/"):
            fail(f"{label} must use repository-relative refs, got {ref}")
        path = (base / ref) if base is not None else (repo_root / ref)
        if not path.exists():
            fail(f"{label} ref does not exist: {ref}")


def validate_proof(repo_root: Path, proof_path: Path, proof: dict[str, Any]) -> None:
    if proof.get("schema") != PROOF_SCHEMA:
        fail("proof has unexpected schema")
    if proof.get("issue") != 4914:
        fail("proof must be issue 4914")
    if proof.get("parent_issue") != 4639 or proof.get("sprint_issue") != 4656:
        fail("proof must preserve WP-12 issue lineage")
    if proof.get("status") != "passed_with_bounded_residuals":
        fail("proof status must be passed_with_bounded_residuals")
    operator_ref = proof.get("operator_ref")
    if (
        not isinstance(operator_ref, str)
        or len(operator_ref) != 16
        or operator_ref == "operator_identity_hash_only"
    ):
        fail("operator_ref must be a retained short hash, not a literal label")

    runtime = proof.get("runtime_surface")
    if not isinstance(runtime, dict):
        fail("runtime_surface must be an object")
    if runtime.get("owner_binary") != "csm":
        fail("runtime_surface must be owned by csm")
    if runtime.get("integrated_csm_path") is not False:
        fail("runtime_surface must not claim integrated_csm_path without live boundary-crossing proof")
    if runtime.get("http_runtime_api_integrated") is not False:
        fail("HTTP runtime API integration must remain a non-claim")
    if runtime.get("websocket_runtime_api_integrated") is not False:
        fail("WebSocket runtime API integration must remain a non-claim")
    if runtime.get("cloud_hook_mode") != "local_denial_no_aws_mutation":
        fail("cloud hook mode must be local denial with no AWS mutation")

    redaction = proof.get("redaction")
    if not isinstance(redaction, dict):
        fail("redaction must be an object")
    for key in (
        "secret_values_retained",
        "raw_credential_paths_retained",
        "host_private_paths_retained",
        "cloud_mutation_performed",
    ):
        if redaction.get(key) is not False:
            fail(f"redaction.{key} must be false")

    scenarios = proof.get("red_blue_scenarios")
    if not isinstance(scenarios, list):
        fail("red_blue_scenarios must be a list")
    scenario_ids = {row.get("id") for row in scenarios if isinstance(row, dict)}
    missing = REQUIRED_SCENARIOS - scenario_ids
    if missing:
        fail(f"missing required scenarios: {sorted(missing)}")
    for row in scenarios:
        if not isinstance(row, dict):
            fail("red_blue_scenarios entries must be objects")
        if row.get("runs_end_to_end") is True or row.get("integrated_csm_path") is True:
            fail(f"scenario {row.get('id')} must not claim integrated end-to-end CSM execution")
        if row.get("decision") not in {"refused", "detected"}:
            fail(f"scenario {row.get('id')} has invalid decision")
        if not row.get("executed_control") or not row.get("observed_result"):
            fail(f"scenario {row.get('id')} must record executed control evidence")
        if row.get("secret_material_retained") is not False:
            fail(f"scenario {row.get('id')} retained secret material")
        if row.get("host_path_retained") is not False:
            fail(f"scenario {row.get('id')} retained host paths")

    register = proof.get("pass_fail_register")
    if not isinstance(register, list) or len(register) != len(scenarios):
        fail("pass_fail_register must cover every scenario")
    if {row.get("scenario_id") for row in register if isinstance(row, dict)} != scenario_ids:
        fail("pass_fail_register scenario IDs must match red_blue_scenarios")
    for row in register:
        if row.get("result") != "pass":
            fail(f"scenario {row.get('scenario_id')} is not pass")
        if not row.get("residual_risk"):
            fail(f"scenario {row.get('scenario_id')} must record residual risk")

    proof_dir = proof_path.parent
    require_refs_exist(repo_root, proof.get("retained_artifacts", []), base=proof_dir, label="retained_artifacts")
    for artifact in proof.get("retained_artifacts", []):
        text = (proof_dir / artifact).read_text(encoding="utf-8")
        for forbidden in ("PRIVATE KEY", "token=", "/Users/", "\\Users\\"):
            if forbidden in text:
                fail(f"retained artifact {artifact} contains forbidden marker {forbidden}")


def validate_parent_gate(parent: dict[str, Any]) -> None:
    if parent.get("schema") != PARENT_GATE_SCHEMA:
        fail("parent gate has unexpected schema")
    rows = parent.get("requirements")
    if not isinstance(rows, list):
        fail("parent gate requirements must be a list")
    matches = [row for row in rows if isinstance(row, dict) and row.get("id") == CAV_ROW_ID]
    if len(matches) != 1:
        fail("parent gate must contain exactly one CAV row")
    row = matches[0]
    if row.get("owner_issue") != 4914:
        fail("CAV row must be owned by #4914")
    if row.get("state") != "boundary_proven":
        fail("CAV row must be boundary_proven until live boundary-crossing proof exists")
    required_evidence = "docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json"
    if required_evidence not in row.get("evidence", []):
        fail("CAV row must cite the #4914 summary")


def validate_coherence(coherence: dict[str, Any]) -> None:
    if coherence.get("schema") != COHERENCE_SCHEMA:
        fail("coherence matrix has unexpected schema")
    rows = coherence.get("rows")
    if not isinstance(rows, list):
        fail("coherence rows must be a list")
    matches = [row for row in rows if isinstance(row, dict) and row.get("id") == "wp12_acip_a2a_access_activation"]
    if len(matches) != 1:
        fail("coherence matrix must contain the WP-12 row")
    row = matches[0]
    evidence = row.get("evidence_refs", [])
    required = "docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json"
    if required not in evidence:
        fail("coherence WP-12 row must consume the #4914 proof")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--proof", required=True, type=Path)
    parser.add_argument("--parent-gate", required=True, type=Path)
    parser.add_argument("--coherence", required=True, type=Path)
    args = parser.parse_args()

    repo_root = Path.cwd()
    proof = load_json(args.proof)
    parent = load_json(args.parent_gate)
    coherence = load_json(args.coherence)
    validate_proof(repo_root, args.proof, proof)
    validate_parent_gate(parent)
    validate_coherence(coherence)
    print("PASS validate_wp12_cav_red_blue_4914")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
