# Tokio Runtime Substrate

## Metadata

- Feature Name: Tokio Runtime Substrate
- Milestone Target: `v0.91.6`
- Status: closeout_in_progress_on_issue_4183
- Owner: ADL maintainers
- Doc Role: primary closeout packet for the Tokio prerequisite wave
- Feature Types: runtime, architecture, closeout
- Proof Modes: focused Rust tests, runtime compatibility proof, issue/PR closeout evidence

## Purpose

Record the bounded closeout truth for the Tokio prerequisite wave before ACIP 2
consumes runtime substrate work. This packet consolidates what landed in
`#4178` through `#4182`, what ACIP runtime work in `#4160` may consume, what
bounded CAV work may consume, and what still remains explicitly routed.

This packet does not claim ACIP runtime completion, integrated runtime soak
completion, or an always-on autonomous red/blue runtime.

## Source Evidence

- `.adl/plans/tokio-runtime-integration-and-scheduler-refactor.md`
- `.adl/v0.91.6/tasks/issue-4178__v0-91-6-runtime-tokio-t-00-expand-tokio-runtime-feature-baseline/sor.md`
- `.adl/v0.91.6/tasks/issue-4179__v0-91-6-runtime-tokio-t-01-migrate-long-lived-agent-cadence-to-tokio-timers-and-tasks/sor.md`
- `.adl/v0.91.6/tasks/issue-4180__v0-91-6-runtime-tokio-t-02-consolidate-runtime-bootstrap-and-supervision-boundaries/sor.md`
- `.adl/v0.91.6/tasks/issue-4181__v0-91-6-runtime-tokio-t-03-prepare-the-acip-facing-tokio-runtime-substrate/sor.md`
- `.adl/v0.91.6/tasks/issue-4182__v0-91-6-runtime-tokio-t-04-define-bounded-cav-cadence-integration-on-the-shared-runtime/sor.md`
- `.adl/v0.91.6/bodies/issue-4183-v0-91-6-runtime-tokio-t-05-complete-tokio-runtime-integration-closeout-proof.md`
- `.adl/v0.91.6/bodies/issue-4185-v0-91-6-runtime-plan-integrated-runtime-soak-sprint.md`
- merged PR `#4211` / merge commit `db60287beef54084c4897757e171db1f0fe1be7f`
- merged PR `#4222` / merge commit `5acc89ea581800894b2a3788fd5c746fd83d5aa4`
- live issue state for `#4177`, `#4178`, `#4179`, `#4180`, `#4181`, `#4182`, `#4183`, `#4185`, and `#4160`
- code surfaces:
  - `adl/Cargo.toml`
  - `adl/src/long_lived_agent.rs`
  - `adl/src/cli/tokio_runtime.rs`
  - `adl/src/execute/runner.rs`
  - `adl/src/remote_exec/types.rs`
  - `adl/src/continuous_verification_self_attack.rs`
  - `adl/tools/test_adl_runtime_compatibility.sh`

## Closeout Posture

Current bounded posture for the Tokio prerequisite wave as authored on
June 19, 2026:

- `#4178` through `#4182` are closed child issues with landed runtime substrate
  changes on `main`
- the wave widened Tokio into the agreed shared-runtime floor and moved
  long-lived cadence to Tokio-backed timing/task execution
- runtime bootstrap duplication is reduced through one shared CLI helper
- ACIP-facing runtime surfaces now have a clearer shared-runtime substrate to
  consume without silently widening remote-exec into orchestration ownership
- bounded CAV cadence is now explicitly routed onto the shared Tokio runtime as
  future scheduled or `continuous_bounded` work rather than a parallel security
  daemon
- one bounded visible runtime proof surface exists today: the runtime
  compatibility path in `adl/tools/test_adl_runtime_compatibility.sh`
- integrated runtime soak and broader runtime coherence claims remain
  explicitly routed to later work

## Live Child-State Snapshot

| Surface | Live state at closeout authoring | Meaning for the wave |
| --- | --- | --- |
| `#4177` Tokio runtime prerequisite umbrella | `open` | Umbrella remains open until this closeout packet merges and the wave is intentionally closed. |
| `#4178` Tokio feature baseline | `closed` | The shared Tokio feature floor landed. |
| `#4179` long-lived cadence migration | `closed` | Long-lived cadence now uses Tokio timers/tasks instead of blocking sleep. |
| `#4180` runtime bootstrap consolidation | `closed` | Shared runtime bootstrap helper landed and reduced duplicated current-thread runtime setup. |
| `#4181` ACIP-facing runtime substrate | `closed` | ACIP runtime work has a clearer shared-runtime floor and explicit remote-exec ownership boundary. |
| `#4182` bounded CAV cadence integration route | `closed` | Bounded CAV cadence now explicitly consumes the shared Tokio runtime without overclaiming always-on adversarial operation. |
| `#4183` closeout proof | `open` on this branch | Final child issue responsible for consolidating wave truth and residual routing. |
| `#4185` integrated runtime soak sprint | `open` | Later owner for runtime coherence and visible integrated soak claims. |
| `#4160` ACIP runtime umbrella | `open` | Downstream consumer of the landed shared-runtime substrate. |

## Delivery Matrix

| Issue | Current bounded status | What landed | What it proves | Limits / non-claims |
| --- | --- | --- | --- | --- |
| `#4178` | `closed_child_truth` | Tokio feature baseline widened to `macros`, `net`, `rt`, `rt-multi-thread`, `sync`, and `time` | ADL now carries the intended shared-runtime floor for later runtime work. | Does not itself claim runtime behavior changes. |
| `#4179` | `closed_child_truth` | Long-lived cadence moved off blocking sleep to Tokio-backed cadence | Long-lived runtime work can use Tokio timing/task mechanics while preserving bounded cycle semantics. | Does not claim integrated runtime soak or broad async conversion. |
| `#4180` | `closed_child_truth` | Shared CLI Tokio runtime helper landed | Runtime bootstrap duplication was reduced on the touched runtime-facing helpers. | Does not claim every runtime entry point is unified. |
| `#4181` | `closed_child_truth` | ACIP-facing runtime substrate and focused runner proof landed | `#4160` can consume a shared-runtime substrate instead of inventing one, and remote-exec keeps explicit single-step ownership boundaries. | Does not claim ACIP runtime execution is complete. |
| `#4182` | `closed_child_truth` | Shared-runtime bounded CAV cadence contract landed | Future scheduled and `continuous_bounded` verification cadence is routed onto the shared Tokio runtime. | Does not claim an always-on autonomous red/blue runtime. |

## Visible Runtime Proof Surface

The current bounded visible runtime proof surface is:

- `bash adl/tools/test_adl_runtime_compatibility.sh`

This proof surface is sufficient for the closeout packet because it shows that:

- `adl-runtime` is a real runtime-facing binary with a stable help and version
  surface
- `adl-runtime run <adl.yaml>` preserves the legacy YAML shortcut plan behavior
- runtime ownership stays fail-closed for C-SDLC issue and tooling commands
- the runtime binary can execute a fixture with a mock provider and write the
  expected bounded outputs

This is a launch-and-inspect proof surface, not a claim that the repository now
has a full integrated long-running runtime environment.

## ACIP 2 Consumption Rule

`#4160` may consume this wave only as:

- the agreed Tokio feature floor
- Tokio-backed long-lived cadence mechanics
- a shared runtime bootstrap pattern for touched CLI/runtime entry points
- a clearer ACIP-facing substrate with explicit remote-exec non-ownership of
  DAG/scheduler orchestration
- focused proof that the runtime compatibility binary remains launchable and
  bounded

`#4160` may not consume this wave as:

- proof that ACIP runtime slices are already implemented
- permission to collapse capability, authority, or observability boundaries
- permission to treat remote-exec as the runtime scheduler/orchestrator
- proof that broader runtime coherence or soak behavior is complete

## Bounded CAV Consumption Rule

Bounded CAV follow-on work may consume this wave only as:

- an explicit shared Tokio runtime route for future scheduled and
  `continuous_bounded` cadence
- a non-claim boundary that distinguishes bounded cadence from an always-on
  autonomous adversarial runtime
- a runtime floor that later soak or integrated runtime work can reuse

Bounded CAV follow-on work may not consume this wave as:

- proof that red and blue teams can already be set running indefinitely
- proof of always-on bug-finding autonomy
- proof of integrated runtime coherence without the later soak sprint

## Residual Ownership And Routing

Residual work intentionally not closed by this packet:

1. ACIP executable runtime behavior
   - Current truth: the substrate is ready enough to consume, but the runtime
     wave did not implement the ACIP message-carrier or authority path.
   - Owner: `#4160`

2. Runtime coherence and integrated soak proof
   - Current truth: there is a bounded launchable runtime proof surface, but no
     milestone-level integrated soak proving the runtime behaves coherently
     across the larger runtime stack.
   - Owner: `#4185`

3. Always-on autonomous adversarial runtime
   - Current truth: bounded CAV cadence is routed onto the shared runtime, but
     no always-on autonomous red/blue capability is claimed or proved.
   - Owner: later runtime/security work; not this wave

4. Broader Tokio ecosystem adoption
   - Current truth: the plan names `tracing`, `tower`, `hyper`, `bytes`,
     `tonic`, and `loom` as candidates or later decisions, but this wave did
     not claim full adoption of that stack.
   - Owner: later runtime and ACIP slices as explicitly scoped

## Reviewer Takeaway

The Tokio prerequisite wave is closeout-clean when reviewers can confirm that:

- every landed child slice from `#4178` through `#4182` is represented
  truthfully in issue records or this closeout packet
- `#4160` can consume the shared-runtime floor without inventing a parallel
  substrate
- bounded CAV work can consume the shared-runtime route without overclaiming
  always-on autonomy
- the runtime compatibility proof surface is explicitly visible and launchable
- integrated soak and broader runtime coherence claims remain explicitly routed
  to `#4185`
