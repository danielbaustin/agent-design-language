# v0.91.6 Standard Closeout-Tail Sprint

## Purpose

The milestone closeout tail is one ordered sprint, not a loose collection of independent mini-sprints. Each work package remains its own issue with its own cards, worktree, review, PR, and closeout, but the sprint-level sequence and wait-state handling are standardized.

## Standard ordered issue wave

| Order | Issue | Role | Must wait for | May spawn follow-on remediation | Watcher expectation |
| --- | --- | --- | --- | --- | --- |
| 1 | `#3976` | Demo convergence | prior milestone implementation wave complete enough for demos | No | Watch demo proofs and PR checks; poll at most every 30 seconds while waiting |
| 2 | `#3977` | Quality gate | demo truth available | Yes, if gate finds bounded gaps | Watch findings, truth drift, and PR checks; poll at most every 30 seconds while waiting |
| 3 | `#3978` | Docs and review alignment | quality-gate truth available | Yes, for stale or contradictory docs surfaces | Watch doc-alignment PR state and downstream truth handoff; poll at most every 30 seconds while waiting |
| 4 | `#3979` | Internal review | docs/review alignment packet ready | Yes, for findings remediation routed into preflight | Watch review packet readiness, review completion, and linked remediation state; poll at most every 30 seconds while waiting |
| 5 | `#3980` | External review | internal review packet ready | Yes, for external findings remediation routed into preflight | Watch external-review packet readiness and returned findings state; poll at most every 30 seconds while waiting |
| 6 | `#3981` | Remediation and final preflight | internal and external review truth settled enough to act | Yes; this is the canonical remediation sink for closeout-tail findings | Watch blockers, PR checks, and release-gate proof; poll at most every 30 seconds while waiting |
| 7 | `#3982` | Next milestone planning | remediation/preflight truth stable enough to size next work | No; fold recurring planning/handoff concerns here, including work previously split into `#3893` | Watch planning-doc readiness and approval state; poll at most every 30 seconds while waiting |
| 8 | `#3983` | Next milestone review | next milestone plan drafted | Yes, but only for bounded planning-quality follow-ons | Watch planning-review findings and approval state; poll at most every 30 seconds while waiting |
| 9 | `#3984` | Release ceremony | remediation/preflight complete and next milestone planning/review no longer blocking release | No | Watch final release evidence, publication state, and release issue closure; poll at most every 30 seconds while waiting |

## Deterministic sequencing rules

- Treat the closeout tail as one sprint umbrella with ordered child issues.
- Do not run these issues as separate sprint umbrellas.
- Each issue still uses the normal ADL lifecycle: GitHub issue truth, `workflow-conductor`, bound worktree execution, bounded review, PR publication, and closeout.
- Do not advance to the next issue merely because a prior issue has a branch or draft PR; advance only when the prior issue's required truth surface is available.
- If an issue is blocked on review, checks, conflicts, or an upstream gate, assign a watcher immediately rather than leaving the sprint in an unmanaged wait state.

## Watcher policy

- Every closeout-tail issue must have a watcher path whenever work is waiting on CI, review, mergeability, upstream truth, or dependent issue completion.
- Watchers are janitor and status-routing helpers, not silent implementers.
- A watcher should check its target at most every 30 seconds while the issue is actively blocked and should stop once the target is healthy, merged, or rerouted to a human decision.
- Watchers should record the blocker class explicitly: review wait, checks failing, merge conflict, upstream dependency wait, or operator decision required.

## Remediation routing rules

- Internal-review, external-review, and quality-gate findings should not be hidden inside unrelated closeout-tail issues.
- Route substantive review findings into `#3981` when they belong to final milestone remediation and preflight.
- Route planning-only findings into `#3982` or `#3983` when they affect the next milestone rather than the current release.
- If a finding is out of scope for the closeout tail, open or defer it explicitly rather than leaving it as undocumented residue.

## Closeout expectations per issue

- Each closeout-tail issue must stop at truthful PR publication and bounded subagent review; do not self-merge or silently self-close unless the operator explicitly requested that authority.
- After merge or intentional closure, run normal issue closeout so `SIP`, `STP`, `SPP`, `SRP`, `SOR`, and GitHub truth agree.
- The sprint umbrella should remain open until the ordered closeout-tail issue wave is actually complete.

## Automation guidance for future milestones

- Seed the closeout-tail issue wave from the standard milestone ordering template rather than authoring the tail from scratch.
- Generate the standard closeout-tail child issues in order for every role that is required in the milestone-specific plan, preserving the milestone-specific version label and milestone-specific titles.
- Pre-wire watcher expectations into the issue body or task bundle so wait states are never unmanaged.
- Pre-classify likely remediation sinks so internal-review, external-review, and quality-gate findings can route deterministically.
- Treat next-milestone planning and review as part of the current milestone closeout tail, not as free-floating backlog work.

## v0.91.6 adoption note

`v0.91.6` should use this file as the closeout-tail sprint truth surface. Future milestones should either reuse the standard pattern directly or intentionally version it when the ordered closeout sequence changes.
