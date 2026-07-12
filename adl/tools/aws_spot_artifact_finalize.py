#!/usr/bin/env python3
"""Finalize one Spot run into redacted, fail-closed retained evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import sys
from typing import Any


SUMMARY_BEGIN = "ADL_AWS_REMOTE_SUMMARY_BEGIN"
SUMMARY_END = "ADL_AWS_REMOTE_SUMMARY_END"
SENSITIVE_KEYS = {
    "account_id",
    "arn",
    "user_id",
    "instance_id",
    "volume_id",
    "vpc_id",
    "subnet_id",
    "security_group_id",
    "command_id",
    "spot_instance_request_id",
}


def sha256(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def redact_text(value: str) -> str:
    patterns = (
        (r"\b\d{12}\b", "<aws-account-id-redacted>"),
        (r"arn:aws(?:-[a-z]+)?:[^\s,\"']+", "<aws-arn-redacted>"),
        (r"\bi-[0-9a-f]{8,17}\b", "<ec2-instance-id-redacted>"),
        (r"\bvol-[0-9a-f]{8,17}\b", "<ebs-volume-id-redacted>"),
        (r"\b(?:vpc|subnet|sg|sir)-[0-9a-f]{8,17}\b", "<aws-resource-id-redacted>"),
        (r"\bAKIA[0-9A-Z]{16}\b", "<aws-access-key-redacted>"),
        (r"(?i)\b(Bearer|Basic)\s+[A-Za-z0-9._~+/=-]+", r"\1 <credential-redacted>"),
        (r"\b(?:\d{1,3}\.){3}\d{1,3}\b", "<ip-address-redacted>"),
    )
    redacted = value
    for pattern, replacement in patterns:
        redacted = re.sub(pattern, replacement, redacted)
    return redacted


def redact_json(value: Any, key: str | None = None) -> Any:
    if key in SENSITIVE_KEYS and isinstance(value, str):
        return {"sha256": sha256(value), "redacted": True}
    if isinstance(value, dict):
        return {name: redact_json(item, name) for name, item in value.items()}
    if isinstance(value, list):
        return [redact_json(item) for item in value]
    if isinstance(value, str):
        return redact_text(value)
    return value


def extract_remote_summary(path: Path) -> dict[str, Any]:
    if not path.is_file():
        return {}
    text = path.read_text(encoding="utf-8", errors="replace")
    start = text.rfind(SUMMARY_BEGIN)
    end = text.rfind(SUMMARY_END)
    if start < 0 or end <= start:
        return {}
    body = text[start + len(SUMMARY_BEGIN) : end].strip()
    try:
        payload = json.loads(body)
    except json.JSONDecodeError:
        return {}
    return payload if isinstance(payload, dict) else {}


def require(condition: bool, failures: list[str], message: str) -> None:
    if not condition:
        failures.append(message)


def redact_artifact_logs(artifact_dir: Path) -> None:
    private_root = artifact_dir / ".private"
    for path in artifact_dir.rglob("*"):
        if not path.is_file() or private_root in path.parents:
            continue
        if path.suffix not in {".json", ".jsonl", ".log", ".txt"}:
            continue
        try:
            text = path.read_text(encoding="utf-8", errors="strict")
        except (OSError, UnicodeDecodeError):
            continue
        path.write_text(redact_text(text), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--summary", required=True, type=Path)
    parser.add_argument("--artifact-dir", required=True, type=Path)
    parser.add_argument("--wrapper-summary", required=True, type=Path)
    parser.add_argument("--expected-source-commit", required=True)
    parser.add_argument("--expected-image", required=True)
    parser.add_argument("--expected-cache-volume-id-sha256", required=True)
    parser.add_argument("--estimated-hourly-cost-usd", required=True, type=float)
    parser.add_argument("--runner-exit-code", required=True, type=int)
    args = parser.parse_args()

    raw: dict[str, Any] = {}
    if args.summary.is_file():
        try:
            loaded = json.loads(args.summary.read_text(encoding="utf-8"))
            if isinstance(loaded, dict):
                raw = loaded
        except json.JSONDecodeError:
            raw = {"status": "unparseable_summary"}

    args.artifact_dir.mkdir(parents=True, exist_ok=True)
    private_dir = args.artifact_dir / ".private"
    private_dir.mkdir(mode=0o700, parents=True, exist_ok=True)
    os.chmod(private_dir, 0o700)
    private_summary = private_dir / "control-summary.json"
    private_summary.write_text(json.dumps(raw, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.chmod(private_summary, 0o600)
    command_status_path = args.artifact_dir / "command-status.log"
    if command_status_path.is_file():
        private_command_status = private_dir / "command-status.log"
        private_command_status.write_bytes(command_status_path.read_bytes())
        os.chmod(private_command_status, 0o600)

    remote = raw.get("remote_summary") if isinstance(raw.get("remote_summary"), dict) else {}
    extracted = extract_remote_summary(args.artifact_dir / "command-stdout.log")
    if extracted:
        remote = extracted
    builder = remote.get("builder_proof") if isinstance(remote.get("builder_proof"), dict) else {}
    expected_digest = args.expected_image.rsplit("@", 1)[-1]
    expected_digest_hash = sha256(expected_digest)

    cache = raw.get("cache_volume") if isinstance(raw.get("cache_volume"), dict) else {}
    cleanup = raw.get("cleanup") if isinstance(raw.get("cleanup"), dict) else {}
    launch = raw.get("launch") if isinstance(raw.get("launch"), dict) else {}
    launch_surface = raw.get("launch_surface") if isinstance(raw.get("launch_surface"), dict) else {}
    timings = raw.get("timings") if isinstance(raw.get("timings"), dict) else {}
    command_status = (
        command_status_path.read_text(encoding="utf-8", errors="replace")
        if command_status_path.is_file()
        else ""
    )
    resume_path = args.artifact_dir / "resume-state.json"
    try:
        resume = json.loads(resume_path.read_text(encoding="utf-8")) if resume_path.is_file() else {}
    except json.JSONDecodeError:
        resume = {}
    attempts = resume.get("attempts") if isinstance(resume.get("attempts"), list) else []
    interrupted_attempts = [
        attempt for attempt in attempts
        if str(attempt.get("status", "")).lower() in {"interrupted_by_aws", "interrupted"}
    ]

    failures: list[str] = []
    require(args.runner_exit_code == 0, failures, "runner_exit_nonzero")
    require(str(raw.get("status", "")).lower() in {"passed", "resumed_after_interruption"}, failures, "run_status_not_passed")
    require(launch.get("purchase_option") == "spot", failures, "purchase_option_not_spot")
    require(cache.get("created") is False, failures, "retained_cache_was_created_or_unproven")
    cache_volume_id = cache.get("volume_id") if isinstance(cache.get("volume_id"), str) else ""
    require(sha256(cache_volume_id) == args.expected_cache_volume_id_sha256, failures, "retained_cache_identity_mismatch")
    require(cache.get("attachment_state") == "attached", failures, "retained_cache_not_attached")
    require(cache.get("mount_path") == "/mnt/adl-cache", failures, "retained_cache_mount_mismatch")
    require(cleanup.get("termination_attempted") is True, failures, "compute_termination_not_attempted")
    require(cleanup.get("final_instance_state") == "terminated", failures, "compute_not_terminated")
    require(not cleanup.get("termination_error"), failures, "compute_termination_error")
    require(launch_surface.get("ssh_debug_enabled") is True, failures, "ssh_debug_not_enabled")
    require("status=ssh_debug_ready" in command_status, failures, "ssh_recovery_not_proven")
    require("status=ssh_tail_started" in command_status, failures, "live_ssh_tail_not_proven")
    require(builder.get("builder_image_immutable") is True, failures, "builder_image_not_immutable")
    require(builder.get("builder_image_digest_sha256") == expected_digest_hash, failures, "builder_image_digest_mismatch")
    require(builder.get("toolchain_verified") is True, failures, "builder_toolchain_not_verified")
    require(builder.get("source_commit_verified") is True, failures, "source_commit_not_verified")
    require(builder.get("source_commit") == args.expected_source_commit, failures, "source_commit_mismatch")
    require(builder.get("cache_mount_verified") is True, failures, "cache_mount_not_verified")
    require(builder.get("cache_writable") is True, failures, "cache_not_writable")
    require(builder.get("host_validation_tools_installed") is False, failures, "host_validation_tool_install_detected")

    total_seconds = int(timings.get("total_seconds") or 0)
    estimated_cost = round(args.estimated_hourly_cost_usd * total_seconds / 3600.0, 6)
    self_verification = {
        "passed": not failures,
        "failures": failures,
        "account_verified_by_wrapper": True,
        "spot_purchase_verified": launch.get("purchase_option") == "spot",
        "immutable_builder_image_verified": builder.get("builder_image_immutable") is True,
        "builder_toolchain_verified": builder.get("toolchain_verified") is True,
        "source_commit_verified": builder.get("source_commit") == args.expected_source_commit,
        "retained_cache_verified": cache.get("created") is False and cache.get("attachment_state") == "attached",
        "retained_cache_identity_verified": sha256(cache_volume_id) == args.expected_cache_volume_id_sha256,
        "cache_mount_health_verified": builder.get("cache_mount_verified") is True and builder.get("cache_writable") is True,
        "ssh_recovery_verified": "status=ssh_debug_ready" in command_status,
        "live_logs_verified": "status=ssh_tail_started" in command_status,
        "compute_teardown_verified": cleanup.get("final_instance_state") == "terminated" and not cleanup.get("termination_error"),
        "host_validation_tools_installed": builder.get("host_validation_tools_installed"),
    }
    wrapper = {
        "schema": "adl.aws_spot_remote_validation_wrapper_summary.v2",
        "status": "passed" if not failures else "failed_self_verification",
        "runner_exit_code": args.runner_exit_code,
        "self_verification": self_verification,
        "source_commit": args.expected_source_commit,
        "builder_image_digest_sha256": expected_digest_hash,
        "builder_image_architecture": builder.get("builder_image_architecture"),
        "cache_target_preexisting_entries": builder.get("cache_target_preexisting_entries"),
        "cache_target_preexisting_bytes": builder.get("cache_target_preexisting_bytes"),
        "cache_free_bytes": builder.get("cache_free_bytes"),
        "timings": {
            "total_seconds": total_seconds,
            "launch_seconds": timings.get("launch_seconds"),
            "ssm_ready_seconds": timings.get("ssm_ready_seconds"),
            "remote_command_seconds": timings.get("remote_command_seconds"),
            "validation_seconds": builder.get("validation_seconds"),
            "teardown_seconds": timings.get("teardown_seconds"),
        },
        "cost": {
            "estimated_hourly_usd": args.estimated_hourly_cost_usd,
            "estimated_compute_cost_usd": estimated_cost,
            "estimate_basis": "observed_instance_lifetime_seconds_x_pre_run_spot_hourly_price",
        },
        "private_recovery_state_retained": True,
        "private_recovery_state_uploaded": False,
        "attempt_count": len(attempts),
        "interrupted_attempt_count": len(interrupted_attempts),
        "resumed_after_interruption": str(raw.get("status", "")).lower() == "resumed_after_interruption",
        "next_action": resume.get("next_action"),
    }

    redacted = redact_json(raw)
    args.summary.parent.mkdir(parents=True, exist_ok=True)
    args.summary.write_text(json.dumps(redacted, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    redact_artifact_logs(args.artifact_dir)
    args.wrapper_summary.write_text(json.dumps(wrapper, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    if failures:
        print("aws_spot_artifact_finalize: self-verification failed: " + ", ".join(failures), file=sys.stderr)
        return 1
    print(json.dumps(wrapper, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
