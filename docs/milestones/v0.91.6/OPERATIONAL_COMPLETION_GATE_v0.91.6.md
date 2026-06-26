# v0.91.6 Operational Completion Gate

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Issue: `#4539`
- Status: tracked gate for product/runtime completion claims

## Purpose

Prevent `v0.91.6` closeout, `v0.91.7` handoff, and `v0.92` activation planning
from treating prerequisite-only proof as finished runtime or product
completion.

This gate applies to product/runtime surfaces only. It does not relabel docs,
planning, or governance-only work as runtime completion work.

## Rule

No product/runtime feature may be called `done`, `complete`, or activation-ready
solely because a mock, seam, docs-only bridge, component proof, local slice, or
demo scaffold exists.

A product/runtime surface counts as operationally complete only when it is
classified `integrated_proven`.

## Completion Classes

Use these classes instead of ambiguous `done` language.

| Class | Meaning | Counts as operational completion? |
| --- | --- | --- |
| `planned` | Design, route, or ownership exists. | No |
| `docs_ready` | A reviewed doc or bridge record exists. | No |
| `seam_ready` | An interface, envelope, or seam exists. | No |
| `mock_proven` | Mock or local fake proves shape only. | No |
| `component_proven` | Focused component proof exists. | No |
| `local_slice_proven` | One local executable slice works. | No |
| `demo_scaffold` | A demo shell or canned-data path exists. | No |
| `integrated_proven` | The integrated runtime/operator path works with durable evidence and negative-case coverage. | Yes |
| `blocked` | A named blocker prevents integrated proof. | No |
| `deferred` | An explicit later-milestone route is accepted. | No |

## Required Evidence For `integrated_proven`

Every product/runtime surface claiming `integrated_proven` must record:

- issue and PR or merge evidence;
- the runtime or operator path exercised;
- the exact command, demo, or soak run used;
- durable artifact or output paths;
- positive-case result;
- negative or failure-case result;
- logging or observability proof when relevant;
- redaction or privacy proof when relevant;
- VPP, PVF, or validation-profile facts when relevant;
- truthful SOR or closeout language naming completion class, non-claims, and
  residual risk.

## Canonical Consumption Rule

This document is a gate and vocabulary contract. It is not a parallel
milestone-status ledger.

When `v0.91.6` closeout or `v0.91.7` / `v0.92` handoff needs current runtime or
product truth, consume that truth from the milestone's canonical current-state
surfaces instead of reconstructing or restating per-surface status here:

- `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`
  for closed bridge umbrellas and retained evidence posture;
- `docs/milestones/v0.91.6/CLOSEOUT_TAIL_SPRINT_v0.91.6.md`
  for the ordered open release-tail wave and wait-state handling;
- `docs/milestones/v0.91.6/review/V0916_RELEASE_AND_BRIDGE_DOC_TRUTH_CONSUMPTION_REVIEW_4522.md`
  for the bounded audit of single-source release-truth consumption.

This gate defines completion classes and consumption rules, but it does not
publish a second per-surface current-state truth table.

## How To Apply The Gate

When a consuming surface needs to talk about a runtime or product row:

1. Read the current state from the retained-evidence matrix or closeout-tail
   sprint surface first, using the `#4522` review as the single-source
   consumption guardrail.
2. Assign one completion class from this gate using the current issue/PR/closeout
   truth already routed there.
3. If the surface is not `integrated_proven`, name the blocker, defer, or
   follow-on route explicitly instead of inventing a milestone-local `done`
   label.
4. Preserve the single-source consumption model from `#4522`; do not create a
   second per-surface status table in another milestone doc.

## Closeout Consumption Gate

Before `v0.91.6` closeout or any `v0.92`-facing handoff consumes a
product/runtime surface, the consuming doc must answer:

1. What completion class applies?
2. What evidence proves that class?
3. If the class is not `integrated_proven`, what blocker, deferral, or follow-on
   owns the residual?
4. Does any touched checklist, release note, handoff, or milestone summary still
   imply ambiguous `done` language?

If a surface is not `integrated_proven`, the milestone truth must say that
plainly. Prerequisite proof is allowed. Overclaiming it as operational
completion is not.
