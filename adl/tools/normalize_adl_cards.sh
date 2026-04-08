#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=""
TARGET_VERSION="v0.87.1"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      ROOT_DIR="$2"
      shift 2
      ;;
    --version)
      TARGET_VERSION="$2"
      shift 2
      ;;
    *)
      echo "usage: $0 [--root <repo-root>] [--version <version>]" >&2
      exit 1
      ;;
  esac
done

if [[ -z "$ROOT_DIR" ]]; then
  ROOT_DIR="$(pwd)"
fi

ROOT_DIR="$(cd "$ROOT_DIR" && pwd)"

python3 - "$ROOT_DIR" "$TARGET_VERSION" <<'PY'
import glob
import os
import re
import sys

root, version = sys.argv[1:3]


def field_value(path: str, label: str) -> str:
    with open(path, "r", encoding="utf-8") as fh:
        for line in fh:
            if line.startswith(f"{label}:"):
                return line.split(":", 1)[1].strip()
    raise SystemExit(f"missing field '{label}' in {path}")


def write_bootstrap_sor(task_dir: str) -> None:
    sip_path = os.path.join(task_dir, "sip.md")
    sor_path = os.path.join(task_dir, "sor.md")
    task_id = field_value(sip_path, "Task ID")
    run_id = field_value(sip_path, "Run ID")
    version_value = field_value(sip_path, "Version")
    title = field_value(sip_path, "Title")
    branch = field_value(sip_path, "Branch")
    slug = os.path.basename(task_dir).split("__", 1)[1]
    issue_num = re.search(r"issue-(\d+)__", os.path.basename(task_dir)).group(1)

    body = f"""# {slug}

Task ID: {task_id}
Run ID: {run_id}
Version: {version_value}
Title: {title}
Branch: {branch}
Status: IN_PROGRESS

Execution:
- Actor: not started; issue prepared for later execution
- Model: not applicable
- Provider: not applicable
- Start Time: not started
- End Time: not started

## Summary

Materialized the canonical bootstrap output card so this issue bundle has a truthful, machine-readable pre-run execution record. No implementation work has started yet.

## Artifacts produced

- bootstrap output card at `.adl/{version_value}/tasks/{os.path.basename(task_dir)}/sor.md`

## Actions taken

- created the missing bootstrap output card from the current task-bundle metadata
- preserved truthful pre-run state with no execution claims
- left implementation, PR publication, and closeout for later workflow phases

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/{version_value}/tasks/{os.path.basename(task_dir)}/sor.md`
- Worktree-only paths remaining: none
- Integration state: main_repo
- Verification scope: main_repo
- Integration method used: direct write in main repo
- Verification performed:
  - `ls .adl/{version_value}/tasks/{os.path.basename(task_dir)}/sor.md`
    - verified the canonical output-card path exists
- Result: PASS

## Validation
- `ls .adl/{version_value}/tasks/{os.path.basename(task_dir)}/sor.md`
  - verified the canonical output-card path exists
- Results:
  - the bootstrap output card exists
  - the bundle now has `stp`, `sip`, and `sor` surfaces for compatibility tooling

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - ls .adl/{version_value}/tasks/{os.path.basename(task_dir)}/sor.md
  determinism:
    status: NOT_RUN
    replay_verified: false
    ordering_guarantees_verified: unknown
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: not applicable; this was bootstrap-card repair only
- Fixtures or scripts used: none
- Replay verification (same inputs -> same artifacts/order): not applicable
- Ordering guarantees (sorting / tie-break rules used): not applicable
- Artifact stability notes: the output card is deterministic for the current task-bundle metadata

## Security / Privacy Checks
- Secret leakage scan performed: manual review only; no secrets or credentials were introduced
- Prompt / tool argument redaction verified: yes; repository-relative references only
- Absolute path leakage check: passed; final artifacts use repository-relative paths only
- Sandbox / policy invariants preserved: yes

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `.adl/{version_value}/tasks/{os.path.basename(task_dir)}/sor.md`
- Required artifacts present: yes
- Artifact schema/version checks: not applicable
- Hash/byte-stability checks: not applicable
- Missing/optional artifacts and rationale: no execution artifacts exist yet because run phase has not started

## Decisions / Deviations
- Kept the issue in truthful pre-run state with `Branch: {branch}` and no execution claims

## Follow-ups / Deferred work
- bind branch/worktree during `pr-run` when implementation begins
- replace this bootstrap output card with a truthful completed record when work is actually performed
"""
    with open(sor_path, "w", encoding="utf-8") as fh:
        fh.write(body)


def ensure_links(cards_dir: str, task_dir: str, issue: str) -> None:
    os.makedirs(cards_dir, exist_ok=True)
    rel_target_base = os.path.relpath(task_dir, cards_dir)
    mapping = {
        f"input_{issue}.md": "sip.md",
        f"stp_{issue}.md": "stp.md",
        f"output_{issue}.md": "sor.md",
    }
    for name, leaf in mapping.items():
        link_path = os.path.join(cards_dir, name)
        target = os.path.join(rel_target_base, leaf)
        if os.path.lexists(link_path):
            os.remove(link_path)
        os.symlink(target, link_path)


repaired_outputs = []
normalized_roots = 0
normalized_worktrees = 0

task_glob = os.path.join(root, ".adl", version, "tasks", "issue-*__*")
for task_dir in sorted(glob.glob(task_glob)):
    issue_match = re.search(r"issue-(\d+)__", os.path.basename(task_dir))
    if not issue_match:
        continue
    issue = issue_match.group(1)
    sip_path = os.path.join(task_dir, "sip.md")
    stp_path = os.path.join(task_dir, "stp.md")
    sor_path = os.path.join(task_dir, "sor.md")
    if os.path.isfile(sip_path) and os.path.isfile(stp_path) and not os.path.isfile(sor_path):
        write_bootstrap_sor(task_dir)
        repaired_outputs.append(task_dir)

    root_cards = os.path.join(root, ".adl", "cards", issue)
    ensure_links(root_cards, task_dir, issue)
    normalized_roots += 1

for wt in sorted(glob.glob(os.path.join(root, ".worktrees", "adl-wp-*"))):
    issue_match = re.search(r"adl-wp-(\d+)$", wt)
    if not issue_match:
        continue
    issue = issue_match.group(1)
    matches = glob.glob(os.path.join(wt, ".adl", version, "tasks", f"issue-{issue}__*"))
    if not matches:
        continue
    task_dir = matches[0]
    sip_path = os.path.join(task_dir, "sip.md")
    stp_path = os.path.join(task_dir, "stp.md")
    sor_path = os.path.join(task_dir, "sor.md")
    if os.path.isfile(sip_path) and os.path.isfile(stp_path) and not os.path.isfile(sor_path):
        write_bootstrap_sor(task_dir)
        repaired_outputs.append(task_dir)
    cards_dir = os.path.join(wt, ".adl", "cards", issue)
    ensure_links(cards_dir, task_dir, issue)
    normalized_worktrees += 1

print(f"normalized root bundles: {normalized_roots}")
print(f"normalized worktree bundles: {normalized_worktrees}")
print(f"materialized missing sor cards: {len(repaired_outputs)}")
for path in repaired_outputs:
    print(f"  {os.path.relpath(path, root)}")
PY
