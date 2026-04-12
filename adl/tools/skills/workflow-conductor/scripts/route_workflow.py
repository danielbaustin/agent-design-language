#!/usr/bin/env python3
import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

from select_next_skill import evaluate


PRIMARY_TARGET_FIELDS = {
    "route_issue": "issue_number",
    "route_task_bundle": "task_bundle_path",
    "route_branch": "branch",
    "route_worktree": "worktree_path",
    "route_pr": "pr_number",
}


def load_payload(path: str):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def fail(message: str):
    print(message, file=sys.stderr)
    raise SystemExit(1)


def run_command(args, cwd: Path):
    return subprocess.run(
        args,
        cwd=str(cwd),
        capture_output=True,
        text=True,
        check=False,
    )


def parse_task_bundle_identity(bundle: Path):
    match = re.match(r"issue-(\d+)__(.+)", bundle.name)
    if not match:
        return None
    issue_number = int(match.group(1))
    slug = match.group(2)
    version = None
    if bundle.parent.name == "tasks":
        version = bundle.parent.parent.name
    return {
        "issue_number": issue_number,
        "slug": slug,
        "version": version,
        "task_bundle_path": str(bundle),
    }


def parse_branch_identity(branch: str):
    match = re.match(r"^codex/(\d+)-([a-z0-9][a-z0-9-]*)$", branch)
    if not match:
        return None
    return {
        "issue_number": int(match.group(1)),
        "slug": match.group(2),
        "branch": branch,
    }


def parse_worktree_list(text: str):
    entries = []
    current = {}
    for raw in text.splitlines():
        line = raw.strip()
        if not line:
            if current:
                entries.append(current)
                current = {}
            continue
        if line.startswith("worktree "):
            current["worktree"] = line.split(" ", 1)[1]
        elif line.startswith("branch "):
            ref = line.split(" ", 1)[1]
            current["branch"] = ref.removeprefix("refs/heads/")
    if current:
        entries.append(current)
    return entries


def find_branch_worktree(repo_root: Path, branch: str):
    result = run_command(["git", "worktree", "list", "--porcelain"], repo_root)
    if result.returncode != 0:
        return None
    for entry in parse_worktree_list(result.stdout):
        if entry.get("branch") == branch:
            return entry.get("worktree")
    return None


def find_unique_issue_glob(repo_root: Path, issue_number: int, pattern: str):
    matches = []
    for path in repo_root.glob(pattern):
        matches.append(path)
    if not matches:
        return None
    matches = sorted(matches)
    if len(matches) > 1:
        fail(
            f"workflow-conductor: multiple canonical surfaces found for issue {issue_number}: "
            + ", ".join(str(path) for path in matches)
        )
    return matches[0]


def find_bundle_for_issue(repo_root: Path, issue_number: int):
    return find_unique_issue_glob(
        repo_root,
        issue_number,
        f".adl/*/tasks/issue-{issue_number}__*",
    )


def find_source_prompt_for_issue(repo_root: Path, issue_number: int):
    return find_unique_issue_glob(
        repo_root,
        issue_number,
        f".adl/*/bodies/issue-{issue_number}-*.md",
    )


def parse_doctor_output(raw: str):
    lines = raw.splitlines()
    for index, line in enumerate(lines):
        if line.lstrip().startswith("{"):
            payload = "\n".join(lines[index:])
            return json.loads(payload)
    raise ValueError("doctor output did not contain JSON")


def doctor_snapshot(repo_root: Path, issue_number: int, slug: str, version: str):
    command = [
        "bash",
        "adl/tools/pr.sh",
        "doctor",
        str(issue_number),
        "--slug",
        slug,
        "--version",
        version,
        "--mode",
        "full",
        "--json",
    ]
    result = run_command(command, repo_root)
    if result.returncode != 0:
        return None
    try:
        return parse_doctor_output(result.stdout)
    except ValueError:
        return None


def classify_doctor_state(doctor):
    if not doctor:
        return "doctor_missing"
    doctor_status = str(doctor.get("doctor_status", "unknown")).upper()
    preflight_status = str(doctor.get("preflight_status", "unknown")).upper()
    open_pr_count = int(doctor.get("open_pr_count", 0) or 0)
    lifecycle_state = doctor.get("lifecycle_state", "unknown")

    if doctor_status == "BLOCK" and preflight_status == "BLOCK" and open_pr_count > 0:
        if lifecycle_state == "execution_done":
            return "open_pr_wave_only"
        if lifecycle_state in {"pre_run", "run_bound"}:
            return "open_pr_wave_only"
    if doctor_status not in {"PASS", "WARN", "BLOCK"}:
        return "doctor_failed_or_inconclusive"
    return "none"


def classify_pr_state(pr):
    state = pr.get("state")
    review = pr.get("reviewDecision")
    merge_state = pr.get("mergeStateStatus")
    checks = summarize_checks(pr.get("statusCheckRollup"))

    pr_state = "open_unknown"
    blocker_class = "none"
    if state == "MERGED":
        pr_state = "merged"
    elif state == "CLOSED":
        pr_state = "intentionally_closed"
    elif review == "CHANGES_REQUESTED":
        pr_state = "review_changes_requested"
        blocker_class = "review_changes_requested"
    elif merge_state == "DIRTY":
        pr_state = "open_merge_conflict"
        blocker_class = "merge_conflict"
    elif checks == "blocked":
        pr_state = "open_checks_failed"
        blocker_class = "checks_failed"
    elif merge_state == "BLOCKED":
        pr_state = "open_with_blockers"
        blocker_class = "merge_blocked"
    elif pr.get("isDraft") or checks == "pending":
        pr_state = "open_draft"
    else:
        pr_state = "open_clean"

    return pr_state, blocker_class


def infer_card_blocker(bundle: Path):
    expected = {
        "stp": bundle / "stp.md",
        "sip": bundle / "sip.md",
        "sor": bundle / "sor.md",
    }
    for blocker, path in expected.items():
        if not path.exists():
            return blocker
    return "none"


def read_output_status(bundle: Path):
    output_card = bundle / "sor.md"
    if not output_card.exists():
        return None
    for line in output_card.read_text(encoding="utf-8").splitlines():
        if line.startswith("Status:"):
            return line.split(":", 1)[1].strip()
    return None


def gather_issue_surface(repo_root: Path, issue_number: int):
    bundle = find_bundle_for_issue(repo_root, issue_number)
    source_prompt = find_source_prompt_for_issue(repo_root, issue_number)
    identity = parse_task_bundle_identity(bundle) if bundle else None
    return bundle, source_prompt, identity


def collect_route_issue(repo_root: Path, payload):
    target = payload.get("target", {})
    issue_number = int(target["issue_number"])
    observed = payload.get("observed_state", {})
    bundle, source_prompt, identity = gather_issue_surface(repo_root, issue_number)

    bootstrap_present = bool(bundle and source_prompt)
    card_blocker = infer_card_blocker(bundle) if bundle else "none"
    workflow = {
        "bootstrap_present": bootstrap_present,
        "card_blocker": card_blocker,
        "lifecycle_state": "unknown",
        "ready_state": "unknown",
        "pr_state": "none",
        "blocker_class": "none",
        "subagent_assigned": bool(observed.get("subagent_assigned", False)),
        "evidence_used": [],
    }

    resolved_target = dict(target)
    if identity:
        resolved_target.setdefault("slug", identity["slug"])
        resolved_target.setdefault("version", identity["version"])
        resolved_target.setdefault("task_bundle_path", str(bundle))
    if source_prompt:
        resolved_target.setdefault("source_prompt_path", str(source_prompt))

    if not bootstrap_present:
        workflow["evidence_used"].append("missing_root_bundle")
        return {"target": resolved_target, "workflow_state": workflow, "policy": payload.get("policy", {})}

    doctor = doctor_snapshot(
        repo_root,
        issue_number,
        resolved_target.get("slug"),
        resolved_target.get("version"),
    )
    if doctor:
        workflow["lifecycle_state"] = doctor.get("lifecycle_state", "unknown")
        workflow["ready_state"] = doctor.get("ready_status", "unknown").lower()
        workflow["blocker_class"] = classify_doctor_state(doctor)
        workflow["evidence_used"].append("doctor_json")
        if doctor.get("worktree"):
            resolved_target.setdefault("worktree_path", doctor["worktree"])
        if doctor.get("branch"):
            resolved_target.setdefault("branch", doctor["branch"])
    else:
        workflow["lifecycle_state"] = "pre_run"
        workflow["evidence_used"].append("bundle_paths")
    return {"target": resolved_target, "workflow_state": workflow, "policy": payload.get("policy", {})}


def collect_route_task_bundle(repo_root: Path, payload):
    target = payload.get("target", {})
    bundle = Path(target["task_bundle_path"])
    if not bundle.is_absolute():
        bundle = repo_root / bundle
    identity = parse_task_bundle_identity(bundle)
    if identity is None:
        fail("workflow-conductor: task bundle path must match issue-<n>__<slug>")
    source_prompt = find_source_prompt_for_issue(repo_root, identity["issue_number"])
    observed = payload.get("observed_state", {})
    workflow = {
        "bootstrap_present": bool(bundle.exists() and source_prompt),
        "card_blocker": infer_card_blocker(bundle) if bundle.exists() else "stp",
        "lifecycle_state": "pre_run",
        "ready_state": "unknown",
        "pr_state": "none",
        "blocker_class": "none",
        "subagent_assigned": bool(observed.get("subagent_assigned", False)),
        "evidence_used": ["task_bundle_path"],
    }
    if ".worktrees" in bundle.parts:
        workflow["lifecycle_state"] = "run_bound"
        workflow["evidence_used"].append("worktree_bundle_path")
        if read_output_status(bundle) == "DONE":
            workflow["lifecycle_state"] = "execution_done"
            workflow["evidence_used"].append("worktree_output_card")
    resolved_target = dict(target)
    resolved_target.setdefault("issue_number", identity["issue_number"])
    resolved_target.setdefault("slug", identity["slug"])
    resolved_target.setdefault("version", identity["version"])
    resolved_target["task_bundle_path"] = str(bundle)
    if source_prompt:
        resolved_target.setdefault("source_prompt_path", str(source_prompt))
    return {"target": resolved_target, "workflow_state": workflow, "policy": payload.get("policy", {})}


def collect_route_branch(repo_root: Path, payload):
    target = payload.get("target", {})
    branch = target["branch"]
    identity = parse_branch_identity(branch)
    if identity is None:
        fail("workflow-conductor: branch must match codex/<issue>-<slug>")
    worktree = find_branch_worktree(repo_root, branch)
    resolved_target = dict(target)
    resolved_target.setdefault("issue_number", identity["issue_number"])
    resolved_target.setdefault("slug", identity["slug"])
    resolved_target["branch"] = branch
    if worktree:
        resolved_target.setdefault("worktree_path", worktree)
        return collect_route_worktree(repo_root, {"target": resolved_target, "policy": payload.get("policy", {}), "observed_state": payload.get("observed_state", {})})
    bundle, source_prompt, bundle_identity = gather_issue_surface(repo_root, identity["issue_number"])
    workflow = {
        "bootstrap_present": bool(bundle and source_prompt),
        "card_blocker": infer_card_blocker(bundle) if bundle else "none",
        "lifecycle_state": "pre_run",
        "ready_state": "unknown",
        "pr_state": "none",
        "blocker_class": "none",
        "subagent_assigned": bool(payload.get("observed_state", {}).get("subagent_assigned", False)),
        "evidence_used": ["branch", "canonical_bundle"],
    }
    if bundle_identity:
        resolved_target.setdefault("version", bundle_identity["version"])
        resolved_target.setdefault("task_bundle_path", str(bundle))
    if source_prompt:
        resolved_target.setdefault("source_prompt_path", str(source_prompt))
    doctor = None
    if bundle_identity:
        doctor = doctor_snapshot(repo_root, bundle_identity["issue_number"], bundle_identity["slug"], bundle_identity["version"])
    if doctor:
        workflow["lifecycle_state"] = doctor.get("lifecycle_state", "unknown")
        workflow["ready_state"] = doctor.get("ready_status", "unknown").lower()
        workflow["blocker_class"] = classify_doctor_state(doctor)
        workflow["evidence_used"].append("doctor_json")
    return {"target": resolved_target, "workflow_state": workflow, "policy": payload.get("policy", {})}


def find_worktree_bundle(worktree_root: Path, issue_number=None):
    matches = sorted(worktree_root.glob(".adl/*/tasks/issue-*__*"))
    if not matches:
        return None
    if issue_number is not None:
        filtered = []
        for path in matches:
            identity = parse_task_bundle_identity(path)
            if identity and identity["issue_number"] == int(issue_number):
                filtered.append(path)
        if len(filtered) == 1:
            return filtered[0]
        if len(filtered) > 1:
            fail(
                f"workflow-conductor: multiple task bundles found in worktree for issue {issue_number}: "
                + ", ".join(str(path) for path in filtered)
            )
    if len(matches) > 1:
        fail(
            "workflow-conductor: multiple task bundles found in worktree: "
            + ", ".join(str(path) for path in matches)
        )
    return matches[0]


def collect_route_worktree(repo_root: Path, payload):
    target = payload.get("target", {})
    worktree = Path(target["worktree_path"])
    if not worktree.is_absolute():
        worktree = repo_root / worktree
    bundle = find_worktree_bundle(worktree, target.get("issue_number"))
    if bundle is None:
        fail("workflow-conductor: could not find a task bundle in the worktree")
    identity = parse_task_bundle_identity(bundle)
    branch_result = run_command(["git", "rev-parse", "--abbrev-ref", "HEAD"], worktree)
    branch = branch_result.stdout.strip() if branch_result.returncode == 0 else target.get("branch")
    source_prompt = find_source_prompt_for_issue(repo_root, identity["issue_number"])
    resolved_target = dict(target)
    resolved_target.setdefault("issue_number", identity["issue_number"])
    resolved_target.setdefault("slug", identity["slug"])
    resolved_target.setdefault("version", identity["version"])
    resolved_target["task_bundle_path"] = str(bundle)
    resolved_target["worktree_path"] = str(worktree)
    if branch:
        resolved_target["branch"] = branch
    if source_prompt:
        resolved_target.setdefault("source_prompt_path", str(source_prompt))
    workflow = {
        "bootstrap_present": bool(source_prompt),
        "card_blocker": infer_card_blocker(bundle),
        "lifecycle_state": "run_bound",
        "ready_state": "pass",
        "pr_state": "none",
        "blocker_class": "none",
        "subagent_assigned": bool(payload.get("observed_state", {}).get("subagent_assigned", False)),
        "evidence_used": ["worktree_path", "task_bundle_path"],
    }
    if read_output_status(bundle) == "DONE":
        workflow["lifecycle_state"] = "execution_done"
        workflow["evidence_used"].append("worktree_output_card")
    doctor = doctor_snapshot(repo_root, identity["issue_number"], identity["slug"], identity["version"])
    if doctor:
        doctor_lifecycle = doctor.get("lifecycle_state", workflow["lifecycle_state"])
        if workflow["lifecycle_state"] != "execution_done" or doctor_lifecycle == "execution_done":
            workflow["lifecycle_state"] = doctor_lifecycle
        workflow["ready_state"] = doctor.get("ready_status", "unknown").lower()
        workflow["blocker_class"] = classify_doctor_state(doctor)
        workflow["evidence_used"].append("doctor_json")
    return {"target": resolved_target, "workflow_state": workflow, "policy": payload.get("policy", {})}


def summarize_checks(status_rollup):
    if not isinstance(status_rollup, list):
        return "unknown"
    conclusions = {item.get("conclusion") for item in status_rollup if item.get("conclusion")}
    states = {item.get("status") for item in status_rollup if item.get("status")}
    if {"FAILURE", "TIMED_OUT", "CANCELLED", "ACTION_REQUIRED"} & conclusions:
        return "blocked"
    if "PENDING" in states or "QUEUED" in states or not conclusions:
        return "pending"
    return "pass"


def collect_route_pr(repo_root: Path, payload):
    target = payload.get("target", {})
    pr_number = int(target["pr_number"])
    result = run_command(
        [
            "gh",
            "pr",
            "view",
            str(pr_number),
            "--json",
            "state,isDraft,reviewDecision,mergeStateStatus,headRefName,statusCheckRollup",
        ],
        repo_root,
    )
    if result.returncode != 0:
        fail(f"workflow-conductor: failed to inspect PR #{pr_number}")
    pr = json.loads(result.stdout)
    resolved_target = dict(target)
    resolved_target.setdefault("branch", pr.get("headRefName"))
    identity = parse_branch_identity(pr.get("headRefName", ""))
    if identity:
        resolved_target.setdefault("issue_number", identity["issue_number"])
        resolved_target.setdefault("slug", identity["slug"])
        bundle, source_prompt, bundle_identity = gather_issue_surface(repo_root, identity["issue_number"])
        if bundle:
            resolved_target.setdefault("task_bundle_path", str(bundle))
        if source_prompt:
            resolved_target.setdefault("source_prompt_path", str(source_prompt))
        if bundle_identity:
            resolved_target.setdefault("version", bundle_identity["version"])
    workflow = {
        "bootstrap_present": True,
        "card_blocker": "none",
        "lifecycle_state": "execution_done",
        "ready_state": "pass",
        "pr_state": "open_unknown",
        "blocker_class": "none",
        "subagent_assigned": bool(payload.get("observed_state", {}).get("subagent_assigned", False)),
        "evidence_used": ["gh_pr"],
    }
    workflow["pr_state"], workflow["blocker_class"] = classify_pr_state(pr)
    return {"target": resolved_target, "workflow_state": workflow, "policy": payload.get("policy", {})}


def collect_state(payload):
    repo_root = Path(payload["repo_root"]).resolve()
    mode = payload["mode"]
    primary = PRIMARY_TARGET_FIELDS.get(mode)
    if primary is None:
        fail(f"workflow-conductor: unsupported mode {mode}")
    target = payload.get("target", {})
    specified = [field for field in PRIMARY_TARGET_FIELDS.values() if target.get(field) not in (None, "")]
    if primary not in specified:
        fail(f"workflow-conductor: mode {mode} requires target.{primary}")
    allowed_secondary_targets = {
        "route_worktree": {"issue_number"},
    }
    allowed = allowed_secondary_targets.get(mode, set())
    if len(specified) > 1:
        extras = {field for field in specified if field != primary}
        if not extras.issubset(allowed):
            fail("workflow-conductor: exactly one primary target should drive the mode")

    if mode == "route_issue":
        return collect_route_issue(repo_root, payload)
    if mode == "route_task_bundle":
        return collect_route_task_bundle(repo_root, payload)
    if mode == "route_branch":
        return collect_route_branch(repo_root, payload)
    if mode == "route_worktree":
        return collect_route_worktree(repo_root, payload)
    return collect_route_pr(repo_root, payload)


def render_markdown(result):
    target = result["target"]
    workflow = result["workflow_state"]
    selected = result["selected_skill"]
    compliance = result["workflow_compliance"]
    handoff = result["handoff_state"]
    lines = [
        "# Workflow Conductor Review",
        "",
        "## status",
        f"- status: {result['status']}",
        "",
        "## target",
        f"- issue_number: {target.get('issue_number')}",
        f"- task_bundle_path: {target.get('task_bundle_path')}",
        f"- branch: {target.get('branch')}",
        f"- worktree_path: {target.get('worktree_path')}",
        f"- pr_number: {target.get('pr_number')}",
        "",
        "## workflow_state",
        f"- detected_phase: {workflow.get('detected_phase')}",
        f"- blocker_class: {workflow.get('blocker_class')}",
        f"- evidence_used: {', '.join(workflow.get('evidence_used', [])) or 'none'}",
        "",
        "## selected_skill",
        f"- phase: {selected.get('phase')}",
        f"- skill_name: {selected.get('skill_name')}",
        f"- editor_skill: {selected.get('editor_skill')}",
        "",
        "## workflow_compliance",
        f"- policy_result: {compliance.get('policy_result')}",
        f"- subagent_requirement: {compliance.get('subagent_requirement')}",
        f"- subagent_assigned: {compliance.get('subagent_assigned')}",
        f"- bypasses: {json.dumps(compliance.get('bypasses', []))}",
        "",
        "## handoff_state",
        f"- next_phase: {handoff.get('next_phase')}",
        f"- continuation: {handoff.get('continuation')}",
        f"- escalation_reason: {handoff.get('escalation_reason')}",
        "",
    ]
    return "\n".join(lines)


def default_artifact_path(repo_root: Path):
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    return repo_root / ".adl" / "reviews" / f"{stamp}-workflow-conductor.md"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    parser.add_argument("--artifact-path")
    parser.add_argument("--no-artifact", action="store_true")
    args = parser.parse_args()

    payload = load_payload(args.input)
    snapshot = collect_state(payload)
    result = evaluate(snapshot)
    result["artifact"] = {"path": None}

    if not args.no_artifact:
        repo_root = Path(payload["repo_root"]).resolve()
        artifact_path = Path(args.artifact_path) if args.artifact_path else default_artifact_path(repo_root)
        if not artifact_path.is_absolute():
            artifact_path = repo_root / artifact_path
        artifact_path.parent.mkdir(parents=True, exist_ok=True)
        artifact_path.write_text(render_markdown(result), encoding="utf-8")
        result["artifact"]["path"] = str(artifact_path)
        result.setdefault("actions_taken", []).append(f"wrote routing artifact to {artifact_path}")

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
