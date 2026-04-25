# ADR 0014: Contract-Market Architecture

- Status: Accepted
- Date: 2026-04-24
- Related issue: #2438
- Related milestone: v0.90.4
- Review source: v0.90.4 third-party review P2 remediation
- Builds on: ADR 0013

## Context

v0.90.4 introduces ADL's first bounded contract-market substrate. Before this
milestone, ADL had citizen-state continuity, standing, access-control,
projection, challenge, appeal, and quarantine semantics from v0.90.3, but it
did not have one durable architecture record for contracts, bids, lifecycle
transitions, bounded external counterparties, delegation/subcontracting,
resource stewardship, and reviewer-facing proof of contract-market behavior.

The third-party v0.90.4 review found no P0 or P1 issues and marked the
milestone ready after one concrete P2 remediation: add ADR 0014 for the
contract-market architecture. The implementation, feature docs, demos, and
release-tail packet are already present; the remaining gap is architectural
consolidation and boundary clarity.

This ADR is grounded in:

- `docs/adr/0013-runtime-v2-citizen-state-continuity-substrate.md`
- `docs/milestones/v0.90.4/README.md`
- `docs/milestones/v0.90.4/DESIGN_v0.90.4.md`
- `docs/milestones/v0.90.4/DECISIONS_v0.90.4.md`
- `docs/milestones/v0.90.4/WBS_v0.90.4.md`
- `docs/milestones/v0.90.4/ECONOMICS_INHERITANCE_AND_AUTHORITY_AUDIT_v0.90.4.md`
- `docs/milestones/v0.90.4/FEATURE_PROOF_COVERAGE_v0.90.4.md`
- `docs/milestones/v0.90.4/features/EVALUATION_AND_TRANSITION_AUTHORITY.md`
- `docs/milestones/v0.90.4/features/COUNTERPARTY_AND_DELEGATION.md`
- `docs/milestones/v0.90.4/features/RESOURCE_STEWARDSHIP_BRIDGE.md`
- `docs/milestones/v0.90.4/features/CONTRACT_MARKET_DEMO_AND_RUNNER.md`
- `adl/src/runtime_v2/contract_schema.rs`
- `adl/src/runtime_v2/bid_schema.rs`
- `adl/src/runtime_v2/evaluation_selection.rs`
- `adl/src/runtime_v2/transition_authority.rs`
- `adl/src/runtime_v2/contract_lifecycle_state.rs`
- `adl/src/runtime_v2/external_counterparty.rs`
- `adl/src/runtime_v2/delegation_subcontract.rs`
- `adl/src/runtime_v2/resource_stewardship_bridge.rs`
- `adl/src/runtime_v2/tests`

This ADR does not introduce new runtime behavior. It records the architecture
that v0.90.4 already implements and validates.

## Decision

ADL adopts a bounded contract-market architecture for v0.90.4.

At the v0.90.4 boundary, the contract market is not a payment rail, a pricing
engine, a legal-contracting system, an inter-polis exchange, or a governed
tool-execution substrate. It is a reviewable, fixture-backed architecture for
contracts, bids, evaluation, authority-checked lifecycle transitions, bounded
delegation, and proof artifacts.

This decision requires:

1. Contract artifacts are explicit, traceable, and non-citizen by default

   Parent contracts, bids, evaluations, transitions, and review summaries are
   explicit artifacts with trace links and authority basis. Contract artifacts
   do not by themselves create citizen standing, redefine identity, or grant
   private-state access.

2. Bid evaluation is reviewer-visible rather than hidden winner selection

   Selection must expose mandatory checks, criterion-level reasoning,
   aggregation, recommendation, and override rationale. A chosen bid must be
   explained, not merely named.

3. Lifecycle transitions are authority-checked and fail closed

   Contract state changes require explicit actor and authority basis. Award,
   acceptance, execution, completion, dispute, cancellation, and other
   transitions must reject unauthorized or malformed attempts and preserve
   reviewable evidence.

4. External counterparties are bounded participants, not citizens

   External actors participate through explicit trust, assurance, sponsor,
   gateway, revocation, and allowed-action boundaries. They do not inherit
   citizen standing or private-state inspection rights by default.

5. Delegation preserves parent responsibility

   Subcontracts must remain linked to the parent contract, delegated scope,
   inherited constraints, and trace surfaces. Delegation cannot launder parent
   responsibility or silently expand authority.

6. Resource stewardship is a bridge, not a payment system

   v0.90.4 may record compute, memory, attention, bandwidth, artifact-storage,
   review-time, and tool-resource requirements as bounded stewardship claims.
   Those claims do not become pricing, billing, settlement, banking, or tax
   infrastructure.

7. Tool needs remain constraints, not execution authority

   Contracts, bids, and subcontracts may declare tool requirements or adapter
   expectations, but that declaration is not permission to execute tools.
   Governed tool-call semantics, UTS, ACC, registry binding, executor
   authority, denial records, and tool-call model testing remain deferred to
   v0.90.5.

8. The proof surface is one bounded runner/demo packet

   The architecture culminates in a deterministic fixture packet, runner,
   proof packet, negative packet, operator report, review summary, and demo
   coverage record. The proof is reviewer-facing and bounded; it does not claim
   a complete economy.

9. v0.90.4 consumes citizen-state authority instead of redefining it

   Contract-market work inherits standing, access-control, projection,
   challenge, appeal, quarantine, sanctuary, and continuity boundaries from
   ADR 0013 and the v0.90.3 citizen-state substrate. Economics must not weaken
   those protections.

10. Payments, legal rails, and inter-polis economics remain explicit non-goals

   The v0.90.4 contract-market architecture stops before payment settlement,
   legal/compliance rails, inter-polis trade, production identity verification,
   or open-ended market autonomy.

## Rationale

ADL needed a bounded economics milestone that could be reviewed and proven
without overclaiming money, law, or tool authority. The implementation already
established the right shape:

- contracts and bids are explicit artifacts
- evaluation is visible and reviewable
- transitions require authority
- counterparties stay bounded
- delegation keeps parent responsibility visible
- resource stewardship records constraints without pretending to settle money
- the demo packet proves one bounded market loop
- governed tools remain a later milestone

Without one ADR, reviewers would have to reconstruct that architecture from
many feature docs, tests, and proof packets. ADR 0014 makes the milestone
legible as one coherent architecture while preserving the careful non-claims
that kept the milestone disciplined.

## Consequences

### Positive

- Gives v0.90.4 one durable architecture decision record for the contract-market
  substrate.
- Makes the line between contract-market proof and governed-tool authority
  explicit.
- Consolidates contract, bid, evaluation, transition, counterparty, delegation,
  stewardship, and proof-packet decisions into one reviewer-facing surface.
- Preserves the handoff from v0.90.4 into v0.90.5 without implying tool
  execution is already solved.
- Preserves the handoff from ADR 0013 without weakening citizen-state
  authority.

### Negative

- Future changes to contract schemas, evaluation semantics, transition
  authority, counterparty rules, delegation boundaries, or resource-stewardship
  meaning now carry architectural weight.
- Future milestones must either preserve or explicitly supersede the boundary
  between tool requirements and governed tool authority.
- Payment or inter-polis work can no longer be casually implied through market
  language; it must be explicitly planned and justified.

## Alternatives Considered

### 1. Defer the ADR and leave the milestone documented only by feature docs

Pros:

- Less release-tail documentation work.

Cons:

- The third-party review explicitly identified the missing ADR as the remaining
  architecture gap.
- Reviewers would have to reconstruct one architecture from many docs and proof
  surfaces.
- The governed-tools handoff boundary would remain easier to blur.

### 2. Fold the contract-market story into ADR 0013 only

Pros:

- Fewer ADR files.
- Keeps economics close to citizen-state continuity.

Cons:

- ADR 0013 is about citizen-state continuity, not contract-market mechanics.
- Folding v0.90.4 into ADR 0013 would blur the distinction between continuity
  authority and economics artifacts.
- The contract-market substrate is substantial enough to deserve its own
  accepted boundary.

### 3. Widen v0.90.4 to include payment rails or governed tool execution

Pros:

- A superficially more complete market story.

Cons:

- Violates the milestone's explicit non-goals.
- Breaks the reviewed handoff to v0.90.5 governed tools.
- Would turn a bounded proof milestone into a much riskier architecture jump.

## Validation Evidence

The decision is supported by:

- contract, bid, evaluation, transition, counterparty, delegation, and
  stewardship implementation in `adl/src/runtime_v2/`
- focused runtime-v2 test families for those surfaces
- v0.90.4 feature docs and proof coverage records
- the bounded contract-market demo/runner packet and negative packet
- the v0.90.4 demo matrix and release-plan truth surfaces
- the third-party review result that classified ADR 0014 as the remaining P2
  remediation item

## Non-Claims

This ADR does not claim:

- payment settlement
- Lightning, x402, stablecoin, banking, invoicing, tax, or legal-contracting
  rails
- full inter-polis economics
- production counterparty identity or compliance systems
- governed tool-call semantics
- UTS, ACC, registry binding, executor authority, or direct model-to-tool
  execution
- v0.91 moral governance and wellbeing completion
- v0.92 identity, continuity, birthday, or citizenship expansion

## Notes

This ADR records the architecture that v0.90.4 already shipped as a bounded
contract-market substrate. Future ADRs may refine payment, inter-polis
economics, governed-tool execution, identity-bearing markets, or constitutional
governance, but those later decisions should cite this ADR when they build on
the v0.90.4 contract-market boundary.
