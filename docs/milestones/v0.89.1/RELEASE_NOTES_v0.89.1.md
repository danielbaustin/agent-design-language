# Release Notes - v0.89.1

## Metadata
- Product: `ADL`
- Version: `v0.89.1`
- Release date: `TBD`
- Tag: `v0.89.1`

## How To Use
- keep statements implementation-accurate and test-validated
- prefer concise bullets over marketing language
- explicitly separate shipped behavior from `What's Next`

# `ADL` `v0.89.1` Release Notes

## Summary

`v0.89.1` is the milestone where ADL turns adversarial runtime and exploit-evidence carry-forward into a first-class package. The current draft release story is that the adversarial/runtime proof band, integration demos, manuscript workflow packet, quality-gate surface, docs-review convergence pass, internal review record, internal-review remediation, and next-milestone planning handoff are ready for third-party review, while external review, accepted-finding remediation, final planning reconciliation, and release ceremony remain open.

## Current Draft Highlights
- explicit adversarial runtime model for contested operation
- red / blue / purple execution architecture
- exploit artifact and replay contracts
- continuous verification and self-attack proof surfaces
- adversarial demo and governed execution substrate
- bounded `arxiv-paper-writer` skill and the initial three-paper manuscript program
- quality-gate, docs-review, and internal-review surfaces that make the release-tail proof story reproducible
- internal-review remediation for signing trust, provider error handling,
  provider endpoint trust, deterministic setup failure classification, and
  closeout truth

## What's New In Detail

### Adversarial runtime core
- contested-runtime assumptions become explicit architecture
- adversarial execution is bounded, posture-aware, and trace-linked

### Exploit and replay evidence
- exploit hypotheses, evidence, classification, replay, mitigation, and promotion become first-class artifact families
- replay is treated as the bridge between discovery, mitigation validation, and regression prevention

### Governed execution substrate
- operational skills substrate and composition surfaces make the adversarial band executable without collapsing into ad hoc prompt choreography

### Publication and manuscript workflow
- a bounded `arxiv-paper-writer` skill uses the Paper Sonata-style manuscript workflow to support serious technical writing with explicit source packets and review packets
- the milestone aims to produce manuscript packets for:
  - What Is ADL?
  - Gödel Agents and ADL
  - Cognitive Spacetime Manifold

## Upgrade Notes
- no migration step is currently required for users of the local ADL CLI examples
- final tag links, release links, and reviewer-finding outcomes remain `TBD` until `WP-17` through `WP-20` complete
- this draft has been rewritten from shipped proof artifacts through `WP-16`, but it is not final release copy until external review, remediation, and ceremony land

## Known Limitations
- this document is still pre-release and must not be treated as a shipped-claims document yet
- delegation/refusal, negotiation, and provider-extension questions remain intentionally bounded so the adversarial/runtime core does not sprawl

## Validation Notes
- final release notes must cite shipped proof surfaces only
- `WP-02` - `WP-13` proof surfaces are available as draft evidence
- `WP-14` owns the quality-gate proof surface: `QUALITY_GATE_v0.89.1.md` and `bash adl/tools/demo_v0891_quality_gate.sh`
- `WP-15` owns the docs-review convergence surface: `DOCS_REVIEW_v0.89.1.md`
- `WP-16` owns the internal review surface: `INTERNAL_REVIEW_v0.89.1.md`
- `#1994` through `#1997` and `#1992` close the internal-review remediation
  issues that must be settled before third-party review
- `#1999` owns the local third-party review handoff surface
- 3rd-party review, remediation, and release-ceremony outputs should be cited before release

## What's Next
- later reasoning, identity, moral-governance, and broader constitutional bands continue after this milestone

## Exit Criteria
- notes reflect only shipped behavior
- known limitations and future work are explicitly separated
- final text is ready to paste into GitHub Release UI without further editing
