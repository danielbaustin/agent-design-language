# Decisions - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Date: `2026-04-14`
- Owner: `Daniel Austin`

## Purpose

Capture milestone-critical scope and packaging decisions for `v0.89.1`.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Treat `v0.89.1` as the explicit adversarial/runtime follow-on milestone rather than a vague carry-forward placeholder. | accepted | `v0.89` already made the carry-forward explicit; leaving it as a floating note would recreate drift. | Keep only local planning docs and no tracked milestone package. | Makes the follow-on band reviewable and seedable. | `#1860` |
| D-02 | Promote the strongest non-empty adversarial/runtime docs into tracked `v0.89.1` feature docs. | accepted | The source corpus is already substantial enough to justify a tracked feature package. | Leave all inputs local-only. | Gives the milestone a real canonical feature set. | `FEATURE_DOCS_v0.89.1.md` |
| D-03 | Keep delegation/refusal, negotiation, proposed skills, and the empty provider/security demo notes as supporting inputs rather than promoted tracked feature commitments. | accepted | They matter, but not all of them are mature enough to overstate as first-line tracked contracts. | Promote every local `v0.89.1` planning input into tracked features. | Keeps the package bounded and honest. | `FEATURE_DOCS_v0.89.1.md` |
| D-04 | Preserve the standard ADL release-tail pattern (`WP-11` - `WP-20`) instead of inventing a special short tail for this milestone. | accepted | The existing milestone cadence has worked and keeps review/release discipline legible. | Compress the tail into a smaller custom pattern. | Makes `v0.89.1` consistent with recent milestones. | `WBS_v0.89.1.md` |
| D-05 | Make the package review-ready and mechanically issueizable before the wave opens. | accepted | The next milestone should open quickly from settled docs, not by rediscovering scope during issue creation. | Leave the package merely "good enough" and rely on later ad hoc clarifications. | Improves fast-start discipline and reduces kickoff drift. | `README.md`, `SPRINT_v0.89.1.md`, `WP_ISSUE_WAVE_v0.89.1.yaml` |
| D-06 | Include a bounded `arxiv-paper-writer` skill and the initial three-paper arXiv program inside `v0.89.1` rather than leaving them as a later backlog-only follow-on. | accepted | The repo already has a strong Paper Sonata manuscript workflow, an existing bounded writing-skill surface, and an explicit three-paper program. Making the publication lane real in `v0.89.1` gives the milestone a stronger proof story and a concrete internal-use skill target. | Keep the writer skill backlog-only, or defer the papers until after the adversarial/runtime band is complete. | Commits `v0.89.1` to both a bounded publication skill and reviewer-legible manuscript outputs for the three-paper slate. | `FEATURE_DOCS_v0.89.1.md`, `WBS_v0.89.1.md`, `DEMO_MATRIX_v0.89.1.md` |
| D-07 | Keep the broad provider-security capabilities extension out of `v0.89.1` while landing bounded provider capability packaging and proof. | accepted | The provider substrate already exposes capability metadata that belongs in the milestone, but provider attestation, trust scoring, network posture, secret lifecycle, sandboxing, and external security demos are not yet authored or tested enough to promote. | Promote `PROVIDER_SECURITY_CAPABILITIES_EXTENSION.md` as a delivered feature, or ignore the provider extension question entirely. | Gives reviewers a machine-readable WP-10 proof hook while preventing under-authored provider-security claims from inflating the release. | `FEATURE_DOCS_v0.89.1.md`, `adl/src/provider_extension_packaging.rs` |

## Open Questions

- How much of the operational-skill substrate should land as code in `v0.89.1` versus remain design-contract work? (Owner: Daniel Austin)
- Which later issue should promote the full provider-security extension once its schema, tests, and demos are authored? (Owner: Daniel Austin)
- Which proof surfaces are sufficient for `v0.89.1` itself versus intentionally deferred to later security/governance bands? (Owner: Daniel Austin)
- Should `v0.89.1` aim to finish all three papers as submission-ready manuscripts, or stop at strong review-ready packets with explicit post-milestone submission cleanup? (Owner: Daniel Austin)

## Exit Criteria

- all milestone-critical scope and packaging decisions are logged with rationale
- promotion and non-promotion decisions are explicit rather than implicit
- open questions have a clear home in the future issue wave
