# v0.91.8 Feature Preservation Crosswalk

Source: `docs/planning/ADL_FEATURE_LIST.md`

Pinned feature rows: 122

Normalized row digest: `89a94d430a99cae0b529f5447587af41bda9903cecf1e3ebbdc25542d733ed9e`

This crosswalk prevents a canonical feature from disappearing during ADL v2
or Runtime v3 cutover. It is a planning disposition, not implementation proof.
Every source row has a source-line-pinned, human-reviewable decision in
`.csdlc/prepared/issues/5594/feature_decisions_5594.rb`; no keyword or
first-match heuristic assigns ownership.
The retained row-by-row artifact is
[feature_preservation_crosswalk_5594.v1.json](feature_preservation_crosswalk_5594.v1.json);
it records canonical status/evidence/target, owner issues, and an explicit
cutover disposition and decision basis for all 122 real feature rows. The
matrix header is explicitly excluded.

## Decision Groups

| Class | Owner | Decision boundary | Required terminal disposition |
| --- | --- | --- | --- |
| `kernel_continuity_ingress` | #5591 | Runtime execution, lifecycle, continuity, replay, and canonical ingress | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `reasoning_adaptive_cognition` | #5592 | Reasoning, memory, cognition, affect, and adaptive behavior | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `governed_operations` | #5589 | Governance, identity, provider, state, time, tool, and operational services | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `secure_access_observatory` | #5590 | Secure access, communications, telemetry, guardian, and Observatory behavior | Runtime v3 implementation or explicit accepted non-runtime/defer disposition |
| `csdlc_v2_acceptance` | #5358 | C-SDLC authoring, review, validation, quality, and control-plane capabilities | Exact C-SDLC v2 acceptance or explicit residual blocker |
| `adl_v2_signing` | #5342 | Signing, verification, and trust-policy replacement | Exact ADL v2 WP-07 signing acceptance or explicit residual blocker |
| `provider_and_secure_transport` | #5589 and #5590 | Provider operations plus secure transport and remote access | Both Runtime Parity-C and Parity-D proof, or an explicit accepted disposition |
| `retained_or_later_milestone` | #5347 | Retained evidence, product/demo surfaces, and canonical later-milestone work | Preserve through deletion eligibility and defer to the canonical target |

The group definitions do not classify rows. The explicit source-line decision
table does. Any new, removed, or reordered feature row therefore fails closed
until a reviewer assigns that exact row to a decision group.

## Gate

The validator fails if:

- the source row count or digest changes;
- a row has empty feature, status, evidence, or next-target fields;
- feature names are duplicated;
- the explicit decision table does not exactly cover the source lines;
- any class lacks a named issue owner;
- the retained per-row artifact differs from the canonical row, explicit
  class, owner, disposition, or decision basis.

WP-02 `#5336` may deliberately revise the pinned baseline and classification
rules, but that change requires review. Runtime v2 deletion remains forbidden
until #5591/#5592/#5589/#5590 and #5347 consume the resulting per-row
dispositions at exact revisions.

WP-21 `#5362` consumes this preservation crosswalk together with the closed
WP-21 child-track handoff inputs #5352, #4758, #4759, #4760, #4761, #4762,
#4763, #5007, and #5107. That consumption preserves feature-list disposition
truth only; it does not start v0.92 implementation or convert retained handoff
evidence into birthday, launch, or Adaptive Learning runtime claims.
