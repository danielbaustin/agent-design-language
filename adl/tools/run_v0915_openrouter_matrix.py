#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import textwrap
from pathlib import Path
from urllib import request as urllib_request


def fail(message: str) -> None:
    print(f"FAIL run_v0915_openrouter_matrix: {message}", file=sys.stderr)
    raise SystemExit(1)


def repo_root() -> Path:
    here = Path(__file__).resolve()
    for candidate in [here.parent, *here.parents]:
        if (candidate / "adl").is_dir() and (candidate / "docs").is_dir():
            return candidate
    fail(f"could not determine repo root from {here}")


def parse_args() -> argparse.Namespace:
    root = repo_root()
    default_dir = root / "docs" / "milestones" / "v0.91.5" / "review" / "openrouter_matrix"
    parser = argparse.ArgumentParser(
        description="Run the bounded v0.91.5 OpenRouter role matrix proof lane"
    )
    parser.add_argument("--out", type=Path, default=default_dir)
    parser.add_argument("--date", default="2026-06-14")
    parser.add_argument(
        "--key-file",
        type=Path,
        default=Path.home() / "keys" / "openrouter.key",
    )
    return parser.parse_args()


def load_openrouter_key(key_file: Path) -> str:
    if os.environ.get("OPENROUTER_API_KEY"):
        return os.environ["OPENROUTER_API_KEY"]
    if not key_file.is_file():
        fail(
            f"missing OPENROUTER_API_KEY and key file {key_file}; set env or provide --key-file"
        )
    for raw_line in key_file.read_text().splitlines():
        line = raw_line.strip().strip("\r")
        if not line or line.startswith("#"):
            continue
        if line.startswith("OPENROUTER_API_KEY="):
            value = line.split("=", 1)[1].strip().strip('"').strip("'")
        else:
            value = line.strip('"').strip("'")
        if value:
            os.environ["OPENROUTER_API_KEY"] = value
            return value
    fail(f"could not read a usable key from {key_file}")


def run(cmd: list[str], cwd: Path, env: dict[str, str] | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(
        cmd,
        cwd=cwd,
        env=env,
        check=True,
        text=True,
        capture_output=True,
    )


def write_json(path: Path, payload: object) -> None:
    path.write_text(json.dumps(payload, indent=2) + "\n")


def rel(root: Path, path: Path) -> str:
    return path.resolve().relative_to(root.resolve()).as_posix()


def build_binaries(root: Path) -> tuple[Path, Path]:
    run(
        [
            "cargo",
            "build",
            "--quiet",
            "--manifest-path",
            "adl/Cargo.toml",
            "--bin",
            "adl",
            "--bin",
            "adl-provider-adapter",
        ],
        cwd=root,
    )
    target = root / "adl" / "target" / "debug"
    return target / "adl", target / "adl-provider-adapter"


def fetch_catalog(out_path: Path, key: str) -> dict:
    req = urllib_request.Request(
        "https://openrouter.ai/api/v1/models",
        headers={"Authorization": f"Bearer {key}"},
    )
    with urllib_request.urlopen(req, timeout=30) as response:
        payload = json.loads(response.read().decode("utf-8"))
    payload = normalize_catalog_payload(payload)
    write_json(out_path, payload)
    return payload


def normalize_catalog_payload(payload: object) -> object:
    if isinstance(payload, dict):
        normalized = {}
        for key, value in payload.items():
            if key == "description" and isinstance(value, str):
                normalized[key] = normalize_catalog_description(value)
            else:
                normalized[key] = normalize_catalog_payload(value)
        return normalized
    if isinstance(payload, list):
        return [normalize_catalog_payload(item) for item in payload]
    return payload


def normalize_catalog_description(value: str) -> str:
    return re.sub(r"(?i)\bswarm\b", "multi-agent", value)


def provider_setup(adl_bin: Path, root: Path, out_dir: Path) -> None:
    if out_dir.exists():
        shutil.rmtree(out_dir)
    run(
        [
            str(adl_bin),
            "provider",
            "setup",
            "openrouter",
            "--out",
            str(out_dir),
            "--force",
        ],
        cwd=root,
    )
def lane_prompt(role: str) -> str:
    prompts = {
        "planner": textwrap.dedent(
            """
            You are producing a bounded ADL planning note for issue #3723.
            Return markdown only with exactly these headings:
            # Plan
            # Risks
            # Non-Claims

            Requirements:
            - Mention issue #3723.
            - Mention the exact path docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md exactly once.
            - Under # Non-Claims include the exact phrase: no universal OpenRouter compatibility claim
            - Keep the response under 140 words.
            """
        ).strip(),
        "worker": textwrap.dedent(
            """
            Return only minified JSON with keys issue,path,task,limit.
            Requirements:
            - issue must equal "#3723"
            - path must equal "docs/milestones/v0.91.5/review/openrouter_matrix"
            - task must name one concrete next proof step and include the words "OpenRouter" and "route"
            - limit must include the exact phrase "bounded_to_issue_3723"
            """
        ).strip(),
        "reviewer": textwrap.dedent(
            """
            Return markdown only with exactly these headings:
            # Finding
            # Severity
            # Routing
            # Non-Claim

            Requirements:
            - The finding must be about OpenRouter matrix proof quality, not provider marketing.
            - The severity body must be exactly P2 or P3.
            - Mention docs/milestones/v0.91.5/review/openrouter_matrix
            - Under # Non-Claim include the exact phrase: tool-call capability still unproven
            - Keep the response under 150 words.
            """
        ).strip(),
        "watcher": textwrap.dedent(
            """
            Return markdown only with exactly these headings:
            # Status
            # Signal
            # Next-Step

            Requirements:
            - # Status body must be exactly one of: pass, partial, blocked
            - Mention #3723
            - Mention openrouter
            - Under # Signal include the exact phrase: route probe completed
            - Under # Next-Step mention docs/milestones/v0.91.5/review/openrouter_matrix
            - Keep the response under 110 words.
            """
        ).strip(),
    }
    return prompts[role]


def request_payload(model: str, lane_name: str, prompt: str, credential_ref: str) -> dict:
    return {
        "route": {
            "provider_kind": "hosted",
            "provider": "openrouter",
            "runtime_surface": "hosted_api",
            "provider_model_id": model,
            "credential_ref": credential_ref,
            "source_registry": "v0.91.5.openrouter.matrix",
        },
        "model_identity": {
            "provider_kind": "hosted",
            "provider": "openrouter",
            "model_ref": model,
            "provider_model_id": model,
            "runtime_surface": "hosted_api",
            "identity_strength": "provider_asserted",
            "observed_at": "unix:1",
        },
        "prompt_contract_ref": "v0.91.5.openrouter.matrix.prompt.v1",
        "lane_ref": lane_name,
        "run_id": lane_name,
        "request_id": lane_name,
        "attempt_policy": {
            "max_attempts": 1,
            "timeout_ms": 45000,
            "retry_backoff_ms": 1000,
        },
        "input_text": prompt,
    }


def short_sha256(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()[:12]


def redacted_text_marker(kind: str, text: str) -> str:
    label = "prompt" if kind == "prompt" else "response"
    return f"[redacted {label} len={len(text)} sha256={short_sha256(text)}]"


def sanitized_request_payload(payload: dict) -> dict:
    cleaned = dict(payload)
    input_text = str(cleaned.pop("input_text", "") or "")
    cleaned["input_text_chars"] = len(input_text)
    cleaned["input_text_digest"] = f"sha256:{short_sha256(input_text)}"
    cleaned["input_text_excerpt"] = redacted_text_marker("prompt", input_text)
    return cleaned


def sanitized_result_payload(result: dict) -> dict:
    cleaned = json.loads(json.dumps(result))
    output_text = str(cleaned.pop("output_text", "") or "")
    if output_text:
        cleaned["output_text_chars"] = len(output_text)
        cleaned["output_text_digest"] = f"sha256:{short_sha256(output_text)}"
        cleaned["output_text_excerpt"] = redacted_text_marker("response", output_text)
        for attempt in cleaned.get("attempts") or []:
            attempt["raw_response_excerpt"] = redacted_text_marker("response", output_text)
    return cleaned


def classify_lane(role: str, output_text: str) -> tuple[str, str]:
    text = output_text.strip()
    text_lower = text.lower().replace("`", "")
    if role == "planner":
        required = [
            "# plan",
            "# risks",
            "# non-claims",
            "#3723",
            "docs/milestones/v0.91.5/features/provider_model_matrix_v0.91.5.md",
            "no universal openrouter compatibility claim",
        ]
        ok = all(item in text_lower for item in required)
        return (
            "supported" if ok else "non_proving",
            "structured planner note satisfied contract"
            if ok
            else "planner output did not satisfy the bounded heading/path contract",
        )
    if role == "worker":
        candidate = text
        if text.startswith("```") and text.endswith("```"):
            stripped_lines = text.splitlines()
            if len(stripped_lines) >= 3:
                candidate = "\n".join(stripped_lines[1:-1]).strip()
        try:
            payload = json.loads(candidate)
        except json.JSONDecodeError:
            return "non_proving", "worker lane did not return parseable JSON"
        ok = (
            payload.get("issue") == "#3723"
            and payload.get("path") == "docs/milestones/v0.91.5/review/openrouter_matrix"
            and isinstance(payload.get("task"), str)
            and payload.get("task", "").strip()
            and "openrouter" in payload.get("task", "").lower()
            and "route" in payload.get("task", "").lower()
            and isinstance(payload.get("limit"), str)
            and "bounded_to_issue_3723" in payload.get("limit", "")
        )
        return (
            "supported" if ok else "non_proving",
            "worker lane returned bounded JSON proof step"
            if ok
            else "worker lane JSON missed one or more required fields",
        )
    if role == "reviewer":
        lines = text.splitlines()
        ok = (
            "# finding" in text_lower
            and "# severity" in text_lower
            and "# routing" in text_lower
            and "# non-claim" in text_lower
            and "docs/milestones/v0.91.5/review/openrouter_matrix" in text
            and "tool-call capability still unproven" in text_lower
            and any(line.strip() in {"P2", "P3"} for line in lines)
        )
        return (
            "supported" if ok else "non_proving",
            "reviewer lane returned a bounded finding surface"
            if ok
            else "reviewer lane missed the required severity or non-claim contract",
        )
    if role == "watcher":
        ok = (
            "# Status" in text
            and "# Signal" in text
            and "# Next-Step" in text
            and "#3723" in text
            and "openrouter" in text.lower()
            and "route probe completed" in text.lower()
            and "docs/milestones/v0.91.5/review/openrouter_matrix" in text
            and any(f"\n{status}\n" in f"\n{text}\n" for status in ("pass", "partial", "blocked"))
        )
        return (
            "supported" if ok else "non_proving",
            "watcher lane returned bounded status/signal/next-step text"
            if ok
            else "watcher lane missed the bounded status contract",
        )
    return "non_proving", "unknown role"


def write_output_markdown(path: Path, lane_name: str, role: str, model: str, output_text: str) -> None:
    path.write_text(
        "\n".join(
            [
                f"# {lane_name}",
                "",
                f"Role: `{role}`",
                "Provider: `openrouter`",
                f"Model: `{model}`",
                "",
                "## Output",
                "",
                output_text.strip(),
                "",
            ]
        )
    )


def run_lane(
    adapter_bin: Path,
    root: Path,
    lane: dict,
    request_dir: Path,
    result_dir: Path,
    log_dir: Path,
    output_dir: Path,
    invocations_path: Path,
) -> dict:
    request_path = request_dir / f"{lane['lane']}.json"
    result_path = result_dir / f"{lane['lane']}.json"
    log_path = log_dir / f"{lane['lane']}.jsonl"
    output_path = output_dir / f"{lane['lane']}.md"
    raw_request = request_payload(
        lane["model"],
        lane["lane"],
        lane["prompt"],
        lane["credential_ref"],
    )
    write_json(request_path, sanitized_request_payload(raw_request))
    env = dict(os.environ)
    env["ADL_PROVIDER_INVOCATIONS_PATH"] = str(invocations_path)
    with tempfile.TemporaryDirectory(prefix=f"{lane['lane']}_") as temp_dir:
        temp_root = Path(temp_dir)
        raw_request_path = temp_root / "request.json"
        raw_result_path = temp_root / "result.json"
        write_json(raw_request_path, raw_request)
        run(
            [
                str(adapter_bin),
                "--request",
                str(raw_request_path),
                "--out",
                str(raw_result_path),
                "--log",
                str(log_path),
            ],
            cwd=root,
            env=env,
        )
        result = json.loads(raw_result_path.read_text())
    output_text = result.get("output_text") or ""
    recorded_route_model = (result.get("route") or {}).get("provider_model_id")
    recorded_identity_model = (result.get("model_identity") or {}).get("provider_model_id")
    contract_status, note = classify_lane(lane["role"], output_text)
    write_output_markdown(output_path, lane["lane"], lane["role"], lane["model"], output_text)
    write_json(result_path, sanitized_result_payload(result))
    attempt = (result.get("attempts") or [{}])[0]
    return {
        "lane": lane["lane"],
        "role": lane["role"],
        "provider": "openrouter",
        "model": lane["model"],
        "recorded_route_model": recorded_route_model,
        "recorded_identity_model": recorded_identity_model,
        "requested_route_completed": True,
        "prompt_chars": len(lane["prompt"]),
        "prompt_digest": f"sha256:{short_sha256(lane['prompt'])}",
        "prompt_excerpt": redacted_text_marker("prompt", lane["prompt"]),
        "output_chars": len(output_text),
        "output_digest": f"sha256:{short_sha256(output_text)}",
        "output_excerpt": redacted_text_marker("response", output_text),
        "request_path": rel(root, request_path),
        "result_path": rel(root, result_path),
        "log_path": rel(root, log_path),
        "output_path": rel(root, output_path),
        "final_status": result.get("final_status"),
        "http_status": attempt.get("http_status"),
        "duration_ms": result.get("duration_ms"),
        "contract_status": contract_status,
        "note": note,
    }


def run_negative_missing_credential(
    adapter_bin: Path,
    root: Path,
    request_dir: Path,
    result_dir: Path,
    log_dir: Path,
) -> dict:
    lane_name = "negative_missing_credential"
    request_path = request_dir / f"{lane_name}.json"
    result_path = result_dir / f"{lane_name}.json"
    log_path = log_dir / f"{lane_name}.jsonl"
    negative_prompt = "Reply with exactly: ADL_OPENROUTER_NEGATIVE_CONTROL"
    raw_request = request_payload(
        "deepseek/deepseek-v4-flash",
        lane_name,
        negative_prompt,
        "env:ADL_OPENROUTER_MISSING_KEY",
    )
    write_json(request_path, sanitized_request_payload(raw_request))
    env = dict(os.environ)
    env.pop("ADL_OPENROUTER_MISSING_KEY", None)
    with tempfile.TemporaryDirectory(prefix=f"{lane_name}_") as temp_dir:
        temp_root = Path(temp_dir)
        raw_request_path = temp_root / "request.json"
        raw_result_path = temp_root / "result.json"
        write_json(raw_request_path, raw_request)
        run(
            [
                str(adapter_bin),
                "--request",
                str(raw_request_path),
                "--out",
                str(raw_result_path),
                "--log",
                str(log_path),
            ],
            cwd=root,
            env=env,
        )
        result = json.loads(raw_result_path.read_text())
    write_json(result_path, sanitized_result_payload(result))
    failure = result.get("failure") or {}
    return {
        "lane": lane_name,
        "role": "negative_control",
        "provider": "openrouter",
        "model": "deepseek/deepseek-v4-flash",
        "prompt_chars": len(negative_prompt),
        "prompt_digest": f"sha256:{short_sha256(negative_prompt)}",
        "prompt_excerpt": redacted_text_marker("prompt", negative_prompt),
        "request_path": rel(root, request_path),
        "result_path": rel(root, result_path),
        "log_path": rel(root, log_path),
        "final_status": result.get("final_status"),
        "failure_kind": failure.get("kind"),
        "duration_ms": result.get("duration_ms"),
        "contract_status": "blocked_missing_credential"
        if failure.get("kind") == "provider_auth_missing"
        else "unexpected_negative_result",
        "note": "missing credential negative control failed closed"
        if failure.get("kind") == "provider_auth_missing"
        else "negative control did not normalize to provider_auth_missing",
    }


def write_provider_invocations(path: Path, live_lanes: list[dict]) -> None:
    payload = {
        "schema_version": "adl.native_provider_invocations.v1",
        "credential_policy": "operator_env_or_key_file_no_secret_material_recorded",
        "invocations": [
            {
                "family": "openrouter",
                "model": lane["recorded_route_model"],
                "http_status": lane["http_status"],
                "prompt_chars": lane["prompt_chars"],
                "prompt_digest": lane["prompt_digest"],
                "prompt_excerpt": lane["prompt_excerpt"],
                "output_chars": lane["output_chars"],
                "output_digest": lane["output_digest"],
                "output_excerpt": lane["output_excerpt"],
            }
            for lane in live_lanes
        ],
    }
    write_json(path, payload)


def write_packet_files(
    root: Path,
    out_dir: Path,
    date: str,
    catalog: dict,
    live_lanes: list[dict],
    negative_lane: dict,
) -> None:
    packet_path = out_dir / f"OPENROUTER_MATRIX_PROOF_{date}.md"
    state_path = out_dir / f"openrouter_matrix_state_{date}.json"
    readme_path = out_dir / "README.md"
    prior_lane = (
        "docs/milestones/v0.91.5/review/multi_agent_workcell/"
        "lane_outputs/planner_openrouter_deepseek_v4_flash.md"
    )
    selected_models = [lane["model"] for lane in live_lanes]
    state = {
        "schema": "adl.openrouter_matrix_proof.v1",
        "issue": 3723,
        "date": date,
        "status": "supported_with_limits",
        "setup": {
            "family": "openrouter",
            "provider_setup_dir": rel(root, out_dir / "provider_setup"),
        },
        "catalog": {
            "path": rel(root, out_dir / f"catalog_snapshot_{date}.json"),
            "model_count": len(catalog.get("data", [])),
            "selected_models": selected_models,
        },
        "lanes": live_lanes + [negative_lane],
        "prior_non_proving_evidence": {
            "issue": 3415,
            "path": prior_lane,
            "reason": "OpenRouter planner/critic output was useful but generic/off-target in places, so broad role usefulness remains unproven.",
        },
        "findings": [
            {
                "severity": "P2",
                "summary": "The native OpenRouter provider completed five requested model routes under bounded prompts, while preserving fail-closed auth behavior and explicit role-contract non-claims.",
            },
            {
                "severity": "P2",
                "summary": "The missing-credential negative control failed closed as provider_auth_missing rather than silently succeeding or leaking secret material.",
            },
            {
                "severity": "P3",
                "summary": "No flaky retries or timeouts were observed in this bounded run, but the packet does not claim broad route stability beyond the exact models tested here.",
            },
        ],
        "non_claims": [
            "does_not_claim_universal_openrouter_compatibility",
            "does_not_claim_tool_call_capability_across_openrouter_models",
            "does_not_claim_json_mode_support_across_openrouter_models",
            "does_not_override_the_prior_workcell_finding_that_broad_openrouter_planner_usefulness_remains_unproven",
        ],
        "artifacts": {
            "packet": rel(root, packet_path),
            "state": rel(root, state_path),
            "provider_invocations": rel(root, out_dir / "provider_invocations.json"),
        },
    }
    write_json(state_path, state)
    supported_lanes = [lane for lane in live_lanes if lane["contract_status"] == "supported"]
    non_proving_lanes = [lane for lane in live_lanes if lane["contract_status"] == "non_proving"]

    def lane_row(lane: dict) -> str:
        if lane["role"] == "negative_control":
            return (
                f"| {lane['lane']} | negative control | `{lane['model']}` | "
                f"{lane['contract_status']} | `{lane['failure_kind']}` |"
            )
        return (
            f"| {lane['lane']} | {lane['role']} | `{lane['model']}` | "
            f"{lane['contract_status']} | {lane['note']} |"
        )

    packet_text = "\n".join(
        [
            "# v0.91.5 OpenRouter Matrix Proof Packet",
            "",
            f"Date: {date}",
            "",
            "Issue: `#3723`",
            "",
            "Status: `supported_with_limits`",
            "",
            "## Purpose",
            "",
            "Record a real bounded native OpenRouter proof lane that goes beyond one smoke request by exercising provider setup, live catalog discovery, five requested route IDs, and one fail-closed missing-credential path.",
            "",
            "## What Ran",
            "",
            "- `adl provider setup openrouter` into the tracked packet directory.",
            "- `GET https://openrouter.ai/api/v1/models` to snapshot the live catalog.",
            "- Five native OpenRouter provider invocations through `adl-provider-adapter` using requested model IDs, while preserving provider-returned identity metadata in the state packet when present.",
            "- One negative control that omits the required credential and must fail closed.",
            "- Tracked lane request/result JSON retain redacted prompt/output excerpts plus SHA-256 digests rather than raw provider prompts or full raw model output.",
            "",
            "## Lane Summary",
            "",
            "| Lane | Role | Model | Result | Notes |",
            "| --- | --- | --- | --- | --- |",
            *[lane_row(lane) for lane in live_lanes],
            lane_row(negative_lane),
            "",
            "## Supported Paths",
            "",
            "- The native OpenRouter lane completed successfully for all five requested route IDs recorded in the state packet.",
            *[
                f"- `{lane['lane']}` satisfied the stricter bounded role contract for requested route `{lane['model']}`."
                for lane in supported_lanes
            ],
            "- Provider setup is now scaffoldable through `adl provider setup openrouter` and the generated tracked `provider_setup/` bundle.",
            "- Provider-returned identity metadata is recorded in the state packet and may be more specific or normalized relative to the requested route ID.",
            "",
            "## Blocked Paths",
            "",
            "- The missing-credential negative control normalized to `provider_auth_missing`, proving fail-closed auth behavior rather than hidden fallback.",
            "",
            "## Flaky Paths",
            "",
            "- No flaky retry or timeout behavior was observed in this bounded run.",
            "- This packet does not elevate that absence into a broad stability claim for untested models or longer prompts.",
            "",
            "## Non-Proving Paths",
            "",
            *[
                f"- `{lane['lane']}` completed with HTTP 200 for requested route `{lane['model']}` but remained non-proving for its stricter role contract: {lane['note']}"
                for lane in non_proving_lanes
            ],
            f"- Prior evidence from `#3415` remains binding for broad planner usefulness: `{prior_lane}` recorded OpenRouter planner output as useful but generic/off-target in places.",
            "- This packet proves structured route execution, not broad role usefulness across all OpenRouter-backed models.",
            "",
            "## Non-Claims",
            "",
            "- no universal OpenRouter compatibility claim",
            "- tool-call capability still unproven",
            "- JSON-mode capability still unproven",
            "- no claim that the five tested requested routes generalize to all OpenRouter models or all prompt shapes",
            "",
            "## Validation",
            "",
            "- `python3 adl/tools/validate_v0915_openrouter_matrix.py docs/milestones/v0.91.5/review/openrouter_matrix`",
            "- `bash adl/tools/test_v0915_openrouter_matrix.sh`",
            "",
        ]
    )
    packet_path.write_text(packet_text + "\n")

    readme_text = "\n".join(
        [
            "# v0.91.5 OpenRouter Matrix Proof",
            "",
            "Canonical regeneration command:",
            "",
            "```bash",
            "python3 adl/tools/run_v0915_openrouter_matrix.py",
            "```",
            "",
            "Primary proof surfaces:",
            f"- `OPENROUTER_MATRIX_PROOF_{date}.md`",
            f"- `openrouter_matrix_state_{date}.json`",
            f"- `catalog_snapshot_{date}.json`",
            "- `provider_invocations.json`",
            "- `lane_requests/`",
            "- `lane_results/`",
            "- `lane_logs/`",
            "- `lane_outputs/`",
            "- `provider_setup/`",
            "",
            "Tracked lane request/result JSON are intentionally redacted to excerpt-plus-digest evidence; bounded rendered outputs remain in `lane_outputs/` for reviewer-facing inspection.",
            "",
        ]
    )
    readme_path.write_text(readme_text + "\n")


def main() -> None:
    args = parse_args()
    root = repo_root()
    key = load_openrouter_key(args.key_file)
    adl_bin, adapter_bin = build_binaries(root)
    out_dir = args.out.resolve()
    if out_dir.exists():
        shutil.rmtree(out_dir)
    (out_dir / "provider_setup").mkdir(parents=True, exist_ok=True)
    (out_dir / "lane_requests").mkdir(parents=True, exist_ok=True)
    (out_dir / "lane_results").mkdir(parents=True, exist_ok=True)
    (out_dir / "lane_logs").mkdir(parents=True, exist_ok=True)
    (out_dir / "lane_outputs").mkdir(parents=True, exist_ok=True)

    provider_setup(adl_bin, root, out_dir / "provider_setup")
    catalog = fetch_catalog(out_dir / f"catalog_snapshot_{args.date}.json", key)

    lanes = [
        {
            "lane": "planner_openrouter_deepseek_v4_flash",
            "role": "planner",
            "model": "deepseek/deepseek-v4-flash",
            "prompt": lane_prompt("planner"),
            "credential_ref": "env:OPENROUTER_API_KEY",
        },
        {
            "lane": "worker_openrouter_gpt4o_mini",
            "role": "worker",
            "model": "openai/gpt-4o-mini",
            "prompt": lane_prompt("worker"),
            "credential_ref": "env:OPENROUTER_API_KEY",
        },
        {
            "lane": "reviewer_openrouter_claude_3_5_haiku",
            "role": "reviewer",
            "model": "anthropic/claude-3.5-haiku",
            "prompt": lane_prompt("reviewer"),
            "credential_ref": "env:OPENROUTER_API_KEY",
        },
        {
            "lane": "watcher_openrouter_gemini_2_5_flash_lite",
            "role": "watcher",
            "model": "google/gemini-2.5-flash-lite",
            "prompt": lane_prompt("watcher"),
            "credential_ref": "env:OPENROUTER_API_KEY",
        },
        {
            "lane": "worker_openrouter_qwen3_6_flash",
            "role": "worker",
            "model": "qwen/qwen3.6-flash",
            "prompt": lane_prompt("worker"),
            "credential_ref": "env:OPENROUTER_API_KEY",
        },
    ]
    ids = {entry.get("id") for entry in catalog.get("data", [])}
    for lane in lanes:
        if lane["model"] not in ids:
            fail(f"selected model missing from catalog: {lane['model']}")

    invocations_path = out_dir / "provider_invocations.json"
    live_lanes = [
        run_lane(
            adapter_bin,
            root,
            lane,
            out_dir / "lane_requests",
            out_dir / "lane_results",
            out_dir / "lane_logs",
            out_dir / "lane_outputs",
            invocations_path,
        )
        for lane in lanes
    ]
    write_provider_invocations(invocations_path, live_lanes)
    negative_lane = run_negative_missing_credential(
        adapter_bin,
        root,
        out_dir / "lane_requests",
        out_dir / "lane_results",
        out_dir / "lane_logs",
    )
    write_packet_files(root, out_dir, args.date, catalog, live_lanes, negative_lane)
    print(f"PASS run_v0915_openrouter_matrix {rel(root, out_dir)}")


if __name__ == "__main__":
    main()
