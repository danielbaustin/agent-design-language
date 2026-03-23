

# Layer 8 Provider Implementation

## Purpose

This document captures the current implementation status and intended architecture of the **Layer 8 provider** in ADL.

The term **Layer 8** is used here to describe the reasoning-facing provider layer that sits above deterministic runtime execution and below external model/tool backends. It is not merely an adapter to a model API. It is the place where ADL should eventually enforce disciplined inference contracts, provider capability boundaries, and replayable reasoning surfaces.

For now, the goal of this document is to record:

- what already exists in the repository
- what appears partially implemented
- what is still missing
- what should likely belong to v0.85 and later milestones

---

## Working Definition

The Layer 8 provider is the ADL subsystem responsible for connecting higher-level reasoning and execution surfaces to external inference providers in a bounded, inspectable way.

In a mature form, it should support:

- structured prompt/inference contracts
- provider abstraction
- capability negotiation
- deterministic prompt construction where applicable
- replayable inference traces
- normalized response handling
- bounded fallback/routing behavior

That is broader than a simple API wrapper.

---

## Current Repository Status

### What appears to exist now

Based on the current repository state, the following infrastructure appears to be present:

- `swarm/src/provider.rs`
- integration with execution/runtime surfaces
- provider-facing logic separated from some higher-level runtime code

This strongly suggests that the basic **provider abstraction** already exists.

That is a meaningful milestone, because it implies ADL no longer needs to bind core execution logic directly to one external model/vendor surface.

### What this likely means in practice

The current provider layer likely already covers some combination of:

- model invocation
- tool-call mediation
- response normalization
- limited retry/error handling
- isolation of backend-specific behavior

This is enough to say the Layer 8 groundwork is in place.

---

## What Does Not Yet Look Finished

The more ambitious Layer 8 vision appears to remain incomplete.

In particular, the repository does **not yet clearly demonstrate** a full reasoning-provider contract with all of the following properties:

- deterministic inference artifacts
- replayable prompt/inference traces
- capability negotiation between providers
- explicit inference contracts as first-class artifacts
- bounded fallback/routing policy
- confidence-aware provider behavior

So the current state is best understood as:

- **provider abstraction exists**
- **full Layer 8 reasoning discipline does not yet exist**

That distinction matters.

---

## Current Maturity Assessment

A practical way to describe current maturity is:

### Implemented or mostly implemented

- provider abstraction
- execution/runtime integration points
- isolation of provider-specific logic

### Partial or unclear

- provider contract formalization
- structured inference boundaries
- normalized capability description

### Not clearly implemented yet

- deterministic inference artifacts
- replayable inference traces
- provider capability negotiation
- fallback/routing policy
- confidence-aware inference selection

A fair summary is that the Layer 8 implementation is **foundational but not mature**.

---

## Relationship to the Runtime Stack

The current architecture appears to be evolving toward a stack like this:

1. Authoring / ADL surfaces
2. Execution runtime
3. Gödel loop and ObsMem
4. Provider layer
5. External model/tool backends

That is a sensible direction.

It means the provider layer is no longer just “LLM plumbing.” It becomes the controlled boundary through which reasoning surfaces access model capabilities.

This is exactly where Layer 8 belongs.

---

## Why This Matters

Without a disciplined provider layer, ADL risks having:

- opaque inference behavior
- model-specific assumptions leaking into runtime logic
- poor replayability
- ad hoc prompt construction
- weak reviewability for reasoning steps

With a disciplined Layer 8 provider, ADL can instead move toward:

- bounded inference contracts
- explicit provider capability assumptions
- inspectable prompt/inference artifacts
- replayable reasoning traces
- verifiable inference surfaces

This is strategically important because ADL’s public claims are not just about orchestration. They are about **dependable execution** and **verifiable inference**.

---

## Relationship to Verifiable Inference

Layer 8 is one of the places where the claim of **verifiable inference** must become real.

If the provider layer is only a thin API wrapper, then model behavior remains largely opaque.

If the provider layer becomes a proper contract surface, then ADL can start answering questions like:

- What prompt/inference structure was sent?
- What provider capabilities were assumed?
- Why was this provider chosen?
- What response was normalized into runtime artifacts?
- Can the same reasoning step be replayed or at least inspected later?

Those questions are central to ADL’s credibility.

---

## Relationship to the Gödel Loop and ObsMem

The Layer 8 provider should eventually interact cleanly with:

- Gödel hypothesis generation
- mutation suggestion
- bounded experiment planning
- ObsMem-informed retrieval and confidence ranking

But it should do so through explicit contracts, not through hidden prompt magic.

A future mature relationship could look like this:

- ObsMem retrieves prior cases
- higher-level runtime selects bounded reasoning task
- Layer 8 provider constructs inference request under a known contract
- response is normalized into an inspectable artifact
- downstream evaluation decides adopt/reject/review

That is a better design than letting external model behavior leak directly into runtime control flow.

---

## Relationship to AEE

The Adaptive Execution Engine should not depend on vague model improvisation.

If AEE evolves in v0.85+, it will likely need Layer 8 to provide:

- bounded strategy-selection requests
- explicit confidence or rationale artifacts where appropriate
- provider capability boundaries
- replayable or at least inspectable evidence of why a recommendation occurred

In that sense, Layer 8 is part of the discipline that prevents AEE from becoming “adaptive magic.”

---

## What v0.8 Likely Needs

For v0.8, the provider layer only needs to be good enough to support:

- stable runtime execution
- clean provider abstraction
- bounded integration with the rest of the system

That means the current infrastructure is probably sufficient for v0.8 so long as it remains stable and does not leak too much provider-specific complexity upward.

v0.8 does **not** need the full Layer 8 vision.

---

## What v0.85 Should Likely Add

v0.85 is a more natural place for explicit Layer 8 maturation.

Likely near-term goals include:

### 1. Provider contract definition

Define a first-class contract for provider requests and normalized responses.

### 2. Capability description

Represent provider capabilities explicitly, such as:

- tool support
- structured output support
- reasoning depth assumptions
- determinism/temperature boundaries

### 3. Replayable inference artifacts

Record enough structure that inference steps can be inspected and, where possible, replayed.

### 4. Prompt/inference normalization

Make prompt construction and response normalization more deterministic and reviewable.

### 5. Provider test harness

Add strong tests for provider behavior, capability flags, and contract compliance.

---

## Suggested v0.85 Deliverables

A concrete v0.85 Layer 8 work package could include:

- `provider_contract.rs`
- `provider_capabilities.rs`
- `inference_trace.rs`
- `provider_test_harness.rs`
- normalized provider artifacts under a versioned schema

The exact filenames may differ, but the architectural idea is:

- contract
- capability description
- traceability
- tests

That would move the provider layer from “useful plumbing” to a true Layer 8 reasoning boundary.

---

## Design Principles

If Layer 8 becomes a mature subsystem, it should follow these principles:

1. **No hidden prompt magic**
   Prompt/inference structure should be inspectable and attributable.

2. **No provider leakage into core runtime**
   Runtime logic should depend on contracts, not vendor quirks.

3. **No fake determinism claims**
   The system should distinguish clearly between deterministic construction and stochastic model behavior.

4. **Replayability where possible**
   If exact replay is not possible, traceability and inspection should still be strong.

5. **Bounded fallback behavior**
   Provider routing or fallback should be explicit rather than ad hoc.

6. **Capability honesty**
   The system should not assume provider features that are not represented explicitly.

---

## Proposed Architecture Sketch

A cleaner long-term architecture would look like:

- authoring surfaces define intent
- execution runtime defines bounded tasks
- Layer 8 provider translates bounded tasks into provider requests
- provider capabilities constrain what may be asked
- normalized response artifacts return into runtime evaluation
- ObsMem/Gödel/AEE consume those artifacts through explicit structures

This keeps the reasoning boundary clear and reviewable.

---

## Open Questions

This area still leaves important open questions:

- What should the canonical provider contract artifact look like?
- How much provider capability data should be static versus runtime-discovered?
- How should ADL record inference traces without overcommitting to exact replay?
- How should Layer 8 interact with confidence/rationale artifacts from future Gödel/AEE work?
- What parts of the provider layer belong in `swarm/` versus future crates?
- How should fallback/routing policy be bounded and testable?

These should remain open until the provider layer becomes an explicit milestone target.

---

## Current Bottom Line

The Layer 8 provider work is best described as:

- **foundational infrastructure exists**
- **reasoning-provider maturity is still incomplete**
- **good enough for v0.8 plumbing**
- **not yet sufficient for the full verifiable-inference vision**

So the current status is not failure. It is partial completion.

The provider abstraction appears to exist and to be wired into the runtime. What remains is the more interesting part: turning that provider layer into a disciplined reasoning boundary with explicit contracts, traces, and capability control.

That is likely a v0.85 and v0.9 story rather than a v0.8 blocker.