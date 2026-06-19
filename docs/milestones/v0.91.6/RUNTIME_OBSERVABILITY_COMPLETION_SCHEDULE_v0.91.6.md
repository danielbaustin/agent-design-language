# Runtime Observability Completion Schedule

## Metadata

- Milestone: `v0.91.6`
- Status: scheduled boundary and handoff record
- Owner: ADL maintainers
- Issue: `#3922`
- Related issues: `#4001`, `#4048`
- Related proof surfaces: `#3705`, `#3707`, `#3708`, `#3709`, `#3710`, `#3711`

## Purpose

Turn runtime logging and observability completion into an explicit schedule
instead of leaving it split implicitly across the `v0.91.5` logging
mini-sprint, `v0.91.6` proof-loop reliability work, `v0.91.7` bridge
planning, and the `v0.92` provider/runtime architecture band.

## Scheduling Rule

This record distinguishes four levels of truth:

1. `v0.91.5` established bounded contracts and proof packets.
2. `v0.91.6` must finish the logging/proof-loop reliability and consumption
   classification work needed for truthful closeout.
3. `v0.91.7` may carry bridge-only runtime and Observatory handoff rows, but
   must not claim provider-backed completion.
4. `v0.92` is the earliest home for full provider/runtime observability
   completion, including any optional OTEL/export implementation.

## What Is Already Established By `v0.91.5`

The `v0.91.5` logging mini-sprint established a real baseline:

- shared observability vocabulary and OTEL boundary contract from `#3705`
- bounded runtime/provider logging proof from `#3707`
- bounded heartbeat/timeout/progress proof from `#3708`
- OTEL boundary and non-claim proof from `#3709`
- Observatory example-stream consumption proof from `#3710`
- a scoped logging validation checklist from `#3711`

These are valid proof surfaces. They are not a claim that runtime observability
is complete everywhere in the repo.

## `v0.91.6` Completion Bar

`v0.91.6` is the milestone where logging/proof-loop reliability becomes
truthful enough for closeout and release-facing docs.

The `v0.91.6` completion bar is:

- keep the `v0.91.5` contracts authoritative without rewriting them
- finish GitHub/token/release/projection observability for the proof loop
- classify Observatory/OTEL/security consumption for the bounded WP surfaces
- record WP-03 closeout truth so logging completion claims are exact
- preserve non-claims about provider-backed runtime completion

## Current Logging Status In Scope

The bounded logging baseline now in place for the proven surfaces is:

- machine-readable JSON remains the authoritative stdout channel for the
  covered control-plane flows
- human-oriented `adl_event` observability remains stderr-by-default unless an
  explicit compatibility log path is in play
- command-driven `runtime-v2` and proof-loop logging has bounded proof support
  for redaction, path hygiene, and failure classification
- Observatory consumption is proven only as a bounded example-stream consumer,
  not as provider-backed production completion

This issue does not widen those claims.

## `v0.91.6` Issue Mapping

| Issue | Scope in this schedule | Milestone home |
| --- | --- | --- |
| `#4001` | GitHub/token/release/projection observability for the tooling proof loop | `v0.91.6` WP-03 |
| `#3922` | This scheduling and claim-boundary record | `v0.91.6` runtime-observability planning |
| `#4034` | Routed out of this wave; Observatory logging and OTEL/security consumption proof belongs with the Observatory sprint rather than this `v0.91.6` logging completion pass | observatory sprint route |
| `#4048` | WP-03 tooling proof-loop closeout and final claim normalization, including live issue-graph truth for `#3963`, `#3965`, and the separately completed routed `#3985` follow-on | `v0.91.6` WP-03 closeout |

## Proof Surface Map

| Proof surface | What it proves now | `v0.91.6` use | Deferred home |
| --- | --- | --- | --- |
| `#3705` Shared observability contract | Shared vocabulary, channel policy, durable-artifact and OTEL boundary contract | Authoritative contract for `#4001`, `#4034`, and `#4048` | Extend or implement further in `v0.92` only when architecture work lands |
| `#3707` Runtime/provider logging proof | Covered command-driven runtime/provider log artifacts and redacted evidence paths | Supports bounded proof-loop and runtime-v2 claims only | Full provider/runtime correlation and long-lived completion in `v0.92` |
| `#3708` Heartbeat/timeout/progress proof | Covered progress signal truth for the proven slice | Supports bounded progress/heartbeat claims in `v0.91.6` docs and review packets | Wider runtime/provider heartbeat coverage in `v0.92`; bridge notes may appear in `v0.91.7` |
| `#3709` OTEL integration boundary proof | OTEL is bounded and optional, not already fully implemented | Keeps `v0.91.6` from overclaiming OTEL/export completion | Optional OTEL/export implementation in `v0.92` |
| `#3710` Observatory log consumption proof | Example-stream consumption proof, not production Observatory completion | Supports `#4034` consumption classification and proof language | Wider Observatory/runtime integration claims deferred to `v0.91.7` bridge notes and `v0.92` implementation |
| `#3711` Logging validation checklist | Minimum proof-selection contract for logging-affecting work | Required validation-selection guide for `#4001`, `#4034`, and `#4048` | Remains active until replaced by a later tracked contract |

## `v0.91.7` Bridge-Only Handoff Rows

`v0.91.7` may carry only bridge-accounting and planning truth for the
remaining runtime/observability surfaces. Observatory-consumption execution
that has been intentionally routed out of this wave should stay with the
Observatory sprint rather than being silently pulled back into this scheduling
issue. `v0.91.7` is not the milestone to claim the architecture is complete.

Bridge-only rows that belong in `v0.91.7`:

- residual runtime/loop observability hooks needed by reasoning-graph and loop
  runtime planning
- bridge accounting for Observatory-facing consumption after the bounded
  `v0.91.6` proof packet
- handoff rows for any runtime/provider durable-log surfaces that remain
  conceptual before `v0.92`
- explicit non-claims for anything still lacking provider/runtime correlation,
  long-lived execution evidence, or durable end-to-end proof

## `v0.91.7` Bridge Handoff Register

| Surface | Why it is only bridge work in `v0.91.7` | Expected home |
| --- | --- | --- |
| Reasoning-graph and loop-runtime observability hooks | These surfaces need runtime-observability accounting so later bridge docs can reference logging expectations without claiming the provider/runtime architecture is already complete. | `v0.91.7` WP-04 planning and handoff docs |
| Observatory-facing consumption accounting after the bounded `v0.91.6` proof packet | The current proof establishes bounded consumption behavior, not production Observatory completion. | `v0.91.7` bridge notes and Observatory sprint planning |
| Residual runtime/provider durable-log handoff rows | Durable end-to-end runtime/provider evidence is still conceptual outside the bounded slices already proven. | `v0.91.7` handoff rows only, then `v0.92` implementation |
| Carry-forward non-claims | The repo must keep explicit non-claims visible wherever bridge docs summarize runtime logging posture. | Every `v0.91.7` bridge doc that mentions runtime observability |

## `v0.92` Provider/Runtime Architecture Band

The following work belongs to `v0.92`, not to `v0.91.6` or `v0.91.7`:

- complete provider/runtime correlation
- long-lived or provider-backed runtime observability completion
- durable-log completion across the provider/runtime architecture
- optional OTEL/export implementation beyond the current boundary proofs
- any claim that runtime observability is complete repo-wide

## `v0.92` Completion Handoff Register

| Surface | Why deferred to `v0.92` |
| --- | --- |
| Full provider/runtime correlation | Not established by the bounded `runtime-v2` or proof-loop surfaces proven so far. |
| Long-lived or provider-backed runtime observability completion | Requires runtime architecture work, not only command-driven proof bundles. |
| Durable-log completion across provider/runtime paths | Needs architecture-level completion rather than bridge accounting. |
| Optional OTEL/export implementation | Current proof establishes a boundary and non-claim, not an implementation. |
| Repo-wide runtime observability completion claims | No current milestone has proven exhaustive coverage across the repo. |

## Command-Driven Versus Provider-Backed Boundary

The strongest safe distinction for this schedule is:

- command-driven `runtime-v2` and tooling proof bundles prove bounded logging
  behavior for those named command surfaces
- they do not prove long-lived runtime loops, provider-backed execution, or
  full provider/runtime correlation
- any doc that summarizes the current logging posture must preserve this split
  instead of collapsing it into a generic "runtime observability complete"
  claim

## Non-Claims Preserved By This Schedule

This schedule does not authorize these claims:

- full OTEL implementation is done
- provider/runtime correlation is complete
- runtime observability is complete repo-wide
- command-driven `runtime-v2` proof bundles prove long-lived/provider-backed
  runtime completion
- Observatory example-stream proof is the same as production Observatory
  integration completion

## Exit Condition For This Scheduling Issue

`#3922` is complete when `v0.91.6` has an explicit runtime observability
schedule, every remaining surface has a concrete milestone home, and the docs
no longer rely on vague spillover language for runtime logging completion.
