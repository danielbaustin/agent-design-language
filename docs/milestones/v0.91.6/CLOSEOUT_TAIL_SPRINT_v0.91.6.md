# v0.91.6 Standard Closeout-Tail Sprint

## Purpose

The milestone closeout tail is one ordered sprint, not a loose collection of independent mini-sprints. Each work package remains its own issue with its own cards, worktree, review, PR, and closeout, but the sprint-level sequence and wait-state handling are standardized.

## Rescue Gate

The control-plane rescue sprint is complete and the ordered closeout-tail wave
has resumed under sprint umbrella `#4604`.

Retained rescue evidence lives in
[CONTROL_PLANE_RESCUE_SPRINT_v0.91.6.md](CONTROL_PLANE_RESCUE_SPRINT_v0.91.6.md).
Any incomplete child, blocked follow-on, or retained watcher residue requires
an explicit operator waiver in the rescue sprint packet with release-tail
impact, retained watcher evidence, and follow-on owner.

The rescue gate exists because watcher, validation, SRP/SOR, closeout, and
remote-build relief must be operational in the actual release-tail path, not
merely implemented as isolated components.

## Standard ordered issue wave

Active sprint umbrella: `#4604`.

Current progress: `#3976` WP-11, `#3977` WP-12, and `#3978` WP-13 are merged
and closed. `#4582` WP-14A is the current active internal-review owner, while
release-tail documentation truth cleanup remains explicitly routed through
follow-on issue `#4609`.

| Order | Issue | Role | Must wait for | May spawn follow-on remediation | Watcher expectation |
| --- | --- | --- | --- | --- | --- |
| 1 | `#3976` | Demo convergence | prior milestone implementation wave complete enough for demos | No | Watch demo proofs and PR checks; poll at most every 30 seconds while waiting |
| 2 | `#3977` | Quality gate | demo truth available | Yes, if gate finds bounded gaps | Watch findings, truth drift, and PR checks; poll at most every 30 seconds while waiting |
| 3 | `#3978` | Docs and review alignment | quality-gate truth available | Yes, for stale or contradictory docs surfaces | Watch doc-alignment PR state and downstream truth handoff; poll at most every 30 seconds while waiting |
| 4 | `#4582` | WP-14A internal review and pre-`v0.92` burn-down | docs/review alignment packet ready; closed `#3979` consumed as retained source evidence only | Yes, for findings remediation routed into preflight | Watch review packet readiness, review completion, burn-down checklist publication, and linked remediation state; poll at most every 30 seconds while waiting |
| 5 | `#3980` | External review | internal review packet ready | Yes, for external findings remediation routed into preflight | Watch external-review packet readiness and returned findings state; poll at most every 30 seconds while waiting |
| 6 | `#3981` | Remediation and final preflight | internal and external review truth settled enough to act | Yes; this is the canonical remediation sink for closeout-tail findings | Watch blockers, PR checks, and release-gate proof; poll at most every 30 seconds while waiting |
| 7 | `#3982` | Next milestone planning | remediation/preflight truth stable enough to size next work | No; fold recurring planning/handoff concerns here, including work previously split into `#3893` | Watch planning-doc readiness and approval state; poll at most every 30 seconds while waiting |
| 8 | `#3983` | Next milestone review | next milestone plan drafted | Yes, but only for bounded planning-quality follow-ons | Watch planning-review findings and approval state; poll at most every 30 seconds while waiting |
| 9 | `#3984` | Release ceremony | remediation/preflight complete and next milestone planning/review no longer blocking release | No | Watch final release evidence, publication state, and release issue closure; poll at most every 30 seconds while waiting |

Closed source packet:

- `#3979` is closed. It remains retained internal-review planning/source
  evidence, but it is no longer the active internal-review execution owner.
  Use `#4582` for the active WP-14A execution step.

## Deterministic sequencing rules

- Treat the closeout tail as one sprint umbrella with ordered child issues.
- Do not run these issues as separate sprint umbrellas.
- Treat `#3979` as closed retained planning/source evidence only. Active
  internal-review execution for this tail is `#4582` / WP-14A.
- Each issue still uses the normal ADL lifecycle: GitHub issue truth, `workflow-conductor`, bound worktree execution, bounded review, PR publication, and closeout.
- Do not advance to the next issue merely because a prior issue has a branch or draft PR; advance only when the prior issue's required truth surface is available.
- If an issue is blocked on review, checks, conflicts, or an upstream gate, assign a watcher immediately rather than leaving the sprint in an unmanaged wait state.
- Use `docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md` as the
  operator-facing rescue-sprint workflow contract while completing this tail.
- Workflow-critical command paths should be binary-first per `#4590`. A
  closeout-tail issue should not rely on hidden `cargo run` behavior unless its
  validation plan explicitly opts into that fallback.

## Watcher policy

- Before a release-tail issue enters a long CI wait, run the cheapest
  applicable local guardrail first and record it in the issue SOR or sprint
  packet. Examples include `git diff --check`, docs/path hygiene for docs-only
  changes, and `cargo fmt --all -- --check` for Rust changes. A release-tail
  PR should not spend a long CI window discovering a deterministic formatting
  or whitespace failure that the focused local preflight could have caught.
- Every closeout-tail issue must have a watcher path whenever work is waiting on CI, review, mergeability, upstream truth, or dependent issue completion.
- Every closeout-tail wait state must retain one of these evidence records in
  the issue SOR, SRP, closeout artifact, or sprint execution packet before the
  issue can truthfully advance:
  - a repo-native `pr.sh watch <issue-or-pr> --json` packet;
  - a summary that names the retained watcher packet path and disposition; or
  - an explicit not-applicable reason for issues that never entered a wait
    state.
- After the rescue gate, every real wait state must have a retained or
  summarized `adl/tools/pr.sh watch <issue> --json` packet, or a one-line
  not-applicable reason.
- Required watcher routing must use the packet's top-level
  `classification`, `tail_owner`, and `next_skill` fields as the authority.
  Nested fields such as `linked_pr.validation.disposition` are supporting
  check evidence, not the lifecycle routing key.
- Current watcher classifications route as follows:
  - `pr_open` or `checks_running`: keep the issue in watcher-owned
    `pr_waiting` state with `next_skill: issue-watcher`;
  - `checks_failed`, review/action blockers, or merge-conflict blockers: route
    to `pr-janitor` and preserve the watcher packet as janitor input;
  - `checks_green_but_draft`: route to `pr-janitor` because the draft-state
    transition is an actionable PR-tail task;
  - `checks_green`: preserve the `next_skill: human_review` handoff and the
    merge-authority boundary before claiming completion;
  - `merged_pending_closeout` or `closeout_needed`: route to `pr-closeout`;
  - `closed`: record the no-PR or already-settled rationale before advancing;
  - `ready_for_run` or `blocked`: treat as pre-publication readiness truth and
    route to the packet's declared `next_skill`.
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
- If the issue had any wait state, closeout must reference the retained watcher
  packet or the explicit not-applicable reason. A closeout that says the issue
  is complete while omitting watcher evidence for a known wait state is not
  release-tail clean.
- After merge or intentional closure, run normal issue closeout so `SIP`, `STP`, `SPP`, `SRP`, `SOR`, and GitHub truth agree.
- The sprint umbrella should remain open until the ordered closeout-tail issue wave is actually complete.

## Automation guidance for future milestones

- Seed the closeout-tail issue wave from the standard milestone ordering template rather than authoring the tail from scratch.
- Generate the standard closeout-tail child issues in order for every role that is required in the milestone-specific plan, preserving the milestone-specific version label and milestone-specific titles.
- Pre-wire watcher expectations into the issue body or task bundle so wait states are never unmanaged.
- Pre-classify likely remediation sinks so internal-review, external-review, and quality-gate findings can route deterministically.
- Treat next-milestone planning and review as part of the current milestone closeout tail, not as free-floating backlog work.
- Pre-wire prep-scout handoffs only as read-only readiness lanes. Promotion
  from prep scout to execution still requires a normal session claim, `pr.sh
  run`, issue-bound goal creation, and bound-worktree edits.
- State scheduler claims narrowly. In v0.91.6 the scheduler is advisory
  planning/evidence infrastructure, not autonomous sprint conduction, GitHub
  mutation, provider authority, or timed execution.

## v0.91.6 adoption note

`v0.91.6` should use this file as the closeout-tail sprint truth surface.
Future milestones should either reuse the standard pattern directly or
intentionally version it when the ordered closeout sequence changes.
