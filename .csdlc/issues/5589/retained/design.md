# Issue #5589 Runtime v3 Parity-C Design

Status: preparation complete; implementation blocked on a clean reviewed #5591 Parity-A contract and a collision-free typed claim amendment.

## Objective

Replace the Runtime v3 operational placeholders owned by Parity-C with small
production component adapters that execute admitted work through the canonical
`adl-runtime-kernel`. The accepted result must prove governance before
actuation, attenuating delegation, bounded resources and cancellation,
provider/scheduler/tool execution, identity and private-state authority,
qualified time, checkpoint and lifelog continuity, and fail-closed shutdown.

No fixture, metadata projection, library-only helper, fixed bootstrap graph, or
`DegradedOperationExecutor` receives parity credit. Each receives zero parity credit
in the acceptance ledger.

## Dependency And Execution Gate

Preparation may complete on this branch. Product implementation may begin only
after #5591 has all of the following at one exact revision:

1. a clean typed C-SDLC review record for the Parity-A ingress and service
   contract;
2. no actionable review findings;
3. a stable canonical ingress/service boundary consumable by Parity-C;
4. a narrowed or released protected-path claim that permits a disjoint #5589
   implementation claim.

The observed #5591 branch currently has product commits but its committed typed
record remains `phase: bound` with no review assignment or review record. That
is not a clean reviewed Parity-A contract, so it is an implementation blocker.

## Architecture Boundary

- `adl-runtime-kernel` owns Runtime v3 execution, governance enforcement,
  provider/tool ports, bounded scheduling, operational state, and continuity.
- ADL v2 owns language and portable plan/event contracts, not runtime
  actuation.
- C-SDLC v2 owns issue cards, claims, review, publication, and closeout, not
  Runtime execution.
- #5591 owns canonical ingress, kernel lifecycle, topology/backpressure,
  configuration, and base continuity mechanics.
- #5592 owns reasoning and adaptive cognition.
- #5590 owns ACIP/A2A/cloud boundary, remote access, Observatory, weather,
  guardian/soak, and rollback.
- #5589 owns only the governed operational adapters and authority/state
  behavior listed in the matrix.

## Runtime Shape

The accepted Parity-A ingress admits a signed, typed work envelope into the
kernel. A governance coordinator verifies identity, authority, delegation,
resource, cancellation, and Freedom Gate/AEE decisions before dispatch. The
bounded scheduler selects production provider and governed-tool ports. Every
actuation and terminal disposition is tied to qualified time, a checkpoint
transition, and a non-authoritative lifelog event. Private state is read or
written only under the admitted identity and capability scope.

Production adapters must be explicit components with declared service
contracts. They may wrap maintained COTS libraries or existing production
stores, but they must not smuggle policy into transport, use a synthetic test
executor as operational authority, or turn provider output into an unsigned
actuation decision.

## Gate-Before-Actuation

For every admitted operation, the observable ordering is:

1. authenticate citizen/agent identity and resolve private-state scope;
2. validate the signed capability and attenuating delegation chain;
3. reserve bounded resources and attach cancellation/idempotency identity;
4. evaluate Freedom Gate and AEE policy against the exact proposed actuation;
5. qualify time and persist the pre-actuation checkpoint;
6. schedule the provider or governed tool through a production adapter;
7. persist terminal checkpoint and append a redacted lifelog event;
8. release resources and propagate cancellation or shutdown truthfully.

Missing, expired, revoked, widened, replayed, ambiguous, corrupt, or
unauthorized authority fails before provider/tool invocation. A provider or
tool cannot bypass governance by retry, resume, scheduler requeue, or shutdown
recovery.

## Continuity And State Authority

- Citizen identity is the stable authority key; display names and provider
  session identifiers are non-authoritative.
- Private state is partitioned by identity and capability scope, encrypted or
  otherwise protected by the selected production store, and redacted from
  retained logs.
- Checkpoints are authoritative for execution recovery and include identity,
  delegation, resource, cancellation, idempotency, policy, provider/tool, and
  qualified-time references.
- Lifelog entries are append-only autobiographical evidence. They may refer to
  checkpoints but never restore execution state or grant authority.
- Recovery verifies checkpoint authenticity and current revocation state before
  re-admission; it must not repeat completed side effects.
- Lifelog failure cannot corrupt a valid checkpoint, and checkpoint failure
  prevents readiness rather than silently degrading continuity.

## Provider, Scheduler, And Tool Boundary

At least one configured production provider and one governed tool must execute
representative admitted work. The scheduler is bounded and deterministic for
identical accepted state. Provider/tool adapters classify timeout, cancellation,
quota, auth, policy refusal, malformed output, and unavailable service without
claiming degraded success as parity. Credentials remain outside retained
evidence and are not required for deterministic local negative proof.

## Protected Paths

The active preparation claim is intentionally disjoint and contains only:

- `.csdlc/issues/5589`
- `.csdlc/locks/5589.lock`
- `.csdlc/prepared/issues/5589`
- `.csdlc/evidence/5589`

It grants no product authority. After #5591's reviewed contract and claim gate
clear, a typed `csdlc-bind` claim amendment must select the smallest paths from
the matrix. No parent directory already reserved by #5591 may be added. If a
disjoint path set cannot be proven, implementation stops for operator routing.

## Proof Contract

Parity requires a live initialized kernel process using production component
adapters. The exact-revision proof packet must include:

- signed gate-before-actuation success and denial/revocation/quarantine cases;
- attenuating delegation plus widening, expiry, replay, and cross-identity
  rejection;
- resource exhaustion, cancellation, retry, idempotency, and cleanup proof;
- live multi-agent provider/scheduler/governed-tool execution;
- identity/private-state isolation and unauthorized disclosure negatives;
- qualified-time rollback/staleness refusal;
- checkpoint/lifelog restart continuity and corruption/failure separation;
- provider timeout/auth/quota/malformed-output and scheduler cancellation cases;
- graceful shutdown with no post-revocation or post-cancellation actuation;
- explicit inventory showing zero `DegradedOperationExecutor`, fixture-only,
  or synthetic-adapter parity credit;
- Runtime v3 line/test budget and duplicate/placeholder deletion evidence.

## Stop Conditions

Stop before product edits if #5591 lacks current clean review truth, its
contract changes, or its protected claim overlaps the proposed #5589 paths.
Stop execution on any degraded/fixture-only proof, skipped release gate,
governance bypass, continuity ambiguity, private-state disclosure, credential
requirement for deterministic proof, AWS dependency, Runtime v2 edit, or budget
breach without a reviewed exception.
