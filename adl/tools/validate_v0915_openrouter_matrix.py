#!/usr/bin/env python3
import json
import re
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


def require_redacted_excerpt(
    value: object,
    key: str,
    expected_label: str,
    expected_chars: object,
    expected_digest: object,
) -> None:
    require(isinstance(value, str) and value.strip(), f"{key} must be a non-empty string")
    require(isinstance(expected_chars, int) and expected_chars > 0, f"{key} sibling chars field must be > 0")
    require(isinstance(expected_digest, str) and expected_digest.startswith("sha256:"), f"{key} sibling digest must start with sha256:")
    match = re.fullmatch(
        rf"\[redacted {re.escape(expected_label)} len=(\d+) sha256=([0-9a-f]{{12}})\]",
        value,
    )
    require(match is not None, f"{key} must be a redacted {expected_label} excerpt with a sha256 digest marker")
    marker_chars = int(match.group(1))
    marker_digest = match.group(2)
    require(marker_chars == expected_chars, f"{key} len marker must match sibling chars field")
    require(f"sha256:{marker_digest}" == expected_digest, f"{key} sha256 marker must match sibling digest field")


def require_digest(value: object, key: str) -> None:
    require(isinstance(value, str) and value.startswith("sha256:"), f"{key} must start with sha256:")
    digest = value.split(":", 1)[1]
    require(len(digest) == 12, f"{key} digest must use the short 12-character sha256 form")


def load_jsonl(path: Path) -> list[dict]:
    rows = []
    for line_no, raw_line in enumerate(path.read_text().splitlines(), start=1):
        line = raw_line.strip()
        require(line, f"{path} line {line_no} must not be blank")
        try:
            payload = json.loads(line)
        except Exception as exc:  # pragma: no cover - fail closed
            fail(f"could not parse {path} line {line_no}: {exc}")
        require(isinstance(payload, dict), f"{path} line {line_no} must decode to an object")
        rows.append(payload)
    require(rows, f"{path} must contain at least one JSONL event")
    return rows


def require_log_redaction(log_rows: list[dict], key: str) -> None:
    allowed_event_types = {"run_start", "attempt_start", "attempt_success", "attempt_failure", "run_finish"}
    for index, row in enumerate(log_rows, start=1):
        event_key = f"{key}[{index}]"
        require(row.get("schema_version") == "provider_communication.v1", f"{event_key} schema_version must be provider_communication.v1")
        require(isinstance(row.get("event_type"), str) and row["event_type"] in allowed_event_types, f"{event_key} event_type must be recognized")
        forbidden_keys = {
            "input_text",
            "output_text",
            "raw_response",
            "raw_prompt",
            "prompt",
            "response",
            "messages",
            "authorization",
        }
        for forbidden_key in forbidden_keys:
            require(forbidden_key not in row, f"{event_key} must not retain raw field {forbidden_key}")
        serialized = json.dumps(row).lower()
        forbidden_substrings = (
            "openrouter_api_key",
            "authorization",
            "bearer ",
            "\"input_text\"",
            "\"output_text\"",
            "# plan\n",
            "# finding\n",
            "bounded_to_issue_3723",
            "adl_openrouter_negative_control",
        )
        for substring in forbidden_substrings:
            require(substring not in serialized, f"{event_key} must not contain raw prompt/output or credential material")


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
        packet_dir / "provider_setup" / "env.example",
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
        require(isinstance(lane.get("prompt_chars"), int) and lane.get("prompt_chars") > 0, f"{lane.get('lane')} prompt_chars must be > 0")
        require(isinstance(lane.get("output_chars"), int) and lane.get("output_chars") > 0, f"{lane.get('lane')} output_chars must be > 0")
        require_digest(lane.get("prompt_digest"), f"{lane.get('lane')}.prompt_digest")
        require_redacted_excerpt(
            lane.get("prompt_excerpt"),
            f"{lane.get('lane')}.prompt_excerpt",
            "prompt",
            lane.get("prompt_chars"),
            lane.get("prompt_digest"),
        )
        require_digest(lane.get("output_digest"), f"{lane.get('lane')}.output_digest")
        require_redacted_excerpt(
            lane.get("output_excerpt"),
            f"{lane.get('lane')}.output_excerpt",
            "response",
            lane.get("output_chars"),
            lane.get("output_digest"),
        )
        request_path = require_rel_path(repo_root, lane.get("request_path", ""), f"{lane.get('lane')}.request_path")
        result_path = require_rel_path(repo_root, lane.get("result_path", ""), f"{lane.get('lane')}.result_path")
        log_path = require_rel_path(repo_root, lane.get("log_path", ""), f"{lane.get('lane')}.log_path")
        output_path = require_rel_path(repo_root, lane.get("output_path", ""), f"{lane.get('lane')}.output_path")
        request = load_json(request_path)
        result = load_json(result_path)
        require("input_text" not in request, f"{lane.get('lane')} request json must not retain raw input_text")
        require(isinstance(request.get("input_text_chars"), int) and request.get("input_text_chars") > 0, f"{lane.get('lane')} request input_text_chars must be > 0")
        require_digest(request.get("input_text_digest"), f"{lane.get('lane')}.request.input_text_digest")
        require_redacted_excerpt(
            request.get("input_text_excerpt"),
            f"{lane.get('lane')}.request.input_text_excerpt",
            "prompt",
            request.get("input_text_chars"),
            request.get("input_text_digest"),
        )
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
        require("output_text" not in result, f"{lane.get('lane')} result json must not retain raw output_text")
        require(isinstance(result.get("output_text_chars"), int) and result.get("output_text_chars") > 0, f"{lane.get('lane')} result output_text_chars must be > 0")
        require_digest(result.get("output_text_digest"), f"{lane.get('lane')}.result.output_text_digest")
        require_redacted_excerpt(
            result.get("output_text_excerpt"),
            f"{lane.get('lane')}.result.output_text_excerpt",
            "response",
            result.get("output_text_chars"),
            result.get("output_text_digest"),
        )
        attempts = result.get("attempts") or []
        require(attempts, f"{lane.get('lane')} result attempts must be present")
        require_redacted_excerpt(
            attempts[0].get("raw_response_excerpt"),
            f"{lane.get('lane')}.attempts[0].raw_response_excerpt",
            "response",
            result.get("output_text_chars"),
            result.get("output_text_digest"),
        )
        log_rows = load_jsonl(log_path)
        require_log_redaction(log_rows, f"{lane.get('lane')}.log")
        output_text = output_path.read_text()
        require("Provider: `openrouter`" in output_text, f"{lane.get('lane')} output markdown missing provider banner")

    negative = next((lane for lane in lanes if lane.get("role") == "negative_control"), None)
    require(negative is not None, "missing negative control lane")
    require(
        negative.get("contract_status") == "blocked_missing_credential",
        "negative control must record blocked_missing_credential",
    )
    require(negative.get("failure_kind") == "provider_auth_missing", "negative control failure kind must be provider_auth_missing")
    require(isinstance(negative.get("prompt_chars"), int) and negative.get("prompt_chars") > 0, "negative control prompt_chars must be > 0")
    require_digest(negative.get("prompt_digest"), "negative_control.prompt_digest")
    require_redacted_excerpt(
        negative.get("prompt_excerpt"),
        "negative_control.prompt_excerpt",
        "prompt",
        negative.get("prompt_chars"),
        negative.get("prompt_digest"),
    )
    negative_request = load_json(
        require_rel_path(repo_root, negative.get("request_path", ""), "negative_control.request_path")
    )
    require("input_text" not in negative_request, "negative control request json must not retain raw input_text")
    require_digest(negative_request.get("input_text_digest"), "negative_control.request.input_text_digest")
    require_redacted_excerpt(
        negative_request.get("input_text_excerpt"),
        "negative_control.request.input_text_excerpt",
        "prompt",
        negative_request.get("input_text_chars"),
        negative_request.get("input_text_digest"),
    )
    negative_result = load_json(
        require_rel_path(repo_root, negative.get("result_path", ""), "negative_control.result_path")
    )
    require(negative_result.get("final_status") == "failed", "negative control result must fail")
    require("output_text" not in negative_result, "negative control result json must not retain raw output_text")
    negative_log_rows = load_jsonl(
        require_rel_path(repo_root, negative.get("log_path", ""), "negative_control.log_path")
    )
    require_log_redaction(negative_log_rows, "negative_control.log")

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
        require(isinstance(row.get("prompt_chars"), int) and row.get("prompt_chars") > 0, "invocation prompt_chars must be > 0")
        require(isinstance(row.get("output_chars"), int) and row.get("output_chars") > 0, "invocation output_chars must be > 0")
        require_digest(row.get("prompt_digest"), "invocation.prompt_digest")
        require_redacted_excerpt(
            row.get("prompt_excerpt"),
            "invocation.prompt_excerpt",
            "prompt",
            row.get("prompt_chars"),
            row.get("prompt_digest"),
        )
        require_digest(row.get("output_digest"), "invocation.output_digest")
        require_redacted_excerpt(
            row.get("output_excerpt"),
            "invocation.output_excerpt",
            "response",
            row.get("output_chars"),
            row.get("output_digest"),
        )

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
        "redacted prompt/output excerpts plus SHA-256 digests",
    ]:
        require(phrase in packet_text, f"packet missing required phrase: {phrase}")

    for path in required:
        text = path.read_text() if path.suffix in {".md", ".json", ".yaml", ".example"} or path.name.endswith(".yaml") else ""
        require("OPENROUTER_API_KEY=" not in text or "replace-me" in text, f"unexpected secret-like content in {path}")
        require(str(Path.home() / "keys" / "openrouter.key") not in text, f"key file path leaked in {path}")

    print("PASS validate_v0915_openrouter_matrix")


if __name__ == "__main__":
    main()
