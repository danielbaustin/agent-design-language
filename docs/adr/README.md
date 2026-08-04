# Architecture Decision Records

This directory contains ADL architecture decision records, grouped by status.

Candidate ADRs live in `docs/architecture/adr/` until they are reviewed and
promoted. ADR 0020 through ADR 0028 were authored as candidates during the
v0.91.2 ADR planning pass and accepted during the v0.91.3 review tail.

The v0.91.7 ADR set is indexed by
`docs/milestones/v0.91.7/review/V0917_ADR_INDEX_4989.md`. ADR 0043 through ADR
0050 are accepted records. ADR 0051 is intentionally retained as a deferred
Chronosense/Memory Palace disposition; it must not be promoted without the
implementation evidence named by that record.

ADR 0052 through ADR 0058 record the architecture decisions accepted by the
v0.91.8 clean-room, Runtime v3, C-SDLC v2, reversible-cutover, and Memory
Palace proof-handoff work.

## Accepted Records

- `0001-determinism.md`
- `0002-signing-ed25519.md`
- `0003-remote-exec-mvp.md`
- `0004-provider-profiles.md`
- `0005-hitl-pause-resume.md`
- `0006-remote-signing-canonicalization.md`
- `0007-obsmem-external-boundary.md`
- `0008-godel-stage-loop-v08.md`
- `0009-bounded-cognitive-system-architecture.md`
- `0010-chronosense-substrate.md`
- `0011-long-lived-agent-runtime.md`
- `0012-runtime-v2-bounded-csm-run.md`
- `0013-runtime-v2-citizen-state-continuity-substrate.md`
- `0014-contract-market-architecture.md`
- `0015-governed-tools-execution-authority-architecture.md`
- `0016-moral-evidence-and-cognitive-being-substrate.md`
- `0017-secure-local-agent-comms-and-a2a-boundary.md`
- `0018-structured-planning-and-review-policy-artifacts.md`
- `0019-theory-of-mind-foundation.md`
- `0020-universal-tool-schema-portable-tool-description-standard.md`
- `0021-adl-capability-contract-runtime-authority-boundary.md`
- `0022-speculative-decoding-deterministic-commit-boundary.md`
- `0023-google-workspace-cms-bridge-canonical-promotion-boundary.md`
- `0024-workflow-guardrails-issue-lifecycle-control-plane.md`
- `0025-codefriend-review-packet-product-boundary.md`
- `0026-repo-visibility-manifest-linkage-layer.md`
- `0027-governed-code-modernization-moderne-openrewrite-lst.md`
- `0028-c-sdlc-tracked-workflow-state-and-signed-trace.md`
- `0029-c-sdlc-default-software-development-lane.md`
- `0032-parallel-validation-fabric.md`
- `0033-merge-readiness-and-pr-gate-truth-boundary.md`
- `0035-local-polis-ssm-operations-boundary.md`
- `0036-validation-lane-selector-pvf-test-cost-policy.md`
- `0037-github-csdlc-projection-ownership.md`
- `0038-runtime-integration-soak-boundary.md`
- `0039-cognitive-scheduler-v1-authority-boundary.md`
- `0041-provider-model-suitability-boundary-v2.md`
- `0042-public-prompt-records-publication-boundary.md`
- `0043-adl-platform-cli-binary-taxonomy.md`
- `0044-c-sdlc-operational-coordination-boundary.md`
- `0045-validation-manager-and-fast-slow-proof-boundary.md`
- `0046-repo-native-workflow-transport-boundary.md`
- `0047-repo-binaries-and-warm-cache-validation-boundary.md`
- `0048-runtime-observability-and-otel-boundary.md`
- `0049-runtime-soak2-pre-v092-readiness-boundary.md`
- `0050-scheduler-provider-local-agent-delegation-boundary.md`
- `0052-adl-v2-modular-execution-architecture.md`
- `0053-portable-signed-records-and-external-trust.md`
- `0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md`
- `0055-runtime-v3-unified-redb-state.md`
- `0056-c-sdlc-v2-sole-lifecycle-authority.md`
- `0057-reversible-adl-v2-default-and-rollback.md`
- `0058-memory-palace-context-handoff-architecture.md`

## Deferred Records

- `0051-chronosense-and-memory-palace-adr-disposition.md` consumed by ADR 0058
  for the Memory Palace decision; retained as the deferred-disposition record.
