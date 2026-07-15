# Structured Review Prompt

Template: 1.0.0

Issue: 5390

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel
adl-runtime/src/guardian.rs
adl/src/cli/runtime_v3_cmd.rs
demos/v0.91.7/html-observatory
docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md
docs/architecture/RUNTIME_V3_GUARDIAN_AND_SOAK.md
docs/architecture/RUNTIME_V3_GUARDIAN_FALLBACK_DECISION.md
docs/architecture/runtime_v3_guardian_fallback_matrix.v1.json
docs/architecture/runtime_v3_guardian_matrix.v1.json
docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md
infra/horust
infra/runtime-v3
infra/rustysd/adl-runtime-kernel.service
infra/systemd/adl-runtime-kernel.service
adl/tools/check_coverage_impact.sh
adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_run_pr_fast_coverage_lane.sh

## Prompts

- Review TLS trust boundaries, private-key handling, and absence of plain-HTTP production paths.
- Review bound-address propagation and ephemeral-port test truth.
- Review local/remote Observatory compatibility and release non-claims.

## Findings

[
  {
    "id": "R-5390-1",
    "severity": "p1",
    "summary": "Selector and guardian/service launch contracts retained plaintext or omitted explicit init configuration.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  },
  {
    "id": "R-5390-2",
    "severity": "p2",
    "summary": "Local Observatory instructions, TLS failure proof, and graceful TLS shutdown were incomplete.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  },
  {
    "id": "R-5390-3",
    "severity": "p2",
    "summary": "Guardian duplicated endpoint state and readiness preceded server-owned listening truth.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  },
  {
    "id": "R-5390-4",
    "severity": "p2",
    "summary": "Guardian wire schema and retained machine-readable guardian contracts did not disclose changed or superseded truth.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  },
  {
    "id": "R-5390-5",
    "severity": "p1",
    "summary": "Runtime v3 guardian and selector changes lacked manifest-correct PR-fast coverage mappings, and the first combined mapping sent an adl-runtime binary selector to the adl workspace.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  },
  {
    "id": "R-5390-6",
    "severity": "p1",
    "summary": "The Runtime v3 selector report path remained below the required changed-source coverage threshold.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  },
  {
    "id": "R-5390-7",
    "severity": "p2",
    "summary": "Selector output-path tests initially raised coverage without asserting meaningful text or JSON output.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Operators must provision browser-trusted certificates and private keys; no private key or trust-store mutation is retained in the repository.
- Environment-dependent Horust, systemd, remote, and GPU tests remain outside this local proof; retained Horust qualification remains explicitly blocked by upstream restart-budget behavior.

## Review Result

Revision: Some("git-blake3:699da9a4c5ea9643e64da599bf06f5d1b7286040:9415c94595c369c529d50dd3556148dc5b7528c584dbe2adc49d0ae8bd978ea7")

Reviewer: Some("subagents-019f66d7-019f66f8-and-019f671d")

Result: pass
