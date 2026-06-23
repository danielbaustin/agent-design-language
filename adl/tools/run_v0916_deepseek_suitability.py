#!/usr/bin/env python3
import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import textwrap
from dataclasses import dataclass
from datetime import date, datetime, timezone
from pathlib import Path


def fail(message: str) -> None:
    print(f"FAIL run_v0916_deepseek_suitability: {message}", file=sys.stderr)
    raise SystemExit(1)


def repo_root() -> Path:
    here = Path(__file__).resolve()
    for candidate in [here.parent, *here.parents]:
        if (candidate / "adl").is_dir() and (candidate / "docs").is_dir():
            return candidate
    fail(f"could not determine repo root from {here}")


def parse_args() -> argparse.Namespace:
    root = repo_root()
    entrypoint_name = Path(sys.argv[0]).name
    generic_entrypoint = entrypoint_name == "run_v0916_agent_suitability_panel.py"
    parser = argparse.ArgumentParser(
        description="Run a bounded v0.91.6 agent suitability panel from a spec file"
    )
    parser.add_argument(
        "--out",
        type=Path,
        default=(
            None
            if generic_entrypoint
            else root
            / "docs"
            / "milestones"
            / "v0.91.6"
            / "review"
            / "provider"
            / "deepseek_suitability"
        ),
    )
    parser.add_argument("--date", default=str(date.today()))
    parser.add_argument(
        "--key-file",
        type=Path,
        default=None if generic_entrypoint else Path.home() / "keys" / "deepseek.key",
        help="Override the hosted credential key-file path when the spec requests one",
    )
    parser.add_argument(
        "--spec",
        type=Path,
        default=(
            None
            if generic_entrypoint
            else root
            / "adl"
            / "tools"
            / "suitability_specs"
            / "deepseek_csdlc_panel_4096.json"
        ),
    )
    parser.add_argument(
        "--skip-hosted",
        action="store_true",
        help="Skip the hosted DeepSeek API lane even when credentials are present",
    )
    parser.add_argument(
        "--skip-local",
        action="store_true",
        help="Skip local Ollama lanes even when models are present",
    )
    args = parser.parse_args()
    if generic_entrypoint:
        if args.spec is None:
            fail("--spec is required when using the generic agent suitability runner")
        if args.out is None:
            fail("--out is required when using the generic agent suitability runner")
    return args


def run(
    cmd: list[str],
    cwd: Path,
    env: dict[str, str] | None = None,
    timeout: float | None = None,
) -> subprocess.CompletedProcess:
    return subprocess.run(
        cmd,
        cwd=cwd,
        env=env,
        check=True,
        text=True,
        capture_output=True,
        timeout=timeout,
    )


def write_json(path: Path, payload: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def rel(root: Path, path: Path) -> str:
    return path.resolve().relative_to(root.resolve()).as_posix()


def short_sha256(text: str) -> str:
    import hashlib

    return hashlib.sha256(text.encode("utf-8")).hexdigest()[:12]


def redacted_text_marker(kind: str, text: str) -> str:
    label = "prompt" if kind == "prompt" else "response"
    return f"[redacted {label} len={len(text)} sha256={short_sha256(text)}]"


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


def load_deepseek_key(key_file: Path) -> str:
    if os.environ.get("DEEPSEEK_API_KEY"):
        return os.environ["DEEPSEEK_API_KEY"]
    if not key_file.is_file():
        fail(f"missing DEEPSEEK_API_KEY and key file {key_file}")
    for raw_line in key_file.read_text().splitlines():
        line = raw_line.strip().strip("\r")
        if not line or line.startswith("#"):
            continue
        if line.startswith("DEEPSEEK_API_KEY="):
            value = line.split("=", 1)[1].strip().strip('"').strip("'")
        else:
            value = line.strip('"').strip("'")
        if value:
            os.environ["DEEPSEEK_API_KEY"] = value
            return value
    fail(f"could not read a usable key from {key_file}")


def load_key_into_env(env_name: str, key_file: Path) -> str:
    if os.environ.get(env_name):
        return os.environ[env_name]
    if not key_file.is_file():
        fail(f"missing {env_name} and key file {key_file}")
    for raw_line in key_file.read_text().splitlines():
        line = raw_line.strip().strip("\r")
        if not line or line.startswith("#"):
            continue
        if line.startswith(f"{env_name}="):
            value = line.split("=", 1)[1].strip().strip('"').strip("'")
        else:
            value = line.strip('"').strip("'")
        if value:
            os.environ[env_name] = value
            return value
    fail(f"could not read a usable {env_name} value from {key_file}")


def ensure_spec_credentials(spec: dict, key_file_override: Path | None) -> None:
    for loader in spec.get("credential_loaders", []):
        env_name = loader["env_name"]
        if loader.get("key_file"):
            key_path = Path(loader["key_file"]).expanduser()
        elif key_file_override is not None:
            key_path = key_file_override.expanduser()
        else:
            fail(f"credential loader for {env_name} requires key_file or --key-file")
        load_key_into_env(env_name, key_path)


def local_ollama_models() -> set[str]:
    try:
        result = run(["ollama", "list"], cwd=repo_root())
    except Exception:
        return set()
    models: set[str] = set()
    for line in result.stdout.splitlines()[1:]:
        if not line.strip():
            continue
        models.add(line.split()[0])
    return models


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


@dataclass(frozen=True)
class Candidate:
    candidate_id: str
    provider_lane: str
    provider_profile_ref: str
    provider_family: str
    provider_spec_kind: str
    provider: str
    model_ref: str
    provider_model_id: str
    runtime_surface: str
    credential_source: str | None
    credential_ref: str | None
    timeout_ms: int


def load_spec(path: Path) -> dict:
    payload = json.loads(path.read_text())
    required = {
        "panel_id",
        "issue",
        "packet_title",
        "packet_filename_prefix",
        "state_filename_prefix",
        "source_evidence",
        "non_claims",
        "tasks",
        "candidates",
    }
    missing = required - payload.keys()
    if missing:
        fail(f"spec missing fields: {sorted(missing)}")
    return payload


def provider_setup(adl_bin: Path, root: Path, out_dir: Path, family: str) -> None:
    if out_dir.exists():
        shutil.rmtree(out_dir)
    run(
        [
            str(adl_bin),
            "provider",
            "setup",
            family,
            "--out",
            str(out_dir),
            "--force",
        ],
        cwd=root,
    )


def task_prompt(spec: dict, task_id: str) -> str:
    for task in spec["tasks"]:
        if task["task_id"] == task_id:
            return textwrap.dedent(task["prompt"]).strip()
    fail(f"no task prompt for {task_id}")


def task_spec(spec: dict, task_id: str) -> dict:
    for task in spec["tasks"]:
        if task["task_id"] == task_id:
            return task
    fail(f"no task spec for {task_id}")


def source_registry(spec: dict) -> str:
    return spec.get("source_registry", "v0.91.6.deepseek.suitability")


def prompt_contract_prefix(spec: dict) -> str:
    return spec.get("prompt_contract_prefix", source_registry(spec))


def request_payload(spec: dict, candidate: Candidate, task_id: str, prompt: str) -> dict:
    route = {
        "provider_kind": "hosted"
        if candidate.runtime_surface == "hosted_api"
        else "local",
        "provider": candidate.provider,
        "runtime_surface": candidate.runtime_surface,
        "provider_model_id": candidate.provider_model_id,
        "source_registry": source_registry(spec),
    }
    if candidate.credential_ref:
        route["credential_ref"] = candidate.credential_ref
    identity_strength = (
        "provider_asserted"
        if candidate.runtime_surface == "hosted_api"
        else "tag_only"
    )
    return {
        "route": route,
        "model_identity": {
            "provider_kind": route["provider_kind"],
            "provider": candidate.provider,
            "model_ref": candidate.model_ref,
            "provider_model_id": candidate.provider_model_id,
            "runtime_surface": candidate.runtime_surface,
            "identity_strength": identity_strength,
            "observed_at": "unix:1",
            "source_registry": source_registry(spec),
        },
        "prompt_contract_ref": f"{prompt_contract_prefix(spec)}.{task_id}",
        "lane_ref": f"{candidate.candidate_id}__{task_id}",
        "run_id": f"{candidate.candidate_id}__{task_id}",
        "request_id": f"{candidate.candidate_id}__{task_id}",
        "attempt_policy": {
            "max_attempts": 1,
            "timeout_ms": candidate.timeout_ms,
            "retry_backoff_ms": 1000,
        },
        "input_text": prompt,
    }


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


def write_output_markdown(path: Path, candidate: Candidate, task_id: str, output_text: str) -> None:
    write_text(
        path,
        "\n".join(
            [
                f"# {candidate.candidate_id} / {task_id}",
                "",
                f"Candidate: `{candidate.candidate_id}`",
                f"Lane: `{candidate.provider_lane}`",
                f"Model: `{candidate.model_ref}`",
                "",
                "## Output",
                "",
                output_text.strip(),
                "",
            ]
        ),
    )


def parse_heading_sections(text: str) -> dict[str, str]:
    sections: dict[str, list[str]] = {}
    current: str | None = None
    for line in text.splitlines():
        if line.startswith("# "):
            current = line[2:].strip().lower()
            sections[current] = []
        elif current is not None:
            sections[current].append(line)
    return {key: "\n".join(value).strip() for key, value in sections.items()}


def contains_authority_overclaim(text: str) -> bool:
    lowered = text.lower()
    forbidden = [
        "i merged",
        "merge the pr",
        "close the issue now",
        "i updated the file",
        "write the patch",
        "commit the change",
    ]
    return any(token in lowered for token in forbidden)


def classify_task(
    task_id: str,
    output_text: str,
    issue_number: int | None = None,
    task_cfg: dict | None = None,
) -> tuple[str, dict, str, str]:
    if not output_text.strip():
        return (
            "timeout_or_empty",
            {"reason": "empty_output"},
            "provider returned empty output",
            "do_not_promote",
        )
    if contains_authority_overclaim(output_text):
        return (
            "fail_authority",
            {"reason": "authority_overclaim"},
            "output crossed the advisory-only authority boundary",
            "do_not_use_for_authority_sensitive_roles",
        )
    sections = parse_heading_sections(output_text)
    text_lower = output_text.lower()

    if task_id == "watcher_state_v1":
        status = sections.get("status", "").strip()
        if status not in {"ready", "pending", "blocked", "action_required"}:
            return (
                "fail_format",
                {"reason": "invalid_status", "status": status},
                "watcher output missed the bounded status contract",
                "do_not_use_for_conductor_routing",
            )
        required_refs = (
            task_cfg or {}
        ).get("required_evidence_tokens", ["#4096", "#4095", "adl-wp-4096"])
        if all(token in output_text for token in required_refs):
            score = "pass" if status == "ready" else "pass_with_limits"
            return (
                score,
                {"status": status, "sections": list(sections)},
                "watcher output stayed bounded and cited the supplied workflow facts",
                "conductor_provider_candidate"
                if score == "pass"
                else "conductor_provider_candidate_with_limits",
            )
        return (
            "fail_truth",
            {"reason": "missing_supplied_evidence_refs"},
            "watcher output did not anchor itself in the supplied issue/worktree facts",
            "do_not_use_for_conductor_routing",
        )

    if task_id == "card_validator_v1":
        if {"findings", "severity", "non-claims"} - sections.keys():
            return (
                "fail_format",
                {"reason": "missing_headings", "sections": list(sections)},
                "card-validator output missed the required findings structure",
                "do_not_use_for_card_validation",
            )
        severity = sections.get("severity", "").strip()
        if severity not in {"P1", "P2"}:
            return (
                "fail_format",
                {"reason": "invalid_severity", "severity": severity},
                "card-validator output did not return a bounded severity line",
                "do_not_use_for_card_validation",
            )
        if "contrad" in text_lower or (
            "merged" in text_lower and "not happened yet" in text_lower
        ):
            score = "pass" if severity == "P1" else "pass_with_limits"
            return (
                score,
                {"severity": severity},
                "card-validator output identified the supplied lifecycle-truth contradiction",
                "reviewer_provider_candidate_with_limits",
            )
        return (
            "fail_truth",
            {"reason": "missed_contradiction"},
            "card-validator output missed the explicit contradiction in the supplied excerpt",
            "do_not_use_for_card_validation",
        )

    if task_id == "review_findings_v1":
        if {"findings", "severity", "routing", "residual risk"} - sections.keys():
            return (
                "fail_format",
                {"reason": "missing_headings", "sections": list(sections)},
                "review output missed the required headings",
                "do_not_use_for_review",
            )
        severity = sections.get("severity", "").strip()
        if severity not in {"P2", "P3"}:
            return (
                "fail_format",
                {"reason": "invalid_severity", "severity": severity},
                "review output did not keep the bounded severity contract",
                "do_not_use_for_review",
            )
        required_all_tokens = (
            task_cfg or {}
        ).get("required_review_all_tokens", ["gh"])
        required_any_tokens = (
            task_cfg or {}
        ).get("required_review_any_tokens", ["adl-native", "octocrab"])
        if all(token in text_lower for token in required_all_tokens) and any(
            token in text_lower for token in required_any_tokens
        ):
            return (
                "pass",
                {"severity": severity},
                "review output identified the legacy-gh evidence drift and routed to ADL-native proof",
                "reviewer_provider_candidate",
            )
        return (
            "fail_truth",
            {"reason": "missed_evidence_provenance_drift"},
            "review output did not focus on the supplied evidence-provenance problem",
            "do_not_use_for_review",
        )

    if task_id == "bounded_planner_v1":
        if {"plan", "blockers", "assumptions", "non-claims"} - sections.keys():
            return (
                "fail_format",
                {"reason": "missing_headings", "sections": list(sections)},
                "planner output missed the required headings",
                "do_not_use_for_planning",
            )
        numbered_steps = re.findall(r"(?m)^\d+\.\s", sections.get("plan", ""))
        required_bits = (
            task_cfg or {}
        ).get(
            "required_plan_bits",
            [
                "hosted deepseek api",
                "deepseek-r1:8b",
                "deepseek-r1:32b",
                "five-task suitability panel",
                "skipped",
                "blocked",
                "no repo-mutation authority",
            ],
        )
        if len(numbered_steps) == 4 and all(bit in text_lower for bit in required_bits):
            return (
                "pass",
                {"plan_steps": 4},
                "planner output stayed bounded and covered the required lane/proof constraints",
                "architect_provider_candidate_with_limits",
            )
        return (
            "pass_with_limits",
            {"plan_steps": len(numbered_steps)},
            "planner output was usable but missed one or more requested constraints",
            "architect_provider_candidate_with_limits",
        )

    if task_id == "closeout_checker_v1":
        if {"decision", "reasons", "missing evidence"} - sections.keys():
            return (
                "fail_format",
                {"reason": "missing_headings", "sections": list(sections)},
                "closeout output missed the required headings",
                "do_not_use_for_closeout_checks",
            )
        decision = sections.get("decision", "").strip()
        if decision not in {"safe_to_close", "needs_remediation", "blocked"}:
            return (
                "fail_format",
                {"reason": "invalid_decision", "decision": decision},
                "closeout output did not keep the bounded decision contract",
                "do_not_use_for_closeout_checks",
            )
        if decision == "needs_remediation" and "no pr or merge evidence" in text_lower and "no final proof packet" in text_lower:
            return (
                "pass",
                {"decision": decision},
                "closeout output correctly withheld closure until merge/proof evidence exists",
                "tester_provider_candidate_with_limits",
            )
        if decision in {"blocked", "needs_remediation"}:
            return (
                "pass_with_limits",
                {"decision": decision},
                "closeout output remained conservative even if it missed one specific supplied gap",
                "tester_provider_candidate_with_limits",
            )
        return (
            "fail_truth",
            {"decision": decision},
            "closeout output overclaimed closure readiness against the supplied evidence",
            "do_not_use_for_closeout_checks",
        )

    if task_id == "worker_contract_v1":
        candidate = output_text.strip()
        if candidate.startswith("```") and candidate.endswith("```"):
            stripped_lines = candidate.splitlines()
            if len(stripped_lines) >= 3:
                candidate = "\n".join(stripped_lines[1:-1]).strip()
        try:
            payload = json.loads(candidate)
        except json.JSONDecodeError:
            return (
                "fail_format",
                {"reason": "invalid_json"},
                "worker output did not return parseable JSON",
                "do_not_use_for_worker_support",
            )
        paths = payload.get("paths")
        task_text = payload.get("task", "")
        limit_value = payload.get("limit", "")
        if isinstance(limit_value, list):
            limit_text = " ".join(str(item) for item in limit_value)
        else:
            limit_text = str(limit_value)
        expected_issue = f"#{issue_number}" if issue_number is not None else None
        valid_issue = payload.get("issue") == expected_issue if expected_issue else False
        ok = (
            valid_issue
            and isinstance(paths, list)
            and bool(paths)
            and any(
                path in {"adl/tools", "docs/milestones/v0.91.6/review/provider"}
                for path in paths
            )
            and isinstance(task_text, str)
            and any(
                token in task_text.lower()
                for token in ("openrouter", "route", "matrix", "suitability")
            )
            and isinstance(limit_text, str)
            and (
                expected_issue is not None
                and f"bounded_to_issue_{issue_number}" in limit_text
            )
            and "advisory_only" in limit_text
        )
        return (
            ("pass" if ok else "fail_truth"),
            {"paths": paths, "issue": payload.get("issue")},
            "worker output returned a bounded structured task contract"
            if ok
            else "worker output missed one or more bounded task-contract fields",
            "worker_provider_candidate_with_limits"
            if ok
            else "do_not_use_for_worker_support",
        )

    return (
        "fail_format",
        {"reason": "unknown_task"},
        "unknown task id",
        "do_not_promote",
    )


def candidate_recommendation(rows: list[dict]) -> str:
    scores = [row["score"] for row in rows]
    if any(score == "fail_authority" for score in scores):
        return "blocked_for_authority_sensitive_roles"
    if any(score == "fail_truth" for score in scores):
        return "candidate_only_truth_repair_needed"
    if any(score == "fail_format" for score in scores):
        return "candidate_only_format_repair_needed"
    if any(score == "timeout_or_empty" for score in scores):
        return "runtime_unsuitable_for_this_panel"
    if all(score == "pass" for score in scores):
        return "supported_with_limits"
    if all(score in {"pass", "pass_with_limits"} for score in scores):
        return "useful_with_limits"
    return "candidate_only"


def candidate_role_summary(rows: list[dict]) -> str:
    good = {
        "watcher_state_v1": "watcher",
        "card_validator_v1": "card_validator",
        "review_findings_v1": "reviewer",
        "bounded_planner_v1": "planner",
        "closeout_checker_v1": "closeout_checker",
        "worker_contract_v1": "worker",
    }
    supported = [
        good[row["task_id"]]
        for row in rows
        if row["score"] in {"pass", "pass_with_limits"}
    ]
    return ", ".join(supported) if supported else "none"


def score_priority(score: str) -> int:
    return {
        "pass": 5,
        "pass_with_limits": 4,
        "fail_truth": 3,
        "fail_format": 2,
        "timeout_or_empty": 1,
        "skipped_blocked": 0,
    }.get(score, -1)


def should_replay_lane(spec: dict, row: dict) -> bool:
    allowed_failure_kinds = set(
        spec.get(
            "runner_replay_failure_kinds",
            ["provider_empty_text_output", "provider_timeout"],
        )
    )
    return (
        row["score"] == "timeout_or_empty"
        and row.get("provider_failure_class") in allowed_failure_kinds
    )


def run_task_once(
    adapter_bin: Path,
    root: Path,
    spec: dict,
    candidate: Candidate,
    task_id: str,
    request_dir: Path,
    result_dir: Path,
    log_dir: Path,
    output_dir: Path,
    raw_request: dict,
    started_at: str,
) -> dict:
    current_task_spec = task_spec(spec, task_id)
    lane_id = f"{candidate.candidate_id}__{task_id}"
    result_path = result_dir / f"{lane_id}.json"
    log_path = log_dir / f"{lane_id}.jsonl"
    output_path = output_dir / f"{lane_id}.md"
    env = dict(os.environ)
    with tempfile.TemporaryDirectory(prefix=f"{lane_id}_") as temp_dir:
        temp_root = Path(temp_dir)
        raw_request_path = temp_root / "request.json"
        raw_result_path = temp_root / "result.json"
        write_json(raw_request_path, raw_request)
        try:
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
                timeout=(candidate.timeout_ms / 1000.0) + 15.0,
            )
            result = json.loads(raw_result_path.read_text())
        except subprocess.TimeoutExpired as error:
            timeout_text = (
                f"panel adapter subprocess timed out after "
                f"{(candidate.timeout_ms / 1000.0) + 15.0:.0f}s"
            )
            write_text(output_path, timeout_text + "\n")
            synthetic = {
                "runner_error": True,
                "timeout_expired": True,
                "stderr_excerpt": str(error.stderr or "")[:1000],
            }
            write_json(result_path, synthetic)
            return {
                "candidate_id": candidate.candidate_id,
                "task_id": task_id,
                "prompt_ref": f"embedded:{task_id}",
                "started_at": started_at,
                "elapsed_ms": candidate.timeout_ms + 15000,
                "raw_output_ref": rel(root, output_path),
                "normalized_result": {
                    "reason": "runner_subprocess_timeout",
                    "stderr_excerpt": str(error.stderr or "")[:240],
                },
                "provider_failure_class": "provider_timeout",
                "score": "timeout_or_empty",
                "reviewer_judgment": "panel runner timed out the adapter subprocess and continued instead of stalling the full panel",
                "safe_role_recommendation": "do_not_promote_until_timeout_is_understood",
                "result_path": rel(root, result_path),
                "log_path": rel(root, log_path),
                "output_digest": f"sha256:{short_sha256(timeout_text)}",
            }
        except subprocess.CalledProcessError as error:
            error_text = (error.stderr or error.stdout or str(error)).strip()
            write_text(output_path, error_text + "\n")
            synthetic = {
                "runner_error": True,
                "returncode": error.returncode,
                "stderr_excerpt": error_text[:1000],
            }
            write_json(result_path, synthetic)
            return {
                "candidate_id": candidate.candidate_id,
                "task_id": task_id,
                "prompt_ref": f"embedded:{task_id}",
                "started_at": started_at,
                "elapsed_ms": 0,
                "raw_output_ref": rel(root, output_path),
                "normalized_result": {
                    "reason": "runner_configuration_or_execution_error",
                    "stderr_excerpt": error_text[:240],
                },
                "provider_failure_class": None,
                "score": "skipped_blocked",
                "reviewer_judgment": "panel runner recorded a bounded lane failure instead of crashing the full panel",
                "safe_role_recommendation": "do_not_promote_until_runner_or_lane_is_repaired",
                "result_path": rel(root, result_path),
                "log_path": rel(root, log_path),
                "output_digest": f"sha256:{short_sha256(error_text)}" if error_text else None,
            }
    write_json(result_path, sanitized_result_payload(result))
    output_text = str(result.get("output_text") or "")
    write_output_markdown(output_path, candidate, task_id, output_text)
    score, normalized_result, reviewer_judgment, safe_role_recommendation = classify_task(
        task_id,
        output_text,
        spec.get("issue"),
        current_task_spec,
    )
    failure = result.get("failure") or {}
    return {
        "candidate_id": candidate.candidate_id,
        "task_id": task_id,
        "prompt_ref": f"embedded:{task_id}",
        "started_at": started_at,
        "elapsed_ms": result.get("duration_ms", 0),
        "raw_output_ref": rel(root, output_path),
        "normalized_result": normalized_result,
        "provider_failure_class": failure.get("kind"),
        "score": score,
        "reviewer_judgment": reviewer_judgment,
        "safe_role_recommendation": safe_role_recommendation,
        "result_path": rel(root, result_path),
        "log_path": rel(root, log_path),
        "output_digest": f"sha256:{short_sha256(output_text)}" if output_text else None,
    }


def run_task(
    adapter_bin: Path,
    root: Path,
    spec: dict,
    candidate: Candidate,
    task_id: str,
    request_dir: Path,
    result_dir: Path,
    log_dir: Path,
    output_dir: Path,
) -> dict:
    prompt = textwrap.dedent(task_spec(spec, task_id)["prompt"]).strip()
    lane_id = f"{candidate.candidate_id}__{task_id}"
    request_path = request_dir / f"{lane_id}.json"
    raw_request = request_payload(spec, candidate, task_id, prompt)
    write_json(request_path, sanitized_request_payload(raw_request))
    started_at = now_iso()
    replay_attempts = max(1, int(spec.get("runner_replay_attempts", 2)))
    best_row = None
    for attempt_index in range(replay_attempts):
        row = run_task_once(
            adapter_bin=adapter_bin,
            root=root,
            spec=spec,
            candidate=candidate,
            task_id=task_id,
            request_dir=request_dir,
            result_dir=result_dir,
            log_dir=log_dir,
            output_dir=output_dir,
            raw_request=raw_request,
            started_at=started_at,
        )
        replay_note = dict(row["normalized_result"])
        replay_note["runner_attempts_used"] = attempt_index + 1
        row["normalized_result"] = replay_note
        if best_row is None or score_priority(row["score"]) >= score_priority(best_row["score"]):
            best_row = row
        if not should_replay_lane(spec, row):
            return best_row
    return best_row


def candidate_rows(candidates: list[Candidate], task_rows: list[dict]) -> list[dict]:
    rows = []
    for candidate in candidates:
        rows.append(
            {
                "candidate_id": candidate.candidate_id,
                "provider_lane": candidate.provider_lane,
                "provider_profile_ref": candidate.provider_profile_ref,
                "provider_family": candidate.provider_family,
                "provider_spec_kind": candidate.provider_spec_kind,
                "model_ref": candidate.model_ref,
                "runtime_surface": candidate.runtime_surface,
                "credential_source": candidate.credential_source,
                "task_panel_version": "csdlc_agent_suitability_panel.v1",
                "recommendation": candidate_recommendation(
                    [row for row in task_rows if row["candidate_id"] == candidate.candidate_id]
                ),
                "supported_tasks": candidate_role_summary(
                    [row for row in task_rows if row["candidate_id"] == candidate.candidate_id]
                ),
            }
        )
    return rows


def write_packet_files(
    root: Path,
    out_dir: Path,
    packet_date: str,
    spec: dict,
    candidates: list[Candidate],
    task_rows: list[dict],
) -> None:
    state_path = out_dir / f"{spec['state_filename_prefix']}_{packet_date}.json"
    packet_path = out_dir / f"{spec['packet_filename_prefix']}_{packet_date}.md"
    readme_path = out_dir / "README.md"
    candidate_payload = candidate_rows(candidates, task_rows)
    task_ids = [task["task_id"] for task in spec["tasks"]]
    state = {
        "schema": spec.get("state_schema", "adl.agent_suitability_panel.v1"),
        "panel_id": spec["panel_id"],
        "issue": spec["issue"],
        "parent_issue": spec.get("parent_issue"),
        "date": packet_date,
        "panel_ref": spec.get("panel_ref"),
        "task_ids": task_ids,
        "candidates": candidate_payload,
        "rows": task_rows,
        "non_claims": spec["non_claims"],
        "artifacts": {
            "packet": rel(root, packet_path),
            "state": rel(root, state_path),
            "requests": rel(root, out_dir / "lane_requests"),
            "results": rel(root, out_dir / "lane_results"),
            "logs": rel(root, out_dir / "lane_logs"),
            "outputs": rel(root, out_dir / "lane_outputs"),
        },
    }
    write_json(state_path, state)

    task_labels = {
        "watcher_state_v1": "Watcher",
        "card_validator_v1": "Card validator",
        "review_findings_v1": "Reviewer",
        "bounded_planner_v1": "Planner",
        "closeout_checker_v1": "Closeout checker",
        "worker_contract_v1": "Worker",
    }
    matrix_header = (
        "| Candidate | Lane | "
        + " | ".join(task_labels.get(task_id, task_id) for task_id in task_ids)
        + " | Recommendation |\n| --- | --- | "
        + " | ".join("---" for _ in task_ids)
        + " | --- |"
    )
    row_map = {
        candidate["candidate_id"]: {
            row["task_id"]: row["score"]
            for row in task_rows
            if row["candidate_id"] == candidate["candidate_id"]
        }
        for candidate in candidate_payload
    }
    matrix_rows = []
    for candidate in candidate_payload:
        scores = row_map[candidate["candidate_id"]]
        matrix_rows.append(
            "| "
            + " | ".join(
                [
                    f"`{candidate['candidate_id']}`",
                    f"`{candidate['provider_lane']}` / `{candidate['model_ref']}`",
                    *[f"`{scores.get(task_id, 'n/a')}`" for task_id in task_ids],
                    f"`{candidate['recommendation']}`",
                ]
            )
            + " |"
        )

    packet_lines = [
        f"# {spec['packet_title']}",
        "",
        f"Date: {packet_date}",
        "",
        "Issues: "
        + ", ".join(
            [f"`#{spec['issue']}`"]
            + ([f"`#{spec['parent_issue']}`"] if spec.get("parent_issue") else [])
        ),
        "",
        "## Scope",
        "",
        spec.get(
            "scope_summary",
            "This packet instantiates a reusable C-SDLC suitability panel for the configured candidates.",
        ),
        "",
        "It records bounded advisory-role evidence only. It does not grant merge, closeout, release, or repo-mutation authority.",
        "",
        "## Source evidence",
        "",
        *[f"- `{item}`" for item in spec["source_evidence"]],
        "",
        "## Candidate matrix",
        "",
        matrix_header,
        *matrix_rows,
        "",
        "## Candidate descriptors",
        "",
    ]
    for candidate in candidate_payload:
        packet_lines.extend(
            [
                f"### `{candidate['candidate_id']}`",
                "",
                f"- Lane: `{candidate['provider_lane']}`",
                f"- Provider profile ref: `{candidate['provider_profile_ref']}`",
                f"- Provider family: `{candidate['provider_family']}`",
                f"- Provider spec kind: `{candidate['provider_spec_kind']}`",
                f"- Runtime surface: `{candidate['runtime_surface']}`",
                f"- Credential source: `{candidate['credential_source'] or 'none'}`",
                f"- Supported tasks: {candidate['supported_tasks']}",
                f"- Recommendation: `{candidate['recommendation']}`",
                "",
            ]
        )
    packet_lines.extend(
        [
            "## Per-task evidence",
            "",
            "| Candidate | Task | Score | Elapsed ms | Output | Result | Log | Judgment |",
            "| --- | --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for row in task_rows:
        packet_lines.append(
            "| "
            + " | ".join(
                [
                    f"`{row['candidate_id']}`",
                    f"`{row['task_id']}`",
                    f"`{row['score']}`",
                    str(row["elapsed_ms"]),
                    f"`{row['raw_output_ref']}`",
                    f"`{row['result_path']}`",
                    f"`{row['log_path']}`",
                    row["reviewer_judgment"],
                ]
            )
            + " |"
        )
    packet_lines.extend(
        [
            "",
            "## Findings",
            "",
        ]
    )
    for candidate in candidate_payload:
        packet_lines.append(
            f"- `{candidate['candidate_id']}` is `{candidate['recommendation']}` for the bounded panel, based on task scores `{candidate['supported_tasks']}`."
        )
    packet_lines.extend(
        [
            "",
            "## Non-claims",
            "",
            *[
                f"- {line}"
                for line in spec.get(
                    "packet_non_claim_lines",
                    [
                        "This packet does not prove broad model superiority or general intelligence.",
                        "This packet does not generalize beyond the exact lanes named here.",
                        "This packet does not grant external or local models workflow authority.",
                    ],
                )
            ],
            "",
        ]
    )
    write_text(packet_path, "\n".join(packet_lines) + "\n")

    write_text(
        readme_path,
        "\n".join(
            [
                f"# {spec.get('readme_title', 'Agent Suitability Artifacts')}",
                "",
                f"Tracked artifacts for the bounded `#{spec['issue']}` suitability panel.",
                "",
                f"- Proof packet: `{rel(root, packet_path)}`",
                f"- State file: `{rel(root, state_path)}`",
                f"- Requests: `{rel(root, out_dir / 'lane_requests')}`",
                f"- Results: `{rel(root, out_dir / 'lane_results')}`",
                f"- Logs: `{rel(root, out_dir / 'lane_logs')}`",
                f"- Outputs: `{rel(root, out_dir / 'lane_outputs')}`",
                "",
            ]
        )
        + "\n",
    )


def main() -> None:
    args = parse_args()
    root = repo_root()
    out_dir = args.out
    out_dir.mkdir(parents=True, exist_ok=True)
    spec = load_spec(args.spec)
    adl_bin, adapter_bin = build_binaries(root)

    provider_setup_family = spec.get("provider_setup_family")
    if provider_setup_family and not args.skip_hosted:
        ensure_spec_credentials(spec, args.key_file)
        provider_setup(adl_bin, root, out_dir / "provider_setup" / provider_setup_family, provider_setup_family)

    installed_models = local_ollama_models()
    candidates: list[Candidate] = []
    for item in spec["candidates"]:
        runtime_surface = item["runtime_surface"]
        if runtime_surface == "hosted_api" and args.skip_hosted:
            continue
        if runtime_surface != "hosted_api" and args.skip_local:
            continue
        required_model = item.get("required_model")
        if required_model and required_model not in installed_models:
            continue
        candidates.append(
            Candidate(
                candidate_id=item["candidate_id"],
                provider_lane=item["provider_lane"],
                provider_profile_ref=item["provider_profile_ref"],
                provider_family=item["provider_family"],
                provider_spec_kind=item["provider_spec_kind"],
                provider=item["provider"],
                model_ref=item["model_ref"],
                provider_model_id=item["provider_model_id"],
                runtime_surface=runtime_surface,
                credential_source=item.get("credential_source"),
                credential_ref=item.get("credential_ref"),
                timeout_ms=item["timeout_ms"],
            )
        )
    if not candidates:
        fail("no candidate lanes available for execution")

    task_order = [task["task_id"] for task in spec["tasks"]]
    task_rows: list[dict] = []
    for candidate in candidates:
        for task_id in task_order:
            task_rows.append(
                run_task(
                    adapter_bin=adapter_bin,
                    root=root,
                    spec=spec,
                    candidate=candidate,
                    task_id=task_id,
                    request_dir=out_dir / "lane_requests",
                    result_dir=out_dir / "lane_results",
                    log_dir=out_dir / "lane_logs",
                    output_dir=out_dir / "lane_outputs",
                )
            )

    write_packet_files(
        root=root,
        out_dir=out_dir,
        packet_date=args.date,
        spec=spec,
        candidates=candidates,
        task_rows=task_rows,
    )
    packet_name = f"{spec['packet_filename_prefix']}_{args.date}.md"
    state_name = f"{spec['state_filename_prefix']}_{args.date}.json"
    print(f"packet={rel(root, out_dir / packet_name)}")
    print(f"state={rel(root, out_dir / state_name)}")


if __name__ == "__main__":
    main()
