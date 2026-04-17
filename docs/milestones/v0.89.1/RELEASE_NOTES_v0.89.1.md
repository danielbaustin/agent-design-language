# Release Notes - v0.89.1

## Metadata
- Product: `ADL`
- Version: `v0.89.1`
- Release date: `2026-04-17`
- Tag: `v0.89.1`
- GitHub Release: `https://github.com/danielbaustin/agent-design-language/releases/tag/v0.89.1`

## How To Use
- keep statements implementation-accurate and test-validated
- prefer concise bullets over marketing language
- explicitly separate shipped behavior from `What's Next`

# `ADL` `v0.89.1` Release Notes

## Summary

`v0.89.1` is the milestone where ADL turns adversarial runtime and exploit-evidence carry-forward into a first-class package. The release story is that the adversarial/runtime proof band, integration demos, manuscript workflow packet, quality-gate surface, docs-review convergence pass, internal review record, internal-review remediation, third-party review, next-milestone planning handoff, and release ceremony are complete.

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
- third-party review completed with no additional P0/P1/P2 findings beyond the
  internal review work already fixed or explicitly deferred

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
- release tag: `v0.89.1`
- release URL: `https://github.com/danielbaustin/agent-design-language/releases/tag/v0.89.1`
- no migration step is required for existing local ADL CLI examples

## Known Limitations
- delegation/refusal, negotiation, and provider-extension questions remain intentionally bounded so the adversarial/runtime core does not sprawl

## Validation Notes
- final release notes cite shipped proof surfaces only
- `WP-02` - `WP-13` proof surfaces are available as draft evidence
- `WP-14` owns the quality-gate proof surface: `QUALITY_GATE_v0.89.1.md` and `bash adl/tools/demo_v0891_quality_gate.sh`
- `WP-15` owns the docs-review convergence surface: `DOCS_REVIEW_v0.89.1.md`
- `WP-16` owns the internal review surface: `INTERNAL_REVIEW_v0.89.1.md`
- `#1994` through `#1997` and `#1992` close the internal-review remediation
  issues that must be settled before third-party review
- `#1999` owns the local third-party review handoff surface
- `WP-17` third-party review closed with no additional P0/P1/P2 findings
- `WP-18` review remediation closed with internal review fixes recorded and F8 deferred to v0.90 maintainability work
- `WP-19` promoted the v0.90 planning package
- `WP-20` completed release ceremony, tag, and GitHub release publication

## What's Next
- later reasoning, identity, moral-governance, and broader constitutional bands continue after this milestone

## Exit Criteria
- notes reflect only shipped behavior
- known limitations and future work are explicitly separated
- final text is ready for GitHub Release publication
