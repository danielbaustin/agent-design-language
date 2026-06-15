#!/usr/bin/env python3
import json
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(f"FAIL validate_v0915_openrouter_matrix: {message}", file=sys.stderr)
    raise SystemExit(1)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load_json(path: Path):
    try:
        return json.loads(path.read_text())
    except Exception as exc:  # pragma: no cover - fail closed
        fail(f"could not parse {path}: {exc}")


def repo_relative(path: str) -> bool:
    return bool(path) and not path.startswith("/") and ".." not in Path(path).parts


def find_repo_root(start: Path) -> Path:
    for candidate in [start, *start.parents]:
        if (candidate / "adl").is_dir() and (candidate / "docs").is_dir():
            return candidate
    fail(f"could not determine repo root from {start}")


def require_rel_path(repo_root: Path, rel: str, key: str) -> Path:
    require(repo_relative(rel), f"{key} must be repo-relative without traversal")
    require(not rel.startswith(".adl/"), f"{key} must not point into local-only .adl state")
    target = repo_root / rel
    require(target.exists(), f"{key} target missing: {rel}")
    return target


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0915_openrouter_matrix.py <packet_dir>")

    packet_dir = Path(sys.argv[1]).resolve()
    repo_root = find_repo_root(packet_dir)
    date = "2026-06-14"

    required = [
        packet_dir / "README.md",
        packet_dir / f"OPENROUTER_MATRIX_PROOF_{date}.md",
        packet_dir / f"openrouter_matrix_state_{date}.json",
        packet_dir / f"catalog_snapshot_{date}.json",
        packet_dir / "provider_invocations.json",
        packet_dir / "provider_setup" / "provider.adl.yaml",
        packet_dir / "provider_setup" / ".env.example",
        packet_dir / "provider_setup" / "README.md",
    ]
    for path in required:
        require(path.exists(), f"missing required packet file: {path}")

    state = load_json(packet_dir / f"openrouter_matrix_state_{date}.json")
    require(state.get("schema") == "adl.openrouter_matrix_proof.v1", "unexpected state schema")
    require(state.get("issue") == 3723, "state issue must be 3723")
    require(state.get("status") == "supported_with_limits", "unexpected state status")

    setup = state.get("setup", {})
    setup_dir = require_rel_path(repo_root, setup.get("provider_setup_dir", ""), "setup.provider_setup_dir")
    require((setup_dir / "provider.adl.yaml").exists(), "provider setup yaml missing")

    catalog = state.get("catalog", {})
    catalog_path = require_rel_path(repo_root, catalog.get("path", ""), "catalog.path")
    catalog_json = load_json(catalog_path)
    ids = {entry.get("id") for entry in catalog_json.get("data", [])}
    selected = catalog.get("selected_models", [])
    require(len(selected) == 5, "expected five selected live models")
    for model in selected:
        require(model in ids, f"selected model missing from catalog snapshot: {model}")

    lanes = state.get("lanes", [])
    require(len(lanes) == 6, "expected five live lanes plus one negative control")
    live_lanes = [lane for lane in lanes if lane.get("role") != "negative_control"]
    require(len(live_lanes) == 5, "expected five live routed lanes")
    supported = [lane for lane in lanes if lane.get("contract_status") == "supported"]
    non_proving = [lane for lane in live_lanes if lane.get("contract_status") == "non_proving"]
    require(
        len(supported) + len(non_proving) == 5,
        "expected every live lane to classify as supported or non_proving",
    )
    for lane in live_lanes:
        require(lane.get("provider") == "openrouter", "live lane provider must be openrouter")
        require(lane.get("final_status") == "ok", "live lane final_status must be ok")
        require(lane.get("http_status") == 200, "live lane http_status must be 200")
        require(
            lane.get("requested_route_completed") is True,
            "live lane must record requested_route_completed=true",
        )
        require(
            lane.get("recorded_route_model") == lane.get("model"),
            "live lane recorded_route_model must match the requested route id",
        )
        require(
            isinstance(lane.get("recorded_identity_model"), str)
            and lane.get("recorded_identity_model", "").strip(),
            "live lane recorded_identity_model must be present",
        )
        require_rel_path(repo_root, lane.get("request_path", ""), f"{lane.get('lane')}.request_path")
        result_path = require_rel_path(repo_root, lane.get("result_path", ""), f"{lane.get('lane')}.result_path")
        require_rel_path(repo_root, lane.get("log_path", ""), f"{lane.get('lane')}.log_path")
        output_path = require_rel_path(repo_root, lane.get("output_path", ""), f"{lane.get('lane')}.output_path")
        result = load_json(result_path)
        require(result.get("final_status") == "ok", f"{lane.get('lane')} result json must be ok")
        require(
            (result.get("route") or {}).get("provider_model_id") == lane.get("model"),
            f"{lane.get('lane')} result route.provider_model_id must match claimed model",
        )
        require(
            (result.get("model_identity") or {}).get("provider_model_id")
            == lane.get("recorded_identity_model"),
            f"{lane.get('lane')} result model_identity.provider_model_id must match recorded identity",
        )
        require((result.get("output_text") or "").strip(), f"{lane.get('lane')} output_text must be present")
        output_text = output_path.read_text()
        require("Provider: `openrouter`" in output_text, f"{lane.get('lane')} output markdown missing provider banner")

    negative = next((lane for lane in lanes if lane.get("role") == "negative_control"), None)
    require(negative is not None, "missing negative control lane")
    require(
        negative.get("contract_status") == "blocked_missing_credential",
        "negative control must record blocked_missing_credential",
    )
    require(negative.get("failure_kind") == "provider_auth_missing", "negative control failure kind must be provider_auth_missing")
    negative_result = load_json(
        require_rel_path(repo_root, negative.get("result_path", ""), "negative_control.result_path")
    )
    require(negative_result.get("final_status") == "failed", "negative control result must fail")

    invocations = load_json(packet_dir / "provider_invocations.json")
    require(
        invocations.get("schema_version") == "adl.native_provider_invocations.v1",
        "unexpected provider invocation artifact schema",
    )
    invocation_rows = invocations.get("invocations", [])
    require(len(invocation_rows) == 5, "expected exactly five successful live provider invocations")
    for row in invocation_rows:
        require(row.get("family") == "openrouter", "invocation family must be openrouter")
        require(row.get("http_status") == 200, "invocation http_status must be 200")

    prior = state.get("prior_non_proving_evidence", {})
    require(prior.get("issue") == 3415, "prior non-proving evidence must point at #3415")
    require_rel_path(repo_root, prior.get("path", ""), "prior_non_proving_evidence.path")

    packet_text = (packet_dir / f"OPENROUTER_MATRIX_PROOF_{date}.md").read_text()
    for phrase in [
        "## Supported Paths",
        "## Blocked Paths",
        "## Flaky Paths",
        "## Non-Proving Paths",
        "no universal OpenRouter compatibility claim",
        "tool-call capability still unproven",
        "five requested route IDs",
    ]:
        require(phrase in packet_text, f"packet missing required phrase: {phrase}")

    for path in required:
        text = path.read_text() if path.suffix in {".md", ".json", ".yaml", ".example"} or path.name.endswith(".yaml") else ""
        require("OPENROUTER_API_KEY=" not in text or "replace-me" in text, f"unexpected secret-like content in {path}")
        require(str(Path.home() / "keys" / "openrouter.key") not in text, f"key file path leaked in {path}")

    print("PASS validate_v0915_openrouter_matrix")


if __name__ == "__main__":
    main()
