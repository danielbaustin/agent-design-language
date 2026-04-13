#!/usr/bin/env python3
import argparse
import json
from pathlib import Path


def load_payload(path: str):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def editor_for(blocker: str) -> str:
    return {
        "stp": "stp-editor",
        "sip": "sip-editor",
        "sor": "sor-editor",
    }.get(blocker, "none")


def evaluate(payload):
    target = payload.get("target", {})
    workflow = payload.get("workflow_state", {})
    policy = payload.get("policy", {})

    bootstrap_present = workflow.get("bootstrap_present", True)
    card_blocker = workflow.get("card_blocker", "none")
    lifecycle_state = workflow.get("lifecycle_state", "unknown")
    ready_state = workflow.get("ready_state", "unknown")
    pr_state = workflow.get("pr_state", "none")
    blocker_class = workflow.get("blocker_class", "none")
    subagent_assigned = bool(workflow.get("subagent_assigned", False))
    subagent_requirement = policy.get("subagent_requirement", "optional")

    selected_phase = "blocked"
    skill_name = "none"
    editor_skill = "none"
    detected_phase = lifecycle_state
    continuation = "ask_operator"
    escalation_reason = "manual_review_required"

    if not bootstrap_present:
        detected_phase = "bootstrap_missing"
        selected_phase = "init"
        skill_name = "pr-init"
        continuation = "continue"
        escalation_reason = "none"
    elif card_blocker in ("stp", "sip", "sor"):
        detected_phase = "card_local_blocker"
        selected_phase = "editor"
        skill_name = editor_for(card_blocker)
        editor_skill = skill_name
        continuation = "continue"
        escalation_reason = "none"
    elif pr_state in (
        "open_with_blockers",
        "open_failing",
        "open_conflicted",
        "review_changes_requested",
        "open_linkage_only",
        "open_checks_failed",
        "open_merge_conflict",
    ):
        detected_phase = "pr_in_flight"
        selected_phase = "janitor"
        skill_name = "pr-janitor"
        continuation = "continue"
        escalation_reason = "none"
    elif pr_state in ("open_clean", "open_unknown", "open_draft", "open_waiting_for_review"):
        detected_phase = "pr_in_flight"
        selected_phase = "blocked"
        skill_name = "none"
        continuation = "ask_operator"
        escalation_reason = "healthy_pr_waiting"
    elif pr_state in ("merged", "intentionally_closed", "closed_no_pr", "superseded", "duplicate"):
        detected_phase = "closed_out"
        selected_phase = "closeout"
        skill_name = "pr-closeout"
        continuation = "continue"
        escalation_reason = "none"
    elif blocker_class == "satisfied_by_child_issue_wave":
        detected_phase = "already_satisfied"
        selected_phase = "blocked"
        skill_name = "none"
        continuation = "ask_operator"
        escalation_reason = "child_issue_wave_satisfied"
    elif blocker_class == "satisfied_by_related_issue_refs":
        detected_phase = "already_satisfied"
        selected_phase = "blocked"
        skill_name = "none"
        continuation = "ask_operator"
        escalation_reason = "related_issue_ref_satisfied"
    elif blocker_class == "active_child_issue_wave":
        detected_phase = "tracker_in_flight"
        selected_phase = "blocked"
        skill_name = "none"
        continuation = "ask_operator"
        escalation_reason = "child_issue_wave_active"
    elif blocker_class == "related_issue_ref_active":
        detected_phase = "tracker_in_flight"
        selected_phase = "blocked"
        skill_name = "none"
        continuation = "ask_operator"
        escalation_reason = "related_issue_ref_active"
    elif lifecycle_state == "execution_done":
        selected_phase = "finish"
        skill_name = "pr-finish"
        continuation = "continue"
        escalation_reason = "none"
    elif lifecycle_state == "run_bound":
        selected_phase = "run"
        skill_name = "pr-run"
        continuation = "continue"
        escalation_reason = "none"
    elif lifecycle_state == "pre_run":
        if ready_state == "pass":
            selected_phase = "run"
            skill_name = "pr-run"
            continuation = "continue"
            escalation_reason = "none"
        else:
            selected_phase = "ready"
            skill_name = "pr-ready"
            continuation = "continue"
            escalation_reason = "none"

    if blocker_class in ("open_pr_wave_only", "doctor_failed_or_inconclusive", "tracked_adl_residue"):
        continuation = "ask_operator"
        if blocker_class == "open_pr_wave_only" and skill_name in ("pr-run", "pr-finish"):
            escalation_reason = "operator_override_required"
        elif blocker_class == "doctor_failed_or_inconclusive":
            escalation_reason = "ambiguous_live_state"
        elif blocker_class == "tracked_adl_residue":
            escalation_reason = "repo_policy_residue"

    bypasses = []
    policy_result = "PASS"
    blocked_by_policy = False
    if subagent_requirement == "required" and not subagent_assigned:
        policy_result = "FAIL"
        bypasses.append({"component": "subagent_requirement", "reason": "required_but_not_assigned"})
        blocked_by_policy = True
    elif subagent_requirement == "recommended" and not subagent_assigned:
        policy_result = "PARTIAL"
        bypasses.append({"component": "subagent_requirement", "reason": "recommended_but_not_assigned"})
    elif subagent_requirement == "forbidden" and subagent_assigned:
        policy_result = "FAIL"
        bypasses.append({"component": "subagent_requirement", "reason": "forbidden_but_assigned"})
        blocked_by_policy = True

    required_skill_by_phase = policy.get("required_skill_by_phase", {})
    expected_skill = required_skill_by_phase.get(selected_phase)
    if expected_skill and skill_name not in ("none", expected_skill):
        policy_result = "FAIL"
        blocked_by_policy = True
        bypasses.append({"component": "required_skill_by_phase", "reason": "selected_skill_mismatch"})

    required_card_skill_by_type = policy.get("required_card_skill_by_type", {})
    if card_blocker in required_card_skill_by_type and editor_skill != required_card_skill_by_type.get(card_blocker):
        policy_result = "FAIL"
        blocked_by_policy = True
        bypasses.append({"component": "required_card_skill_by_type", "reason": "selected_editor_mismatch"})

    if card_blocker in ("stp", "sip", "sor") and policy.get("card_editor_skills_required", True) and editor_skill == "none":
        policy_result = "FAIL"
        blocked_by_policy = True
        bypasses.append({"component": "card_editor_skills_required", "reason": "card_blocker_not_routed_to_editor"})

    if skill_name == "none" and policy.get("skills_required", True) and selected_phase != "blocked":
        policy_result = "FAIL"
        blocked_by_policy = True
        bypasses.append({"component": "skills_required", "reason": "no_skill_selected"})

    next_phase = skill_name if skill_name != "none" else "human_review"
    status = "done" if skill_name != "none" else "blocked"
    if blocked_by_policy and not policy.get("bypass_without_explicit_blocker", False):
        status = "blocked"
        next_phase = "blocked"
        continuation = "stop"
        escalation_reason = "policy_block"

    return {
        "status": status,
        "target": {
            "issue_number": target.get("issue_number"),
            "task_bundle_path": target.get("task_bundle_path"),
            "branch": target.get("branch"),
            "worktree_path": target.get("worktree_path"),
            "pr_number": target.get("pr_number"),
        },
        "workflow_state": {
            "detected_phase": detected_phase,
            "blocker_class": blocker_class,
            "evidence_used": workflow.get("evidence_used", []),
        },
        "selected_skill": {
            "phase": selected_phase,
            "skill_name": skill_name,
            "editor_skill": editor_skill,
        },
        "workflow_compliance": {
            "skills_required": bool(policy.get("skills_required", True)),
            "card_editor_skills_required": bool(policy.get("card_editor_skills_required", True)),
            "subagent_requirement": subagent_requirement,
            "subagent_assigned": subagent_assigned if subagent_requirement != "forbidden" else subagent_assigned,
            "bypasses": bypasses,
            "policy_result": policy_result,
        },
        "actions_taken": [f"selected {skill_name} from detected phase {detected_phase}"],
        "handoff_state": {
            "next_phase": next_phase,
            "continuation": continuation,
            "escalation_reason": escalation_reason,
        },
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    args = parser.parse_args()
    print(json.dumps(evaluate(load_payload(args.input)), indent=2))


if __name__ == "__main__":
    main()
