# v0.91.5 ADR Mini-Sprint Candidate Register

## Status

Tracked ADR candidate register for issue `#3782`.

This is a docs-only curation packet. It does not accept, reject, supersede, or
publish architecture decisions. It records which decision surfaces are already
covered by existing ADRs, which candidate ADRs should be refreshed, and which
new decisions should be proposed, deferred, split, or blocked before later
milestone work consumes them.

## Source Evidence

- `docs/architecture/adr/`
- `docs/architecture/adr/CANDIDATE_ADRS.md`
- `docs/milestones/v0.91.5/ADR_PLAN_v0.91.5.md`
- `docs/planning/FEATURE_DOC_PRODUCTION_MINI_SPRINT_v0.91.5.md`
- `docs/planning/V093_V095_MVP_FEATURE_DOC_PLAN_v0.91.5.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.91.5/PRE_V092_BRIDGE_FEATURE_DOC_LEDGER_v0.91.5.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.91.6/`
- `docs/milestones/v0.91.7/`

## Status Vocabulary

- `accepted_existing`: already accepted in `docs/adr/`; do not duplicate.
- `proposed_refresh`: already has a candidate ADR that should be reviewed,
  refreshed, and promoted or deferred by a separate ADR issue.
- `proposed_new`: source evidence supports a new ADR candidate, but no
  acceptance is claimed here.
- `split_required`: too broad for one ADR; create smaller candidates first.
- `deferred`: decision surface exists, but evidence or implementation truth is
  not mature enough for an ADR yet.
- `blocked`: cannot draft safely without named missing evidence or operator
  decision.

## Existing ADR Boundary

Accepted ADRs through `0028` remain authoritative in `docs/adr/`. The
v0.91.4 C-SDLC candidate ADRs `0029` through `0034` exist under
`docs/architecture/adr/` as candidates and should not be treated as accepted
until promoted into `docs/adr/` by a reviewed ADR issue.

## Candidate Register

| Surface | Status | Evidence | Required ADR action |
| --- | --- | --- | --- |
| C-SDLC default software-development lane | `proposed_refresh` | `docs/architecture/adr/0029-c-sdlc-default-software-development-lane.md`, `docs/planning/ADL_FEATURE_LIST.md` | Refresh against v0.91.4/v0.91.5 closeout truth, then promote or defer. |
| Actor standing and shard ownership | `proposed_refresh` | `docs/architecture/adr/0030-software-development-polis-actor-standing-and-shard-ownership.md` | Refresh with current worktree/conductor policy and promote or defer. |
| Multi-agent parallel execution boundary | `proposed_refresh` | `docs/architecture/adr/0031-c-sdlc-multi-agent-parallel-execution-boundary.md`, v0.91.5 multi-agent proof packets | Refresh with v0.91.5 provider/model and multi-agent reliability evidence. |
| Parallel Validation Fabric | `proposed_refresh` | `docs/architecture/adr/0032-parallel-validation-fabric.md`, `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md` | Refresh with validation split and proof-loop reliability boundaries; do not imply completed distributed validation. |
| Merge-readiness and PR gate truth | `proposed_refresh` | `docs/architecture/adr/0033-merge-readiness-and-pr-gate-truth-boundary.md` | Refresh with current `pr.sh` lifecycle, C-SDLC card truth, CI, review, and closeout behavior. |
| Evidence convergence, signed trace, and ObsMem handoff | `proposed_refresh` | `docs/architecture/adr/0034-c-sdlc-evidence-convergence-signed-trace-and-obsmem-handoff.md` | Refresh with v0.91.5 public records, activation, and ObsMem handoff routing. |
| Provider/model reliability and model-role matrix | `proposed_new` | `docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md`, `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md`, OpenRouter and multi-agent proof packets | Draft ADR candidate for provider identity, model-role suitability, Gemma/OpenRouter/local/remote limits, and multi-agent proof boundaries. |
| Public prompt records versus local `.adl` authoring | `proposed_new` | `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md`, public prompt packet evidence, `docs/milestones/v0.91.6/FEATURE_DOCS_v0.91.6.md` | Draft ADR candidate defining editable local authoring, public export, redaction, validation, indexing, and canonical-public-record boundaries. |
| Observability, logging, and OTel boundary | `proposed_new` | `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`, `docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md`, logging observability proof packets | Draft ADR candidate for deterministic event/log contracts, OTel boundary, Observatory consumption, and non-claims about production observability. |
| Reasoning graph direction and `adl.skill.v1` bridge | `split_required` | `docs/milestones/v0.91.5/review/reasoning_graph/REASONING_GRAPH_CURRENT_CONTRACT_3688.md`, `docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md` | Split into a current reasoning-graph contract ADR and a later `adl.skill.v1` / loop-standard ADR after v0.91.7 bridge docs land. |
| Upstream delegation authority | `proposed_new` | `docs/milestones/v0.91.5/review/upstream_delegation/UPSTREAM_DELEGATION_CONTRACT_3689.md`, `docs/milestones/v0.91.5/review/reasoning_graph_upstream_delegation/REASONING_GRAPH_UPSTREAM_DELEGATION_PROOF_3691.md` | Draft ADR candidate for parent responsibility, non-inherited authority, ACC evidence, ACIP delegation intent, and v0.93 governance handoff. |
| v0.92 activation bridge gate | `deferred` | `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md` | Defer until v0.91.6/v0.91.7 execution produces complete, blocked, deferred, or routed evidence. |
| CodeFriend v1 and portable adapter v2 | `deferred` | `docs/planning/V093_V095_MVP_FEATURE_DOC_PLAN_v0.91.5.md`, ADR 0025 | Defer product ADR until adapter v2 proof exists; ADR 0025 remains current product-boundary baseline. |
| Aptitude Atlas boundary | `deferred` | `docs/planning/ADL_FEATURE_LIST.md`, `docs/planning/V093_V095_MVP_FEATURE_DOC_PLAN_v0.91.5.md` | Defer ADR until v0.95 package reconciliation decides whether an ADR is needed; current boundary is evidence consumption only, productization post-v0.95. |

## Recommended Follow-On Order

1. Refresh and promote or defer C-SDLC candidate ADRs `0029` through `0034`.
2. Draft provider/model reliability, public prompt records, logging/OTel, and
   upstream delegation ADR candidates.
3. Split reasoning graph and `adl.skill.v1` ADR work after v0.91.7 bridge docs
   exist.
4. Revisit v0.92 activation, CodeFriend, and Aptitude Atlas only after their
   planned evidence gates land.

## Non-Goals

- Do not accept ADRs in this register.
- Do not duplicate accepted ADRs in `docs/adr/`.
- Do not claim v0.92 readiness.
- Do not replace feature docs with ADR prose.
- Do not implement runtime or tooling behavior.

## Validation Plan

When this register is updated:

- run `git diff --check`
- verify all required issue `#3782` ADR surfaces appear in the register
- scan added lines for host-local paths, secret markers, and local
  authoring-workspace links
- scan for accidental `accepted` wording that promotes proposed ADRs without
  evidence
- run bounded pre-PR review focused on source grounding, duplicate avoidance,
  and status truth

## Current Verdict

The v0.91.5 ADR mini-sprint has enough source truth to proceed as a series of
small docs-only ADR issues. The register keeps the first pass deliberately
conservative: existing candidates are refreshed, new candidates are proposed,
and immature product or activation surfaces are deferred until their evidence
exists.
