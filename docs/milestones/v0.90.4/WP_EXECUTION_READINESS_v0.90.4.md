# WP Execution Readiness - v0.90.4

## Purpose

This document is the card-authoring source for the future v0.90.4 WP issue
wave. WP-01 should copy the relevant section into each issue body before
implementation begins.

v0.90.4 is the citizen economics and contract-market substrate milestone. It
must produce concrete contracts, fixtures, validators, runner artifacts, and
reviewable demos. It must not become an economics essay or a payment-rail
placeholder.

## Global Execution Rules

- Consume v0.90.3 citizen-state authority instead of redefining citizenship,
  standing, private state, projection, challenge, appeal, quarantine, sanctuary,
  or continuity.
- Keep all contract-market artifacts fixture-backed and reviewable.
- Treat payment settlement, Lightning, x402, stablecoins, banking, invoicing,
  tax, production legal contracting, and full inter-polis economics as explicit
  later work.
- Treat tool requirements as constraints, resource estimates, adapter
  expectations, or evidence requirements. v0.90.4 must not authorize direct tool
  execution, implement UTS/ACC, or treat model output as permission to act.
- Require negative cases for unauthorized transitions, invalid bids, unsupported
  delegation, revoked counterparties, missing trace links, and tool execution
  attempts that lack governed-tool authority.
- Preserve the demo-matrix WP before quality/docs/review convergence.
- Use the release-tail pattern: quality/docs convergence, internal review,
  external review, remediation, next-milestone planning, release ceremony.

## WP-01: Promote v0.90.4 Milestone Package

Required outputs:

- Reviewed v0.90.4 planning package.
- Issue wave opened from WP_ISSUE_WAVE_v0.90.4.yaml.
- Issue numbers written back into WBS_v0.90.4.md and
  WP_ISSUE_WAVE_v0.90.4.yaml.
- Issue cards updated with the relevant readiness section from this document.

Required validation:

- Check the issue wave matches WP ordering.
- Check no WP body is generic or missing required outputs and validation.
- Check planning docs contain no unresolved template placeholders or local host
  paths.
- Check the v0.90.3 handoff truth still shows no blocking carryover before
  opening the wave.

## WP-02: Economics Inheritance And Authority Audit

Required outputs:

- Authority dependency report mapping v0.90.3 standing, access-control,
  projection, challenge/appeal, quarantine/sanctuary, continuity witness, and
  receipt surfaces to v0.90.4 market requirements.
- Gap list for any v0.90.3 surface not yet sufficient for contract-market use.
- Narrowing recommendation for v0.90.4 if a required authority surface is not
  available.

Required validation:

- Verify referenced v0.90.3 docs and artifacts exist.
- Classify each dependency as inherited, fixture-backed, deferred, or blocked.

## WP-03: Contract Schema

Required outputs:

- Contract schema or strongly typed contract artifact.
- Valid parent-contract fixture.
- Invalid contract fixtures for missing trace requirements, missing authority
  basis, unsupported lifecycle state, incomplete evaluation criteria, and tool
  requirements that imply direct execution authority.
- Validator tests or fixture validation command.

Required validation:

- Valid fixture passes.
- Invalid fixtures fail for the intended reasons.
- Schema does not grant citizen standing by itself.
- Schema records tool-mediated requirements as constraints, not execution grants.

## WP-04: Bid Schema

Required outputs:

- Bid schema or strongly typed bid artifact.
- At least two valid bid fixtures against the parent contract.
- Invalid bid fixtures for wrong contract, late bid, ineligible counterparty,
  missing commitments, missing trace/signature requirements, and attempted tool
  authority beyond the bid's contract constraints.
- Validator tests or fixture validation command.

Required validation:

- Valid bid fixtures pass.
- Invalid bid fixtures fail for the intended reasons.
- Bid artifact preserves room for later pricing or payment rails without
  implementing settlement.
- Bid artifact preserves room for governed-tool integration without bypassing
  v0.90.5 UTS/ACC authority.

## WP-05: Evaluation And Selection Model

Required outputs:

- Evaluation artifact shape with mandatory checks, criterion-level scores,
  weighting, aggregation, recommendation, and override rationale.
- Selection fixture using the WP-03 and WP-04 artifacts.
- Negative or warning cases for critical criterion failure, tie-break, and
  unsupported override.
- Tool-readiness field or warning path when a selected bid depends on
  tool-mediated execution.

Required validation:

- Selection artifact explains the chosen bid rather than merely naming a
  winner.
- Override requires traceable rationale.
- Evaluation does not bypass authority checks.
- Evaluation does not treat valid JSON, model confidence, or adapter availability
  as authority to execute a tool.

## WP-06: Transition Authority Model

Required outputs:

- Transition authority matrix for draft, open, bidding, awarded, accepted,
  executing, completed, failed, disputed, and cancelled states.
- Authority-basis fixture for each allowed transition.
- Denial fixtures for unauthorized award, wrong actor acceptance, execution
  before acceptance, cancellation after completion, and completion without
  artifacts.
- Denial fixture for a transition that attempts to execute a tool without
  governed-tool authority.

Required validation:

- Allowed transitions pass only with explicit actor and authority basis.
- Denied transitions fail safely and leave reviewable evidence.

## WP-07: Contract Lifecycle State

Required outputs:

- Lifecycle state machine or equivalent runtime contract.
- Fixtures covering normal completion, failed execution, cancellation, and
  dispute resolution.
- Tests proving terminal states cannot be silently reopened.

Required validation:

- State transitions are deterministic.
- Lifecycle events include temporal anchor, trace link, and validation result.

## WP-08: External Counterparty Model

Required outputs:

- External counterparty record shape.
- Trust, assurance, sponsor, gateway, revocation, and allowed-action fields.
- Denial fixtures for insufficient assurance, revoked counterparty, missing
  gateway, private-state inspection attempt, and tool-mediated action outside
  allowed scope.

Required validation:

- External counterparties are not citizens by default.
- Counterparty participation does not grant private-state inspection rights.
- Human out-of-band action is not counted as citizen action.

## WP-09: Delegation And Subcontract Model

Required outputs:

- Subcontract artifact shape linked to parent contract, delegated scope,
  authority basis, inherited constraints, deliverables, and trace links.
- Delegated output fixture.
- Parent integration fixture.
- Negative cases for missing parent link, scope expansion, unsupported
  subcontractor, integration without review, and delegated tool use outside
  parent contract constraints.

Required validation:

- Subcontractor cannot silently inherit parent authority.
- Parent responsibility remains reviewable after delegation.

## WP-10: Resource Stewardship Bridge

Required outputs:

- Resource claim model for compute, memory, attention, bandwidth, artifact
  storage, review/operator time, and tool-adapter budget where relevant.
- Fixture showing contract and bid resource estimates, including at least one
  tool-mediated requirement recorded as a constraint.
- Boundary note explaining what remains outside v0.90.4.

Required validation:

- Resource claims are policy-bound and do not override standing, access
  control, quarantine, sanctuary, or challenge rights.
- Payment and pricing remain explicitly out of scope.
- Tool-resource requirements remain explicitly non-executable until v0.90.5
  governed-tool authority exists.

## WP-11: Contract-Market Fixture Set

Required outputs:

- Coherent fixture packet containing parent contract, two bids, evaluation,
  award transition, acceptance transition, subcontract, delegated output, parent
  integration output, completion event, trace bundle, review summary seed, and
  demo manifest.
- Optional tool-requirement fixture showing a needed tool as a constraint, not a
  grant.
- Invalid fixture packet for the negative-case suite.

Required validation:

- Fixture packet is portable and contains no local absolute paths.
- Manifest identifies every artifact and its proof purpose.

## WP-12: Contract-Market Runner

Required outputs:

- Deterministic runner that loads the fixture packet, validates artifacts,
  executes allowed lifecycle transitions, rejects negative cases, and emits
  transition/review artifacts.
- Runner behavior that recognizes tool requirements as constraints and refuses
  to execute them without governed-tool authority.
- Runner tests or smoke command.

Required validation:

- Same fixture input produces the same result.
- Negative fixtures fail safely.
- Runner output contains no secrets, prompt text, tool arguments, or local host
  paths.
- Runner output distinguishes contract-market proof from governed-tool proof.

## WP-13: Review Summary Shape

Required outputs:

- Reviewer-facing summary schema and rendered example.
- Summary sections for scope, participants, authority basis, bid comparison,
  selection rationale, delegation, artifacts, trace, validation, caveats, and
  residual risk.
- Summary language for tool requirements that were recorded, denied, or deferred.

Required validation:

- Summary preserves warnings and non-claims.
- Summary distinguishes proof, judgment, and residual risk.

## WP-14: Bounded Contract-Market Demo And Negative Cases

Required outputs:

- End-to-end contract-market proof packet.
- Operator/reviewer report for the successful path.
- Negative authority and trace packet covering unauthorized transition, invalid
  bid, unsupported delegation, revoked counterparty, missing trace link, and
  unauthorized tool execution attempt if the demo includes tool-mediated work.

Required validation:

- Successful demo proves only bounded contract-market mechanics.
- Negative demo cases fail for the expected reason.
- Demo claims do not imply payment settlement or full economics.
- Demo claims do not imply Governed Tools v1.0, UTS, ACC, or production tool
  execution authority.

## WP-14A: Demo Matrix And Feature Proof Demos

Required outputs:

- Updated DEMO_MATRIX_v0.90.4.md with landed, skipped, failed, non-proving, or
  deferred status for every feature claim.
- Feature proof coverage record showing which schema, fixture, runner, summary,
  and negative-case surfaces prove each claim, including any non-proving
  governed-tool handoff claims.

Required validation:

- No feature claim reaches docs/review convergence without proof status.
- Non-proving and deferred items are explicit.

## WP-15: Quality Gate, Docs, And Review Convergence

Required outputs:

- README, feature index, WBS, sprint plan, release plan, release notes, demo
  matrix, and checklist aligned with actual implementation.
- Cargo metadata, changelog, README, feature list, tracker docs, and review
  entry surfaces checked and updated if needed.

Required validation:

- Quality gate evidence is fresh.
- Docs contain no stale issue-wave state, local absolute paths, or aspirational
  shipped claims.

## WP-16: Internal Review

Required outputs:

- Findings-first internal review packet covering code, docs, demos, tests,
  issue truth, release boundaries, and proof claims.
- Findings register and proof register.

Required validation:

- Review packet has exact scope, exact files, requested review questions, and
  severity rubric.
- Accepted findings are routed to WP-18 or explicitly dispositioned.

## WP-17: External Review

Required outputs:

- Third-party review handoff packet.
- External review summary and full review artifact when available.
- Finding register updates.

Required validation:

- Handoff names the exact scope and non-claims.
- Review artifacts are stored in the review directory, not loose project dirs.

## WP-18: Review Findings Remediation

Required outputs:

- Accepted internal and external findings fixed or explicitly deferred with
  owner, rationale, and follow-up issue.
- Docs, tests, demos, and records updated to reflect remediation.

Required validation:

- No accepted P1/P2 finding remains unresolved without explicit human-approved
  deferral.
- Review and closeout records agree with the work performed.

## WP-19: Next-Milestone Planning And Handoff

Required outputs:

- Next-milestone planning package updated for the economic follow-on, v0.91,
  v0.92, v0.93, or v0.90.5 governed-tools lane as appropriate.
- Backlog updated with any deferred payment, reputation, resource accounting,
  inter-polis economics, contract/legal/billing work, or governed-tool
  requirements discovered during contract-market execution.

Required validation:

- No future-scope document is left orphaned or unmanaged.
- Handoff avoids reducing already planned milestones.

## WP-20: Release Ceremony

Required outputs:

- Release notes updated from actual evidence.
- End-of-milestone report written and stored in the local planning lane or other explicitly designated closeout surface.
- Changelog, README, Cargo metadata, feature list, milestone checklist, review
  records, and issue closeout state aligned.
- Ceremony script run using the milestone's accepted closeout pattern.

Required validation:

- Root main can be fast-forwarded cleanly after final merge.
- No stale planning package or ignored review artifact contradicts the tracked
  milestone docs.
