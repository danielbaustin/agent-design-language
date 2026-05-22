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
- verify and, when needed, repair structured prompt readiness across the entire
  sprint before issue execution begins
- assemble a robust sprint-end review with code-facing review expectations
- record sprint-end closeout truth including coverage and Rust tracker numbers
- manage the sprint-management issue to completion, including closing it only
  after the sprint review and closeout are truly complete
- stop immediately on blocker rather than wandering across adjacent issue work

This skill must remain lightweight.

It must not replace:
- `workflow-conductor`
- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-finish`
- `pr-janitor`
- `issue-watcher`
- `pr-closeout`
- `stp-editor`
- `sip-editor`
- `spp-editor`
- `srp-editor`
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
- `scripts/check_installed_skill_parity.py`
- `scripts/check_sprint_truth.py`
- `scripts/record_child_issue_closeout.py`
- `scripts/update_sprint_state.py`
- `scripts/write_sprint_closeout_artifact.py`
- `scripts/validate_review_subagent_policy.py`

## Entry Conditions

Use this skill when all of the following are true:
- there is one concrete sprint-management issue, or sprint policy explicitly
  allows the skill to create the missing sprint-management issue first
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
- `sprint.issue_number` when it already exists
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
`blocked`, unless sprint policy explicitly allows the skill to create the
missing sprint-management issue first.

## Quick Start

1. Resolve the concrete sprint issue and ordered child issue list. If the sprint
   issue is missing and policy allows creation, create it through the bundled
   helper first.
2. Create or load the sprint-state artifact.
3. When live execution is about to begin, run the installed-skill parity/readiness gate first.
4. Run sprint-wide structured prompt review before issue execution begins.
5. If any child issue cards are not ready, repair them through the editor skills first.
6. Confirm the current issue is the earliest not-yet-closed issue in the list.
7. Route that issue through `workflow-conductor`.
8. Re-check live issue and PR truth before acting. This is a blocking gate, not a suggestion.
9. Run only the selected downstream lifecycle or editor skill.
10. Re-check issue truth. Every sprint-state transition consumes the last successful truth check, so the next transition requires a fresh recheck.
11. If the issue is healthy but waiting on review, checks, or merge, route it into the bounded watch/janitor path rather than surfacing that healthy waiting state as a default sprint stop.
12. If the issue is merged or otherwise closed but not locally closeouted yet, immediately route `pr-closeout` and finish the child-closeout gate.
13. If the issue is fully closed out, use the deterministic child-closeout helper path to advance sprint state.
14. If the issue is still active after the fresh truth check, repeat the routing loop for the same issue.
15. Only after child-closeout truth is satisfied may the sprint advance to the next ordered issue.
16. After the final issue closes, assemble sprint review evidence.
17. Record sprint closeout metrics including coverage and Rust tracker counts.
18. Write the bounded sprint closeout artifact before closing the sprint-management issue.
19. Stop with one bounded sprint review/closeout result.

## Execution Model

This skill enforces:
- exactly one active child issue at a time
- no child issue execution before the whole sprint batch passes structured prompt review
- no child issue execution before the whole sprint batch passes design-time
  card-completion review for `SIP`, `STP`, `SPP`, and `SRP`
- no issue `N+1` work before issue `N` is fully closed out
- editor-skill routing when cards drift
- immediate stop on true blocker
- no silent creation of extra sprint-management issues
- no missing sprint-management issue workaround outside the skill when sprint
  policy allows the skill to create it directly
- resumable per-issue state with PR URLs and artifact links when available
- no sprint-state advancement without a fresh matched live GitHub truth check
- no next-child advancement without explicit child-closeout gate satisfaction
- no live sprint execution without an explicit installed-skill parity/readiness result when sprint policy requires it
- healthy child waiting states are monitored issue-local lifecycle states, not default operator stop states
- merged-but-not-closeouted children are immediate closeout work, not natural pause points

Preferred per-issue routing model:
- sprint-wide structured prompt preflight not ready ->
  `check_sprint_structured_prompt_readiness.py`, then route flagged child issues
  through the matching editor skills before starting issue execution
- sprint-wide design-time card preflight not ready -> route generic `SIP`,
  incomplete `STP`, generic/truncated or unreviewed `SPP`, and legacy/incomplete
  `SRP` cards through their matching editor skills before starting issue
  execution
- missing sprint-management issue and policy allows creation ->
  `create_missing_sprint_issue.py`, then continue with the ordered child issue
  flow from the created sprint anchor
- bootstrap missing -> `pr-init`
- card-local STP issue -> `stp-editor`
- card-local SIP issue -> `sip-editor`
- card-local SPP issue -> `spp-editor`
- card-local SRP issue -> `srp-editor`
- card-local SOR issue -> `sor-editor`
- structurally ready but not bound -> `pr-ready`
- ready for execution bind -> `pr-run`
- execution complete, needs publication -> `pr-finish`
- healthy PR open and awaiting human review, checks, or merge -> `issue-watcher`
- PR in flight with actual blockers -> `pr-janitor`
- merged or intentionally closed -> `pr-closeout`
- child issue fully closed out and ready to advance -> `record_child_issue_closeout.py`

Auto-advance rules:
- do not treat `pr_open`, `checks_rerunning`, `waiting_for_review`, or
  `merged_pending_closeout` as default sprint stop boundaries
- treat those as active issue-local lifecycle states that still have a known
  next routing step
- only surface `ask_operator` when there is a real blocker, scope decision, or
  ambiguity that the existing lifecycle skills cannot resolve truthfully
- keep operator updates short and do not convert normal lifecycle narration into
  a sprint-level halt

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

## Sprint Follow-up Policy

This skill must use one explicit policy for sprint-discovered follow-up issues.

Recommended default:
- bounded follow-up issues discovered during sprint review are recorded as
  `post_sprint_follow_on`
- they do not block sprint closure unless the sprint policy explicitly marks
  them `must_land_before_sprint_close`

That policy must be:
- explicit in the structured input/policy contract
- recorded in sprint state and closeout artifacts
- reflected in any discovered follow-up issue records written by the skill

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
- final sprint-management issue closure state

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
- final sprint-management issue closure when the sprint is truly complete
- surfacing any blocker that prevents safe continuation

It must not:
- parallelize child issues silently
- absorb the underlying issue lifecycle logic
- skip issue closeout to save time
- invent coverage or Rust tracker numbers
- close the sprint-management issue early
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
