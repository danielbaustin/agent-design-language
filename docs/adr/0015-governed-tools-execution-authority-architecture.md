# ADR 0015: Governed Tools Execution Authority Architecture

- Status: Accepted
- Date: 2026-05-04
- Related issue: #2717
- Related milestone: v0.90.5
- Builds on: ADR 0014

## Context

v0.90.5 introduces ADL's first governed-tools architecture. Before this
milestone, ADL had durable runtime, citizen-state continuity, and
contract-market boundaries, but it did not have one accepted architecture
record for model tool proposals, portable tool description, ADL-native
authority contracts, registry binding, policy mediation, governed execution,
and reviewable trace evidence.

The implementation, feature docs, demos, and review-entry package already
exist. What was missing was one durable decision record that makes the boundary
between description, authority, mediation, execution, and evidence explicit.

This ADR is grounded in:

- `docs/adr/0014-contract-market-architecture.md`
- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/WBS_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_DOCS_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/features/TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`
- `docs/milestones/v0.90.5/features/UTS_PUBLIC_SPEC_AND_CONFORMANCE.md`
- `docs/milestones/v0.90.5/features/ACC_AUTHORITY_AND_VISIBILITY.md`
- `docs/milestones/v0.90.5/features/TOOL_REGISTRY_AND_COMPILER.md`
- `docs/milestones/v0.90.5/features/GOVERNED_EXECUTION_AND_TRACE.md`
- `docs/milestones/v0.90.5/features/MODEL_TESTING_AND_FLAGSHIP_DEMO.md`
- `docs/milestones/v0.90.5/features/AGENT_COMMS_v1.md`
- `adl/src/uts.rs`
- `adl/src/uts_conformance.rs`
- `adl/src/acc.rs`
- `adl/src/tool_registry.rs`
- `adl/src/uts_acc_compiler.rs`
- `adl/src/policy_authority.rs`
- `adl/src/freedom_gate.rs`
- `adl/src/runtime_v2/`

This ADR does not introduce new runtime behavior. It records the architecture
that v0.90.5 already implements and validates.

## Decision

ADL adopts a governed execution authority stack for tools in v0.90.5.

At the v0.90.5 boundary, tool handling is not ordinary model function-calling.
It is a bounded architecture where:

- the model may propose;
- UTS may describe;
- the registry may bind;
- the compiler may normalize and construct ACC or reject;
- policy and Freedom Gate may mediate;
- the governed executor may act only after approval; and
- trace/redaction surfaces record what happened.

This decision requires:

1. Tool calls are proposals, not actions

   A model-emitted tool call is untrusted input. Valid JSON, schema
   compatibility, natural-language urgency, or model confidence do not
   constitute authority or execution permission.

2. UTS is portable description, not runtime authority

   Universal Tool Schema v1.0 is the portable, model-facing description layer.
   It may describe names, schemas, side-effect classes, determinism, replay,
   sensitivity, and execution hints. It does not grant actor authority,
   adapter permission, execution permission, or visibility rights.

3. ACC is the ADL-native authority contract

   ADL Capability Contract v1.0 is the runtime-facing governance layer for
   capability exercise. Actor identity, grantor attribution, delegation,
   standing, privacy, visibility, replay posture, and failure posture must be
   explicit here or the action fails closed.

4. Registry binding is explicit and deterministic

   Known tools, approved adapters, version matches, capability matches, and
   dry-run posture must be resolved through the registry. Unknown tools,
   mismatched adapters, inactive bindings, or unsafe dry-run posture are
   deterministic rejection conditions.

5. Compilation is a governance step, not a convenience transform

   The UTS-to-ACC compiler validates, normalizes, and either emits a bounded
   ACC or a deterministic rejection record. It must not silently coerce unsafe,
   ambiguous, overbroad, or under-attributed proposals into executable actions.

6. Policy authority and Freedom Gate mediate candidate actions before execution

   Policy evaluation and Freedom Gate are mandatory control layers between ACC
   construction and action. They decide whether a candidate is allowed, denied,
   deferred, challenged, or escalated. No governed action may bypass this
   mediation path.

7. The governed executor acts only on approved capability exercise

   The executor may run only registered, approved, ACC-backed actions. Direct
   model-to-tool execution is forbidden. Denied and deferred paths are first-
   class outcomes and must remain reviewable evidence.

8. Trace and redaction are part of runtime truth

   Proposal, normalization, ACC construction, policy evidence, Freedom Gate
   decision, selected action, refusal, and redaction posture must be
   reconstructable from reviewable artifacts. Trace must support accountability
   without leaking private arguments, secrets, protected state, or unsafe local
   path information.

9. Dangerous classes fail closed by default

   Process, arbitrary network, destructive, and exfiltration-shaped tool
   proposals do not become executable merely because they are well-formed. The
   milestone proof requires denial evidence and bounded dry-run or fixture
   paths where explicitly allowed, not generalized ambient authority.

10. Model testing is architecture evidence, not marketing garnish

   Governed-tools claims are not complete without proving how actual models
   behave against the authority stack. Benchmarking, local-model evaluation,
   dangerous negative cases, and the flagship demo exist to show that the
   architecture is meaningful in contact with real model behavior.

11. Agent communication may carry invocation, but not bypass governance

   The first landed ACIP tranche may carry invocation contracts, decision refs,
   and reviewable message chronology. It does not replace or bypass the
   governed execution stack for tool authority.

## Rationale

ADL needed a tool architecture that was reviewable, bounded, and different in
kind from ordinary function-calling stories. The shipped v0.90.5 surfaces
already established the right shape:

- proposals are explicit
- schemas are portable
- authority is ADL-native
- binding is deterministic
- compilation is accountable
- mediation is mandatory
- execution is governed
- trace is evidence
- dangerous categories fail closed
- demos and benchmarks prove the stack against real model behavior

Without one ADR, reviewers would have to reconstruct that architecture from
many feature docs, implementation modules, proofs, and review surfaces. ADR
0015 makes the milestone legible as one coherent authority architecture and
preserves the main non-claims that keep the milestone disciplined.

## Consequences

### Positive

- Gives v0.90.5 one durable architecture record for governed tool authority.
- Makes the line between UTS portability and ACC authority explicit.
- Makes the line between proposal and action explicit.
- Preserves the boundary between communication substrate and execution
  authority.
- Consolidates schema, registry, compiler, policy, gate, executor, and trace
  responsibilities into one reviewer-facing surface.
- Creates a stable handoff from the contract-market milestone into governed
  tools without implying payment or market semantics are tool authority.

### Negative

- Future changes to UTS meaning, ACC fields, binding semantics, policy
  authority, Freedom Gate behavior, executor posture, or trace/redaction rules
  now carry architectural weight.
- Later ACIP, A2A, identity, or economics work must preserve or explicitly
  supersede this boundary instead of informally blurring it.
- Public-facing language about “tool calling” must remain disciplined; it can
  no longer casually imply execution authority from schema validity alone.

## Alternatives Considered

### 1. Leave v0.90.5 documented only by feature docs and demos

Pros:

- Less ADR writing.

Cons:

- Reviewers would have to reconstruct one authority model from many docs and
  proof packets.
- The distinction between schema description, authority, and execution would be
  easier to blur.
- The milestone’s most important architectural idea would remain scattered.

### 2. Treat UTS as the main authority boundary

Pros:

- Simpler apparent story for external readers.

Cons:

- False to the implementation.
- Blurs portable schema description with runtime permission.
- Weakens the explicit ADL-native authority stack that the milestone actually
  shipped.

### 3. Fold governed-tools authority into ADR 0014

Pros:

- Fewer ADR files.

Cons:

- ADR 0014 is about the contract-market architecture, not governed tool
  authority.
- Folding them together would blur economics and tool governance.
- v0.90.5 is substantial enough to deserve its own accepted boundary.

## Validation Evidence

The decision is supported by:

- the v0.90.5 tool threat-model, UTS, ACC, registry/compiler, and governed
  execution feature docs
- the Rust implementation in `adl/src/uts.rs`, `adl/src/uts_conformance.rs`,
  `adl/src/acc.rs`, `adl/src/tool_registry.rs`,
  `adl/src/uts_acc_compiler.rs`, `adl/src/policy_authority.rs`, and
  `adl/src/freedom_gate.rs`
- the feature-proof coverage record and review-entry package
- the dangerous negative suite, model benchmark, local-model evaluation, and
  flagship demo proof surfaces
- the landed first-level ACIP feature docs where governed invocation references
  the authority stack without bypassing it

## Non-Claims

This ADR does not claim:

- that UTS is a public standard
- that UTS validity grants execution authority
- arbitrary shell or unrestricted network execution by model output
- production secret-management or unrestricted credential use
- payment rails, billing, legal contracting, or inter-polis economics
- full ACIP 1.0 completion
- A2A or public-network agent transport
- full v0.91 moral/cognitive-being substrate
- full v0.91.1 identity, capability, memory, ToM, ANRM/Gemma, or learning
  follow-on work

## Notes

This ADR records the architecture that v0.90.5 already shipped as a governed
execution authority stack. Future ADRs may refine ACIP, A2A, planning/review
workflow artifacts, identity-bearing invocation, secure external transport, or
later cognitive and moral governance layers, but those later decisions should
cite this ADR when they build on the v0.90.5 governed-tools boundary.
