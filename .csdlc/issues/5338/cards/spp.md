# Structured Planning Prompt

Template: 1.0.0

Issue: 5338

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

The retained #5339 terminal receipt is closed_out with claim released, PR #5612 merged at reviewed head ba604e5f0ee16af901a4d8d7cb801c323500828d, and squash merge 860aa9f18946a2cd9407b610d5c00d44ddc89053 is an ancestor of this issue worktree. Implement only against the landed typed adl-language API: lower validated sequential or concurrent workflows plus explicit saved-state dependencies into an inert deterministic ExecutionPlan; resolve task, agent, provider, model, and tool references; canonicalize with ordered collections; generate versioned domain-separated stable node identities; and fail closed on limits, collisions, cycles, or unsupported inputs. Legacy top-level patterns remain intentionally outside the compiler input contract because adl-language rejects and cannot represent them. Preserve the reviewed COTS set and executable ceilings: focused <=120 seconds, quality <=120 seconds, replay <=300 seconds, and full measurement plus tests <=600 seconds. Any boundary, dependency, protected-path, or budget variance requires typed replanning and review.

## Plan

Revision 6

## Steps

[
  {
    "id": "S1",
    "action": "Complete and review all six issue-specific cards, clean-room design and diagram, protected paths, COTS choices, strict LoC/test/time budgets, and executable preparation validation without product implementation",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Watch live dependency truth and only after #5339 is merged and typed closed_out align the issue branch to the landed dependency revision through the typed-authorized integration route",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement the isolated compiler pipeline, inert ExecutionPlan data model, bounded deterministic lowering, stable diagnostics, and versioned stable node identities",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add canonical fixtures, negative proof, #5339 fixture mapping, permutations, repeated clean-process replay, identity-locality tests, and strict COTS and budget enforcement",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run exact-revision validation and bounded code review, fix all actionable findings, publish through typed v2, shepherd green required CI and authorized merge, capture post-merge proof, and complete typed closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- The compiler is a total side-effect-free transformation from validated typed document to plan or diagnostics
- ExecutionPlan is inert data and contains no execution authority
- Node identity depends only on the versioned semantic preimage, never incidental ordering or machine state
- Unspecified order is made explicit and deterministic or rejected
- Every applicable #5339 fixture is mapped without silent normalization or skipping
- The crate never depends on incumbent ADL, Runtime v2/v3, C-SDLC, async, network, cloud, provider, database, executor, petgraph, or RNG families
- No product implementation before #5339 merged and typed closed_out
- Root main remains clean and all canonical #5338 artifacts remain issue-local

## Risks

- The landed #5339 API may differ from the provisional compiler input assumed during preparation
- Composition and pattern semantics may contain unspecified ordering or expansion bounds
- Stable identity can accidentally include traversal order or omit a meaning-bearing semantic component
- Canonical JSON can drift through unordered nested metadata or diagnostic ordering
- Compiler lowering may absorb scheduling or source-language validation authority
- A hard per-crate budget can pressure proof quality or move logic into unmeasured surfaces

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5338/retained/design.md

Digest: 621c4ebfc1f549a011b4e37680d5024cfdaf9658bdbc980a2203738d3b05a43c

## Diagram

.csdlc/issues/5338/retained/diagram.mmd

Digest: 80b58c32f620c43cb0aaedfa8ebfe6ede9501523d87e67718f12eb4c1b843756

## Stop Conditions

- #5339 is not both merged and typed closed_out when product implementation would begin
- The landed adl-language API or fixtures are missing, stale, ambiguous, or not reviewed
- Required behavior would copy, adapt, import, or link incumbent ADL implementation or fixtures
- Resolution, expansion, ordering, bounds, or identity semantics cannot be specified deterministically from reviewed inputs
- A proposed dependency introduces execution, runtime, lifecycle, network, cloud, provider, database, async, or RNG authority
- LoC, test, dependency, or latency variance lacks exact-revision evidence-backed design review
- Required CI, review, merge authorization, post-merge evidence, or terminal lifecycle truth is incomplete

## Handoff

Proceed only after doctor readiness.
