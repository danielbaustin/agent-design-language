# Design - v0.90.2

## Design Center

Runtime v2 v0.90.2 is about the first bounded CSM run plus trustworthy failure
behavior around that run.

The design center is:

- CSM run packet
- manifold boot and citizen admission
- governed episode execution
- resource scheduling under pressure
- local snapshot, rehydrate, and wake continuity
- Observatory packet and operator report
- invariant expansion
- stable violation artifacts
- recovery decision records
- quarantine state
- operator review reports
- governed adversarial hook
- security-boundary evidence
- release evidence that ties the hardening proof together

## Hardening Layers

### First CSM Run

The milestone should run `proto-csm-01` in bounded form:

- boot one manifold
- admit two ordinary worker citizens
- execute one governed episode
- schedule work under a small resource constraint
- route at least one action through the Freedom Gate
- reject one invalid action
- snapshot, rehydrate, and wake locally
- emit Observatory-visible artifacts

This run is the spine of the milestone. Hardening work should attach to this
spine instead of becoming a disconnected set of negative tests.

### Invariant Expansion

v0.90.2 should expand Runtime v2 invariant coverage beyond the v0.90.1 happy
path. The tests and artifacts should cover normal execution, failure paths,
recovery attempts, and cases where recovery must be refused.

### Violation Artifacts

Invariant failures should emit stable, reviewer-readable artifacts. A violation
artifact should identify the attempted action, violated invariant, actor, kernel
stage, decision, trace anchors, and resulting state.

### Recovery And Quarantine

Recovery is allowed only when the kernel can prove the resumed state preserves
declared invariants. Otherwise the manifold, citizen, or episode should enter a
quarantine state that preserves evidence and blocks unsafe execution.

### Operator Review

Operators should be able to inspect the current runtime state, violation
history, recovery eligibility, quarantine rationale, and security-boundary
evidence without reconstructing the milestone by hand.

### Security Boundary

Security proof in v0.90.2 should defend the polis, not define Runtime v2. One
distinct governed adversarial hook should be included so hostile pressure has a
safe, named entry point. The hook should run under explicit operator rules,
attempt one bounded abuse or boundary-crossing scenario, preserve all evidence,
and prove containment or rejection through ordinary Runtime v2 mechanisms.

This hook should not grow into a full red/blue/purple security ecology in
v0.90.2. It is a defensive proof surface attached to the first CSM run.

## Required Artifact Families

- `csm_run/*.json`
- `observatory/*.json`
- `observatory/*.md`
- `invariants/*.json`
- `violations/*.json`
- `recovery/*.json`
- `quarantine/*.json`
- `operator_review/*.md`
- `security_boundary/adversarial_hook_*.json`
- `security_boundary/*.json`
- `release_evidence/*.md`

Exact paths can change during implementation, but the artifact families should
remain visible in the planning package and demo matrix.

## Compression Design

The milestone should be planned for compression from the start:

- front-load contracts and fixture packets before broad implementation
- run CSM Observatory packet/report work in parallel with runtime implementation
- keep one integrated first-run demo as the central proof target
- use focused validation for docs, fixtures, and report-only changes
- require fuller validation for kernel, invariant, recovery, security, and
  release changes
- keep release evidence continuously updated instead of rebuilding it at the end

## Later-Band Boundary

v0.91 adds moral, emotional, kindness, humor, wellbeing, cultivation,
harm-prevention, and civic substrate.

v0.92 adds the first true birthday, identity/capability rebinding, richer
memory, and migration semantics.

v0.90.2 hardens the substrate they inherit. It does not claim their outcomes.
