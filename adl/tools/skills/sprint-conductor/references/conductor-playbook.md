# Sprint Conductor Playbook

## Purpose

Run one sprint through the existing ADL issue workflow without replacing the
issue-level skill family.

The default safe path is still sequential, but the sprint may declare
`sequential`, `parallel`, or `hybrid` execution intent through a Sprint
Execution Packet (SEP). Parallel and hybrid execution require named safe lanes,
candidate lane classifications, serial gates, and PVF notes before child work
is delegated to separate issue workers or sessions. The current helper state
remains single-current-issue and does not itself automate multi-active issue
execution.

## Inputs

Required:
- one sprint-management issue, or policy authority to let the skill create it
- ordered child issue list
- declared execution mode
- Sprint Execution Packet for parallel or hybrid modes
- descriptive sprint objective
- explicit policy block

Optional:
- current issue
- completed issues
- blocked issue
- issue records with PR URLs and artifact links
- prior sprint-state artifact

## Core Loop

1. Load or create sprint-state.
2. Resolve the Sprint Execution Packet:
   - if `execution_mode` is `sequential`, record the ordered child issue list
     and serial closeout bar
   - if `execution_mode` is `parallel`, require candidate lane
     classifications, write-set boundaries, proof lanes, dependency gates,
     watcher/subagent assignments, and coordination notes
   - if `execution_mode` is `hybrid`, require both candidate parallel lanes
     and the serial gates that control later work
   - if required SEP fields are missing, stop and repair the sprint umbrella
     before starting child issue work
3. Before live issue execution begins, run the installed-skill parity/readiness gate when sprint policy requires it:
   - if parity matches, continue
   - if installed skill drift is detected, stop and repair the live install before running the sprint
3. Run sprint-wide structured prompt review before starting issue execution:
   - if every child issue is ready, continue
   - if any child issue needs repair, route the matching editor skill before issue execution
   - if any child issue is blocked, stop and report it
4. If the sprint-management issue is missing:
   - create it through the bundled helper when policy allows, then continue
   - otherwise stop and report `missing_sprint_issue`
5. Re-check live GitHub truth for the current sprint-state and require a matched result before any sprint-state transition.
6. Choose the next child issue or operator-approved lane handoff:
   - in `sequential`, choose the earliest child issue that is not yet closed
     out
   - in `parallel`, use the SEP to identify ready lane work for separate
     workers/sessions; this helper records and resumes one current child at a
     time
   - in `hybrid`, use the SEP to identify ready lane work until a named serial
     gate blocks progress
7. Route the selected child issue or lane handoff through `workflow-conductor`.
8. When the route starts or resumes live child execution, attach the
   issue-bound session-goal requirement in the same handoff:
   - create the goal after bind/readiness succeeds and before implementation
     starts
   - include the sprint issue number when present, the child issue number, and
     the bounded session objective
   - for `parallel` or `hybrid` lanes, each separate worker/session creates its
     own child-session goal rather than sharing one sprint-global goal
   - treat the sprint objective as descriptive coordination context only; it
     must not occupy the active Codex session-goal slot during child issue
     implementation unless nested-goal support is explicitly proven later
9. Run only the selected downstream lifecycle or editor skill.
9a. When issue-goal metrics are available from the active session or closeout handoff, record them in the local JSONL sink:
   - use `record_issue_goal_metrics.py`
   - capture stage should be one of `issue_start`, `pr_publication`,
     `review_handoff`, `merge_closeout`, or `sprint_closeout`
   - preserve `unknown` / `not_available` for missing elapsed or token data;
     do not substitute zero
10. Re-check issue truth.
11. If the issue is in a healthy PR-open waiting state, route `issue-watcher`
   so the sprint remains attached to the active child without turning healthy
   waiting into a default conversational stop.
12. If the watcher finds failed checks, conflicts, requested changes, or
   ambiguous path-policy state, route `pr-janitor`.
13. If the watcher finds that the PR merged or the issue otherwise closed,
   immediately route `pr-closeout`.
14. If the issue is fully closed out, use the deterministic child-closeout
   helper to mark it complete and move to the next issue.
15. If the issue is still active after the fresh truth check, repeat the
   routing loop for the same issue.
16. If any true blocker is encountered, stop and report the blocker in
   sprint-state.
17. During sprint closeout, compare planned versus actual achieved
    parallelism, record prediction misses, and explain why speculative or
    blocked lanes did not start.

Important: sprint-level aggregate proof must not hide failed, pending,
deferred, blocked, skipped, or unreviewed child lanes.

Preferred installed-skill parity helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/check_installed_skill_parity.py --repo-root <repo> --state <path>`

Preferred structured-prompt preflight helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py --repo-root <repo> --ordered-issues <csv> --state <path>`

Preferred missing-sprint-issue helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/create_missing_sprint_issue.py --repo-root <repo> --ordered-issues <csv> --title <title> --goal <descriptive-sprint-objective> --state <path>`

Issue command policy note:
- the missing-sprint-issue helper should use the repo-native typed issue
  surface by default for child-title lookup and sprint-issue creation
- test harnesses or controlled bootstrap fixtures may override the helper's
  issue-view or issue-create command through explicit environment variables,
  but that override is a bounded fixture hook rather than the normal
  operational backend

Preferred live-truth helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py --repo-root <repo> --state <path> --require-match`

Preferred child-closeout advancement helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/record_child_issue_closeout.py --state <path> --issue-number <n> --issue-closed true --pr-state <merged|closed_no_merge|not_applicable> --root-sor-status <done|failed> --worktree-status <pruned|retained_with_reason|not_applicable>`

Preferred issue-goal metrics helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py --state <path> --issue-number <n> --sink <jsonl> --capture-stage <issue_start|pr_publication|review_handoff|merge_closeout|sprint_closeout> --data-source <codex_goal_tool|manual_entry|derived_sprint_state|unknown>`

## Editor-Skill Rule

If `workflow-conductor` selects:
- `stp-editor`
- `sip-editor`
- `spp-editor`
- `srp-editor`
- `sor-editor`

then the sprint conductor must run that editor skill first and must not try to
work around malformed cards by hand.
That rule applies both to sprint-wide preflight repair and to later issue-local drift.
Sprint-scoped `spp.md` surfaces are not mandatory unless the sprint schema
explicitly declares them; do not widen issue-level `spp-editor` into a
sprint-level planning editor by accident.

## Review Phase

After the final child issue closes out:

1. Build a bounded sprint review packet.
2. Run a code-level review of the actual changed implementation surfaces.
3. Run a test-focused review of validation adequacy and gaps.
4. Run docs review if tracked docs changed materially.
5. Run security review if trust boundaries or execution authority changed.
6. Run synthesis and separate:
   - confirmed findings
   - non-findings
   - unresolved questions
   - residual risk

Recommended review stack:
- `repo-packet-builder`
- `repo-review-code`
- `repo-review-tests`
- `repo-review-docs` when needed
- `repo-review-security` when needed
- `repo-review-synthesis`

Bounded review-subagent exception:
- child issue execution stays local to the normal issue skill path
- one bounded reviewer subagent may be used during sprint review when sprint
  policy explicitly enables it
- forked reviewer subagents inherit the parent session's model; do not pass an
  explicit model override in forked review-subagent handoffs
- if disabled, record that no review subagent was used
- validate the declared reviewer-subagent set before review execution:
  - `python3 adl/tools/skills/sprint-conductor/scripts/validate_review_subagent_policy.py --allow-review-subagent-exception <bool> --max-review-subagents 1`

## Closeout Phase

Record:
- sprint issue
- ordered issue list
- completed issues
- blocked/deferred/carryover state
- per-issue PR URLs
- per-issue artifact links
- sprint review result
- issue-goal metrics rollup, when available
- coverage snapshot
- Rust tracker counts
- next action
- sprint-management issue closeout

Preferred metrics sources:
- coverage when a fresh local snapshot is required by sprint policy:
  - `cargo llvm-cov --workspace --all-features --summary-only`
- Rust tracker:
  - `bash adl/tools/report_large_rust_modules.sh --format tsv`

For the Rust tracker report, count the number of rows at each level:
- `WATCH`
- `REVIEW`
- `RATIONALE`

If metrics are taken from CI or an existing quality-gate artifact instead of a
fresh local run, say so explicitly.
`not_applicable` is also a normal outcome for docs-only, workflow-only,
planning-only, or similarly light sprint surfaces.

Follow-up issue policy:
- default follow-up disposition should be `post_sprint_follow_on`
- those issues are recorded in sprint state and the closeout artifact
- they do not block sprint closure unless the explicit sprint policy marks them
  `must_land_before_sprint_close`
- blocking follow-ups should be visible in the sprint closeout cleanliness field

Final sprint-management issue closeout:
- keep the sprint-management issue open until all child issues are closed out
- keep it open until sprint review and sprint closeout are complete
- write one bounded closeout artifact before closing the sprint issue:
  - `python3 adl/tools/skills/sprint-conductor/scripts/write_sprint_closeout_artifact.py --state <path> --out <path>`
- close it last through the bundled helper:
  - `python3 adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py --state <path> --summary <text>`

## Stop Conditions

Stop when:
- all ordered child issues are fully closed out, the sprint review/closeout
  artifacts are written, and the sprint-management issue is closed
- a child issue blocks and the operator must decide how to proceed
- sprint scope changes materially and requires operator approval

Do not stop merely because:
- a healthy PR is open and still waiting on checks or review
- checks are rerunning without a concrete blocker
- a merge already happened but local closeout still needs to run

## Non-Goals

Do not:
- parallelize child issues
- skip child issue closeout to save time
- silently create additional sprint-management issues when policy does not allow it
- widen the sprint because nearby work looks tempting

## Standard-Path Guardrails

If `sprint-conductor` becomes a normal operating path rather than a narrow
trial, the next hardening step should be:
- preserve the mandatory GitHub truth gate before and after each child issue handoff while reducing operator friction around it
- require one explicit routing artifact per child issue
- fail review startup when more than the allowed bounded reviewer-subagent set
  is declared
