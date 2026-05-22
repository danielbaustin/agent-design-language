# Design-Time Card Completion Plan

## Status

Planning document for issue `#3264`.

This plan institutionalizes the rule that milestone issue waves should not be
treated as ready merely because card files exist. Before execution starts,
`SIP`, `STP`, and `SPP` should already be detailed, issue-specific,
reviewable, and useful.

This plan does not generate v0.91.4 cards. It defines the contract WP-17 should
use when preparing the v0.91.4 core issue wave, sprint umbrellas, and
CodeFriend sidecar.

## Core Decision

ADL issue creation should become a design-readiness step.

When a new issue, sprint umbrella, or sidecar issue is created from a good
source issue prompt, the issue bundle should be prepared as follows:

- `SIP`: complete design-time issue intent.
- `STP`: complete design-time selected task framing.
- `SPP`: complete design-time operative execution plan.
- `SRP`: complete review prompt, with review results left open until review.
- `SOR`: scaffold only before execution, then truthful execution/output record
  during run, finish, merge or closure, and closeout.

The goal is not to pretend execution already happened. The goal is to make the
next agent start from reviewed intent, selected task, and operative plan rather
than from generic bootstrap text.

## Why This Matters

ADL has repeatedly lost time when issue bundles were structurally present but
not semantically ready. Generic cards create several failure modes:

- agents infer scope from memory or chat instead of tracked issue truth
- execution starts before dependencies, validation, and stop conditions are
  clear
- `SPP` becomes a vague planning memo instead of an operative issue-local plan
- `SRP` review prompts are too generic to produce useful findings
- `SOR` cards overclaim or remain stale because the expected proof surface was
  never clear

Design-time card completion moves more cognition into tracked, inspectable
state. That supports C-SDLC, ObsMem, sprint resumption, and cross-session
handoff.

## Card Completion Contract

### SIP: Complete Before Execution

The `SIP` should answer:

- what issue is being addressed
- why it matters
- what sources bound the work
- what dependencies must be true before execution
- what outcome is required
- what acceptance criteria prove completion
- what is explicitly out of scope
- what validation and demo/proof expectations apply
- what workflow invariants must be preserved

For milestone waves, `SIP` should be concrete enough that a future session can
read it without chat history and understand the issue boundary.

### STP: Complete Before Execution

The `STP` should answer:

- what task or solution is selected
- what deliverables are expected
- what docs, code, demos, review packets, cards, or records may be touched
- which invariants cannot be broken
- what the post-change state should look like
- why this issue is a bounded slice rather than adjacent work

For docs/review/release-tail WPs, the `STP` should make clear whether the work
is a review, a remediation, a planning pass, a quality gate, or a ceremony.

### SPP: Complete Before Execution

The `SPP` is the tracked equivalent of `/plan` for one issue or transition.
It should be complete before execution begins.

The `SPP` should answer:

- what step is first
- what ordered steps follow
- what proof or validation is required before moving on
- what artifacts should be produced
- what review must happen before PR publication
- what conditions force a stop or replan
- what is explicitly out of bounds
- how dependency changes should be handled

If the real execution sequence changes materially, the agent must update the
tracked `SPP` before continuing and record why.

`SPP` remains issue-local. Sprint orchestration belongs to sprint-conductor
state and sprint closeout artifacts, not to a child issue `SPP`.

### SRP: Prompt Complete Before Review, Results After Review

The `SRP` is the Structured Review Prompt. It should be prepared before review,
but it must not invent review results.

Before review, `SRP` should contain:

- review scope
- review instructions
- evidence policy
- acceptance and refusal rules
- validation inputs
- allowed finding dispositions
- residual-risk expectations
- non-claims

After review, `SRP` should be updated with:

- findings
- dispositions
- reviewer notes
- residual risks
- recommended outcome

### SOR: Scaffold Before Execution, Truth After Execution

The `SOR` should not be completed at design time.

Before execution, it may exist as a scaffold for path stability. During and
after execution, it becomes the outcome record:

- actual changed paths
- actual validation run
- actual review state
- PR publication state
- merge or closure truth
- closeout state
- residual follow-ups
- final issue truth

`SOR` must not claim work, validation, review, PR state, merge state, or
closeout that has not happened.

## Milestone Wave Preparation Rule

WP-01 of a milestone should seed issues and prepare the full design-time card
set for the wave.

For each core WP, sprint umbrella, and approved sidecar issue:

1. Create the GitHub issue through the normal conductor / `pr init` path.
2. Generate a complete source issue prompt.
3. Generate `SIP`, `STP`, and `SPP` with issue-specific content.
4. Generate `SRP` as a complete review prompt with results left open.
5. Generate `SOR` as a truthful pre-execution scaffold only.
6. Validate the structured prompt contracts.
7. Review the bundle before the issue is allowed to execute.
8. Fix card findings with editor skills only.

The wave is not execution-ready if `SIP`, `STP`, or `SPP` still read like
generic bootstrap text.

## v0.91.4 WP-17 Card Preparation Checklist

When WP-17 prepares v0.91.4 cards, it should cover:

- all core v0.91.4 WPs in `docs/milestones/v0.91.4/WP_ISSUE_WAVE_v0.91.4.yaml`
- all four v0.91.4 sprint umbrellas
- the CodeFriend pre-alpha sidecar umbrella
- the four CodeFriend sidecar child issues

For every prepared issue bundle:

- `SIP` names the real issue, dependencies, acceptance criteria, and non-goals
- `STP` names the selected task, expected deliverables, and proof shape
- `SPP` contains a concrete ordered plan, proof gates, validation plan, stop
  conditions, and replan triggers
- `SRP` contains a useful review prompt and explicitly says review results are
  not run yet
- `SOR` is a truthful pre-execution scaffold and does not claim work has begun
- card paths validate mechanically
- issue text and local cards agree
- dependencies match the issue wave and sprint plan
- CodeFriend sidecar cards are complete but clearly marked as sidecar product
  setup, not C-SDLC core proof

WP-17 should not simply open issues and leave their cards generic. Its exit bar
is design-time reviewability for the full v0.91.4 execution set.

## Review Standard

Design-time card review should ask:

- Can a future session execute this issue without rereading chat history?
- Does the issue boundary prevent scope drift?
- Does `SPP` tell the agent what to do first, next, and when to stop?
- Are proof gates and validation commands specific enough to avoid guessing?
- Are `SRP` review instructions useful enough for a real reviewer?
- Does `SOR` avoid pre-claiming execution truth?
- Do cards agree with GitHub issue text, WBS, sprint plan, and issue wave?

If the answer is no, the issue is not ready to execute.

## Tooling Implications

This plan can be implemented incrementally.

Near-term human/process changes:

- WP-01/WP-17 agents must treat design-time card completion as a deliverable.
- Reviewers should flag generic `SIP`, `STP`, or `SPP` cards as readiness
  findings.
- Sprint-conductor setup should check child issue cards before execution.

Required tooling changes before relying on this process:

- `pr create` / `pr init` should generate richer `SIP`, `STP`, and `SPP`
  content from the source issue prompt.
- `pr doctor` should distinguish generic bootstrap text from design-time
  complete cards.
- validator or doctor output should expose a `design_time_complete` status for
  `SIP`, `STP`, and `SPP`.
- `workflow-conductor` should route generic or incomplete design-time cards to
  the matching editor skill before allowing execution or publication to
  continue.
- `sprint-conductor` should run a sprint-wide design-time card preflight before
  the first child starts, and it should stop or repair through editor skills
  when `SIP`, `STP`, or `SPP` remain generic.
- `sip-editor`, `stp-editor`, and `spp-editor` should know the
  design-time-complete bar for their own card types, not only mechanical
  formatting and branch truth.
- `srp-editor` should preserve the distinction between a complete review prompt
  and review results that have not run yet.
- `sor-editor` should preserve the distinction between pre-execution scaffold
  and post-execution output truth.
- sprint setup should fail or warn when a child issue has generic design-time
  cards.

These tooling changes should be routed as separate implementation issues, but
they are not optional. The design-time card-completion rule should not become a
trusted v0.91.3/v0.91.4 operating practice until the conductor, sprint
conductor, and editor/tooling surfaces enforce or at least explicitly report the
new readiness bar.

## Non-Goals

This plan does not:

- generate the v0.91.4 card wave
- edit tooling implementation
- change the `SIP -> STP -> SPP -> SRP -> SOR` lifecycle
- make `SRP` review results available before review
- make `SOR` outcome truth available before execution
- replace conductor routing, editor skills, worktrees, PR review, or closeout

## Success Criteria

The process is institutionalized when:

- new milestone waves contain useful design-time `SIP`, `STP`, and `SPP` cards
- reviewers consistently reject generic bootstrap cards before execution
- future sessions can resume issue work from tracked cards rather than chat
  memory
- `SRP` and `SOR` remain truthful runtime/review/output surfaces
- v0.91.4 opens with all core and sidecar cards prepared to this standard
