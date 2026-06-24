# Default Workflow (adl_pr_cycle + pr.sh)

This is the default contributor path for ADL development:

`issue creation/bootstrap -> pr ready -> pr run -> codex -> run_if_required -> pr finish -> pr janitor -> pr closeout -> report`

Tracked mirror of the local skill contract:

- `docs/tooling/adl_pr_cycle_skill.md`

Install or resync the local skill with:

```bash
bash adl/tools/install_adl_pr_cycle_skill.sh
```

The canonical issue-card lifecycle is:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

See `docs/tooling/card-lifecycle.md` for the role of each card and the
creation-order versus activation-order distinction. Tooling may create card
stubs early, but new issue work should follow this semantic order only.

The active control-plane surface is:

- `pr issue`
- `pr init`
- `pr doctor`
- `pr run`
- `pr finish`

The browser/editor adapter remains narrower:

- browser-direct adapter support remains narrower than the full repo-native control plane
- direct browser/editor execution of `pr ready`, `pr run`, and `pr finish` is not the canonical workflow surface

## 1) Bootstrap Canonical Issue Bundle

```bash
bash ./adl/tools/pr.sh init <issue_num> --slug <slug> --version <milestone_version>
```

Canonical local task bundle:
- `.adl/<scope>/tasks/<task-id>__<slug>/sip.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/stp.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/spp.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/srp.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/sor.md`
- `.adl/<scope>/tasks/<task-id>__<slug>/`

Minimum init contract:
- canonical task-bundle directory
- validated issue-card scaffold surfaces
- `SIP` as issue-intent truth
- `STP` as selected task/solution truth
- `SPP` as execution-plan truth before implementation proceeds
- `SRP` as review prompt truth before PR publication, then review-result truth
- `SOR` as final outcome truth after execution, publication, merge or closure,
  and closeout

## 2) Confirm GitHub Issue Exists And Inspect Live Issue Truth

Live GitHub issue operations use the ADL typed GitHub transport. Configure one
shared token source for the ADL command environment without printing the token:
`GITHUB_TOKEN`, `GH_TOKEN`, `ADL_GITHUB_TOKEN_FILE`, or
`ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE`. The keychain source uses the macOS
`security find-generic-password` command; when using it, optionally set
`ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT` to disambiguate the keychain item. Do not
fall back to direct `gh` commands or connector-specific issue APIs when the ADL
issue surface needs credentials.

```bash
bash ./adl/tools/pr.sh issue view <issue_num> --json
```

`pr.sh` no longer creates or reconciles GitHub issues. The issue must already exist before kickoff continues.

When you need live queue or backlog inspection before execution, use the same
surface instead of raw `gh`:

```bash
bash ./adl/tools/pr.sh issue list --state open --limit 50 --json
bash ./adl/tools/pr.sh issue search --query "<text>" --state all --json
```

## 3) Confirm Readiness And Bind Run Phase

```bash
bash ./adl/tools/pr.sh doctor <issue_num> --slug <slug> --version <milestone_version> --mode ready --json
bash ./adl/tools/pr.sh run <issue_num> --slug <slug> --version <milestone_version>
```

Use `doctor --mode full --json` when you also need milestone-wave/open-PR
preflight truth before binding execution.

Legacy compatibility card paths:
- `.adl/cards/<issue_num>/input_<issue_num>.md`
- `.adl/cards/<issue_num>/output_<issue_num>.md`

These legacy paths still appear in some older records and tools, but the
canonical issue bundle for current work is the task-bundle directory under
`.adl/<scope>/tasks/`.

Preferred execution clone:
- `.worktrees/adl-wp-<issue_num>`

Structured Card Templates v2 (required sections):
- SIP/SPP card surfaces:
  - `System Invariants (must remain true)`
  - `Reviewer Checklist (machine-readable hints)`
  - `Card Automation Hooks (prompt generation)`
- SOR card surface:
  - `Determinism Evidence`
  - `Security / Privacy Checks`
  - `Replay Artifacts`
  - `Artifact Verification`

These sections are designed to support deterministic replay/security verification and
machine-parsable prompt automation.

## 4) Create The Issue-Bound Session Goal

After the issue is ready and the execution worktree is bound, create a session
goal before implementation starts.

Minimum goal content:

- the tracked issue number
- the concrete session objective

Preferred pattern:

```text
create_goal objective="#<issue_num> <bounded objective>"
```

The repository workflow currently treats this as an agent-session requirement,
not as an ADL runtime-enforced object. `pr.sh` can remind, but it does not
claim to introspect Codex goal state.

When execution is being routed by a Sprint Execution Packet through
`sprint-conductor`, attach this same goal step directly to the child-issue
handoff instead of leaving it as a separate manual reminder. For SEP-routed
child sessions, the goal should include the sprint issue number when present,
the child issue number, and the concrete bounded session objective.

Use `update_goal` only for truthful terminal state:

- `complete` when the current session objective is actually achieved, including
  publication-to-review handoff when opening or updating the PR was the bounded
  session objective
- `blocked` only when the repeated blocking threshold is met and meaningful
  progress cannot continue without user input or an external state change

When issue-local time/token accounting matters, preserve a durable local goal
snapshot before the live goal disappears:

1. save the `get_goal` tool payload to a temporary local path
2. normalize it into the issue task bundle's canonical goal-metrics artifact
   set immediately after the lifecycle event you are recording:

```bash
python3 adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_artifacts.py \
  --goal-state /tmp/issue-<n>-goal-state.json \
  --issue-number <n> \
  --artifacts-dir .adl/<version>/tasks/issue-<n>__<slug>/artifacts/goal_metrics \
  --capture-stage issue_start \
  --issue-goal-ref "goal:<version>:issue:<n>"
```

Record the same artifact set again at later lifecycle checkpoints such as
`pr_publication`:

```bash
python3 adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_artifacts.py \
  --goal-state /tmp/issue-<n>-goal-state-pr-publication.json \
  --issue-number <n> \
  --artifacts-dir .adl/<version>/tasks/issue-<n>__<slug>/artifacts/goal_metrics \
  --capture-stage pr_publication \
  --issue-goal-ref "goal:<version>:issue:<n>"
```

This helper uses canonical stage-specific snapshot filenames under the task
bundle, rewrites the summary in place, and replaces any older row for the same
issue/stage instead of appending duplicates. Use the resulting summary artifact
as the preferred `SOR` source ref when `Goal metrics data source` is
`codex_goal_tool`. If no authoritative snapshot exists, keep the metrics fields
`unknown` rather than fabricating values.

## 5) Implement

Read the active issue cards, stay inside the issue edit fence, and make the tracked repo changes.

After `pr finish` publishes or updates the PR, do not treat that publication as
the natural end of issue work. The next active phase is PR shepherding through
`issue-watcher` for healthy waiting states and through `pr-janitor` only when
concrete blockers need remediation, until settlement allows `pr-closeout` to
run truthfully.

The canonical cross-phase ownership model for that handoff is:

- `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md`

That contract defines the shared issue-lifecycle shepherd states above
`workflow-conductor`, `pr-run`, `pr-finish`, `issue-watcher`, `pr-janitor`,
and `pr-closeout`.

## 6) Run (when the issue requires a bounded runtime proof surface)

```bash
bash ./adl/tools/pr.sh run <adl-file> [run arguments...]
```

Use `pr run` when the issue's proof surface requires emitted run artifacts, replay, or bounded runtime execution.
For docs-only or non-runtime issues, skip `pr run` truthfully and record that in the SOR or report rather than inventing a hidden step.

## 7) Validate

Typical local preflight:

```bash
./adl/tools/batched_checks.sh
```

Canonical regression proof surface for the implemented editing story:

```bash
bash adl/tools/test_five_command_regression_suite.sh
```

Bounded lifecycle proof/demo surfaces should come from the current issue or
milestone packet rather than a legacy one-size-fits-all demo note.

### Compression-Safe Validation

The v0.90 milestone compression pilot distinguishes execution compression from
validation compression.

Low-risk docs/static-tooling issues may use the
`FOCUSED_LOCAL_CI_GATED` profile in
`docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md`
when focused local checks directly prove the changed surface. The output record
must say that full local validation was not run, list the focused commands that
did run, and keep CI required before merge.

`pr finish` chooses this profile from the actual changed tracked paths after
staging, not from the broad operator staging request. For example, a finish
invocation that stages `.` but only changes docs should use the docs-only local
profile rather than paying for a full local Rust cycle that GitHub CI will run
again on the same branch.

Current implementation is intentionally narrower than the general policy: the
focused `pr finish` lane currently applies only to docs-only paths and a
bounded publication-control-plane slice (`pr_cmd`, `finish_support`, inline
finish tests, CI path-policy / coverage-impact scripts, and the matching
workflow/docs surfaces). Docs-only paths may accompany these focused tooling
paths without forcing full local Rust validation, because the selector uses the
actual changed tracked paths after staging and treats docs as non-widening when
the code side is already covered by an explicit focused lane.

Not every focused C-SDLC path uses the same Rust selector. Finish-validation
planner changes such as `adl/src/cli/pr_cmd/finish_support.rs` and
`adl/src/cli/tests/pr_cmd_inline/finish/*` are proved by formatter checks plus
narrow `adl-csdlc` filtered tests for finish validation planning and focused
runner dispatch. Broader lifecycle surfaces such as `doctor`, `git_support`,
`github`, and `lifecycle` still use the broader `cli::pr_cmd` filtered test
until they have their own narrower proof lane.

Public prompt packet tooling is also a focused lane: changes to
`adl/src/cli/tooling_cmd/public_prompt_packet.rs` or its paired
`public_prompt_packet` tests are proved by formatter checks and the
`adl-csdlc public_prompt_packet` filtered test, while merge-context CI remains
required before merge. Other actual changed paths still escalate to full local
validation until more focused lanes are explicitly implemented and tested.

Use full local validation for runtime, schema, security, release, broad tooling,
or ambiguous changes.

For broad non-coverage Rust lanes, prefer `cargo nextest run` over raw
`cargo test` when the lane is executing the whole runnable test graph rather
than a narrow filtered proof. The tracked CI workflow now routes ordinary PR
test execution through `adl/tools/run_pr_fast_test_lane.sh`, which may use a
focused `cargo nextest` expression for bounded changed surfaces and otherwise
fails closed to the full ordinary nextest sweep. Keep `cargo llvm-cov` on its
own coverage lanes, and preserve doc-test signal explicitly with
`cargo test --doc` where needed. Do not add `--all-features` back onto the
ordinary PR lane when opt-in heavy features such as `slow-proof-tests` are
being used to keep proof-materialization surfaces out of routine validation.

### Issue-Class Validation Rule

Before choosing validation, classify the issue using the narrowest truthful
changed-surface class.

Recommended classes:

- `docs-only`
- `milestone-package-truth`
- `workflow-docs`
- `tooling-focused`
- `rust-focused`
- `demo-focused`
- `review-remediation`
- `release-tail`

Default rule:

- docs-heavy classes do not require the full Rust test cycle
- bounded tooling classes use focused shell or unit checks
- runtime or schema classes use targeted Rust validation and widen only when the
  changed surface demands it
- release-tail classes rely on tracker, gap, closeout, review-truth, path, and
  guardrail checks unless tracked code changed

Always record:

- the chosen issue class
- the selected validation profile
- the exact commands run
- what was intentionally not run

Do not use full local validation as a reflex. Use it because the changed
surface needs it.

## 8) Finish PR

```bash
bash ./adl/tools/pr.sh finish <issue_num> \
  --title "<title>" \
  --paths "<comma-separated repo paths>"
```

Finish should only open or update the PR after review truth is captured or
explicitly deferred according to policy. The current publication-time `SOR`
should be synced to the canonical task bundle under:

- `.adl/<scope>/tasks/issue-<padded_issue>__<slug>/sor.md`

Do not treat legacy `.adl/cards/<issue_num>/...` paths as the canonical finish
surface for new issue work.

After publication and terminal issue-session disposition, update the active
goal truthfully before closeout rather than leaving the session without a final
completion or blocked record.

## 9) Report

Write a per-issue report under:

- `.adl/reports/pr-cycle/<issue_num>/<timestamp_utc_z>/report.md`

## Common Pitfalls and Remediations

- Dirty repo-local execution clone:
  - Commit/stash first, then re-run the relevant command from `.worktrees/adl-wp-<issue_num>`.
- Wrong paths at `finish`:
  - Ensure `--paths` only includes intended tracked repo paths; do not include local `.adl` SIP/STP/SPP/SRP/SOR task-bundle artifacts. Use `--output-card` for the SOR truth surface.
- Missing canonical STP:
  - Re-run `pr.sh init <issue_num> --slug <slug> --version <milestone_version>`.
- Stale GitHub issue body:
  - Reconcile the GitHub issue outside `pr.sh`, then re-run `pr.sh init <issue_num>` if the local root bundle must be reseeded.
- Missing card files:
  - Re-run `pr.sh init <issue_num>` to reseed root bundle surfaces, then `pr.sh run <issue_num>` to bind the execution worktree if the issue is entering run phase.
- Browser/editor overclaims:
  - Use `docs/tooling/editor/command_adapter.md` as the truth boundary; do not treat browser/editor entrypoints as the canonical repo-native execution path.
- Worktree branch base problems:
  - Update from `origin/main`, then re-run `run` in the repo-local execution clone.
