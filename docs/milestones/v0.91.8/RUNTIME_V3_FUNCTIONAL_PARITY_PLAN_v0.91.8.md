# Runtime v3 Functional Parity Plan

Status: accepted execution ledger. This document records the parity plan and
its bounded v0.91.8 outcome. It does not by itself authorize further cutover,
deployment, deletion, or v0.92 activation beyond the exact evidence cited here.

## Why This Exists

The retained v0.91.7 parity packet classified nine Runtime v3 capability
groups as `runtime_v3_fixture_only`, one reasoning group as a live-equivalent
fixture, seven groups as accepted intentional divergences, and one migration
surface as retained Runtime v2 behavior behind an adapter. Those packets are
useful design evidence, but they do not prove that a configured Runtime v3
process can accept and execute representative domain work across all planned
features.

v0.91.8 closed the bounded parity gap through #5591, #5592, #5589, #5590,
#5361, and the integrated WP-16 quality gate. Later consumers must cite the
retained evidence rather than treating this planning text as proof.

## Canonical Runtime And Budget

- `adl-runtime-kernel` is the canonical Runtime v3 process.
- `adl-runtime` is transitional evidence/compatibility code. Surviving guardian
  or protocol behavior must be absorbed behind the canonical runtime contract
  or explicitly owned outside Runtime v3; duplicate implementations are then
  deleted.
- #5336 pins the exact source trees, measurement rule, and ownership boundary
  in `BASELINE_AND_OWNERSHIP_v0.91.8.md`. The pinned owner report measures
  12,209 physical source lines and 189 Rust test functions. The source posture
  is a bounded 209-line exception above the 12,000 reviewed target, while the
  10,000-line challenge remains active and the exclusive 1,000-test ceiling
  passes. The exception does not authorize feature or proof reduction.
- Feature work must pay for growth by removing duplicate, placeholder, or
  degraded code, or obtain a later bounded reviewed exception. Four lanes must
  not produce four new frameworks.

## What Counts As Live

A capability counts toward cutover only when all of these are true:

1. a real initialized `adl-runtime-kernel` process accepts representative work
   through the canonical secure typed domain ingress;
2. production component code executes, rather than only a test fixture,
   library helper, fixed bootstrap graph, metadata projection, or degraded
   executor;
3. deterministic positive and negative evidence is retained at an exact
   revision;
4. stateful work proves checkpoint, graceful shutdown, and recovery behavior;
5. authority, redaction, backpressure, and bounded-termination failures close
   safely;
6. the HTML Observatory or another declared consumer can inspect the resulting
   runtime-owned state without receiving mutation authority.

## Parallel Lanes

| Lane | Issue | Owned groups | Primary output |
| --- | ---: | --- | --- |
| A | #5591 | kernel lifecycle; topology/backpressure; service contracts/configuration; continuity/replay/recovery; canonical domain ingress | One live initialized execution path consumed by all later lanes |
| B | #5592 | reasoning graphs/loops; adaptive DAG; affect; curiosity; Constructability; Godel mechanics; guild; economics context; skill-standard preservation | Live governed cognition and complete feature dispositions |
| C | #5589 | Freedom Gate/AEE; delegation/resources; agents/Shepherd/providers/scheduler; private state; identity/memory; Chronosense/checkpoint/lifelog | Production operational adapters replacing degraded placeholders |
| D | #5590 | ACIP/A2A/cloud boundary; secure local/remote access; Observatory; weather/telemetry; guardian/soak/rollback | Secure operable runtime and live HTML Observatory proof |

The global writable-actor cap remains four. Lanes B, C, and D consume Lane A's
reviewed ingress/service-contract revision. That dependency order is now
retained as completed execution evidence: WP-16 classifies #5591, #5592, #5589,
and #5590 as working-code outcomes, while keeping broader v0.92 adaptive
learning and public cloud claims out of scope.

## Ten Proof Groups

| Group | Historical posture | Required v0.91.8 proof | Owner |
| --- | --- | --- | ---: |
| 1. Kernel lifecycle | fixture-only | Guardian-launched initialized process, component startup/shutdown, fatal-child and restart behavior | #5591 |
| 2. Topology and backpressure | fixture-only | Real admitted work across typed bounded channels, pressure serialization, graceful stop | #5591 |
| 3. Service contracts and configuration | fixture-only | Init-file-driven topology, capability negotiation, invalid/ambiguous configuration refusal | #5591 |
| 4. Continuity, replay, and recovery | fixture-only | Real work checkpoint, deterministic resume, corruption quarantine, missing-state restart | #5591 |
| 5. Reasoning and adaptive learning | reasoning fixture plus adaptive fixture | Submitted graph/loop execution, bounded termination, signed mutation, durable evidence, rollback | #5592 |
| 6. Governance, Freedom Gate, and AEE | fixture-only | Signed gate-before-actuation on real work, appeals/revocation/quarantine negative paths | #5589 |
| 7. Delegation and resources | fixture-only | Attenuating delegation, cancellation/resource cleanup, retry/idempotency bounds on live work | #5589 |
| 8. Agents, providers, and scheduler | fixture-only/degraded | At least one admitted agent executes scheduled governed provider/tool work through production adapters | #5589 |
| 9. ACIP/A2A/cloud network boundary | fixture-only | Authenticated secure local/remote contract, no hard-coded IPs, credential-free external-route boundary | #5590 |
| 10. Observatory, guardian, soak, and rollback | opt-in/retained | HTML Observatory consumes real runtime state; guardian, pressure stop, restart, soak, and selector rollback pass | #5590 |

WP-16 retained the integrated Runtime v3 proof as a release input:
`docs/milestones/v0.91.8/evidence/wp16/QUALITY_GATE.md` records the Runtime v3
locked all-target suite as `pass`, and
`docs/milestones/v0.91.8/evidence/wp16/ISSUE_OUTCOME_AUDIT.md` records
#5589, #5590, #5591, and #5592 as working code. The release-tail evidence still
separates runtime parity from v0.92 feature expansion, public cloud hosting,
subjective affect claims, and distributed relocation.

## Feature Preservation Overlay

Every implemented row in `docs/planning/ADL_FEATURE_LIST.md` and every v0.91.7
feature document must remain classified as exactly one of:

- `live_runtime_v3`: production behavior executes through canonical ingress;
- `owned_outside_runtime`: the feature is implemented by ADL v2, C-SDLC v2,
  Observatory, or another named owner and has no Runtime v2 deletion edge;
- `boundary_or_non_claim`: the artifact intentionally defines a safe boundary
  rather than runtime behavior;
- `deferred_with_owner`: not required for cutover and has an explicit owner and
  downstream milestone;
- `blocker`: deletion/cutover cannot proceed.

No feature may receive `live_runtime_v3` credit from metadata alone. No Runtime
v2 file is deletion-eligible while it remains the last implementation of a
feature that is not explicitly deferred or boundary-only.

## Dependency Gate

1. #5336 approved the pinned source-tree baseline, architecture, ownership
   boundary, budget method, and this ledger.
2. #5591 froze canonical ingress and service contracts.
3. #5592, #5589, and #5590 implemented on disjoint surfaces after #5591.
4. #5341 connected ADL v2 plans/events to the reviewed Runtime v3 ingress;
   #5349 aligns provider/governed-tool ports.
5. WP-11 #5350 ran exact-revision parity across ADL v2 and the Runtime
   v3 proof groups.
6. Runtime v3 acceptance #5361 consumed the four parity lanes and WP-11.
7. WP-12 soak and reversible cutover consumed closed #5361 acceptance.
8. WP-13 deletion followed cutover and current C-SDLC v2 acceptance.

## Non-Claims

- No AWS execution is required or authorized by this plan.
- Remote/GPU delivery is not a cutover claim unless separately approved.
- No HTTP-only runtime access is permitted.
- Hosts, IP addresses, credentials, and discovery ports are configuration or
  listener truth, never hard-coded constants.
- This plan does not claim subjective affect, consciousness, suffering,
  happiness, or unbounded self-improvement.
