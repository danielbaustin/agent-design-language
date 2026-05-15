# SRP, SOR, And ObsMem Handoff Model

## Purpose

This document defines the bounded `v0.91.2` handoff model for review results,
outcome records, closeout truth, and future `ObsMem` ingestion in the ADL card
lifecycle.

The canonical lifecycle remains:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

This document focuses only on the final two lifecycle surfaces:

- `SRP`: Structured Review Prompt.
- `SOR`: Structured Outcome Record.

It does not implement full Cognitive SDLC runtime behavior, automatic ObsMem
ingestion, GitHub review replacement, or merge authority.

## Boundary

`SRP` and `SOR` are linked, but they are not the same artifact.

`SRP` answers:

```text
What should be reviewed, what did review find, how were findings dispositioned,
and what residual risk remains?
```

`SOR` answers:

```text
What changed, what was validated, what is the integration state, and what is
now true after execution, publication, merge or closure, and closeout?
```

Collapsing those questions recreates the lifecycle drift this migration is
fixing. Review cognition and outcome truth should remain separate enough to be
auditable, while still cross-linking each other so a future agent can reconstruct
the whole issue path.

## SRP Responsibility

The `SRP` is the review prompt and review-result surface.

Before review, it should record:

- review scope
- changed surfaces to inspect
- required reviewer posture
- issue-specific invariants
- known risks and non-goals
- acceptance criteria the review must test against

After review, it should record:

- findings, ordered by severity
- non-findings when they clarify important reviewed risks
- dispositions for every finding
- evidence that fixes were applied or intentionally deferred
- residual risk
- recommended publication outcome

The final `SRP` should be fit for future memory ingestion because it preserves
how the work was judged, not merely what files changed.

## SOR Responsibility

The `SOR` is the outcome and integration truth surface.

During publication and closeout, it should record:

- actual changed paths
- validation commands actually run
- validation result and scope
- review evidence used before publication
- PR URL and PR state when a PR exists
- merge, closure, or no-PR disposition
- closeout state
- unresolved follow-ups
- final issue truth

The `SOR` should summarize and link to the `SRP`; it should not absorb all
review cognition. Its job is to record the terminal state of the issue, including
whether the review findings were fixed, deferred, or routed into follow-up work.

## Finish Readiness

`pr finish` readiness should require both review truth and outcome truth.

A finish-ready issue should have:

- `SIP`, `STP`, and `SPP` complete enough to prove the execution path was
  intentional.
- `SRP` updated with review instructions and review results, or an explicit
  policy exception when review is not applicable.
- all blocking review findings dispositioned before publication.
- `SOR` updated through the current publication boundary, usually
  `Integration state: pr_open`.
- validation commands and review scope recorded truthfully.
- no unmentioned worktree-only artifacts required for the PR to be complete.

`pr finish` should not silently convert missing review results into success.
If review was skipped by policy, that exception belongs in the `SRP` and the
`SOR`.

## Closeout Readiness

Closeout readiness begins after the issue is merged, intentionally closed, or
otherwise terminal.

A closeout-ready issue should have:

- live GitHub truth checked for issue and PR state.
- final `SOR` truth updated to `merged`, `closed_no_pr`, or the correct terminal
  state.
- final `SRP` preserved as the review-learning surface.
- worktree-only residue classified and either copied, pruned, or explicitly
  retained with a reason.
- follow-ups routed instead of hidden in closeout prose.
- sprint or umbrella state updated when the issue belongs to an orchestrated
  sprint.

Closeout must not mark an issue complete just because a PR exists. It must
reflect the real terminal state.

## Review-Skill Handoff

Review skills and review subagents should hand their results into the `SRP`.

The expected handoff shape is:

- reviewer scope and evidence reviewed
- confirmed findings
- non-findings when useful
- unresolved questions
- residual risk
- recommended outcome

The issue owner then updates the `SRP` with dispositions:

- fixed before publication
- not applicable with evidence
- deferred to a named follow-up
- accepted residual risk

Blocking findings should remain blocking until dispositioned. Non-blocking
findings can be routed to follow-up work, but the `SRP` should say why they do
not block the current issue.

GitHub PR review remains part of the review ecosystem. This model does not
replace human review, requested-changes workflows, CI checks, or merge policy.

## ObsMem Handoff

`ObsMem` should receive two distinct memory inputs when an issue is complete:

- SRP memory: review cognition, findings, dispositions, residual risks, and
  reviewer judgement.
- SOR memory: changed artifacts, validation, integration state, closeout state,
  and final outcome truth.

Those memories should share stable join keys:

- issue number
- PR number or no-PR disposition
- branch name
- card bundle path
- final changed tracked paths
- closeout timestamp when available

This lets future agents ask different questions without collapsing the record:

- "What did reviewers learn?"
- "What actually changed?"
- "Which findings were fixed?"
- "Which risks were accepted or deferred?"
- "Was the issue merged, closed, or superseded?"

The `v0.91.2` work defines the handoff model only. Runtime ingestion, indexing,
retrieval, and memory ranking remain follow-on work.

## Follow-On Enforcement Routing

The remaining enforcement should be routed rather than left implicit.

Recommended `v0.91.3` work:

- enforce final `SRP` review-result readiness before `pr finish`.
- update doctor output so legacy-compatible `SRP` scaffolds cannot satisfy final
  review readiness without findings, explicit non-findings, or a policy
  exception.
- teach review skills to emit SRP-ready finding and disposition blocks.
- harden `pr finish` so it refuses publication when blocking review findings are
  undispositioned.
- harden closeout so terminal `SOR` truth matches live GitHub issue and PR state.

Recommended `v0.91.4` work:

- implement the first bounded `ObsMem` ingestion path for final `SRP` and `SOR`
  records.
- preserve separate review-learning and outcome-truth memory entries.
- connect sprint closeout artifacts to the same memory handoff model.
- add retrieval fixtures that prove future agents can recover findings,
  dispositions, validation, and terminal issue state.
- connect the handoff model to transition/DAG execution only after the card
  lifecycle is stable.

## Non-Claims

This document does not claim:

- automatic ObsMem ingestion exists today.
- PR review can be skipped.
- CI or human review can be replaced by SRP text.
- merge authority is automated.
- all historical cards already satisfy the final lifecycle model.

It defines the target handoff shape needed before the next Cognitive SDLC
implementation milestones.
