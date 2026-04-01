# Release Notes: v0.87

## Metadata
- Product: `Agent Design Language (ADL)`
- Version: `0.87`
- Release date: `TBD`
- Tag: `TBD`

## Purpose
Capture the release notes for `v0.87` as a truthful, implementation-aligned summary of the milestone.

This is a substrate release. The notes should emphasize:
- what became more coherent and credible
- which foundational surfaces are now real
- what remains intentionally out of scope for later milestones

No statement in this document should imply shipped behavior that is not backed by implementation, demos, or reviewable proof surfaces.

# `Agent Design Language (ADL)` `0.87` Release Notes

## Summary
`v0.87` is the milestone where ADL consolidates the bounded cognitive system from `v0.86` into a more coherent, deterministic, and externally credible substrate. This release strengthens the foundations that later milestones will depend on by tightening trace truth, provider portability, shared-memory coherence, operational skills, and control-plane stability.

Rather than expanding into identity, governance, or higher-order cognition, `v0.87` focuses on making the existing system more correct, more inspectable, and easier for internal and external reviewers to evaluate truthfully.

## Highlights
- Trace v1 becomes a first-class substrate surface for reconstruction-oriented execution truth.
- Provider handling is normalized around explicit vendor / transport / model separation.
- Shared ObsMem and operational/control-plane surfaces are advanced as real, reviewable substrate layers.

## What's New In Detail

### Trace v1 and execution truth
- ADL introduces a more explicit trace substrate so major control decisions can be recorded as structured, reviewable events.
- Trace is treated as execution truth for reconstruction and review, rather than as narrative commentary layered on top of runtime behavior.

### Provider, memory, and operational substrate work
- Provider/transport handling is pushed toward an explicit substrate model with cleaner portability across common providers.
- Shared ObsMem work is advanced as a bounded foundation layer tied to execution truth instead of opaque memory behavior.
- Operational skills and control-plane/tooling work are treated as core substrate surfaces rather than auxiliary convenience scripts.

### Review, docs, and reviewer-facing proof surfaces
- Review output expectations are strengthened around findings, evidence, triggers, and system-level assessment.
- Canonical milestone docs are aligned more tightly to implementation and proof surfaces.
- The milestone demo program is structured around bounded substrate proofs so a reviewer can inspect what is actually being claimed.

## Upgrade Notes
- `v0.87` should be understood as a substrate/consolidation release, not as a new agent-capability release.
- Reviewer-facing proof now depends more explicitly on trace, artifact roots, and structured demo/review surfaces; downstream milestone docs and issue cards should reference these surfaces directly.

## Known Limitations
- Persistent identity, chronosense, and other later `v0.9+` cognitive/personhood surfaces are not part of `v0.87`.
- PR Demo execution, capability-aware routing, and later governance/delegation layers remain intentionally out of scope for this milestone.

## Validation Notes
- Final milestone claims should be backed by the `v0.87` demo matrix, issue output cards, review artifacts, and release-tail validation evidence.
- Determinism in this release is judged primarily by stable structure, schemas, event vocabulary, and proof-surface truth rather than by byte-identical runtime metadata.

## What's Next
- `v0.88` is expected to deepen persistence, instinct, aptitudes, and bounded agency on top of the more credible substrate established here.
- Later milestones will build identity, Freedom Gate evolution, governance, and PR Demo execution on top of the trace/provider/shared-memory/control-plane foundations strengthened in `v0.87`.

## Exit Criteria
- Notes reflect only shipped or demonstrably landed `v0.87` behavior.
- Known limitations and future work are explicitly separated from shipped surfaces.
- Final text is ready to paste into GitHub Release UI without further editing once release date and tag are known.
