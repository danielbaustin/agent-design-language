---
name: records-hygiene
description: Scan ADL lifecycle records for truth drift, report bounded machine-readable findings, and optionally apply narrow safe repairs.
---

# Records Hygiene

This skill owns scoped hygiene for ADL lifecycle records across STP, SIP, SOR,
and related workflow evidence.

Its job is to:

- detect common record-truth drift (status, linkage, placeholder, ID, and integration-state drift)
- classify each finding by safety and ambiguity
- emit structured evidence and repair recommendations in a machine-readable shape
- apply only narrow, deterministic safe fixes when explicitly allowed
- recommend follow-on issues when the drift signals tooling gaps

This skill is a focused helper. It does not replace editor skills when the fix is
qualitative, and it does not expand into broader repo cleanup beyond the bounded
target.

## Entry Conditions

Use this skill when all of the following are true:

- you have a concrete target (issue, task bundle, or worktree)
- the request is to perform record truth hygiene, not general code implementation
- the caller wants machine-readable output (or reviewable remediation steps)

Use `records-hygiene` when:

- `status` fields appear stale for completed, closed, or merged issues
- `worktree_only`/`pr_open` state fields conflict with actual repo state
- placeholder values remain in tracked cards
- PR/issue links or run IDs drift across paired lifecycle surfaces
- a safe, local mechanical correction is desired before broader remediation

Do not use this skill for:

- speculative issue edits without any bounded target
- broad workflow orchestration that should stay with `workflow-conductor`
- broad one-off documentation cleanup outside the bound task bundle
- open-ended truth inference that requires human arbitration

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `issue_number`
  - `task_bundle_path`
  - `branch`
  - `worktree_path`

Useful additional inputs:

- `slug`
- `version`
- explicit target surfaces (`source_prompt_path`, `stp_path`, `sip_path`, `sor_path`)
- `policy` (`report_only`, `apply_safe_repairs`, `include_follow_on_issues`,
  `stop_after_analysis`)

## Quick Start

1. Resolve the issue identity and concrete target first.
2. Read the authoritative surfaces for that target.
3. Run deterministic drift detection for status, linkage, placeholder, IDs, and
   integration claims.
4. Classify each finding as safe fix, skipped, or ambiguous.
5. If `report_only` is false and the finding is safe/ambiguous-free,
   apply the repair.
6. Emit a structured result with recommendations for remaining work.

## Workflow

### 1. Resolve and Normalize the Target

Prefer concrete target mode order:

1. issue number
2. task bundle path
3. worktree path
4. branch

Only repair within that issue scope. Do not infer a different issue unless the
bound data is unambiguous.

### 2. Drift Detection

Detect all of these classes:

- lifecycle-status drift
  - `status` in STP/SIP/SOR indicates completion while other records imply work is
    still active
  - stale `NOT_STARTED`/`IN_PROGRESS` on completed outputs
- PR/worktree state drift
  - `worktree_only`/`pr_open` flags out of sync with real branch-worktree evidence
- linkage drift
  - missing or mismatched PR/issue link in SOR execution sections
- placeholder drift
  - `TODO`, `TBD`, `<RUN_ID>`, `<timestamp>`, `N/A` used in finalized fields
- identity drift
  - run IDs, issue IDs, or branch names that disagree across task-card surfaces
- integration truth drift
  - claims that artifact is in main repository while only worktree copy exists

### 3. Safe Fixes vs Ambiguity

Safe fixes are only those that are mechanical and unambiguous:

- replacing known placeholder markers in clearly bounded non-assertive fields
- normalizing list/order of evidence fields where provenance is deterministic
- back-filling missing worktree path in record surfaces when the target worktree is
  uniquely resolved
- replacing stale run-id fields with deterministic values from already bound evidence

Treat these as ambiguous and report for operator follow-up:

- mixed status signals across card and PR/tooling states
- uncertain scope intent in acceptance criteria
- open PR-state questions where closed/open claims conflict across stale sources

### 4. Reporting and Handoffs

Output must include:

- machine-readable finding list with severity and area
- safe fixes applied
- skipped files
- ambiguity list with recommended review inputs
- follow-on issues for process/tooling gaps
- handoff guidance (operator or editor skill)

This skill should stop before repository-wide planning changes.

## Workflow Boundaries

Allowed writes are mechanical and bounded to the addressed issue surfaces.

Stop before:

- broad code or docs reimplementation
- workflow-wide milestone edits
- PR merge or closeout decisions
- creation of follow-on implementation work that should be a sibling issue

## Output

Use `references/output-contract.md` and emit the structured shape described there.

Artifact path pattern should be an `.adl/reviews` record under the active
repository path.

