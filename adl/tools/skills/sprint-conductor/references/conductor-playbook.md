# Sprint Conductor Playbook

## Purpose

Run one sprint through the existing ADL issue workflow without replacing the
issue-level skill family.

This is the slow path.
One issue at a time.
No parallel child issue execution.

## Inputs

Required:
- one sprint-management issue, or policy authority to let the skill create it
- ordered child issue list
- sprint goal
- explicit policy block

Optional:
- current issue
- completed issues
- blocked issue
- issue records with PR URLs and artifact links
- prior sprint-state artifact

## Core Loop

1. Load or create sprint-state.
2. Run sprint-wide structured prompt review before starting issue execution:
   - if every child issue is ready, continue
   - if any child issue needs repair, route the matching editor skill before issue execution
   - if any child issue is blocked, stop and report it
3. If the sprint-management issue is missing:
   - create it through the bundled helper when policy allows, then continue
   - otherwise stop and report `missing_sprint_issue`
4. Re-check live GitHub truth for the current sprint-state and require a matched result before any sprint-state transition.
5. Choose the earliest child issue in the ordered list that is not yet closed
   out.
6. Route that child issue through `workflow-conductor`.
7. Run only the selected downstream lifecycle or editor skill.
8. Re-check issue truth.
9. If the issue is not fully closed out, repeat the routing loop for the same
   issue.
10. If the issue is in a healthy PR-open waiting state, pause on that issue and
   surface `ask_operator` rather than re-driving execution or janitoring a
   non-blocked PR.
11. If the issue is fully closed out, mark it complete in sprint-state and move
   to the next issue.
12. If any blocker is encountered, stop and report the blocker in sprint-state.

Preferred structured-prompt preflight helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py --repo-root <repo> --ordered-issues <csv> --state <path>`

Preferred missing-sprint-issue helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/create_missing_sprint_issue.py --repo-root <repo> --ordered-issues <csv> --title <title> --goal <goal> --state <path>`

Preferred live-truth helper:
- `python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py --repo-root <repo> --state <path> --require-match`

## Editor-Skill Rule

If `workflow-conductor` selects:
- `stp-editor`
- `sip-editor`
- `sor-editor`

then the sprint conductor must run that editor skill first and must not try to
work around malformed cards by hand.
That rule applies both to sprint-wide preflight repair and to later issue-local drift.

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

Final sprint-management issue closeout:
- keep the sprint-management issue open until all child issues are closed out
- keep it open until sprint review and sprint closeout are complete
- close it last through the bundled helper:
  - `python3 adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py --state <path> --summary <text>`

## Stop Conditions

Stop when:
- all ordered child issues are fully closed out, the sprint review/closeout
  artifacts are written, and the sprint-management issue is closed
- a child issue blocks and the operator must decide how to proceed
- sprint scope changes materially and requires operator approval

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
