# Sprint Conductor Playbook

## Purpose

Run one sprint through the existing ADL issue workflow without replacing the
issue-level skill family.

This is the slow path.
One issue at a time.
No parallel child issue execution.

## Inputs

Required:
- one sprint-management issue
- ordered child issue list
- sprint goal
- explicit policy block

Optional:
- current issue
- completed issues
- blocked issue
- prior sprint-state artifact

## Core Loop

1. Load or create sprint-state.
2. Choose the earliest child issue in the ordered list that is not yet closed
   out.
3. Route that child issue through `workflow-conductor`.
4. Run only the selected downstream lifecycle or editor skill.
5. Re-check issue truth.
6. If the issue is not fully closed out, repeat the routing loop for the same
   issue.
7. If the issue is in a healthy PR-open waiting state, pause on that issue and
   surface `ask_operator` rather than re-driving execution or janitoring a
   non-blocked PR.
8. If the issue is fully closed out, mark it complete in sprint-state and move
   to the next issue.
9. If any blocker is encountered, stop and report the blocker in sprint-state.

## Editor-Skill Rule

If `workflow-conductor` selects:
- `stp-editor`
- `sip-editor`
- `sor-editor`

then the sprint conductor must run that editor skill first and must not try to
work around malformed cards by hand.

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

## Closeout Phase

Record:
- sprint issue
- ordered issue list
- completed issues
- blocked/deferred/carryover state
- sprint review result
- coverage snapshot
- Rust tracker counts
- next action

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

## Stop Conditions

Stop when:
- all ordered child issues are fully closed out and the sprint review/closeout
  artifacts are written
- a child issue blocks and the operator must decide how to proceed
- sprint scope changes materially and requires operator approval

## Non-Goals

Do not:
- parallelize child issues
- skip child issue closeout to save time
- silently create additional sprint-management issues
- widen the sprint because nearby work looks tempting
