---
name: sprint-conductor
description: Lightweight sprint orchestrator for ADL. Use when one sprint issue should drive an explicit child issue wave through existing lifecycle and editor skills with a Sprint Execution Packet that records sequential, parallel, or hybrid intent, robust sprint-end review, and truthful sprint closeout.
---

# Sprint Conductor

This skill is a thin sprint-level orchestrator over the existing ADL operational
skills.

Its job is to:
- intake one concrete sprint issue and one explicit ordered issue list
- intake or verify the Sprint Execution Packet (SEP) for that sprint
- preserve the declared execution mode: `sequential`, `parallel`, or `hybrid`
  as sprint intent and operator routing evidence
- route each issue through the existing lifecycle and editor skills rather than
  reimplementing them
- preserve resumable sprint state in one lightweight local artifact
- verify and, when needed, repair structured prompt readiness across the entire
  sprint before issue execution begins
- preserve declared safe parallel lanes, serial gates, and PVF notes as sprint
  control-plane truth without claiming unproven automated multi-active
  execution
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
- `scripts/check_sprint_readiness.py`
- `scripts/check_installed_skill_parity.py`
- `scripts/check_sprint_truth.py`
- `scripts/check_sprint_closeout_readiness.py`
- `scripts/record_child_issue_closeout.py`
- `scripts/record_issue_goal_metrics.py`
- `scripts/update_sprint_state.py`
- `scripts/write_sprint_closeout_artifact.py`
- `scripts/validate_review_subagent_policy.py`

## Entry Conditions

Use this skill when all of the following are true:
- there is one concrete sprint-management issue, or sprint policy explicitly
  allows the skill to create the missing sprint-management issue first
- the sprint has an explicit ordered list of issue numbers
- the sprint has a declared execution mode and, for parallel or hybrid work,
  a Sprint Execution Packet naming safe lanes and serial gates
- sprint-end review and closeout should be handled as part of the same bounded
  sprint flow

Do not use this skill for:
- multi-sprint planning
- unbounded parallel issue execution without a SEP
- replacing issue-level lifecycle skills
- silently widening sprint scope
- broad portfolio or roadmap management

## Required Inputs

At minimum, gather:
- `repo_root`
- `sprint.issue_number` when it already exists
- `sprint.ordered_issue_numbers`
- `sprint.execution_mode`
- `sprint.goal` or equivalent descriptive sprint objective text
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
- `sprint.execution_packet_path`
- `sprint.candidate_parallel_lanes`
- `sprint.safe_parallel_lanes`
- `sprint.serial_gates`
- `sprint.pvf_notes`
- `sprint.planned_vs_actual_parallelism`
- `resume_from_state_path`

If there is no concrete sprint issue or ordered issue list, stop and report
`blocked`, unless sprint policy explicitly allows the skill to create the
missing sprint-management issue first.

## Quick Start

1. Resolve the concrete sprint issue and ordered child issue list. If the sprint
   issue is missing and policy allows creation, create it through the bundled
   helper first.
2. Resolve the Sprint Execution Packet. If the sprint declares `parallel` or
   `hybrid`, require candidate lane classifications, serial gates, watcher
   assignments, and PVF notes before execution is routed to separate issue
   workers or sessions.
3. Create or load the sprint-state artifact.
4. When live execution is about to begin, run the sprint readiness sweep first.
   Use `check_sprint_readiness.py` to aggregate installed-skill parity,
   structured-prompt preflight, execution-packet presence, review-path
   declaration, and activity-log declaration into one readiness result.
   Review and activity-log paths are declaration surfaces here; they do not
   need to exist on disk yet to satisfy readiness.
5. If the readiness sweep reports `needs_repair`, fix the flagged issue-local
   or sprint-local defects before starting child execution.
6. Run sprint-wide structured prompt review before issue execution begins when
   the readiness sweep or policy requires a fresh pass.
7. If any child issue cards are not ready, repair them through the editor skills first.
   When the child issue needs new or fully re-rendered cards, prefer the
   prompt-template values renderer and `validate-structure` before routing
   issue-local lifecycle truth through the matching editor skill.
8. Select the next child issue or operator-approved lane handoff according to
   the declared execution mode and serial gates. Current helper state remains
   single-current-issue; parallel execution is achieved by separate issue
   workers/sessions using the SEP as the coordination contract.
9. Route the selected child issue or lane handoff through `workflow-conductor`.
10. When that handoff starts or resumes live child execution, attach the
    issue-bound session-goal requirement as part of the SEP handoff rather than
    as a separate manual reminder.
    In other words: attach the issue-bound session-goal requirement as part of the SEP handoff.
    The sprint umbrella goal or objective is descriptive coordination context,
    not a substitute for the active child-session goal.
11. For SEP-routed child execution, create the goal after bind/readiness
    succeeds and before implementation starts. Minimum goal content:
    sprint issue number when present, child issue number, and the bounded
    session objective.
12. Re-check live issue and PR truth before acting. This is a blocking gate, not a suggestion.
13. Run only the selected downstream lifecycle or editor skill.
14. Re-check issue truth. Every sprint-state transition consumes the last successful truth check, so the next transition requires a fresh recheck.
15. If the issue is healthy but waiting on review, checks, or merge, route it into the bounded watch/janitor path rather than surfacing that healthy waiting state as a default sprint stop.
16. If the issue is merged or otherwise closed but not locally closeouted yet, immediately route `pr-closeout` and finish the child-closeout gate.
17. If the issue is fully closed out, use the deterministic child-closeout helper path to advance sprint state.
18. If the issue is still active after the fresh truth check, repeat the routing loop for the same issue.
19. In `sequential` mode, only after child-closeout truth is satisfied may the
    sprint advance to the next ordered issue. In `parallel` or `hybrid` mode,
    do not treat a lane as clear unless its SEP-defined dependencies, serial
    gates, and issue-local closeout truth are satisfied.
20. After the final issue closes, assemble sprint review evidence.
21. Record sprint closeout metrics including coverage and Rust tracker counts.
22. When issue-goal token/time metrics are available, record them in the local goal-metrics sink and refresh the issue/sprint summaries without treating unknown values as zero.
23. Run the deterministic sprint closeout helper to classify `ready_to_close`, `needs_remediation`, or `blocked`, write/update the retained closeout artifact, and generate the final sprint close summary.
23. Only when the closeout helper reports `ready_to_close`, close the sprint-management issue.
24. Stop with one bounded sprint review/closeout result.

## Execution Model

This skill enforces:
- exactly one active child issue in this helper's local sprint state
- one issue at a time, fully closed out before the next in `sequential` mode, unless the SEP explicitly declares a safe parallel or hybrid lane boundary
- SEP-declared active lanes for `parallel` and `hybrid` sprints as
  coordination evidence for separate workers/sessions
- no child issue execution before the whole sprint batch passes structured prompt review
- no child issue execution before the whole sprint batch passes design-time
  card-completion review for `SIP`, `STP`, `SPP`, and `SRP`
- no live sprint execution before the readiness sweep records execution-packet,
  review-path, and activity-log declaration truth
- no issue `N+1` work before issue `N` is fully closed out in `sequential`
  mode
- no intentional parallel lane work unless the SEP names the lane, write-set
  boundary, proof lane, and required coordination
- no opportunistic lane plan may omit whether the lane is safe, serial,
  speculative, or blocked on dependencies
- no live child implementation handoff without the child issue-bound session
  goal created after bind/readiness succeeds
- no sprint-global active session goal may stand in for the child issue-bound
  session goal during implementation; sprint goals are descriptive planning
  context only unless a later issue proves explicit nested-goal support
- no SEP-routed child session goal that omits sprint context when a sprint
  issue exists, the child issue number, or the bounded session objective
- editor-skill routing when cards drift
- prompt-template renderer/schema validation when cards need deterministic
  regeneration rather than lifecycle-truth repair
- immediate stop on true blocker
- no silent creation of extra sprint-management issues
- no missing sprint-management issue workaround outside the skill when sprint
  policy allows the skill to create it directly
- resumable per-issue state with PR URLs and artifact links when available
- no sprint-state advancement without a fresh matched live GitHub truth check
- no next-child advancement without explicit child-closeout gate satisfaction
- no live sprint execution without an explicit installed-skill parity/readiness result when sprint policy requires it
- no sprint goal-metrics rollup may treat unknown or unavailable elapsed/token values as zero
- no child issue execution when `SIP`, `STP`, or `SPP` has a `card_status`
  other than `ready` or `approved`
- no child closeout acceptance when `SRP` or `SOR` overclaims
  `card_status: completed` / `Card Status: completed` without review or
  terminal closeout truth
- healthy child waiting states are monitored issue-local lifecycle states, not default operator stop states
- merged-but-not-closeouted children are immediate closeout work, not natural pause points
- aggregate sprint proof must not hide failed, pending, deferred, blocked, or
  skipped child lanes

## Observability Expectations

- Treat workflow/control-plane logs as supporting sprint evidence when they help
  explain queue gates, waiting states, merge drift, or closeout behavior.
- Keep GitHub truth authoritative; logs explain state, but they do not replace
  the live issue/PR check gates.
- When a sprint is blocked by a workflow/logging defect outside the active
  child issue, record that as a follow-on rather than hiding it inside sprint
  completion claims.

Preferred per-issue routing model:
- sprint readiness not run or stale ->
  `check_sprint_readiness.py`, then repair the flagged sprint-local or
  issue-local defects before child execution
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
- live child execution handoff through `pr-run` must carry the issue-bound
  session-goal requirement directly in the sprint handoff; do not leave SEP
  child sessions dependent on a separate manual `create_goal` reminder
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
- forked reviewer subagents inherit the parent session's model; do not pass an
  explicit model override in the forked review-subagent handoff
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
- parallelize child issues silently or claim automated multi-active execution
  beyond the current helper implementation
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
