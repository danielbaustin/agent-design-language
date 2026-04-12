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
    subagent_assigned = bool(workflow.get("subagent_assigned", False))
    subagent_requirement = policy.get("subagent_requirement", "optional")

    selected_phase = "blocked"
    skill_name = "none"
    editor_skill = "none"
    detected_phase = lifecycle_state

    if not bootstrap_present:
        detected_phase = "bootstrap_missing"
        selected_phase = "init"
        skill_name = "pr-init"
    elif card_blocker in ("stp", "sip", "sor"):
        detected_phase = "card_local_blocker"
        selected_phase = "editor"
        skill_name = editor_for(card_blocker)
        editor_skill = skill_name
    elif pr_state == "open":
        detected_phase = "pr_in_flight"
        selected_phase = "janitor"
        skill_name = "pr-janitor"
    elif pr_state in ("merged", "intentionally_closed", "closed_no_pr", "superseded", "duplicate"):
        detected_phase = "closed_out"
        selected_phase = "closeout"
        skill_name = "pr-closeout"
    elif lifecycle_state == "execution_done":
        selected_phase = "finish"
        skill_name = "pr-finish"
    elif lifecycle_state == "run_bound":
        selected_phase = "run"
        skill_name = "pr-run"
    elif lifecycle_state == "pre_run":
        if ready_state == "pass":
            selected_phase = "run"
            skill_name = "pr-run"
        else:
            selected_phase = "ready"
            skill_name = "pr-ready"

    bypasses = []
    policy_result = "PASS"
    if subagent_requirement == "required" and not subagent_assigned:
        policy_result = "FAIL"
        bypasses.append({"component": "subagent_requirement", "reason": "required_but_not_assigned"})
    elif subagent_requirement == "recommended" and not subagent_assigned:
        policy_result = "PARTIAL"
        bypasses.append({"component": "subagent_requirement", "reason": "recommended_but_not_assigned"})
    elif subagent_requirement == "forbidden" and subagent_assigned:
        policy_result = "FAIL"
        bypasses.append({"component": "subagent_requirement", "reason": "forbidden_but_assigned"})

    next_phase = skill_name if skill_name != "none" else "human_review"

    return {
        "status": "done" if skill_name != "none" else "blocked",
        "target": {
            "issue_number": target.get("issue_number"),
            "task_bundle_path": target.get("task_bundle_path"),
            "branch": target.get("branch"),
            "worktree_path": target.get("worktree_path"),
            "pr_number": target.get("pr_number"),
        },
        "workflow_state": {
            "detected_phase": detected_phase,
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
        },
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    args = parser.parse_args()
    print(json.dumps(evaluate(load_payload(args.input)), indent=2))


if __name__ == "__main__":
    main()
