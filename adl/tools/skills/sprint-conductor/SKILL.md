---
name: sprint-conductor
description: Lightweight slow-path sprint orchestrator for ADL. Use when one sprint issue should drive an ordered list of child issues through the existing lifecycle and editor skills sequentially, with no parallel issue execution, robust sprint-end review, and truthful sprint closeout including coverage and Rust tracker metrics.
---

# Sprint Conductor

This skill is a thin sprint-level orchestrator over the existing ADL operational
skills.

Its job is to:
- intake one concrete sprint issue and one explicit ordered issue list
- enforce the slow path: one issue at a time, fully closed out before the next
  issue starts
- route each issue through the existing lifecycle and editor skills rather than
  reimplementing them
- preserve resumable sprint state in one lightweight local artifact
- assemble a robust sprint-end review with code-facing review expectations
- record sprint-end closeout truth including coverage and Rust tracker numbers
- stop immediately on blocker rather than wandering across adjacent issue work

This skill must remain lightweight.

It must not replace:
- `workflow-conductor`
- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-finish`
- `pr-janitor`
- `pr-closeout`
- `stp-editor`
- `sip-editor`
- `sor-editor`
- review specialist skills

It is a sprint orchestrator, not a second execution engine.

Important execution-boundary rule:
- the sprint conductor itself should not directly implement child issue code
- it may orchestrate child issues that legitimately edit tracked implementation
  files through `pr-run`, `pr-finish`, and the normal issue lifecycle in the
  bound issue worktree
- it may use a bounded review subagent exception during sprint review when the
  sprint policy explicitly enables that path

## Design Basis

This skill should track the repository's canonical operational skill family and
sprint/review/closeout surfaces.

At the moment, the key repo references are:
- `/Users/daniel/git/agent-design-language/adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- `/Users/daniel/git/agent-design-language/docs/templates/SPRINT_TEMPLATE.md`
- `/Users/daniel/git/agent-design-language/docs/templates/RELEASE_PLAN_TEMPLATE.md`
- `/Users/daniel/git/agent-design-language/adl/tools/report_large_rust_modules.sh`
- `/Users/daniel/git/agent-design-language/adl/README.md`

Within this bundle, the operational details live in:
- `references/conductor-playbook.md`
- `references/output-contract.md`
- `scripts/check_sprint_truth.py`
- `scripts/update_sprint_state.py`
- `scripts/validate_review_subagent_policy.py`

## Entry Conditions

Use this skill when all of the following are true:
- there is one concrete sprint-management issue
- the sprint has an explicit ordered list of issue numbers
- the operator wants slow-path sequential execution rather than manual
  issue-by-issue orchestration
- sprint-end review and closeout should be handled as part of the same bounded
  sprint flow

Do not use this skill for:
- multi-sprint planning
- parallel issue execution
- replacing issue-level lifecycle skills
- silently widening sprint scope
- broad portfolio or roadmap management

## Required Inputs

At minimum, gather:
- `repo_root`
- `sprint.issue_number`
- `sprint.ordered_issue_numbers`
- `sprint.goal`
- one explicit policy block

Useful additional inputs:
- `sprint.version`
- `sprint.slug`
- `sprint.stop_date`
- `sprint.current_issue_number`
- `sprint.completed_issue_numbers`
- `sprint.blocked_issue_number`
- `sprint.review_paths`
- `sprint.closeout_paths`
- `sprint.issue_records`
- `resume_from_state_path`

If there is no concrete sprint issue or ordered issue list, stop and report
`blocked`.

## Quick Start

1. Resolve the concrete sprint issue and ordered child issue list.
2. Create or load the sprint-state artifact.
3. Confirm the current issue is the earliest not-yet-closed issue in the list.
4. Route that issue through `workflow-conductor`.
5. Re-check live issue and PR truth before acting. This is a blocking gate, not a suggestion.
6. Run only the selected downstream lifecycle or editor skill.
7. Re-check issue truth until the issue is fully closed out. Every sprint-state transition consumes the last successful truth check, so the next transition requires a fresh recheck.
8. Only then advance to the next ordered issue.
9. After the final issue closes, assemble sprint review evidence.
10. Record sprint closeout metrics including coverage and Rust tracker counts.
11. Stop with one bounded sprint review/closeout result.

## Execution Model

This skill enforces:
- exactly one active child issue at a time
- no issue `N+1` work before issue `N` is fully closed out
- editor-skill routing when cards drift
- immediate stop on blocker
- no silent creation of extra sprint-management issues
- resumable per-issue state with PR URLs and artifact links when available
- no sprint-state advancement without a fresh matched live GitHub truth check

Preferred per-issue routing model:
- bootstrap missing -> `pr-init`
- card-local STP issue -> `stp-editor`
- card-local SIP issue -> `sip-editor`
- card-local SOR issue -> `sor-editor`
- structurally ready but not bound -> `pr-ready`
- ready for execution bind -> `pr-run`
- execution complete, needs publication -> `pr-finish`
- PR in flight with actual blockers -> `pr-janitor`
- merged or intentionally closed -> `pr-closeout`
- healthy PR open and awaiting human review or merge -> remain paused on the
  current issue in a non-blocked waiting state

## Sprint Review Requirement

Sprint review must be robust and code-facing.

Minimum sprint review flow:
1. collect the closed issue list, PR list, validation notes, and changed tracked
   surfaces
2. build a bounded review packet
3. run a code-level review over the actual implementation surfaces changed by the
   sprint
4. run a test-focused review over validation adequacy and gaps
5. run a synthesis pass that separates findings, non-findings, unresolved
   questions, and residual risk

Recommended review-skill stack:
- `repo-packet-builder`
- `repo-review-code`
- `repo-review-tests`
- `repo-review-docs` when tracked docs changed materially
- `repo-review-security` when trust boundaries or execution authority changed
- `repo-review-synthesis`

Bounded review-subagent exception:
- child issue execution remains issue-skill driven rather than subagent driven
- sprint review may use one bounded reviewer subagent when sprint policy
  explicitly enables that exception
- if the exception is disabled, the sprint review must remain local and record
  that no review subagent was used
- the bundle should validate this boundary mechanically before review execution

The sprint review must not claim that docs/cards alone are sufficient when the
sprint changed implementation code.

## Sprint Closeout Requirement

Sprint closeout must record:
- sprint issue identity
- ordered issue list and completion status
- blocked/deferred/carryover state if any
- per-issue PR URLs when they exist
- per-issue artifact links needed to resume or audit the sprint
- sprint review result
- current code coverage snapshot
- current Rust tracker counts
- recommended next sprint or remediation action

Preferred metrics capture surfaces:
- coverage command when the sprint surface warrants a fresh local snapshot:
  - `cargo llvm-cov --workspace --all-features --summary-only`
- Rust tracker report:
  - `bash adl/tools/report_large_rust_modules.sh --format tsv`

Coverage capture must be policy-driven rather than reflexive.

Normal closeout sources include:
- fresh local run when the sprint materially changed implementation surfaces and
  the sprint policy requires it
- CI evidence
- an existing quality-gate or release-evidence artifact
- `not_applicable` for docs-only, workflow-only, planning-only, or similarly
  light sprint surfaces

If local metrics commands are not run, the closeout must say whether the
numbers came from CI, an existing quality gate artifact, or are not applicable
for the sprint surface.

## Stop Boundary

This skill must stop after:
- sequential issue orchestration through the existing skill family
- one bounded sprint review result
- one bounded sprint closeout result
- surfacing any blocker that prevents safe continuation

It must not:
- parallelize child issues silently
- absorb the underlying issue lifecycle logic
- skip issue closeout to save time
- invent coverage or Rust tracker numbers
- reopen completed child issues without explicit operator direction

## Output

Return a concise structured result including:
- sprint issue
- ordered issue list
- completed issues
- blocked/current issue state
- truth-check status and source
- review status and review artifact paths
- coverage status
- Rust tracker status
- continuation or stop recommendation
- sprint-state artifact path
