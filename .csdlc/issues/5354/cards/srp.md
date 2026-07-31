# Structured Review Prompt

Template: 1.0.0

Issue: 5354

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/5354
.csdlc/issues/5354
.csdlc/prepared/issues/5354
adl-v2/Cargo.lock
docs/milestones/v0.91.8/DEMO_MATRIX_v0.91.8.md
docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md

## Prompts

- Can any fixture, screenshot, prose, metadata row, product-local test, or partial lifecycle path be mistaken for the required integrated deployed proof?
- Is #5384 merge, typed closed_out, claim release, retained receipt, and ancestry gating exact and impossible to bypass?
- Does the scenario compose accepted ADL v2, Runtime v3, and C-SDLC v2 interfaces without introducing a duplicate orchestrator or lifecycle authority?
- Are evidence identity, redaction, network/address configuration, public claim boundaries, and negative tests complete and fail-closed?
- Are COTS, protected-path amendment, LoC/module/assertion/time budgets, PVF, no-deferral, exact review, CI, serialized merge, and post-merge contracts complete?

## Findings

[
  {
    "id": "F-5354-4",
    "severity": "p2",
    "summary": "The WSS proof treated ADL_RUNTIME_V3_CA_CERT as an exact peer leaf certificate instead of a CA bundle, so valid CA-signed Runtime v3 deployments could pass curl and fail the Ruby WSS proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:bb537e75ad2360b8ac379feb7edcc3dcf83b9aae:c30b7f383b724c3cec1f49a49f61350ce6927e3984bbadf146f266a67edb9ec1",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The convergence proof remains scoped to the local live Runtime v3 instance and accepted Unity evidence; it does not claim player-build, Runtime v2, cloud deployment, or whole-release completion.

## Review Result

Revision: Some("git-blake3:bb537e75ad2360b8ac379feb7edcc3dcf83b9aae:c30b7f383b724c3cec1f49a49f61350ce6927e3984bbadf146f266a67edb9ec1")

Reviewer: Some("codex:gpt-5.5-exact-head-review")

Result: pass
