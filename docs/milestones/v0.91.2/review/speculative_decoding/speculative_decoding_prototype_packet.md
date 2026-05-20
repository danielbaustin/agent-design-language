# Speculative Decoding Prototype Packet

## Status

WP-11 branch-local evaluation packet for `v0.91.2`.

## Decision Question

Is speculative decoding worth continuing for ADL workloads under deterministic
commit semantics?

## Short Answer

Yes, but only in the bounded way proved here.

The current prototype says speculative decoding is worth continuing for
same-family local backends when draft acceptance remains high enough to reduce
target forward passes materially. It is not worth continuing as a generic
cross-family acceleration claim, and tokenizer-mismatch pairings should fail
before any positive acceleration claim is made.

## Proof Surfaces

- `docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_report.json`
- `adl/src/speculative_decoding_prototype.rs`
- `adl/src/bin/demo_v0912_speculative_decoding_prototype.rs`

## What Happened

`WP-11` now includes a deterministic draft/verify/commit prototype that:

- records speculative proposal blocks
- verifies every committed token against the target trajectory
- rejects mismatched draft tokens and replaces them only with target-authority
  fallback tokens
- preserves replay-safe committed output
- classifies worthwhile, not-worthwhile, and non-proving scenarios separately

## Scenario Summary

| Scenario | Result | Why it matters |
| --- | --- | --- |
| `same_family_code_generation` | worthwhile | high acceptance and fewer target passes produce a real speedup without changing commit authority |
| `perfect_long_form_generation` | worthwhile | shows the upper bound if draft quality is near-perfect |
| `adl_card_generation_mixed_quality` | worthwhile | proves ADL-shaped structured output can still benefit even with some verifier fallbacks |
| `poor_draft_short_chat` | not worthwhile | proves the evaluator can detect when draft overhead cancels the benefit |
| `cross_family_tokenizer_mismatch` | non-proving | prevents a false positive claim when tokenizer compatibility is absent |

## Worthiness Conclusion

The current branch-local report supports this bounded conclusion:

- continue evaluating speculative decoding for same-family local backends
- do not claim generic cross-family worthiness
- do not claim provider-production speedups from this prototype alone

## Authority Boundary

This prototype does not weaken ADL governance:

- speculative proposal is not authoritative commit
- rejected draft tokens are discarded
- target verification remains the commit authority for output tokens
- Freedom Gate and ACC remain the only authority boundary for side effects

## Non-Claims

- no production backend speedup claim
- no live provider-side speculative decoding claim
- no execution-authority expansion
- no claim that rejected speculative branches are safe to hide or omit from
  audit posture

## Validation

- `cargo test --manifest-path adl/Cargo.toml speculative_decoding_prototype -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml demo_v0912_speculative_decoding_prototype -- --nocapture`
- `cargo run --manifest-path adl/Cargo.toml --bin demo_v0912_speculative_decoding_prototype`

## Residual Risk

This is still a deterministic prototype, not a production backend integration.
The next truthful follow-on would be a same-family local-model backend spike if
we want to test whether the worthiness signal survives contact with real model
runtime costs. That follow-on should be treated as optional future product work,
not as an implied requirement for closing `v0.91.2`, and it may fit better as a
post-`v0.95` capability experiment unless roadmap priorities change.
