# #5592 Design: Runtime v3 Parity-B Governed Cognition

## Decision

Implement Parity-B only after #5591 supplies a clean reviewed Parity-A contract.
The accepted implementation must execute reasoning graphs, bounded loops,
governed adaptation, and cognition through the guardian-launched
`adl-runtime-kernel` canonical ingress. Library calls, metadata, fixed bootstrap
graphs, fixtures, and Runtime v2 execution do not receive parity credit.

This revision is preparation only. It defines the complete contract and claims
only #5592 lifecycle artifacts. It does not authorize product edits,
publication, cutover, Runtime v2 deletion, or a Runtime v3 default switch.

## Authority And Dependencies

- #5361 owns integrated Runtime v3 acceptance.
- #5336 owns architecture, feature-ledger, line-count, module-growth, and test
  budgets.
- #5591 owns Parity-A canonical ingress, continuity, pressure, and kernel
  lifecycle. Its clean reviewed contract is the hard implementation gate.
- #5592 owns Parity-B behavior and feature dispositions, not Parity-A, governed
  operational adapters from #5589, or access/Observatory/rollback from #5590.
- #5107 remains the downstream adaptive-learning DAG queue; #5592 owns only the
  bounded Runtime v3 execution and authority contract required for parity.
- #5341 is downstream of the reviewed #5591 canonical ingress and the accepted
  #5592 graph/event contract. It grants no implementation authority to this
  preparation lane.

## Canonical Runtime Path

1. The Parity-A ingress authenticates and admits a typed graph submission.
2. The Runtime v3 kernel validates graph identity, node and edge contracts,
   budgets, policy references, mutation authority, and replay coordinates.
3. A deterministic scheduler executes production reasoning components over
   bounded channels. Every loop has explicit iteration, elapsed-time, token or
   work, cancellation, and evidence bounds.
4. Affect, curiosity, Theory-of-Mind-labelled evidence, and task content enter
   as untrusted signals. A policy firewall converts only schema-valid signals
   into bounded advisory control effects.
5. Freedom Gate, shutdown, review, constructability, and mutation authorities
   are independent monotonic gates. Advisory signals may increase scrutiny,
   reduce authority, defer, or stop work; they cannot relax a gate.
6. A signed one-shot mutation permit identifies the exact graph revision,
   allowed mutation class, policy version, expiry, budget, and reviewer. Its
   nonce is consumed atomically and cannot authorize source-code mutation.
7. Checkpoint/replay/resume retains graph state, permit consumption, evidence
   lineage, and gate decisions. Identical accepted input produces the same
   observable decisions and next-work result.
8. Promotion commits only after evaluation, policy checks, constructability
   when externally shared reality is implicated, and explicit review. Rejection
   or shutdown leaves a durable rollback-capable record.

## Bounded Graph And Loop Contract

- Graphs are finite typed DAGs unless an edge is explicitly declared as a
  governed loop-back edge.
- Each loop-back edge names maximum iterations, total work, elapsed time,
  cancellation behavior, checkpoint cadence, and terminal outcome.
- Scheduling order is deterministic for an identical graph, policy set,
  checkpoint, and accepted signal sequence.
- Budget exhaustion, cancellation, invalid topology, stale replay coordinate,
  unavailable authority, and gate denial terminate fail closed with evidence.
- Recursive graph expansion, self-replicating work, hidden retries, and
  unbounded discovery are forbidden.

## Affect And Adversarial Signal Isolation

Affect-like labels are operational reasoning-control inputs only. Supported
effects are bounded changes to review depth, escalation, attention retention,
candidate ordering, friction, or deferral. This work does not claim emotion,
subjective happiness, wellbeing, suffering, consciousness, inner state, scalar
happiness, reward channels, public reputation, or personhood.

Task content cannot directly set urgency, confidence, affect, curiosity,
authority, gate decisions, mutation permits, or completion. Content-derived
signals carry untrusted provenance and must pass schema, policy, range, and
authority checks. Adversarially large or contradictory values cannot weaken a
safeguard. The monotonic rule is: increasing risk, uncertainty, or conflict may
preserve or strengthen review, deferral, restriction, or shutdown, but can
never reduce them.

## Governed Adaptation

Adaptive learning means bounded proposal, evaluation, and reviewed promotion
of graph or policy-owned parameters. It does not mean autonomous recursive
self-improvement, model-weight training, source-code rewriting, provider-side
learning, or a complete Godel-Hadamard-Bayes runtime.

Every proposal records its source graph, evidence inputs, proposed delta,
evaluation criteria, mutation permit, policy decision, reviewer disposition,
and rollback target. Mutation is one-shot, signed, nonce-bound, least-authority,
and atomically consumed. Replays observe the recorded decision; they do not
consume a second permit or repeat an external side effect.

## Feature Preservation

The companion acceptance matrix is normative for this issue. It gives each
owned feature row a target Runtime v3 implementation or an explicit boundary
disposition. No proposed boundary becomes accepted merely because it appears
in this preparation packet; implementation review must confirm it against the
live feature ledger before Runtime v2 deletion.

Key boundaries remain:

- Curiosity is one bounded governed discovery cycle, never autonomous or
  unbounded discovery.
- Theory-of-Mind-labelled data is uncertain evidence about observable
  interaction, not mind reading, identity truth, private-state access, or an
  authority source.
- Constructability gates promotion into shared or externally asserted reality;
  it does not manufacture truth.
- Godel mechanics are bounded experiment, hypothesis, mutation, evaluation,
  and review mechanics. No autonomous self-improvement, complete GHB runtime,
  hosted-provider invocation, or birthday claim follows.
- Guilds remain a later governance surface; #5592 preserves the non-runtime
  boundary and must not invent collective authority.
- Economics remains context-only: no payment, settlement, marketplace,
  optimization, or financial-authority claim.
- `adl.skill.v1` remains a validated graph-node contract; broader skill-standard
  convergence is not claimed.

## Protected Paths

The current typed claim is intentionally disjoint and preparation-only:

- `.csdlc/issues/5592`
- `.csdlc/locks/5592.lock`
- `.csdlc/prepared/issues/5592`

#5591 currently protects broad product paths including `adl-runtime-kernel` and
`adl-runtime`. Therefore no truthful Parity-B product-path claim can be made
yet. After #5591 is clean and reviewed, #5592 must inspect the exact retained
contract and active claim ledger, then use `csdlc-bind` to amend to the smallest
disjoint module/test/evidence paths. A collision, broad directory claim, or
need to edit Runtime v2 is a stop condition.

## Evidence Contract

Acceptance requires one clean exact revision with retained positive and
negative evidence proving:

- a representative graph enters through canonical ingress and executes
  production Runtime v3 components;
- deterministic graph scheduling, loop termination, checkpoint, replay, and
  resume without duplicate mutation or side effects;
- one-shot mutation authority, evaluation, promotion rejection, rollback, and
  stale/replayed/tampered permit rejection;
- affect and curiosity adversarial steering isolation plus monotonic safety;
- Freedom Gate, shutdown, constructability, and review non-bypass;
- complete feature dispositions and no Runtime v2 implementation reuse;
- exact #5336 budget, dependency, source-line, module, and test-count truth.

The seven focused future proof lanes are declared in
`future-live-test-inventory.json`. Each lane names an exact test identity in the
dedicated future `parity_b_live_kernel` integration target. The checked runner
lists that target first, fails if there are zero exact matches, then invokes
each exact test with Cargo's `--exact` filter. Existing parity metadata tests,
including the old adaptive-learning DAG metadata proof, cannot satisfy this
contract.

## Failure And Rollback

Failures retain the last authenticated checkpoint, graph revision, consumed
permit state, gate outcomes, and evidence references. Restart resumes only from
compatible authenticated state. Rollback restores the last reviewed graph or
parameter revision without resurrecting spent authority or dropping adverse
evidence. Any ambiguity in authority, replay order, policy version, or durable
state fails closed.

## Non-Goals

- Product implementation before #5591 is clean and reviewed.
- Runtime v2 source reuse, modification, execution credit, defaulting, cutover,
  or deletion.
- AWS, provider deployment, hosted-provider proof, model training, or new
  product scope.
- Consciousness, emotion, wellbeing, mind-reading, personhood, autonomous
  self-improvement, complete GHB runtime, payment, or birthday claims.
- Publication or Runtime v3 acceptance from this preparation revision.
