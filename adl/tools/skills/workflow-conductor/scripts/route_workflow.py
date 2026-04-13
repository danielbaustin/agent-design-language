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

SKILL_FILES = {
    "pr-init": "adl/tools/skills/pr-init/SKILL.md",
    "pr-ready": "adl/tools/skills/pr-ready/SKILL.md",
    "pr-run": "adl/tools/skills/pr-run/SKILL.md",
    "pr-finish": "adl/tools/skills/pr-finish/SKILL.md",
    "pr-janitor": "adl/tools/skills/pr-janitor/SKILL.md",
    "pr-closeout": "adl/tools/skills/pr-closeout/SKILL.md",
    "stp-editor": "adl/tools/skills/stp-editor/SKILL.md",
    "sip-editor": "adl/tools/skills/sip-editor/SKILL.md",
    "sor-editor": "adl/tools/skills/sor-editor/SKILL.md",
}

BUILTIN_DISPATCH_COMMANDS = {
    "pr-init": [
        "bash",
        "adl/tools/pr.sh",
        "init",
        "{issue_number}",
        "--slug",
        "{slug}",
        "--version",
        "{version}",
    ],
    "pr-ready": [
        "bash",
        "adl/tools/pr.sh",
        "doctor",
        "{issue_number}",
        "--slug",
        "{slug}",
        "--version",
        "{version}",
        "--mode",
        "full",
        "--json",
    ],
    "pr-run": [
        "bash",
        "adl/tools/pr.sh",
        "run",
        "{issue_number}",
        "--slug",
        "{slug}",
        "--version",
        "{version}",
    ],
    "pr-closeout": [
        "bash",
        "adl/tools/pr.sh",
        "closeout",
        "{issue_number}",
        "--version",
        "{version}",
        "--no-fetch-issue",
    ],
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


def run_command_with_timeout(args, cwd: Path, timeout_secs=None):
    try:
        return subprocess.run(
            args,
            cwd=str(cwd),
            capture_output=True,
            text=True,
            check=False,
            timeout=timeout_secs,
        )
    except subprocess.TimeoutExpired as exc:
        return {
            "timed_out": True,
            "stdout": exc.stdout or "",
            "stderr": exc.stderr or "",
        }


def read_text(path: Path):
    if not path or not path.exists():
        return ""
    return path.read_text(encoding="utf-8")


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


def frontmatter_value(text: str, key: str):
    if not text.startswith("---\n"):
        return None
    lines = text.splitlines()
    for line in lines[1:]:
        if line == "---":
            break
        if line.startswith(f"{key}:"):
            return line.split(":", 1)[1].strip().strip('"').strip("'")
    return None


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


def gh_child_issue_wave_state(repo_root: Path, parent_issue_number: int):
    result = run_command(
        ["gh", "issue", "list", "--state", "all", "--limit", "200", "--json", "number,state,body,title"],
        repo_root,
    )
    if result.returncode != 0 or not result.stdout.strip():
        return None
    try:
        issues = json.loads(result.stdout)
    except json.JSONDecodeError:
        return None
    children = []
    needle = f"child of #{parent_issue_number}"
    for issue in issues:
        body = issue.get("body") or ""
        if needle in body.lower():
            children.append(issue)
    if not children:
        return None
    if all(issue.get("state") == "CLOSED" for issue in children):
        return "satisfied_by_child_issue_wave"
    return "active_child_issue_wave"


def gh_issue_index(repo_root: Path):
    result = run_command(
        ["gh", "issue", "list", "--state", "all", "--limit", "200", "--json", "number,state,body,title"],
        repo_root,
    )
    if result.returncode != 0 or not result.stdout.strip():
        return {}
    try:
        issues = json.loads(result.stdout)
    except json.JSONDecodeError:
        return {}
    return {int(issue.get("number")): issue for issue in issues if issue.get("number") is not None}


def related_issue_reference_state(texts, issue_index):
    refs = set()
    patterns = [
        r"(?:covered by|satisfied by|folded into|superseded by)\s+#(\d+)",
        r"(?:duplicate of|resolved by)\s+#(\d+)",
    ]
    for text in texts:
        lower = text.lower()
        for pattern in patterns:
            refs.update(int(match) for match in re.findall(pattern, lower))
    if not refs:
        return "none"
    referenced = [issue_index.get(number) for number in sorted(refs)]
    referenced = [issue for issue in referenced if issue]
    if not referenced:
        return "none"
    if all(issue.get("state") == "CLOSED" for issue in referenced):
        return "satisfied_by_related_issue_refs"
    return "related_issue_ref_active"


def first_parent_issue_ref(texts):
    for text in texts:
        lower = text.lower()
        match = re.search(r"child of #(\d+)", lower)
        if match:
            return int(match.group(1))
    return None


def mentions_machine_readable_wp_dependency(texts):
    lowered = "\n".join(text.lower() for text in texts if text)
    needles = (
        "machine-readable wp dependency",
        "machine-readable wp dependency surface",
        "machine-readable dependency surface",
        "wp dependency surface",
    )
    return any(needle in lowered for needle in needles)


def sibling_issue_artifact_state(repo_root: Path, texts, issue_number: int, version: str, issue_index):
    if not version or not mentions_machine_readable_wp_dependency(texts):
        return "none"
    parent_issue = first_parent_issue_ref(texts)
    if parent_issue is None:
        return "none"

    artifact = repo_root / "docs" / "milestones" / version / f"WP_ISSUE_WAVE_{version}.yaml"
    if not artifact.exists():
        return "none"

    sibling_needle = f"child of #{parent_issue}"
    for sibling_issue in issue_index.values():
        sibling_number = int(sibling_issue.get("number", 0) or 0)
        if sibling_number == issue_number or sibling_issue.get("state") != "CLOSED":
            continue
        sibling_body = sibling_issue.get("body") or ""
        if sibling_needle not in sibling_body.lower():
            continue
        sibling_title = (sibling_issue.get("title") or "").lower()
        sibling_text = f"{sibling_title}\n{sibling_body.lower()}"
        if "issue wave" in sibling_text and ("generate" in sibling_text or "generator" in sibling_text):
            return "satisfied_by_sibling_issue_artifact"
    return "none"


def detect_tracked_adl_residue(repo_root: Path):
    guard = repo_root / "adl" / "tools" / "check_no_tracked_adl_issue_record_residue.sh"
    if not guard.exists():
        return False
    result = run_command(["bash", str(guard)], repo_root)
    return result.returncode != 0


def worktree_execution_done(repo_root: Path, issue_number: int, slug: str, version: str, worktree_hint):
    if not worktree_hint:
        return False
    worktree = Path(worktree_hint)
    if not worktree.is_absolute():
        worktree = repo_root / worktree
    bundle = worktree / ".adl" / version / "tasks" / f"issue-{issue_number}__{slug}"
    return read_output_status(bundle) == "DONE"


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
    elif merge_state == "BLOCKED" and checks == "pass":
        pr_state = "open_linkage_only"
        blocker_class = "open_linkage_only"
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


def issue_tracker_state(repo_root: Path, bundle: Path, source_prompt: Path, issue_number: int):
    texts = [read_text(source_prompt)]
    if bundle:
        texts.append(read_text(bundle / "stp.md"))
    wp_value = None
    title = None
    for text in texts:
        if not wp_value:
            wp_value = frontmatter_value(text, "wp")
        if not title:
            title = frontmatter_value(text, "title")
    is_tracker_like = bool(wp_value and wp_value != "unassigned") or bool(title and "[WP-" in title)
    if not is_tracker_like:
        return "none"
    gh_state = gh_child_issue_wave_state(repo_root, issue_number)
    return gh_state or "none"


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

    issue_index = gh_issue_index(repo_root)
    tracker_state = issue_tracker_state(repo_root, bundle, source_prompt, issue_number)
    if tracker_state == "satisfied_by_child_issue_wave":
        workflow["blocker_class"] = "satisfied_by_child_issue_wave"
        workflow["evidence_used"].append("child_issue_wave")
    elif tracker_state == "active_child_issue_wave":
        workflow["blocker_class"] = "active_child_issue_wave"
        workflow["evidence_used"].append("child_issue_wave")

    related_state = related_issue_reference_state(
        [read_text(source_prompt), read_text(bundle / "stp.md") if bundle else ""],
        issue_index,
    )
    if workflow["blocker_class"] == "none" and related_state == "satisfied_by_related_issue_refs":
        workflow["blocker_class"] = "satisfied_by_related_issue_refs"
        workflow["evidence_used"].append("related_issue_refs")
    elif workflow["blocker_class"] == "none" and related_state == "related_issue_ref_active":
        workflow["blocker_class"] = "related_issue_ref_active"
        workflow["evidence_used"].append("related_issue_refs")

    sibling_artifact_state = sibling_issue_artifact_state(
        repo_root,
        [read_text(source_prompt), read_text(bundle / "stp.md") if bundle else ""],
        issue_number,
        resolved_target.get("version"),
        issue_index,
    )
    if workflow["blocker_class"] == "none" and sibling_artifact_state == "satisfied_by_sibling_issue_artifact":
        workflow["blocker_class"] = "satisfied_by_sibling_issue_artifact"
        workflow["evidence_used"].append("sibling_issue_artifact")

    doctor = doctor_snapshot(
        repo_root,
        issue_number,
        resolved_target.get("slug"),
        resolved_target.get("version"),
    )
    if doctor:
        workflow["lifecycle_state"] = doctor.get("lifecycle_state", "unknown")
        workflow["ready_state"] = doctor.get("ready_status", "unknown").lower()
        doctor_blocker = classify_doctor_state(doctor)
        if workflow["blocker_class"] == "none":
            workflow["blocker_class"] = doctor_blocker
        workflow["evidence_used"].append("doctor_json")
        if doctor.get("worktree"):
            resolved_target.setdefault("worktree_path", doctor["worktree"])
            if worktree_execution_done(
                repo_root,
                issue_number,
                resolved_target.get("slug"),
                resolved_target.get("version"),
                doctor.get("worktree"),
            ):
                workflow["lifecycle_state"] = "execution_done"
                workflow["evidence_used"].append("worktree_output_card")
        if doctor.get("branch"):
            resolved_target.setdefault("branch", doctor["branch"])
    else:
        workflow["lifecycle_state"] = "pre_run"
        workflow["evidence_used"].append("bundle_paths")
    if workflow["blocker_class"] == "none" and detect_tracked_adl_residue(repo_root):
        workflow["blocker_class"] = "tracked_adl_residue"
        workflow["evidence_used"].append("tracked_adl_residue_guard")
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
    if workflow["blocker_class"] == "none" and detect_tracked_adl_residue(repo_root):
        workflow["blocker_class"] = "tracked_adl_residue"
        workflow["evidence_used"].append("tracked_adl_residue_guard")
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
    if workflow["blocker_class"] == "none" and detect_tracked_adl_residue(repo_root):
        workflow["blocker_class"] = "tracked_adl_residue"
        workflow["evidence_used"].append("tracked_adl_residue_guard")
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
    if workflow["blocker_class"] == "none" and detect_tracked_adl_residue(repo_root):
        workflow["blocker_class"] = "tracked_adl_residue"
        workflow["evidence_used"].append("tracked_adl_residue_guard")
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
    if workflow["blocker_class"] == "none" and detect_tracked_adl_residue(repo_root):
        workflow["blocker_class"] = "tracked_adl_residue"
        workflow["evidence_used"].append("tracked_adl_residue_guard")
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
    dispatch = result.get("dispatch", {})
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
        "## dispatch",
        f"- mode: {dispatch.get('mode')}",
        f"- selected_skill: {dispatch.get('selected_skill')}",
        f"- skill_file: {dispatch.get('skill_file')}",
        f"- command_source: {dispatch.get('command_source')}",
        f"- command: {json.dumps(dispatch.get('command'))}",
        f"- status: {dispatch.get('status')}",
        f"- result: {dispatch.get('result')}",
        f"- exit_code: {dispatch.get('exit_code')}",
        "",
    ]
    return "\n".join(lines)


def default_artifact_path(repo_root: Path):
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    return repo_root / ".adl" / "reviews" / f"{stamp}-workflow-conductor.md"


def dispatch_placeholders(repo_root: Path, result):
    target = result.get("target", {})
    artifact = result.get("artifact", {})
    values = {
        "repo_root": str(repo_root),
        "issue_number": "" if target.get("issue_number") is None else str(target.get("issue_number")),
        "task_bundle_path": target.get("task_bundle_path") or "",
        "branch": target.get("branch") or "",
        "worktree_path": target.get("worktree_path") or "",
        "pr_number": "" if target.get("pr_number") is None else str(target.get("pr_number")),
        "slug": target.get("slug") or "",
        "version": target.get("version") or "",
        "source_prompt_path": target.get("source_prompt_path") or "",
        "artifact_path": artifact.get("path") or "",
    }
    return values


def render_command_template(template, placeholders):
    rendered = []
    missing = []
    for token in template:
        try:
            value = token.format(**placeholders)
        except KeyError as exc:
            missing.append(str(exc))
            continue
        if value == "":
            missing.append(token)
            continue
        rendered.append(value)
    if missing:
        return None, sorted(set(missing))
    return rendered, []


def dispatch_plan(repo_root: Path, payload, result):
    dispatch = payload.get("dispatch", {})
    mode = dispatch.get("mode", "route_only")
    selected_skill = result.get("selected_skill", {}).get("skill_name", "none")
    plan = {
        "mode": mode,
        "selected_skill": selected_skill,
        "skill_file": None,
        "command_source": "none",
        "command": None,
        "status": "not_requested",
        "result": "not_applicable",
        "exit_code": None,
        "stdout": "",
        "stderr": "",
    }
    if selected_skill in SKILL_FILES:
        plan["skill_file"] = str((repo_root / SKILL_FILES[selected_skill]).resolve())
    if mode == "route_only":
        return plan
    if selected_skill in (None, "none"):
        plan["status"] = "blocked"
        plan["result"] = "no_selected_skill"
        return plan

    overrides = dispatch.get("command_overrides", {}) or {}
    command_template = overrides.get(selected_skill)
    if command_template:
        plan["command_source"] = "override"
    elif dispatch.get("allow_builtin_dispatch", True):
        command_template = BUILTIN_DISPATCH_COMMANDS.get(selected_skill)
        if command_template:
            plan["command_source"] = "builtin"
    if not command_template:
        plan["status"] = "unsupported"
        plan["result"] = "dispatch_unsupported_for_selected_skill"
        return plan

    command, missing = render_command_template(command_template, dispatch_placeholders(repo_root, result))
    if not command:
        plan["status"] = "blocked"
        plan["result"] = "missing_dispatch_placeholders"
        plan["stderr"] = f"missing placeholders: {', '.join(missing)}"
        return plan

    plan["command"] = command
    plan["status"] = "planned"
    plan["result"] = "planned"

    if mode != "invoke_subtask":
        return plan

    executed = run_command_with_timeout(command, repo_root, dispatch.get("timeout_secs"))
    if isinstance(executed, dict) and executed.get("timed_out"):
        plan["status"] = "failed"
        plan["result"] = "timeout"
        plan["stderr"] = (executed.get("stderr") or "").strip()
        plan["stdout"] = (executed.get("stdout") or "").strip()
        return plan

    plan["exit_code"] = executed.returncode
    plan["stdout"] = executed.stdout.strip()
    plan["stderr"] = executed.stderr.strip()
    if executed.returncode == 0:
        plan["status"] = "invoked"
        plan["result"] = "success"
    else:
        plan["status"] = "failed"
        plan["result"] = "failure"
    return plan


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    parser.add_argument("--artifact-path")
    parser.add_argument("--no-artifact", action="store_true")
    args = parser.parse_args()

    payload = load_payload(args.input)
    snapshot = collect_state(payload)
    result = evaluate(snapshot)
    result_target = result.setdefault("target", {})
    for key in ("slug", "version", "source_prompt_path"):
        if snapshot.get("target", {}).get(key) not in (None, ""):
            result_target[key] = snapshot["target"][key]
    result["artifact"] = {"path": None}

    if not args.no_artifact:
        repo_root = Path(payload["repo_root"]).resolve()
        artifact_path = Path(args.artifact_path) if args.artifact_path else default_artifact_path(repo_root)
        if not artifact_path.is_absolute():
            artifact_path = repo_root / artifact_path
        artifact_path.parent.mkdir(parents=True, exist_ok=True)
        result["artifact"]["path"] = str(artifact_path)
        result.setdefault("actions_taken", []).append(f"wrote routing artifact to {artifact_path}")

    result["dispatch"] = dispatch_plan(Path(payload["repo_root"]).resolve(), payload, result)
    if result["dispatch"]["status"] in {"invoked", "failed", "unsupported", "blocked"}:
        result.setdefault("actions_taken", []).append(
            f"dispatch {result['dispatch']['result']} for {result['dispatch']['selected_skill']}"
        )
    elif result["dispatch"]["status"] == "planned":
        result.setdefault("actions_taken", []).append(
            f"planned dispatch for {result['dispatch']['selected_skill']}"
        )

    if not args.no_artifact:
        artifact_path = Path(result["artifact"]["path"])
        artifact_path.write_text(render_markdown(result), encoding="utf-8")

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
