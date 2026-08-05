# Developer Throughput Fast Lane

ADL small fixes should move quickly without pretending that a narrow check proves
a broad change. This policy defines a proportional fast lane for tiny,
low-risk work while preserving typed C-SDLC v2, exact-head review, and truthful
closeout.

The validation selector remains the source for lane classification:
`docs/architecture/VALIDATION_LANE_SELECTOR.md`.

## Proportional issue classes

Use the smallest class that truthfully covers the changed surface.

| Class | Typical scope | Local proof floor | Escalate when |
| --- | --- | --- | --- |
| `tiny-docs` | one policy, README, handoff, or routing doc | diff hygiene plus a focused text/link contract | the doc changes release truth, review gates, security claims, or milestone status |
| `tiny-tooling` | one shell/Rust tool contract or help-path fix | focused tool contract or owner lane for that tool | shared workflow control, publication, closeout, or provider behavior changes |
| `narrow-test` | one deterministic test or fixture classification | the changed test plus selector proof | fixture routing affects release gates, coverage, or slow/fast partitioning |
| `runtime-product` | runtime, provider, security, or user-visible behavior | issue-specific integrated proof | never use the fast lane as completion proof |
| `release-control` | CI, merge, shepherd, closeout, or milestone gates | focused contract plus exact-head lifecycle review | any ambiguity; default to escalation |

Fast lane eligibility requires all of the following:

- the changed paths are explicitly protected by the issue claim
- the selector or issue VPP names a focused proof lane
- the proof is deterministic, cheap, and directly tied to the acceptance
  criteria
- exact-head review can inspect the full behavioral surface
- no runtime, provider, security, release, or cross-issue dependency claim is
  widened

## FastWork-required mode

When the operator says to use FastWork, all worktree, temp, cache, and Rust build
output must stay on the declared external build root. Do not silently fall back to the local disk.

Required behavior:

- verify the FastWork mount before starting implementation
- set `TMPDIR` and `CARGO_TARGET_DIR` to FastWork for Rust validation or typed
  binary builds
- stop and report the mount blocker when FastWork is unavailable
- record any tooling problem as issue-local evidence instead of working around
  it invisibly

## Validation Selection

The fast lane changes how much proof is selected, not who owns lifecycle truth.
The typed C-SDLC v2 remains the lifecycle authority.

For fast-lane issues:

1. Run the selector or issue-local VPP lane that matches the changed paths.
2. Run one focused contract test that directly proves the policy or tool
   invariant being changed.
3. Run `git diff --check`.
4. Record what was run and what was deliberately deferred in SOR truth.

Do not use a fast-lane result to claim completion for:

- runtime/product behavior
- live provider behavior
- release readiness
- broad coverage health
- external review remediation
- milestone closeout

## changed-state-only PR watching

PR watchers and shepherds should report only changed state, blockers, or
operator-actionable transitions. Repeating unchanged pending status burns
operator attention and model budget without moving the work.

Changed-state-only PR watching means:

- report new check failures, conflicts, draft/ready transitions, review
  requests, mergeability changes, and exact-head SHA changes
- stay quiet for unchanged pending checks
- stop failed runs promptly when the operator has authorized cancellation
- hand off blockers to the right owner instead of narrating a wait loop

Do not wait on GitHub when no action is possible. If checks are pending and no
new information exists, preserve the watcher state and work on the next
independent issue or return control to the operator.

## Escalation And Stop Conditions

Leave the fast lane and escalate when any of these appears:

- touched paths include shared Rust, provider, runtime, security, publication,
  closeout, or release-gate code
- selector output is `escalated` or `release_gate_required`
- the issue claim does not cover a touched path
- review finds an unresolved behavioral risk
- local proof is fixture-only for an integrated feature
- FastWork or the declared build root is unavailable when required
- GitHub truth contradicts local lifecycle state

Escalation should be explicit: update the issue plan or route a follow-on issue
rather than letting a tiny fix become an unbounded repair session.

## Non-Claims

This policy does not:

- bypass typed C-SDLC v2
- remove exact-head review before publication
- make docs-only proof sufficient for runtime/product work
- authorize raw GitHub lifecycle operations
- authorize AWS or other paid infrastructure
- make local green checks a substitute for required PR checks
- permit local disk fallback when FastWork-required mode is active

The goal is less waiting and less unnecessary ceremony for small fixes, while
keeping every completion claim evidence-bound.
