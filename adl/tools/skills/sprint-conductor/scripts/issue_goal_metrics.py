#!/usr/bin/env python3
from __future__ import annotations

from copy import deepcopy
from datetime import datetime, timezone
import json
from pathlib import Path
import re
from typing import Any


AVAILABILITY_VALUES = {"known", "unknown", "not_collected", "not_applicable", "not_available"}
CAPTURE_STAGES = {
    "issue_init",
    "doctor_readiness",
    "card_repair",
    "execution_ready",
    "issue_start",
    "pr_publication",
    "review_handoff",
    "merge_closeout",
    "sprint_closeout",
}
CAPTURE_SEGMENT_BY_STAGE = {
    "issue_init": "readiness_prep",
    "doctor_readiness": "readiness_prep",
    "card_repair": "readiness_prep",
    "execution_ready": "readiness_prep",
    "issue_start": "bound_execution",
    "pr_publication": "bound_execution",
    "review_handoff": "bound_execution",
    "merge_closeout": "bound_execution",
    "sprint_closeout": "sprint_rollup",
}
DATA_SOURCE_VALUES = {
    "codex_goal_tool",
    "manual_entry",
    "derived_sprint_state",
    "unknown",
}
METRICS_CONFIDENCE_VALUES = {"low", "medium", "high", "unknown"}
COMPLETION_STATE_VALUES = {
    "completed",
    "completed_with_follow_on",
    "blocked",
    "failed",
    "deferred",
    "cancelled",
    "unknown",
}
STAGE_PRIORITY = {
    "issue_init": 1,
    "card_repair": 2,
    "doctor_readiness": 3,
    "execution_ready": 4,
    "issue_start": 5,
    "pr_publication": 6,
    "review_handoff": 7,
    "merge_closeout": 8,
    "sprint_closeout": 9,
}

CODEX_GOAL_STATUS_TO_COMPLETION_STATE = {
    "active": "unknown",
    "completed": "completed",
    "complete": "completed",
    "blocked": "blocked",
    "budgetlimited": "deferred",
    "budget_limited": "deferred",
}

TERMINAL_COMPLETION_STATES = {"completed", "blocked", "failed", "cancelled", "deferred"}
GOAL_KIND_VALUES = {
    "tracked_issue",
    "setup_only",
    "implementation",
    "watcher",
    "janitor",
    "review_only",
    "sprint_child",
    "sprint_umbrella",
}
GOAL_BOUNDARY_VALUES = {
    "handoff_only",
    "pr_green",
    "merged",
    "closed_no_pr",
    "closed_out",
    "watch_target_reached",
    "sprint_rollup_settled",
}
ISSUE_STATE_VALUES = {"open", "closed", "unknown", "not_applicable"}
PR_STATE_VALUES = {"not_opened", "open", "draft", "merged", "closed", "unknown", "not_applicable"}
CHECKS_STATE_VALUES = {"green", "pending", "red", "missing", "skipped", "unknown", "not_applicable"}
RECORD_TRUTH_VALUES = {"current", "missing", "stale", "unknown", "not_applicable"}
TARGET_STATE_VALUES = {"reached", "not_reached", "blocked", "unknown", "not_applicable"}
TERMINAL_TRUTH_STATUS_VALUES = {"satisfied", "not_satisfied", "unknown"}

DEFAULT_BOUNDARY_BY_GOAL_KIND = {
    "tracked_issue": "pr_green",
    "setup_only": "handoff_only",
    "implementation": "pr_green",
    "watcher": "watch_target_reached",
    "janitor": "watch_target_reached",
    "review_only": "handoff_only",
    "sprint_child": "closed_out",
    "sprint_umbrella": "sprint_rollup_settled",
}


def iso_now_utc() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def epoch_seconds_to_iso(value: Any) -> str | None:
    if value is None:
        return None
    if not isinstance(value, int):
        return None
    return datetime.fromtimestamp(value, tz=timezone.utc).replace(microsecond=0).isoformat().replace(
        "+00:00", "Z"
    )


def parse_codex_goal_tool_snapshot(path: str) -> dict[str, Any]:
    payload = json.loads(Path(path).read_text())
    goal = payload.get("goal")
    if not isinstance(goal, dict):
        raise ValueError("goal snapshot does not contain an active goal object")

    created_at_raw = goal.get("createdAt")
    updated_at_raw = goal.get("updatedAt")
    status_raw = str(goal.get("status") or "unknown")
    normalized_status = status_raw.strip().lower()
    completion_state = CODEX_GOAL_STATUS_TO_COMPLETION_STATE.get(normalized_status, "unknown")

    started_at = epoch_seconds_to_iso(created_at_raw)
    completed_at = epoch_seconds_to_iso(updated_at_raw) if completion_state in TERMINAL_COMPLETION_STATES else None
    elapsed_seconds_raw = None
    if (
        isinstance(created_at_raw, int)
        and isinstance(updated_at_raw, int)
        and updated_at_raw >= created_at_raw
    ):
        elapsed_seconds_raw = str(updated_at_raw - created_at_raw)

    tokens_used_raw = goal.get("tokensUsed")
    time_used_raw = goal.get("timeUsedSeconds")

    return {
        "thread_id": goal.get("threadId"),
        "objective": goal.get("objective"),
        "recorded_at": epoch_seconds_to_iso(updated_at_raw) or iso_now_utc(),
        "status": normalized_status,
        "completion_state": completion_state,
        "started_at": started_at,
        "completed_at": completed_at,
        "elapsed_seconds_raw": elapsed_seconds_raw,
        "active_work_seconds_raw": str(time_used_raw) if isinstance(time_used_raw, int) else "unknown",
        "total_tokens_raw": str(tokens_used_raw) if isinstance(tokens_used_raw, int) else "unknown",
    }


def extract_first_json_object(raw: str) -> dict[str, Any] | None:
    text = raw.strip()
    if not text:
        return None
    decoder = json.JSONDecoder()
    for index, char in enumerate(text):
        if char != "{":
            continue
        try:
            value, _ = decoder.raw_decode(text[index:])
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict):
            return value
    return None


def extract_issue_numbers_from_text(raw: str | None) -> set[int]:
    if not isinstance(raw, str):
        return set()
    return {
        int(match.group(1))
        for match in re.finditer(r"(?<!\w)(?:issue\s*)?#(\d+)\b", raw, flags=re.IGNORECASE)
    }


def codex_goal_payload_matches_issue_number(payload: dict[str, Any], issue_number: int | None) -> bool:
    if issue_number is None:
        return True
    goal = payload.get("goal")
    if not isinstance(goal, dict):
        return False
    objective_issue_numbers = extract_issue_numbers_from_text(goal.get("objective"))
    return issue_number in objective_issue_numbers


def find_codex_session_transcript(
    thread_id: str | None,
    session_root: str,
    *,
    issue_number: int | None = None,
) -> str | None:
    root = Path(session_root).expanduser()
    if not root.exists():
        return None
    if thread_id:
        candidates = sorted(root.rglob(f"*{thread_id}*.jsonl"), key=lambda path: path.stat().st_mtime, reverse=True)
    else:
        candidates = sorted(root.rglob("*.jsonl"), key=lambda path: path.stat().st_mtime, reverse=True)
    for candidate in candidates:
        payload = load_codex_goal_payload_from_session_transcript(
            str(candidate),
            thread_id=thread_id,
            issue_number=issue_number,
        )
        if payload is not None:
            return str(candidate)
    return None


def load_codex_goal_payload_from_session_transcript(
    transcript_path: str,
    *,
    thread_id: str | None = None,
    issue_number: int | None = None,
) -> dict[str, Any] | None:
    transcript = Path(transcript_path)
    if not transcript.exists():
        return None

    goal_call_ids: set[str] = set()
    latest_payload: dict[str, Any] | None = None
    for raw_line in transcript.read_text().splitlines():
        if not raw_line.strip():
            continue
        try:
            line = json.loads(raw_line)
        except json.JSONDecodeError:
            continue
        payload = line.get("payload")
        if not isinstance(payload, dict):
            continue
        item = payload.get("item")
        if isinstance(item, dict):
            item_type = item.get("type")
            if item_type == "function_call" and item.get("name") == "get_goal":
                call_id = item.get("call_id")
                if isinstance(call_id, str):
                    goal_call_ids.add(call_id)
                continue
            if item_type == "function_call_output":
                call_id = item.get("call_id")
                if goal_call_ids and call_id not in goal_call_ids:
                    continue
                output_text = item.get("output")
                if not isinstance(output_text, str):
                    output_text = payload.get("output")
                if not isinstance(output_text, str):
                    continue
                candidate = extract_first_json_object(output_text)
                if not isinstance(candidate, dict):
                    continue
                goal = candidate.get("goal")
                if not isinstance(goal, dict):
                    continue
                if thread_id is not None and goal.get("threadId") not in {thread_id, None}:
                    continue
                if not codex_goal_payload_matches_issue_number(candidate, issue_number):
                    continue
                latest_payload = candidate
                continue

        output_text = payload.get("output")
        if not isinstance(output_text, str):
            continue
        candidate = extract_first_json_object(output_text)
        if not isinstance(candidate, dict):
            continue
        goal = candidate.get("goal")
        if not isinstance(goal, dict):
            continue
        if thread_id is not None and goal.get("threadId") not in {thread_id, None}:
            continue
        if not codex_goal_payload_matches_issue_number(candidate, issue_number):
            continue
        latest_payload = candidate
    return latest_payload


def default_goal_metrics_summary() -> dict[str, Any]:
    return {
        "status": "not_recorded",
        "raw_log_path": None,
        "record_count": 0,
        "phases_recorded": [],
        "segments_recorded": [],
        "selected_stage": None,
        "selected_segment": None,
        "recorded_at": None,
        "data_source": "unknown",
        "metrics_confidence": "unknown",
        "issue_goal_ref": None,
        "sprint_goal_ref": None,
        "goal_metrics_rollup_ref": None,
        "goal_id": None,
        "goal_id_availability": "unknown",
        "started_at": None,
        "completed_at": None,
        "elapsed_seconds": None,
        "elapsed_availability": "unknown",
        "active_work_seconds": None,
        "active_work_availability": "unknown",
        "validation_seconds": None,
        "validation_availability": "unknown",
        "pr_wait_seconds": None,
        "pr_wait_availability": "unknown",
        "ci_wait_seconds": None,
        "ci_wait_availability": "unknown",
        "completion_state": "unknown",
        "token_usage": {
            "total_tokens": None,
            "prompt_tokens": None,
            "completion_tokens": None,
            "availability": "unknown",
            "total_availability": "unknown",
            "prompt_availability": "unknown",
            "completion_availability": "unknown",
        },
        "model_ref": None,
        "session_ref": None,
        "thread_id": None,
        "goal_terminal_state": default_goal_terminal_state_summary(),
    }


def default_goal_terminal_state_summary() -> dict[str, Any]:
    return {
        "goal_kind": "tracked_issue",
        "declared_boundary": "pr_green",
        "boundary_source": "default_from_goal_kind",
        "issue_state": "unknown",
        "pr_state": "unknown",
        "checks_state": "unknown",
        "review_truth": "unknown",
        "closeout_truth": "unknown",
        "merge_conflicts": "unknown",
        "watch_target_status": "unknown",
        "sprint_rollup_status": "unknown",
        "completion_allowed": False,
        "truth_status": "unknown",
        "reason": "goal terminal-state truth was not evaluated",
    }


def normalize_choice(
    value: str | None,
    *,
    default: str,
    allowed: set[str],
    field_name: str,
) -> str:
    if value is None:
        return default
    lowered = value.strip().lower()
    if lowered not in allowed:
        raise ValueError(f"invalid {field_name}: {value}")
    return lowered


def default_boundary_for_goal_kind(goal_kind: str) -> str:
    return DEFAULT_BOUNDARY_BY_GOAL_KIND.get(goal_kind, "pr_green")


def evaluate_goal_terminal_state(
    *,
    goal_kind: str | None,
    declared_boundary: str | None,
    issue_state: str | None,
    pr_state: str | None,
    checks_state: str | None,
    review_truth: str | None,
    closeout_truth: str | None,
    merge_conflicts: bool | None,
    watch_target_status: str | None,
    sprint_rollup_status: str | None,
) -> dict[str, Any]:
    normalized_goal_kind = normalize_choice(
        goal_kind,
        default="tracked_issue",
        allowed=GOAL_KIND_VALUES,
        field_name="goal kind",
    )
    if declared_boundary is None:
        normalized_boundary = default_boundary_for_goal_kind(normalized_goal_kind)
        boundary_source = "default_from_goal_kind"
    else:
        normalized_boundary = normalize_choice(
            declared_boundary,
            default="pr_green",
            allowed=GOAL_BOUNDARY_VALUES,
            field_name="goal boundary",
        )
        boundary_source = "explicit"

    normalized_issue_state = normalize_choice(
        issue_state,
        default="unknown",
        allowed=ISSUE_STATE_VALUES,
        field_name="issue state",
    )
    normalized_pr_state = normalize_choice(
        pr_state,
        default="unknown",
        allowed=PR_STATE_VALUES,
        field_name="PR state",
    )
    normalized_checks_state = normalize_choice(
        checks_state,
        default="unknown",
        allowed=CHECKS_STATE_VALUES,
        field_name="checks state",
    )
    normalized_review_truth = normalize_choice(
        review_truth,
        default="unknown",
        allowed=RECORD_TRUTH_VALUES,
        field_name="review truth",
    )
    normalized_closeout_truth = normalize_choice(
        closeout_truth,
        default="unknown",
        allowed=RECORD_TRUTH_VALUES,
        field_name="closeout truth",
    )
    normalized_watch_target_status = normalize_choice(
        watch_target_status,
        default="unknown",
        allowed=TARGET_STATE_VALUES,
        field_name="watch target status",
    )
    normalized_sprint_rollup_status = normalize_choice(
        sprint_rollup_status,
        default="unknown",
        allowed=TARGET_STATE_VALUES,
        field_name="sprint rollup status",
    )
    merge_conflicts_value = "unknown" if merge_conflicts is None else str(bool(merge_conflicts)).lower()

    result = {
        "goal_kind": normalized_goal_kind,
        "declared_boundary": normalized_boundary,
        "boundary_source": boundary_source,
        "issue_state": normalized_issue_state,
        "pr_state": normalized_pr_state,
        "checks_state": normalized_checks_state,
        "review_truth": normalized_review_truth,
        "closeout_truth": normalized_closeout_truth,
        "merge_conflicts": merge_conflicts_value,
        "watch_target_status": normalized_watch_target_status,
        "sprint_rollup_status": normalized_sprint_rollup_status,
        "completion_allowed": False,
        "truth_status": "unknown",
        "reason": "",
    }

    def satisfied(reason: str) -> dict[str, Any]:
        result["completion_allowed"] = True
        result["truth_status"] = "satisfied"
        result["reason"] = reason
        return result

    def not_satisfied(reason: str) -> dict[str, Any]:
        result["completion_allowed"] = False
        result["truth_status"] = "not_satisfied"
        result["reason"] = reason
        return result

    def unknown(reason: str) -> dict[str, Any]:
        result["completion_allowed"] = False
        result["truth_status"] = "unknown"
        result["reason"] = reason
        return result

    if normalized_boundary == "handoff_only":
        return satisfied(
            "goal explicitly declares handoff-only completion; terminal completion may be recorded at the documented handoff boundary"
        )

    if normalized_boundary == "watch_target_reached":
        if normalized_watch_target_status == "reached":
            return satisfied("watch/janitor target state is reached")
        if normalized_watch_target_status in {"not_reached", "blocked"}:
            return not_satisfied("watch/janitor target state has not reached its declared terminal condition")
        return unknown("watch/janitor target state is not yet known")

    if normalized_boundary == "sprint_rollup_settled":
        if normalized_sprint_rollup_status == "reached":
            return satisfied("sprint child-issue rollup is settled")
        if normalized_sprint_rollup_status in {"not_reached", "blocked"}:
            return not_satisfied("sprint child-issue rollup is not settled")
        return unknown("sprint child-issue rollup state is not yet known")

    if merge_conflicts is True:
        return not_satisfied("merge conflicts are still present")
    if normalized_pr_state == "draft":
        return not_satisfied("PR is still draft")
    if normalized_pr_state == "open" and normalized_checks_state == "pending":
        return not_satisfied("required checks are still pending")
    if normalized_pr_state == "open" and normalized_checks_state == "red":
        return not_satisfied("required checks are failing")
    if normalized_pr_state == "open" and normalized_checks_state == "missing":
        return not_satisfied("required checks are missing")
    if normalized_review_truth in {"missing", "stale"}:
        return not_satisfied("SRP/SOR review truth is not current")

    if normalized_boundary == "pr_green":
        if normalized_pr_state != "open":
            return not_satisfied("PR must be open and reviewable for the default tracked-issue terminal boundary")
        if normalized_checks_state in {"green", "skipped"}:
            return satisfied("PR checks are green or explicitly skipped and review truth is current")
        if normalized_checks_state == "unknown":
            return unknown("PR checks state is not known")
        return not_satisfied("PR has not reached a green-or-skipped checks state")

    if normalized_boundary == "merged":
        if normalized_pr_state == "merged":
            return satisfied("PR is merged")
        if normalized_pr_state in {"open", "draft"}:
            return not_satisfied("PR is not merged yet")
        if normalized_pr_state == "unknown":
            return unknown("PR merge state is not known")
        return not_satisfied("PR is not in a merged state")

    if normalized_boundary == "closed_no_pr":
        if normalized_issue_state == "closed" and normalized_pr_state in {"not_applicable", "not_opened", "closed"}:
            return satisfied("issue is closed without an active PR requirement")
        if normalized_issue_state == "unknown":
            return unknown("issue closure state is not known")
        return not_satisfied("issue is not closed in a no-PR terminal state")

    if normalized_boundary == "closed_out":
        if normalized_closeout_truth in {"missing", "stale"}:
            return not_satisfied("closeout truth is not current")
        if normalized_issue_state == "closed" and normalized_closeout_truth == "current":
            return satisfied("issue is closed and closeout truth is current")
        if normalized_issue_state == "unknown" or normalized_closeout_truth == "unknown":
            return unknown("closed-out terminal state cannot be confirmed yet")
        return not_satisfied("issue is not yet closed out")

    return unknown("goal terminal-state boundary could not be evaluated")


def validate_terminal_completion_allowed(record: dict[str, Any]) -> None:
    completion_state = str(record.get("completion_state") or "unknown").strip().lower()
    if completion_state not in {"completed", "completed_with_follow_on"}:
        return
    terminal_truth = record.get("goal_terminal_state") or {}
    if terminal_truth.get("completion_allowed") is True:
        return
    reason = terminal_truth.get("reason") or "goal terminal-state truth is not satisfied"
    raise ValueError(
        "issue goal cannot be recorded as completed before the declared terminal state is satisfied: "
        f"{reason}"
    )


def parse_availability_int(raw: str | None) -> tuple[str, int | None]:
    if raw is None:
        return "unknown", None
    lowered = raw.strip().lower()
    if lowered in AVAILABILITY_VALUES - {"known"}:
        return lowered, None
    return "known", int(lowered)


def normalize_goal_id(goal_id: str | None, goal_id_state: str | None) -> tuple[str | None, str]:
    if goal_id is not None:
        return goal_id, "known"
    if goal_id_state is None:
        return None, "unknown"
    lowered = goal_id_state.strip().lower()
    if lowered not in AVAILABILITY_VALUES:
        raise ValueError(f"invalid goal_id availability: {goal_id_state}")
    return None, lowered


def build_issue_goal_metrics_record(
    *,
    sprint_issue_number: int | None,
    issue_number: int,
    capture_stage: str,
    data_source: str,
    raw_log_path: str,
    recorded_at: str | None,
    issue_goal_ref: str | None,
    sprint_goal_ref: str | None,
    goal_metrics_rollup_ref: str | None,
    goal_id: str | None,
    goal_id_state: str | None,
    started_at: str | None,
    completed_at: str | None,
    elapsed_seconds_raw: str | None,
    active_work_seconds_raw: str | None,
    validation_seconds_raw: str | None,
    pr_wait_seconds_raw: str | None,
    ci_wait_seconds_raw: str | None,
    total_tokens_raw: str | None,
    prompt_tokens_raw: str | None,
    completion_tokens_raw: str | None,
    metrics_confidence: str,
    completion_state: str,
    model_ref: str | None,
    session_ref: str | None,
    thread_id: str | None,
    goal_kind: str | None = None,
    goal_boundary: str | None = None,
    issue_state: str | None = None,
    pr_state: str | None = None,
    checks_state: str | None = None,
    review_truth: str | None = None,
    closeout_truth: str | None = None,
    merge_conflicts: bool | None = None,
    watch_target_status: str | None = None,
    sprint_rollup_status: str | None = None,
) -> dict[str, Any]:
    if capture_stage not in CAPTURE_STAGES:
        raise ValueError(f"invalid capture stage: {capture_stage}")
    if data_source not in DATA_SOURCE_VALUES:
        raise ValueError(f"invalid data source: {data_source}")
    if metrics_confidence not in METRICS_CONFIDENCE_VALUES:
        raise ValueError(f"invalid metrics confidence: {metrics_confidence}")
    if completion_state not in COMPLETION_STATE_VALUES:
        raise ValueError(f"invalid completion state: {completion_state}")

    goal_id_value, goal_id_availability = normalize_goal_id(goal_id, goal_id_state)
    elapsed_availability, elapsed_seconds = parse_availability_int(elapsed_seconds_raw)
    active_work_availability, active_work_seconds = parse_availability_int(active_work_seconds_raw)
    validation_availability, validation_seconds = parse_availability_int(validation_seconds_raw)
    pr_wait_availability, pr_wait_seconds = parse_availability_int(pr_wait_seconds_raw)
    ci_wait_availability, ci_wait_seconds = parse_availability_int(ci_wait_seconds_raw)
    total_availability, total_tokens = parse_availability_int(total_tokens_raw)
    prompt_availability, prompt_tokens = parse_availability_int(prompt_tokens_raw)
    completion_availability, completion_tokens = parse_availability_int(completion_tokens_raw)
    token_availability = (
        "known"
        if any(value == "known" for value in (total_availability, prompt_availability, completion_availability))
        else "not_available"
        if all(value == "not_available" for value in (total_availability, prompt_availability, completion_availability))
        else "unknown"
    )
    goal_terminal_state = evaluate_goal_terminal_state(
        goal_kind=goal_kind,
        declared_boundary=goal_boundary,
        issue_state=issue_state,
        pr_state=pr_state,
        checks_state=checks_state,
        review_truth=review_truth,
        closeout_truth=closeout_truth,
        merge_conflicts=merge_conflicts,
        watch_target_status=watch_target_status,
        sprint_rollup_status=sprint_rollup_status,
    )

    return {
        "schema_version": "issue_goal_metrics.v1",
        "recorded_at": recorded_at or iso_now_utc(),
        "sprint_issue_number": sprint_issue_number,
        "issue_number": issue_number,
        "capture_stage": capture_stage,
        "metrics_segment": CAPTURE_SEGMENT_BY_STAGE[capture_stage],
        "data_source": data_source,
        "metrics_confidence": metrics_confidence,
        "raw_log_path": raw_log_path,
        "issue_goal_ref": issue_goal_ref,
        "sprint_goal_ref": sprint_goal_ref,
        "goal_metrics_rollup_ref": goal_metrics_rollup_ref,
        "goal_id": goal_id_value,
        "goal_id_availability": goal_id_availability,
        "started_at": started_at,
        "completed_at": completed_at,
        "elapsed_seconds": elapsed_seconds,
        "elapsed_availability": elapsed_availability,
        "active_work_seconds": active_work_seconds,
        "active_work_availability": active_work_availability,
        "validation_seconds": validation_seconds,
        "validation_availability": validation_availability,
        "pr_wait_seconds": pr_wait_seconds,
        "pr_wait_availability": pr_wait_availability,
        "ci_wait_seconds": ci_wait_seconds,
        "ci_wait_availability": ci_wait_availability,
        "completion_state": completion_state,
        "token_usage": {
            "total_tokens": total_tokens,
            "prompt_tokens": prompt_tokens,
            "completion_tokens": completion_tokens,
            "availability": token_availability,
            "total_availability": total_availability,
            "prompt_availability": prompt_availability,
            "completion_availability": completion_availability,
        },
        "model_ref": model_ref,
        "session_ref": session_ref,
        "thread_id": thread_id,
        "goal_terminal_state": goal_terminal_state,
    }


def build_issue_goal_metrics_record_from_codex_goal_snapshot(
    *,
    issue_number: int,
    capture_stage: str,
    goal_state_path: str,
    raw_log_path: str,
    issue_goal_ref: str | None,
    sprint_goal_ref: str | None,
    goal_metrics_rollup_ref: str | None,
    metrics_confidence: str,
    completion_state_override: str | None,
    model_ref: str | None,
    session_ref: str | None,
    goal_kind: str | None = None,
    goal_boundary: str | None = None,
    issue_state: str | None = None,
    pr_state: str | None = None,
    checks_state: str | None = None,
    review_truth: str | None = None,
    closeout_truth: str | None = None,
    merge_conflicts: bool | None = None,
    watch_target_status: str | None = None,
    sprint_rollup_status: str | None = None,
) -> dict[str, Any]:
    snapshot = parse_codex_goal_tool_snapshot(goal_state_path)
    completion_state = completion_state_override or snapshot["completion_state"]
    thread_id = snapshot.get("thread_id")
    return build_issue_goal_metrics_record(
        sprint_issue_number=None,
        issue_number=issue_number,
        capture_stage=capture_stage,
        data_source="codex_goal_tool",
        raw_log_path=raw_log_path,
        recorded_at=snapshot.get("recorded_at"),
        issue_goal_ref=issue_goal_ref,
        sprint_goal_ref=sprint_goal_ref,
        goal_metrics_rollup_ref=goal_metrics_rollup_ref,
        goal_id=None,
        goal_id_state="not_available",
        started_at=snapshot.get("started_at"),
        completed_at=snapshot.get("completed_at"),
        elapsed_seconds_raw=snapshot.get("elapsed_seconds_raw"),
        active_work_seconds_raw=snapshot.get("active_work_seconds_raw"),
        validation_seconds_raw="unknown",
        pr_wait_seconds_raw="unknown",
        ci_wait_seconds_raw="unknown",
        total_tokens_raw=snapshot.get("total_tokens_raw"),
        prompt_tokens_raw="unknown",
        completion_tokens_raw="unknown",
        metrics_confidence=metrics_confidence,
        completion_state=completion_state,
        model_ref=model_ref,
        session_ref=session_ref or (f"codex-thread:{thread_id}" if isinstance(thread_id, str) else None),
        thread_id=thread_id if isinstance(thread_id, str) else None,
        goal_kind=goal_kind,
        goal_boundary=goal_boundary,
        issue_state=issue_state,
        pr_state=pr_state,
        checks_state=checks_state,
        review_truth=review_truth,
        closeout_truth=closeout_truth,
        merge_conflicts=merge_conflicts,
        watch_target_status=watch_target_status,
        sprint_rollup_status=sprint_rollup_status,
    )


def build_unknown_issue_goal_metrics_record(
    *,
    issue_number: int,
    capture_stage: str,
    raw_log_path: str,
    issue_goal_ref: str | None,
    sprint_goal_ref: str | None,
    goal_metrics_rollup_ref: str | None,
    metrics_confidence: str,
    completion_state: str,
    model_ref: str | None,
    session_ref: str | None,
    thread_id: str | None,
    goal_kind: str | None = None,
    goal_boundary: str | None = None,
    issue_state: str | None = None,
    pr_state: str | None = None,
    checks_state: str | None = None,
    review_truth: str | None = None,
    closeout_truth: str | None = None,
    merge_conflicts: bool | None = None,
    watch_target_status: str | None = None,
    sprint_rollup_status: str | None = None,
) -> dict[str, Any]:
    return build_issue_goal_metrics_record(
        sprint_issue_number=None,
        issue_number=issue_number,
        capture_stage=capture_stage,
        data_source="unknown",
        raw_log_path=raw_log_path,
        recorded_at=iso_now_utc(),
        issue_goal_ref=issue_goal_ref,
        sprint_goal_ref=sprint_goal_ref,
        goal_metrics_rollup_ref=goal_metrics_rollup_ref,
        goal_id=None,
        goal_id_state="not_available",
        started_at=None,
        completed_at=None,
        elapsed_seconds_raw="unknown",
        active_work_seconds_raw="unknown",
        validation_seconds_raw="unknown",
        pr_wait_seconds_raw="unknown",
        ci_wait_seconds_raw="unknown",
        total_tokens_raw="unknown",
        prompt_tokens_raw="unknown",
        completion_tokens_raw="unknown",
        metrics_confidence=metrics_confidence,
        completion_state=completion_state,
        model_ref=model_ref,
        session_ref=session_ref,
        thread_id=thread_id,
        goal_kind=goal_kind,
        goal_boundary=goal_boundary,
        issue_state=issue_state,
        pr_state=pr_state,
        checks_state=checks_state,
        review_truth=review_truth,
        closeout_truth=closeout_truth,
        merge_conflicts=merge_conflicts,
        watch_target_status=watch_target_status,
        sprint_rollup_status=sprint_rollup_status,
    )


def summarize_issue_goal_metrics(records: list[dict[str, Any]], raw_log_path: str | None) -> dict[str, Any]:
    if not records:
        summary = default_goal_metrics_summary()
        summary["raw_log_path"] = raw_log_path
        return summary

    def sort_key(record: dict[str, Any]) -> tuple[int, str]:
        return (
            STAGE_PRIORITY.get(record.get("capture_stage"), 0),
            record.get("recorded_at") or "",
        )

    selected = sorted(records, key=sort_key)[-1]
    summary = default_goal_metrics_summary()
    summary["status"] = "recorded"
    summary["raw_log_path"] = raw_log_path
    summary["record_count"] = len(records)
    summary["phases_recorded"] = sorted(
        {record.get("capture_stage") for record in records if record.get("capture_stage")}
    )
    summary["segments_recorded"] = sorted(
        {record.get("metrics_segment") for record in records if record.get("metrics_segment")}
    )
    summary["selected_stage"] = selected.get("capture_stage")
    summary["selected_segment"] = selected.get("metrics_segment")
    summary["recorded_at"] = selected.get("recorded_at")
    summary["data_source"] = selected.get("data_source") or "unknown"
    summary["metrics_confidence"] = selected.get("metrics_confidence") or "unknown"
    summary["issue_goal_ref"] = selected.get("issue_goal_ref")
    summary["sprint_goal_ref"] = selected.get("sprint_goal_ref")
    summary["goal_metrics_rollup_ref"] = selected.get("goal_metrics_rollup_ref")
    summary["goal_id"] = selected.get("goal_id")
    summary["goal_id_availability"] = selected.get("goal_id_availability") or "unknown"
    summary["started_at"] = selected.get("started_at")
    summary["completed_at"] = selected.get("completed_at")
    summary["elapsed_seconds"] = selected.get("elapsed_seconds")
    summary["elapsed_availability"] = selected.get("elapsed_availability") or "unknown"
    summary["active_work_seconds"] = selected.get("active_work_seconds")
    summary["active_work_availability"] = selected.get("active_work_availability") or "unknown"
    summary["validation_seconds"] = selected.get("validation_seconds")
    summary["validation_availability"] = selected.get("validation_availability") or "unknown"
    summary["pr_wait_seconds"] = selected.get("pr_wait_seconds")
    summary["pr_wait_availability"] = selected.get("pr_wait_availability") or "unknown"
    summary["ci_wait_seconds"] = selected.get("ci_wait_seconds")
    summary["ci_wait_availability"] = selected.get("ci_wait_availability") or "unknown"
    summary["completion_state"] = selected.get("completion_state") or "unknown"
    summary["token_usage"] = deepcopy(selected.get("token_usage") or summary["token_usage"])
    summary["model_ref"] = selected.get("model_ref")
    summary["session_ref"] = selected.get("session_ref")
    summary["thread_id"] = selected.get("thread_id")
    summary["goal_terminal_state"] = deepcopy(
        selected.get("goal_terminal_state") or default_goal_terminal_state_summary()
    )
    return summary


def compute_goal_metrics_rollup(issue_records: list[dict[str, Any]]) -> dict[str, Any]:
    availability_counts = lambda: {availability: 0 for availability in sorted(AVAILABILITY_VALUES)}
    rollup = {
        "issue_count": 0,
        "issues_with_recorded_metrics": 0,
        "issues_without_recorded_metrics": 0,
        "issues_with_known_elapsed": 0,
        "issues_with_unknown_elapsed": 0,
        "issues_with_known_active_work": 0,
        "issues_with_unknown_active_work": 0,
        "issues_with_known_validation_seconds": 0,
        "issues_with_unknown_validation_seconds": 0,
        "issues_with_known_pr_wait": 0,
        "issues_with_unknown_pr_wait": 0,
        "issues_with_known_ci_wait": 0,
        "issues_with_unknown_ci_wait": 0,
        "issues_with_known_total_tokens": 0,
        "issues_with_unknown_total_tokens": 0,
        "total_elapsed_seconds_known_sum": 0,
        "total_active_work_seconds_known_sum": 0,
        "total_validation_seconds_known_sum": 0,
        "total_pr_wait_seconds_known_sum": 0,
        "total_ci_wait_seconds_known_sum": 0,
        "total_tokens_known_sum": 0,
        "data_source_counts": {source: 0 for source in sorted(DATA_SOURCE_VALUES)},
        "goal_id_availability_counts": {availability: 0 for availability in sorted(AVAILABILITY_VALUES)},
        "completion_state_counts": {state: 0 for state in sorted(COMPLETION_STATE_VALUES)},
        "terminal_truth_status_counts": {state: 0 for state in sorted(TERMINAL_TRUTH_STATUS_VALUES)},
        "elapsed_availability_counts": availability_counts(),
        "active_work_availability_counts": availability_counts(),
        "validation_availability_counts": availability_counts(),
        "pr_wait_availability_counts": availability_counts(),
        "ci_wait_availability_counts": availability_counts(),
        "total_token_availability_counts": availability_counts(),
    }
    for record in issue_records:
        rollup["issue_count"] += 1
        summary = record.get("goal_metrics") or default_goal_metrics_summary()
        if summary.get("status") == "recorded":
            rollup["issues_with_recorded_metrics"] += 1
        else:
            rollup["issues_without_recorded_metrics"] += 1

        data_source = summary.get("data_source") or "unknown"
        if data_source not in rollup["data_source_counts"]:
            rollup["data_source_counts"][data_source] = 0
        rollup["data_source_counts"][data_source] += 1

        goal_id_availability = summary.get("goal_id_availability") or "unknown"
        if goal_id_availability not in rollup["goal_id_availability_counts"]:
            rollup["goal_id_availability_counts"][goal_id_availability] = 0
        rollup["goal_id_availability_counts"][goal_id_availability] += 1

        completion_state = summary.get("completion_state") or "unknown"
        if completion_state not in rollup["completion_state_counts"]:
            rollup["completion_state_counts"][completion_state] = 0
        rollup["completion_state_counts"][completion_state] += 1

        terminal_truth = summary.get("goal_terminal_state") or default_goal_terminal_state_summary()
        truth_status = terminal_truth.get("truth_status") or "unknown"
        if truth_status not in rollup["terminal_truth_status_counts"]:
            rollup["terminal_truth_status_counts"][truth_status] = 0
        rollup["terminal_truth_status_counts"][truth_status] += 1

        elapsed_availability = summary.get("elapsed_availability") or "unknown"
        if elapsed_availability not in rollup["elapsed_availability_counts"]:
            rollup["elapsed_availability_counts"][elapsed_availability] = 0
        rollup["elapsed_availability_counts"][elapsed_availability] += 1
        if elapsed_availability == "known" and isinstance(summary.get("elapsed_seconds"), int):
            rollup["issues_with_known_elapsed"] += 1
            rollup["total_elapsed_seconds_known_sum"] += int(summary["elapsed_seconds"])
        elif elapsed_availability == "unknown":
            rollup["issues_with_unknown_elapsed"] += 1

        active_work_availability = summary.get("active_work_availability") or "unknown"
        if active_work_availability not in rollup["active_work_availability_counts"]:
            rollup["active_work_availability_counts"][active_work_availability] = 0
        rollup["active_work_availability_counts"][active_work_availability] += 1
        if active_work_availability == "known" and isinstance(summary.get("active_work_seconds"), int):
            rollup["issues_with_known_active_work"] += 1
            rollup["total_active_work_seconds_known_sum"] += int(summary["active_work_seconds"])
        elif active_work_availability == "unknown":
            rollup["issues_with_unknown_active_work"] += 1

        validation_availability = summary.get("validation_availability") or "unknown"
        if validation_availability not in rollup["validation_availability_counts"]:
            rollup["validation_availability_counts"][validation_availability] = 0
        rollup["validation_availability_counts"][validation_availability] += 1
        if validation_availability == "known" and isinstance(summary.get("validation_seconds"), int):
            rollup["issues_with_known_validation_seconds"] += 1
            rollup["total_validation_seconds_known_sum"] += int(summary["validation_seconds"])
        elif validation_availability == "unknown":
            rollup["issues_with_unknown_validation_seconds"] += 1

        pr_wait_availability = summary.get("pr_wait_availability") or "unknown"
        if pr_wait_availability not in rollup["pr_wait_availability_counts"]:
            rollup["pr_wait_availability_counts"][pr_wait_availability] = 0
        rollup["pr_wait_availability_counts"][pr_wait_availability] += 1
        if pr_wait_availability == "known" and isinstance(summary.get("pr_wait_seconds"), int):
            rollup["issues_with_known_pr_wait"] += 1
            rollup["total_pr_wait_seconds_known_sum"] += int(summary["pr_wait_seconds"])
        elif pr_wait_availability == "unknown":
            rollup["issues_with_unknown_pr_wait"] += 1

        ci_wait_availability = summary.get("ci_wait_availability") or "unknown"
        if ci_wait_availability not in rollup["ci_wait_availability_counts"]:
            rollup["ci_wait_availability_counts"][ci_wait_availability] = 0
        rollup["ci_wait_availability_counts"][ci_wait_availability] += 1
        if ci_wait_availability == "known" and isinstance(summary.get("ci_wait_seconds"), int):
            rollup["issues_with_known_ci_wait"] += 1
            rollup["total_ci_wait_seconds_known_sum"] += int(summary["ci_wait_seconds"])
        elif ci_wait_availability == "unknown":
            rollup["issues_with_unknown_ci_wait"] += 1

        token_usage = summary.get("token_usage") or {}
        total_token_availability = token_usage.get("total_availability") or "unknown"
        if total_token_availability not in rollup["total_token_availability_counts"]:
            rollup["total_token_availability_counts"][total_token_availability] = 0
        rollup["total_token_availability_counts"][total_token_availability] += 1
        if total_token_availability == "known" and isinstance(token_usage.get("total_tokens"), int):
            rollup["issues_with_known_total_tokens"] += 1
            rollup["total_tokens_known_sum"] += int(token_usage["total_tokens"])
        elif total_token_availability == "unknown":
            rollup["issues_with_unknown_total_tokens"] += 1
    return rollup
