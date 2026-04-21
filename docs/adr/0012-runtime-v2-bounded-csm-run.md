# ADR 0012: Runtime v2 Bounded CSM Run Architecture

- Status: Accepted
- Date: 2026-04-21
- Related issue: #2320
- Related milestone: v0.90.2
- Builds on: ADR 0011

## Context

v0.90.2 takes the long-lived runtime substrate from ADR 0011 and attaches it to
the first bounded CSM run. The milestone is not a hidden daemon, a production
polis, or the first true Gödel-agent birthday. It is a code-backed, bounded,
reviewable run spine for `proto-csm-01` with explicit evidence around normal
execution, invalid action rejection, snapshot/rehydrate/wake continuity,
operator visibility, recovery eligibility, quarantine, and one governed
adversarial pressure path.

This ADR is grounded in:

- `docs/adr/0011-long-lived-agent-runtime.md`
- `docs/milestones/v0.90.2/DECISIONS_v0.90.2.md`
- `docs/milestones/v0.90.2/DESIGN_v0.90.2.md`
- `docs/milestones/v0.90.2/CSM_RUN_PACKET_CONTRACT_v0.90.2.md`
- `docs/milestones/v0.90.2/FEATURE_PROOF_COVERAGE_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_READINESS_v0.90.2.md`
- `docs/milestones/v0.90.2/features/RUNTIME_V2_HARDENING.md`
- `docs/milestones/v0.90.2/features/RECOVERY_AND_QUARANTINE.md`
- `docs/milestones/v0.90.2/features/SECURITY_BOUNDARY_EVIDENCE.md`
- `docs/milestones/v0.90.2/features/VIOLATION_ARTIFACT_CONTRACT.md`
- `docs/milestones/v0.90.2/features/OPERATOR_REVIEW_SURFACES.md`

This ADR does not introduce new runtime behavior. It records the architecture
that v0.90.2 implements and validates.

## Decision

ADL adopts a bounded, evidence-first architecture for the first Runtime v2 CSM
run.

At the v0.90.2 boundary, a CSM run is a governed run packet plus a contiguous
artifact spine. The run packet defines the manifold, citizens, stage order,
artifact families, review target, validation commands, and non-claims. Later
Runtime v2 work may enrich the artifacts, but it must not silently replace this
first-run spine with an incompatible contract.

The v0.90.2 architecture requires:

1. The first CSM run is packet-shaped and bounded

   `proto-csm-01` is represented through a stable run packet contract and
   fixture-backed artifacts. The packet does not claim a live unbounded runtime.
   It is the reviewer-readable contract for boot, admission, governed episode
   execution, scheduling, Freedom Gate mediation, invalid-action rejection,
   snapshot, rehydrate, wake, Observatory projection, recovery, quarantine, and
   hardening evidence.

2. Stage order is part of the architecture

   The first-run spine is contiguous: contract and fixture, invariant and
   violation contract, boot and admission, governed episode and rejection,
   snapshot/wake/Observatory, governed adversarial hardening, and integrated
   first-run proof. Later changes may add evidence, but they must not create
   competing first-run packet contracts or reorder the spine without a new
   architectural decision.

3. Invariants and violations are durable review surfaces

   Runtime v2 hardening must emit stable invariant maps and violation artifacts.
   A violation artifact records the attempted action, violated invariant, actor,
   kernel stage, decision, trace anchors, and resulting state. This keeps
   negative proof reviewable instead of burying it in test logs.

4. Recovery and quarantine are separate decisions

   Recovery is allowed only when declared invariants, trace evidence, snapshot
   evidence, identity constraints, and operator policy permit safe resume.
   Quarantine is required when recovery would be ambiguous or unsafe.
   Quarantine preserves evidence and blocks execution pending operator review;
   it is not itself a successful recovery path.

5. Observatory and operator surfaces are projections, not authority

   The Observatory packet and operator report expose run state, invalid-action
   refusal, wake-continuity evidence, recovery decisions, quarantine rationale,
   and hardening status. These surfaces support review and operator
   understanding. They do not replace the runtime contract, grant mutation
   authority, or claim a full dashboard product.

6. The governed adversarial hook is defensive and scoped

   v0.90.2 includes one operator-scoped adversarial hook under explicit rules of
   engagement. Its job is to pressure the first-run hardening boundary and prove
   containment through ordinary Runtime v2 mechanisms. It is not a full
   red/blue/purple security ecology and does not define Runtime v2 as a security
   product.

7. The integrated first-run demo is a proof bundle

   The integrated demo command ties the landed evidence together into a
   deterministic reviewer bundle and operator report. It is a bounded proof
   surface for v0.90.2, not live autonomy, not citizenship completion, and not
   the first true Gödel-agent birthday.

## Rationale

Runtime v2 needs a bridge between long-lived cycle mechanics and later CSM
identity, governance, moral, emotional, memory, and birthday work. That bridge
must be concrete enough to run and review, but not so broad that it smuggles in
future milestone claims.

The bounded run packet gives ADL a stable middle layer:

- implementation can add code-backed artifacts against a known contract
- reviewers can inspect one coherent run instead of disconnected fixtures
- operators can understand recovery and quarantine without reverse-engineering
  test output
- security pressure enters through a governed hook instead of ad hoc abuse paths
- later v0.91 and v0.92 work inherits a hardened substrate without being claimed
  early

This keeps ADL's core commitments intact: bounded execution, traceability,
deterministic proof where claimed, explicit non-claims, and reviewer-visible
evidence.

## Consequences

### Positive

- Gives v0.90.2 one durable architecture record for the first bounded CSM run.
- Makes the run packet, hardening proof, recovery/quarantine split, and
  Observatory projection easier to review.
- Creates a stable handoff from ADR 0011 into later Runtime v2, polis, citizen,
  memory, and first-birthday work.
- Prevents future docs from treating quarantine as recovery success.
- Prevents the governed adversarial hook from expanding silently into a full
  security ecology.

### Negative

- Commits future Runtime v2 changes to preserve or explicitly supersede the
  first-run packet contract.
- Adds architectural weight to changes in stage order, artifact shape,
  recovery semantics, quarantine release rules, and operator projections.
- Requires later milestones to keep distinguishing bounded proof bundles from
  live autonomy, citizenship completion, and true birthday semantics.

## Alternatives Considered

### 1. Treat the first CSM run as an ordinary demo

Pros:

- Smaller documentation surface.
- Less formal architecture work during release closeout.

Cons:

- Would understate the architectural importance of the run packet, recovery
  boundary, quarantine state, and hardening proof.
- Would make future milestones more likely to drift from the v0.90.2 artifact
  contract.
- Would leave reviewers without a single decision record for why the first run
  is bounded and evidence-first.

### 2. Claim Runtime v2 as live or production-ready

Pros:

- Stronger marketing language.
- Simpler story for a casual reader.

Cons:

- False for v0.90.2.
- Conflicts with ADL boundedness and reviewability.
- Would blur the line between fixture-backed proof, local bounded execution,
  and later live Runtime v2 operation.

### 3. Defer the ADR until v0.91 or v0.92

Pros:

- Later milestones will have richer identity, governance, and birthday context.
- Future work may refine citizen and memory semantics.

Cons:

- v0.90.2 already introduces a substantive first-run architecture.
- Deferral would leave review-tail docs without a durable decision record.
- Later milestones need a clear statement of the substrate they inherit.

## Validation Evidence

The decision is supported by:

- the v0.90.2 decision table accepting first bounded CSM run scope, hardening
  attachment, violation artifacts, recovery/quarantine split, bounded security
  ecology, and later-milestone boundaries
- the CSM run packet contract and contiguous stage contract
- code-backed Runtime v2 artifacts for run packet, Observatory projection,
  recovery, quarantine, and hardening
- golden fixtures for run packet, invariants, violations, boot/admission,
  first-run trace, wake continuity, Observatory, recovery, quarantine, hardening,
  and integrated first-run proof
- the integrated first-run demo command described in v0.90.2 docs
- feature proof coverage and release-readiness docs that classify proof status
  without claiming live unbounded execution

## Non-Claims

This ADR does not claim:

- live unbounded Runtime v2 autonomy
- production polis operation
- the first true Gödel-agent birthday
- full persistent personhood
- final citizenship semantics
- cross-polis migration
- v0.91 moral, emotional, kindness, humor, wellbeing, or civic substrate
  completion
- v0.92 identity, capability, migration, or birthday completion
- complete red/blue/purple security ecology
- operator dashboard product completion
- autonomous release approval

## Notes

Future ADRs may refine citizen standing, identity binding, memory continuity,
capability rebinding, enclave or checkpoint custody, moral governance, or the
first true birthday boundary. Those ADRs should cite this one when they build on
the v0.90.2 first-run packet, evidence, recovery, quarantine, and governed
hardening substrate.
