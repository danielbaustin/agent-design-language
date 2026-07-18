# Structured Review Prompt

Template: 1.0.0

Issue: 5412

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/identity_memory.rs
adl-runtime-kernel/src/private_state.rs
adl-runtime-kernel/tests/identity_memory.rs
adl-runtime-kernel/tests/private_state.rs
adl/tools/run_runtime_v3_guardian_soak.sh
adl/tools/test_run_runtime_v3_guardian_soak.sh
adl/tools/report_runtime_v3_loc.sh
docs/architecture/RUNTIME_V3_STATE_AUTHENTICITY_5412.md
docs/architecture/runtime_v3_state_authenticity_5412.v1.json
.github/workflows/ci.yaml
adl/tools/test_run_aws_spot_ci_profile.sh

## Prompts

- Can any checkpoint field be altered without signature failure?
- Can a validly signed but unaccepted or wrong-lineage record be projected?
- Does the scheduled lane execute the real ignored soak rather than a retained packet assertion?
- Is the LoC disposition reproducible, owned, and bounded?

## Findings

[
  {
    "id": "F-5412-REVIEW-1",
    "severity": "p2",
    "summary": "Signed checkpoint fields changed the wire contract without advancing the checkpoint schema or defining legacy-v1 behavior.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6318706c380b4af8a65ae8792d3baf999113a669:717ff72f6f4f45540dc237e6d4cea012328edea31b693d46e0eef19a048ecd28",
    "route": null
  },
  {
    "id": "F-5412-REVIEW-2",
    "severity": "p2",
    "summary": "The soak wrapper could accept stale or semantically invalid report evidence after a successful command exit.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:6318706c380b4af8a65ae8792d3baf999113a669:717ff72f6f4f45540dc237e6d4cea012328edea31b693d46e0eef19a048ecd28",
    "route": null
  },
  {
    "id": "F-5412-FINAL-1",
    "severity": "p2",
    "summary": "The backend snapshot contract assertions are unreachable behind a stale pre-existing assertion.",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "https://github.com/danielbaustin/agent-design-language/issues/5467"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Runtime v3 remains at 12,034 physical Rust source lines under the explicit 20K exception; v0.91.8 owns the further reduction.

## Review Result

Revision: Some("git-blake3:6318706c380b4af8a65ae8792d3baf999113a669:717ff72f6f4f45540dc237e6d4cea012328edea31b693d46e0eef19a048ecd28")

Reviewer: Some("bounded-subagent-review-5412-final")

Result: pass
