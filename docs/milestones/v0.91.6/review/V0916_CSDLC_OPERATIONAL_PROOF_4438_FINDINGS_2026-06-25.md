# v0.91.6 C-SDLC operational proof findings register for #4438

## Scope

This retained packet records the real operational findings encountered while
executing `#4438` as the serial proof gate for sprint `#4433`.

## Proof intent

`#4438` is meant to prove a full operational C-SDLC path on a fresh bounded
issue, using normal repo-native workflow commands instead of remembered manual
steps.

## Current proof status

- Parent proof issue `#4438` bound successfully through `pr.sh run`.
- A real fresh proof issue was needed because the initial candidate follow-ons
  from `#4434` did not remain suitable:
  - `#4520` was already closed.
  - `#4521` was already closed.
  - `#4522` is still open, but it is already `run_bound` with an existing
    worktree and final review truth, so it does not qualify as a fresh-path
    proof subject.
- Fresh proof issue `#4537` was created from a real workflow failure and
  initialized through `pr.sh init`.
- The full nested proof path has now been completed through fresh-child issue
  publication:
  - `#4537` was executed in a separate truthful execution boundary.
  - focused validation passed in the child worktree.
  - the child fix was independently reviewed and tightened.
  - `pr.sh finish` published the child result as draft PR `#4541`.
- The remaining open follow-on is not proof failure; it is workflow friction:
  nested fresh-issue goal accounting still requires a separate execution
  boundary, which remains routed under `#4538`.

## Findings

### 1. Session identity is not reused automatically for `pr.sh run`

- Observed behavior:
  - A valid active session-ledger claim for `#4438` was created successfully.
  - `pr.sh run 4438 --version v0.91.6` still blocked with a live-claim conflict
    until `CODEX_SESSION_ID` was explicitly exported for the command.
- Why this matters:
  - The workflow is not yet smooth in normal Codex.app use; an operator can
    create the right claim and still be blocked unless they know to pass a
    hidden session-identity environment variable manually.
- Evidence:
  - `pr.sh run` blocked on the claim when `CODEX_SESSION_ID` was absent.
  - The same command succeeded once `CODEX_SESSION_ID` was explicitly set to the
    claimed session id.
- Routed action:
  - Fresh proof issue `#4537` was created specifically for this defect.

### 2. Generic `issue:` claims are not sufficient for C-SDLC execution binding

- Observed behavior:
  - A generic `issue:4438` claim was accepted by `adl session claim`.
  - `pr.sh run` still blocked and suggested the C-SDLC-specific claim shape:
    `csdlc_issue:4438`.
- Why this matters:
  - The operator-visible claim workflow is easy to get wrong unless they know
    the exact resource kind expected by the C-SDLC execution path.
- Evidence:
  - The first binding attempt failed and suggested the exact replacement claim.
- Current posture:
  - This is retained as an operational usability finding under `#4438`. It may
    or may not need its own follow-on once `#4537` is resolved.

### 3. Fresh-issue proof selection is harder than it looks late in the sprint

- Observed behavior:
  - Recent bounded follow-on issues that looked suitable for proof were no
    longer fresh:
    - two were already closed
    - one was already run-bound with a worktree and final review truth
- Why this matters:
  - A sprint-level proof lane that waits too long can lose clean proof
    candidates and end up needing to create a fresh issue on the fly.
- Evidence:
  - `#4520` issue view returned `closed`
  - `#4521` issue view returned `closed`
  - `#4522` doctor returned `lifecycle_state: run_bound`
- Current posture:
  - `#4438` created fresh issue `#4537` to keep the proof honest.

### 4. Nested issue-bound goals are not currently workable in one active thread

- Observed behavior:
  - `#4438` correctly created its own issue-bound goal after binding.
  - After fresh issue `#4537` was created and initialized, `create_goal` for
    `#4537` failed because this thread already has an unfinished active goal for
    `#4438`.
- Why this matters:
  - The parent proof issue requires the fresh proof issue to have its own
    issue-bound goal, but the current goal system does not allow that path in
    one active thread.
- Evidence:
  - `create_goal` returned:
    - `cannot create a new goal because this thread has an unfinished goal; complete the existing goal first`
- Current posture:
  - This is a direct blocker to completing the full nested fresh-issue proof in
    a single thread without a manual shortcut.
  - Routed action:
    - Fresh issue `#4538` was created to track this blocker explicitly.

### 5. Repo-native issue inspection still has noticeable latency in this flow

- Observed behavior:
  - `pr.sh issue view` surfaced repeated cargo-delegate heartbeat events and
    stale-build-lock recovery while selecting a proof candidate.
- Why this matters:
  - Candidate selection is part of the operator experience for a real proof
    path, and long latency increases friction and uncertainty.
- Evidence:
  - The issue-view commands emitted repeated heartbeat lines up to about
    fifty-five seconds before returning the issue payloads.
- Current posture:
  - Retained as proof friction; route separately if it persists outside this
    proof session.

### 6. Repo-native issue creation still does not yield doctor-ready source prompts

- Observed behavior:
  - After `#4438` created fresh follow-on issues `#4537` and `#4538`,
    `pr.sh doctor` failed for both at the source-prompt validation stage.
  - The mirrored issue bodies were missing required STP sections for the
    current validator contract.
- Why this matters:
  - A routed follow-on issue should not require manual prompt-structure repair
    before it can even reach ordinary card-readiness checks.
  - This adds hidden friction precisely where the C-SDLC is supposed to move
    quickly from discovered defect to actionable remediation issue.
- Evidence:
  - `pr.sh doctor 4537 --version v0.91.6 --json` initially failed with missing
    sections including `Required Outcome`, `Deliverables`, `Repo Inputs`,
    `Dependencies`, `Demo Expectations`, `Non-goals`, `Goal`, and
    `Tooling Notes`.
  - `pr.sh doctor 4538 --version v0.91.6 --json` failed with the same prompt
    structure defects.
- Current posture:
  - The source prompt files were manually normalized in this proof session so
    doctor could continue.
  - Retain this as a distinct workflow defect: issue creation/bootstrap is not
    yet producing doctor-ready prompt structure for this path.

### 7. Fresh follow-on issues still require explicit SPP/VPP budget readiness before run

- Observed behavior:
  - After source prompt repair, both `#4537` and `#4538` advanced to ordinary
    doctor readiness, but each remained blocked at `SPP` and `VPP`.
- Why this matters:
  - The proof path is now truthfully showing that even a cleanly routed
    follow-on issue still requires explicit elapsed-seconds / token estimates
    and validation budget fields before execution binding.
  - This is not necessarily a bug, but it is a real operator cost that should
    be visible in the operational proof.
- Evidence:
  - `pr.sh doctor 4537 --version v0.91.6 --json` reported:
    - `SPP must record explicit elapsed-seconds and total-token estimates before execution binding.`
    - `VPP must record explicit validation seconds and token budgets before execution binding.`
  - `pr.sh doctor 4538 --version v0.91.6 --json` reported the same readiness
    requirements.
- Current posture:
  - This is the next honest preparation step for both routed follow-on issues.
  - Treat it as visible operational cost rather than hidden delay or silent
    workaround.

## Fresh proof issue selected

- Issue: `#4537`
- URL: `https://github.com/danielbaustin/agent-design-language/issues/4537`
- Title:
  - `[v0.91.6][tools][sessions] Make issue execution reuse the current Codex session identity without manual CODEX_SESSION_ID export`
- Selection reason:
  - It is a real, bounded tooling defect discovered during `#4438`.
  - It directly improves workflow speed and reliability for future C-SDLC issue
    execution.
  - It is small enough to serve as an honest fresh-path proof subject.

## Routed follow-on issues

- `#4537`
  - `[v0.91.6][tools][sessions] Make issue execution reuse the current Codex session identity without manual CODEX_SESSION_ID export`
- `#4538`
  - `[v0.91.6][tools][goals] Allow nested fresh-issue goal accounting during operational proof and sprint execution`

## Fresh proof issue readiness update

- `#4537` status after source-prompt and planning-card repair:
  - `pr.sh doctor 4537 --version v0.91.6 --json`
  - `ready_status: PASS`
  - `preflight_block_kind: session_claim_required`
- What changed:
  - Source-prompt structure was normalized until doctor accepted the current
    STP contract.
  - `SPP` and `VPP` were updated with explicit estimate and validation-budget
    fields so card-run readiness no longer blocks execution.
- Why this matters:
  - The fresh proof issue is no longer stalled on prompt/card hygiene.
  - The next blocker is now the normal execution handoff boundary: session
    claim, worktree bind, and issue-bound goal in a suitable execution thread.

## Fresh proof issue execution boundary now established

- Observed state:
  - `#4537` now has its own active session-ledger claim in a separate Codex
    agent/session context.
  - `pr.sh doctor 4537 --version v0.91.6 --json` now reports:
    - `lifecycle_state: run_bound`
    - `worktree: .worktrees/adl-wp-4537`
    - `preflight_block_kind: session_active_conflict`
      from the umbrella thread's point of view because the child issue is
      correctly owned by another live session.
- Why this matters:
  - This proves that the operational path can cross into a fresh child issue
    with separate ownership instead of collapsing back into the umbrella
    thread's active goal.
  - The remaining work is now ordinary child implementation/review/publication
    work, not hypothetical routing setup.
- Live implementation state:
  - Active child diff currently touches:
    - `adl/src/cli/pr_cmd.rs`
    - `adl/src/cli/pr_cmd/doctor/preflight.rs`
    - `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs`
    - `adl/src/session_ledger.rs`
  - Focused proving validation now passes in the child worktree:
    - `cargo test --manifest-path adl/Cargo.toml real_pr_start_accepts_codex_thread_id_as_self_claim_identity -- --nocapture`
  - The proving test establishes that `real_pr start` accepts the current
    Codex thread identity as the self-claim session identity when
    `CODEX_SESSION_ID` is unset.
  - Independent bounded review surfaced a mixed-identity regression risk when a
    claim is created under `CODEX_THREAD_ID` and later resumed in an
    environment where `CODEX_SESSION_ID` is also present.
  - The child fix was then tightened so session-ledger self-claim assessment
    treats thread and session IDs as aliases of the same current Codex
    execution context.
  - Additional focused validation now passes:
    - `cargo test --manifest-path adl/Cargo.toml target_claim_assessment_treats_thread_and_session_ids_as_same_current_owner -- --nocapture`
    - `cargo test --manifest-path adl/Cargo.toml --bin adl real_pr_start_accepts_codex_thread_id_as_self_claim_identity -- --nocapture`
- Publication result:
  - The stale child-session claim was released back to the parent thread.
  - A fresh publication ownership claim was created for `#4537`.
  - `pr.sh finish` then published the child result as draft PR `#4541`:
    - `https://github.com/danielbaustin/agent-design-language/pull/4541`
  - The local URL opener failed after publication, but publication itself
    succeeded, so this is non-blocking finish-tail friction rather than proof
    failure.

## Current blocker summary

The `#4438` proof did complete the full nested fresh-issue path to child PR
publication, but it required a separate truthful execution boundary for
`#4537`. The still-open operational gap is therefore narrower:

- one thread cannot currently keep the parent issue goal active and also create
  a fresh child issue goal without routing that child into another execution
  boundary.
- that friction remains explicitly routed to `#4538`.

## Next intended step

The next steps are no longer hypothetical fresh-issue execution steps. They are
the ordinary review/publication tails:

1. shepherd `#4541` through review, checks, merge, and closeout
2. preserve `#4538` as the routed follow-on for nested goal accounting
3. publish `#4438` itself with this retained packet and truthful umbrella SOR
4. update `#4433` sprint truth once the umbrella packet lands
