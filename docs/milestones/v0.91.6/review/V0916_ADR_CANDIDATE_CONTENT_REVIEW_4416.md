# v0.91.6 ADR Candidate Content Review

Issue: `#4416`
Reviewed sprint: `#4324`
Candidate range: `0035` through `0042`
Status: `candidate_content_reviewed`

## Review Rules

- Do not promote candidate ADRs to accepted ADRs.
- Preserve the release-tail boundary: this review checks content quality,
  evidence linkage, and non-claim discipline only.
- Treat missing implementation proof as a routing note, not as hidden acceptance.

## Review Summary

The candidate ADR packet is sound enough to retain as a release-tail routing
surface. Each candidate remains labeled as candidate material and the packet
does not silently accept new decisions. The main content requirement is to keep
promotion gates explicit, especially where the candidate describes a boundary
that is only partly implemented in v0.91.6.

## Candidate Review Matrix

| Candidate | Review result | Notes |
| --- | --- | --- |
| ADR 0035 Local Polis SSM Operations Boundary | `pass_with_non_claims` | Correctly places SSM in the operations plane and excludes polis state, governance, memory, identity, provider selection, model content, and runtime semantics from SSM authority. Promotion should verify current SSM proof packets remain operations-only. |
| ADR 0036 Validation Lane Selector / PVF Test-Cost Policy | `pass_with_promotion_gate` | Correctly frames focused validation as better validation, not less validation. Promotion should verify the selector manifests and runners are current, because PVF/VPP work continues to evolve. |
| ADR 0037 GitHub/C-SDLC Projection Ownership | `pass_with_deferred_residue` | Correctly separates managed projection, drift-checked projection, linked external state, card-local truth, and deferred surfaces. Promotion should cite the remaining legacy closing-linkage disposal or keep it explicit as deferred residue. |
| ADR 0038 Runtime Integration Soak Boundary | `pass_with_scope_boundary` | Correctly rejects component-level proof as runtime-coherence proof. Promotion should avoid claiming Soak #2 or full v0.92 readiness until the later integration sprint proves it. |
| ADR 0039 Cognitive Scheduler v1 Authority Boundary | `pass_with_authority_boundary` | Correctly keeps Scheduler v1 as planning/evidence, not timed automation, provider selection, GitHub mutation, or sprint-conductor replacement. Promotion should preserve this non-authority boundary. |
| ADR 0040 Workflow Lockfile Discipline | `candidate_required_not_acceptance_ready` | Correctly records the evidence gap and should not be promoted until the lockfile fix has a durable source packet or exact tracked proof. This caveat is essential and should remain visible. |
| ADR 0041 Provider/Model Suitability Boundary v2 | `pass_with_compatibility_requirement` | Correctly distinguishes provider reachability, capability, suitability, reliability, failure behavior, and advisory authority. Promotion should explicitly preserve ADR 0004 rather than replacing provider-profile semantics. |
| ADR 0042 Public Prompt Records Publication Boundary | `pass_with_publication_boundary` | Correctly treats public prompt records as reviewed projections, not raw `.adl` publication. Promotion should verify export, redaction, validation, indexing, security/CAV, and distribution closeout remain mandatory gates. |

## Cross-Cutting Findings

- No candidate should be promoted by `#4324` or `#4416`; all remain proposed
  records.
- ADR 0040 is intentionally weaker than the others because it contains an
  unresolved evidence-capture requirement. That is acceptable only because the
  status and validation notes make the gap explicit.
- The candidate set should be consumed as release-tail architecture routing, not
  proof that the related runtime/provider/scheduler/SSM/public-record behavior is
  complete.

## Required Follow-Up Boundary

Promotion of any candidate must happen through a later explicit ADR acceptance
issue that:

- names the candidate;
- verifies the evidence paths still exist;
- resolves or accepts the candidate's promotion gate;
- updates `docs/adr/README.md` only when the ADR is actually accepted;
- keeps candidate history visible under `docs/architecture/adr/`.
