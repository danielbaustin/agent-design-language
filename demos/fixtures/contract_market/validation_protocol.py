#!/usr/bin/env python3
"""
Validation protocol for the v0.90.4 contract-market fixture packet.
"""

from __future__ import annotations

from pathlib import Path
import json
import sys


ROOT = Path(__file__).resolve().parent
NEGATIVE_ROOT = ROOT.parent / "contract_market_invalid"


def load_json(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def contains_host_path(value) -> bool:
    if isinstance(value, str):
        return value.startswith("/Users/") or "file://" in value
    if isinstance(value, list):
        return any(contains_host_path(item) for item in value)
    if isinstance(value, dict):
        return any(contains_host_path(item) for item in value.values())
    return False


def validate_manifest() -> bool:
    ok = True
    manifest_path = ROOT / "packet_manifest.json"
    manifest = load_json(manifest_path)
    if manifest.get("schema") != "adl.v0904.contract_market.packet_manifest.v1":
        print("manifest_schema_mismatch")
        ok = False
    artifacts = manifest.get("artifacts", [])
    if len(artifacts) < 10:
        print("artifact_list_too_small")
        ok = False
    for artifact in artifacts:
        rel = artifact.get("path")
        purpose = artifact.get("proof_purpose")
        if not rel or not purpose:
            print(f"manifest_entry_incomplete: {artifact}")
            ok = False
            continue
        path = ROOT / rel
        print(f"artifact_path: {rel} exists={path.exists()}")
        if not path.exists():
            ok = False
            continue
        payload = load_json(path)
        if contains_host_path(payload):
            print(f"host_path_leak: {rel}")
            ok = False
    return ok


def validate_primary_packet() -> bool:
    ok = True
    required = [
        "parent_contract.json",
        "bid_alpha.json",
        "bid_beta.json",
        "evaluation.json",
        "award_transition.json",
        "acceptance_transition.json",
        "subcontract.json",
        "delegated_output.json",
        "parent_integration_output.json",
        "completion_event.json",
        "trace_bundle.json",
        "review_summary_seed.json",
        "demo_manifest.json",
        "tool_requirement_fixture.json",
    ]
    for rel in required:
        path = ROOT / rel
        try:
            payload = load_json(path)
            if contains_host_path(payload):
                print(f"host_path_leak: {rel}")
                ok = False
            else:
                print(f"json_ok: {rel}")
        except Exception as exc:  # noqa: BLE001
            print(f"json_invalid: {rel}: {exc}")
            ok = False
    tool_fixture = load_json(ROOT / "tool_requirement_fixture.json")
    if tool_fixture["recorded_requirement"]["execution_authority"] != "not_granted":
        print("tool_requirement_grants_authority")
        ok = False
    return ok


def validate_negative_packet() -> bool:
    ok = True
    manifest = load_json(NEGATIVE_ROOT / "invalid_packet_manifest.json")
    reasons = manifest.get("expected_invalid_reasons", [])
    if len(reasons) < 2:
        print("negative_reasons_too_small")
        ok = False
    invalid_bid = load_json(NEGATIVE_ROOT / "invalid_bid_tool_grant.json")
    if invalid_bid["tool_requirements"][0]["execution_authority"] != "granted":
        print("negative_bid_not_invalid")
        ok = False
    invalid_completion = load_json(NEGATIVE_ROOT / "invalid_completion_missing_artifacts.json")
    if invalid_completion.get("required_artifact_refs"):
        print("invalid_completion_has_required_artifacts")
        ok = False
    print(f"negative_packet: {NEGATIVE_ROOT}")
    return ok


def main() -> int:
    ok = True
    fixture_definition = ROOT / "fixture_definition.md"
    if not fixture_definition.exists():
        print("missing_fixture_definition")
        ok = False
    else:
        print(f"fixture_definition: {fixture_definition}")
    ok = validate_manifest() and ok
    ok = validate_primary_packet() and ok
    ok = validate_negative_packet() and ok
    print(f"protocol_health: {'pass' if ok else 'fail'}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
