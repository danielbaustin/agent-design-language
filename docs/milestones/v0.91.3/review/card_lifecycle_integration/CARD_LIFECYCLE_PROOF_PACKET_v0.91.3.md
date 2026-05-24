# Card Lifecycle Proof Packet v0.91.3

## Scope

`WP-03` proves one bounded claim: the canonical C-SDLC card lifecycle can be
represented in a tracked public issue-local bundle and accepted by both the
structured-prompt validator and the doctor lifecycle classifier.

## Proof Bundle

- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/README.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md`
- `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md`

## Expected Result

- the tracked bundle validates directly
- doctor recognizes the canonical order
- doctor reports final review/output truth for the tracked bundle
- no local-only `.adl` path is required to inspect the proof

## Focused Validation

```bash
bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md
bash adl/tools/validate_structured_prompt.sh --type stp --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md
bash adl/tools/validate_structured_prompt.sh --type spp --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md
bash adl/tools/validate_structured_prompt.sh --type srp --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md
bash adl/tools/validate_structured_prompt.sh --type sor --phase final --input docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md
cargo test --manifest-path adl/Cargo.toml tracked_csdlc_card_bundle -- --nocapture
cargo test --manifest-path adl/Cargo.toml card_lifecycle_accepts_tracked_csdlc_bundle -- --nocapture
```

## Non-Claims

- This packet does not claim default-operation lifecycle rollout.
- This packet does not claim full transition substrate completion.
